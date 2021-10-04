use super::{CommentMatch, Start, End, find_comments_impl, strip_comments, CommentStyle};

pub fn strip(script: impl ToString) -> Result<String, &'static str> {
    strip_comments(script.to_string(), CommentStyle::XML, false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    Start,
    Normal,
    CommentStartBracket,
    CommentStartExcl,
    CommentStartMinus1,
    CommentStartMinus2,
    Comment,
    CommentEndMinus1,
    CommentEndMinus2,
    CommentEndBracket,
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
    CommentOrTagStarts,
    CommentConfirmed,
    CommentDismissed,
    CommentEnds,
    CommentsEndsAndCommentOrTagStarts
}

fn state_transition(from: ParseState, current_char: Option<char>) -> (ParseState, ParseAction) {
    match current_char {
        Some(c) => match from {
            ParseState::Start => match c {
                '<'     => (ParseState::CommentStartBracket, ParseAction::CommentOrTagStarts),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::Normal => match c {
                '<'     => (ParseState::CommentStartBracket, ParseAction::CommentOrTagStarts),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::CommentStartBracket => match c {
                '!'     => (ParseState::CommentStartExcl, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::CommentDismissed)
            },
            ParseState::CommentStartExcl => match c {
                '-'     => (ParseState::CommentStartMinus1, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::CommentDismissed)
            },
            ParseState::CommentStartMinus1 => match c {
                '-'     => (ParseState::CommentStartMinus2, ParseAction::CommentConfirmed),
                _       => (ParseState::Normal, ParseAction::CommentDismissed)
            },
            ParseState::CommentStartMinus2 => match c {
                '-'     => (ParseState::CommentEndMinus1, ParseAction::Nothing),
                _       => (ParseState::Comment, ParseAction::Nothing)
            },
            ParseState::Comment => match c {
                '-'     => (ParseState::CommentEndMinus1, ParseAction::Nothing),
                _       => (ParseState::Comment, ParseAction::Nothing)
            },
            ParseState::CommentEndMinus1 => match c {
                '-'     => (ParseState::CommentEndMinus2, ParseAction::Nothing),
                _       => (ParseState::Comment, ParseAction::Nothing)
            },
            ParseState::CommentEndMinus2 => match c {
                '>'     => (ParseState::CommentEndBracket, ParseAction::Nothing),
                '-'     => (ParseState::CommentEndMinus2, ParseAction::Nothing),
                _       => (ParseState::Comment, ParseAction::Nothing)
            },
            ParseState::CommentEndBracket => match c {
                '<'     => (ParseState::CommentStartBracket, ParseAction::CommentsEndsAndCommentOrTagStarts),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::CommentEnds),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::CommentEnds),
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
            ParseState::CommentStartBracket => (ParseState::End, ParseAction::CommentDismissed),
            ParseState::CommentStartExcl    => (ParseState::End, ParseAction::CommentDismissed),
            ParseState::CommentStartMinus1  => (ParseState::End, ParseAction::CommentDismissed),
            ParseState::CommentStartMinus2  => (ParseState::End, ParseAction::CommentDismissed),
            ParseState::Comment             => (ParseState::End, ParseAction::CommentDismissed),
            ParseState::CommentEndMinus1    => (ParseState::End, ParseAction::CommentDismissed),
            ParseState::CommentEndMinus2    => (ParseState::End, ParseAction::CommentDismissed),
            ParseState::CommentEndBracket   => (ParseState::End, ParseAction::CommentEnds),
            _                               => (ParseState::End, ParseAction::Nothing)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommentState {
    NotInComment,
    InCommentOrTag(usize),
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
        ParseAction::CommentOrTagStarts => {
            comment_state = CommentState::InCommentOrTag(position);
        },
        ParseAction::CommentConfirmed => {
            match comment_state {
                CommentState::InCommentOrTag(from) => {
                    comment_state = CommentState::InComment(from);
                },
                _ => {
                    return Err("xml style parser error");
                }
            }
        },
        ParseAction::CommentDismissed => {
            comment_state = CommentState::NotInComment;
        },
        ParseAction::CommentEnds => {
            match comment_state {
                CommentState::InComment(from) => {
                    matches.push(CommentMatch{from: from, to: position});
                    comment_state = CommentState::NotInComment;
                },
                _ => {
                    return Err("xml style parser error");
                }
            }
        },
        ParseAction::CommentsEndsAndCommentOrTagStarts => {
            match comment_state {
                CommentState::InComment(from) => {
                    matches.push(CommentMatch{from: from, to: position});
                    comment_state = CommentState::InCommentOrTag(position);
                },
                _ => {
                    return Err("xml style parser error");
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
        let input = "<tag attr=\"value\">value</tag>";
        let expected = Ok(Vec::new());
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_comment() {
        let input = "<t /><!-- some comment -->\n<tag />";
        let expected = Ok(vec![
            CommentMatch { from: 5, to: 26}
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple_comments() {
        let input = "<t /><!-- some comment --><t></t><!-- another comment -->";
        let expected = Ok(vec![
            CommentMatch { from: 5, to: 26 },
            CommentMatch { from: 33, to: 57 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn comment_in_tag() {
        let input = "<tag <!-- comment -->></tag>";
        let expected = Ok(vec![
            CommentMatch { from: 5, to: 21 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiline_comment() {
        let input = "<!--\nmulti\nline\ncomment\n-->";
        let expected = Ok(vec![
            CommentMatch { from: 0, to: 27 }
        ]);
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_comment_in_string() {
        let input = "<tag key=\"<!-- -->\" />";
        let expected = Ok(Vec::new());
        let actual = find_comments(input);
        assert_eq!(expected, actual);
    }
}
