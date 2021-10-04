use super::{CommentMatch, Start, End, find_comments_impl, CommentStyle, strip_comments};

pub fn strip(script: impl ToString) -> Result<String, &'static str> {
    strip_comments(script.to_string(), CommentStyle::Shell, false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    Start,
    Normal,
    ShebangOrComment,
    Shebang,
    Comment,
    StringDoubleQuotes,
    StringDoubleQuotesEscaped,
    StringSingleQuotes,
    StringSingleQuotesEscaped,
    End
}

impl Start for ParseState {
    fn start() -> Self {
        ParseState::Start
    }
}

impl End for ParseState {
    fn end() -> Self {
        ParseState::End
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseAction {
    Nothing,
    CommentStarts,
    CommentEnds,
    ShebangOrCommentStart,
    ShebangFound
}

fn state_transition(from: ParseState, current_char: Option<char>) -> (ParseState, ParseAction) {
    match current_char {
        Some(c) => match from {
            ParseState::Start => match c {
                '#'     => (ParseState::ShebangOrComment, ParseAction::ShebangOrCommentStart),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::Normal => match c {
                '#'     => (ParseState::Comment, ParseAction::CommentStarts),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::ShebangOrComment => match c {
                '!'     => (ParseState::Shebang, ParseAction::ShebangFound),
                _       => (ParseState::Comment, ParseAction::Nothing)
            },
            ParseState::Shebang => match c {
                '\n'    => (ParseState::Normal, ParseAction::Nothing),
                '#'     => (ParseState::Comment, ParseAction::CommentStarts),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Shebang, ParseAction::Nothing)
            },
            ParseState::Comment => match c {
                '\n'    => (ParseState::Normal, ParseAction::CommentEnds),
                _       => (ParseState::Comment, ParseAction::Nothing)
            },
            ParseState::StringDoubleQuotes => match c {
                '"'     => (ParseState::Normal, ParseAction::Nothing),
                '\\'    => (ParseState::StringDoubleQuotesEscaped, ParseAction::Nothing),
                _       => (ParseState::StringDoubleQuotes, ParseAction::Nothing)
            },
            ParseState::StringDoubleQuotesEscaped =>
                (ParseState::StringDoubleQuotes, ParseAction::Nothing),
            ParseState::StringSingleQuotes => match c {
                '\''     => (ParseState::Normal, ParseAction::Nothing),
                '\\'    => (ParseState::StringSingleQuotesEscaped, ParseAction::Nothing),
                _       => (ParseState::StringSingleQuotes, ParseAction::Nothing)
            },
            ParseState::StringSingleQuotesEscaped =>
                (ParseState::StringSingleQuotes, ParseAction::Nothing),
            ParseState::End =>
                (ParseState::End, ParseAction::Nothing)
        },
        None => match from {
            // ..... return if over and comment was finished or not
            ParseState::Comment => (ParseState::End, ParseAction::CommentEnds),
            ParseState::ShebangOrComment => (ParseState::End, ParseAction::ShebangFound),
            _ => (ParseState::End, ParseAction::Nothing)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommentState {
    NotInComment,
    MaybeInComment(usize),
    InComment(usize)
}

impl Start for CommentState {
    fn start() -> Self {
        CommentState::NotInComment
    }
}

fn do_action(action: ParseAction, mut comment_state: CommentState, 
            position: usize, mut matches: Vec<CommentMatch>) 
    -> Result<(CommentState, Vec<CommentMatch>), &'static str> {
    match action {
        ParseAction::Nothing => {},
        ParseAction::CommentStarts => {
            comment_state = CommentState::InComment(position);
        },
        ParseAction::ShebangOrCommentStart =>  {
            comment_state = CommentState::MaybeInComment(position);
        },
        ParseAction::ShebangFound => {
            comment_state = CommentState::NotInComment;
        },
        ParseAction::CommentEnds => {
            match comment_state {
                CommentState::NotInComment => {
                    return Err("shell sytle parse error");
                },
                CommentState::MaybeInComment(from) => {
                    matches.push(CommentMatch{from: from, to: position});
                    comment_state = CommentState::NotInComment;
                },
                CommentState::InComment(from) => {
                    matches.push(CommentMatch{from: from, to: position});
                    comment_state = CommentState::NotInComment;
                }
            }
        }
    }
    Ok((comment_state, matches))
}


pub fn find_comments(input: &str) -> Result<Vec<CommentMatch>, &'static str> {
    find_comments_impl(input, state_transition, do_action)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::CommentMatch;

    #[test]
    fn no_comment_present() {
        let input = "yes\n yes no\n";
        let expected = Ok(Vec::new());
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_comment_but_shebang() {
        let input = "#!/bin/bash\nyes\n yes no\n";
        let expected = Ok(Vec::new());
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_comment() {
        let input = "yes # line comment\n yes no\n";
        let expected = Ok(vec![
            CommentMatch { from: 4, to: 18}
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple_comments() {
        let input = "yes # line comment\n# another comment with \"string\"\n yes no\n";
        let expected = Ok(vec![
            CommentMatch { from: 4, to: 18 },
            CommentMatch { from: 19, to: 50 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn comment_in_shebang() {
        let input = "#!/bin/bash #shebang\nyes\n";
        let expected = Ok(vec![
            CommentMatch { from: 12, to: 20 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_final_newline() {
        let input = "yes #test";
        let expected = Ok(vec![
            CommentMatch { from: 4, to: 9 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_comment_in_string() {
        let input = "yes 'string\"inner string\"' #test\n";
        let expected = Ok(vec![
            CommentMatch { from: 27, to: 32 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }
}
