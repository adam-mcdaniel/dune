use super::{CommentMatch, Start, End, find_comments_impl, strip_comments, CommentStyle};

pub fn strip(script: impl ToString) -> Result<String, &'static str> {
    strip_comments(script.to_string(), CommentStyle::C, false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    Start,
    Normal,
    FirstSlash,
    SingleLineComment,
    MultiLineComment,
    MultiLineCommentFinalStar,
    MultiLineCommentFinalSlash,
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
    CommentMightStart,
    CommentConfirmed,
    CommentDismissed,
    CommentEnds,
    CommentEndsAndCommentMightStart
}

fn state_transition(from: ParseState, current_char: Option<char>) -> (ParseState, ParseAction) {
    match current_char {
        Some(c) => match from {
            ParseState::Start => match c {
                '/'     => (ParseState::FirstSlash, ParseAction::CommentMightStart),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::Normal => match c {
                '/'     => (ParseState::FirstSlash, ParseAction::CommentMightStart),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing) 
            },
            ParseState::FirstSlash => match c {
                '/'     => (ParseState::SingleLineComment, ParseAction::CommentConfirmed),
                '*'     => (ParseState::MultiLineComment, ParseAction::CommentConfirmed),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::CommentDismissed),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::CommentDismissed),
                _       => (ParseState::Normal, ParseAction::CommentDismissed)
            },
            ParseState::SingleLineComment => match c {
                '\n'    => (ParseState::Normal, ParseAction::CommentEnds),
                _       => (ParseState::SingleLineComment, ParseAction::Nothing)   
            },
            ParseState::MultiLineComment => match c {
                '*'     => (ParseState::MultiLineCommentFinalStar, ParseAction::Nothing),
                _       => (ParseState::MultiLineComment, ParseAction::Nothing)  
            },
            ParseState::MultiLineCommentFinalStar => match c {
                '/'     => (ParseState::MultiLineCommentFinalSlash, ParseAction::Nothing),
                '*'     => (ParseState::MultiLineCommentFinalStar, ParseAction::Nothing),
                _       => (ParseState::MultiLineComment, ParseAction::Nothing)   
            },
            ParseState::MultiLineCommentFinalSlash => match c {
                '/'     => (ParseState::FirstSlash, ParseAction::CommentEndsAndCommentMightStart),
                _       => (ParseState::Normal, ParseAction::CommentEnds)
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
            ParseState::FirstSlash => 
                (ParseState::End, ParseAction::CommentDismissed),
            ParseState::SingleLineComment => 
                (ParseState::End, ParseAction::CommentEnds),
            ParseState::MultiLineComment => 
                (ParseState::End, ParseAction::CommentDismissed),
            ParseState::MultiLineCommentFinalStar => 
                (ParseState::End, ParseAction::CommentDismissed),
            ParseState::MultiLineCommentFinalSlash => 
                (ParseState::End, ParseAction::CommentEnds),
            _ => 
                (ParseState::End , ParseAction::Nothing)
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
        ParseAction::CommentMightStart => {
            comment_state = CommentState::MaybeInComment(position);
        },
        ParseAction::CommentConfirmed => {
            match comment_state {
                CommentState::MaybeInComment(from) => {
                    comment_state = CommentState::InComment(from);
                },
                _ => {
                    // println!("{:?}", (&comment_state, &current_state, &next_state, &current_char, &position));
                    return Err("c style parser error");
                }

            }
        },
        ParseAction::CommentDismissed => {
            comment_state = CommentState::NotInComment
        },
        ParseAction::CommentEnds => {
            match comment_state {
                CommentState::InComment(from) => {
                    matches.push(CommentMatch{from: from, to: position});
                    comment_state = CommentState::NotInComment;
                },
                _ => {
                    return Err("c style parser error");
                }
            }
        },
        ParseAction::CommentEndsAndCommentMightStart => {
            match comment_state {
                CommentState::InComment(from) => {
                    matches.push(CommentMatch{from: from, to: position});
                    comment_state = CommentState::MaybeInComment(position);
                },
                _ => {
                    return Err("c style parser error");
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
        let input = "int main() {};";
        let expected = Ok(Vec::new());
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_comment() {
        let input = "int main() { /* comment */ }";
        let expected = Ok(vec![
            CommentMatch { from: 13, to: 26}
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple_comments() {
        let input = "main()/* comment */\n/* comment */";
        let expected = Ok(vec![
            CommentMatch { from: 6, to: 19 },
            CommentMatch { from: 20, to: 33 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn line_comment() {
        let input = "main() // comment\n";
        let expected = Ok(vec![
            CommentMatch { from: 7, to: 17 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }
    #[test]
    fn line_comment_no_newline() {
        let input = "main() // comment";
        let expected = Ok(vec![
            CommentMatch { from: 7, to: 17 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiline_comment() {
        let input = "/* multi \nline\ncomment */";
        let expected = Ok(vec![
            CommentMatch { from: 0, to: 25 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_comment_in_string() {
        let input = "printf(\"//no comment /* no comment */\")";
        let expected = Ok(Vec::new());
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }
}
