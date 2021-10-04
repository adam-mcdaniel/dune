use super::{CommentMatch, Start, End, find_comments_impl};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    Start,
    Normal,
    SingleBlankline,
    MultiBlankline,
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
    MultiBlanklineStart,
    MultiBlanklineEnd
}

fn state_transition(from: ParseState, current_char: Option<char>) -> (ParseState, ParseAction) {
    match current_char {
        Some(c) => match from {
            ParseState::Start => match c {
                '\n'    => (ParseState::MultiBlankline, ParseAction::MultiBlanklineStart),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::Normal => match c {
                '\n'    => (ParseState::SingleBlankline, ParseAction::Nothing),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::SingleBlankline => match c {
                '\n'    => (ParseState::MultiBlankline, ParseAction::MultiBlanklineStart),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::Nothing),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::Nothing),
                _       => (ParseState::Normal, ParseAction::Nothing)
            },
            ParseState::MultiBlankline => match c {
                '\n'    => (ParseState::MultiBlankline, ParseAction::Nothing),
                '"'     => (ParseState::StringDoubleQuotes, ParseAction::MultiBlanklineEnd),
                '\''    => (ParseState::StringSingleQuotes, ParseAction::MultiBlanklineEnd),
                _       => (ParseState::Normal, ParseAction::MultiBlanklineEnd)
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
            ParseState::MultiBlankline => (ParseState::End, ParseAction::MultiBlanklineEnd),
            _ => (ParseState::End, ParseAction::Nothing)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MultiBlanklineState {
    NotInMultiBlankline,
    InMultiBlankline(usize)
}

impl Start for MultiBlanklineState {
    fn start() -> Self {
        MultiBlanklineState::NotInMultiBlankline
    }
}

fn do_action(action: ParseAction, mut blankline_state: MultiBlanklineState, 
            position: usize, mut matches: Vec<CommentMatch>) 
    -> Result<(MultiBlanklineState, Vec<CommentMatch>), &'static str> {
    match action {
        ParseAction::Nothing => {},
        ParseAction::MultiBlanklineStart => {
            blankline_state = MultiBlanklineState::InMultiBlankline(position);
        },
        ParseAction::MultiBlanklineEnd => {
            match blankline_state {
                MultiBlanklineState::NotInMultiBlankline => {
                    return Err(" blankline parser error");
                },
                MultiBlanklineState::InMultiBlankline(from) => {
                    matches.push(CommentMatch{from: from, to: position});
                    blankline_state = MultiBlanklineState::NotInMultiBlankline;
                }
            }
        }
    }
    Ok((blankline_state, matches))
}

pub fn find_blanklines(input: &str) -> Result<Vec<CommentMatch>, &'static str> {
    find_comments_impl(input, state_transition, do_action)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::CommentMatch;

    #[test]
    fn no_blanklines_present() {
        let input = "yes\n yes no\n";
        let expected = Ok(Vec::new());
        let actual = find_blanklines(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn starts_with_blanklines() {
        let input = "\n\nhello world\n";
        let expected = Ok(vec![
            CommentMatch { from: 0, to: 2}
        ]);
        let actual = find_blanklines(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn normal_blanklines() {
        let input = "hello\n\n\n world\n";
        let expected = Ok(vec![
            CommentMatch { from: 6, to: 8}
        ]);
        let actual = find_blanklines(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn ends_with_blanklines() {
        let input = "hello world\n\n\n";
        let expected = Ok(vec![
            CommentMatch { from: 12, to: 14}
        ]);
        let actual = find_blanklines(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn multiple_blanklines() {
        let input = "\n\nhello\n\n\n\nworld\n\n\n";
        let expected = Ok(vec![
            CommentMatch { from: 0, to: 2},
            CommentMatch { from: 8, to: 11},
            CommentMatch { from: 17, to: 19}
        ]);
        let actual = find_blanklines(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn no_newline_in_string() {
        let input = "\n'string\"inner string\"\n\n\n\n'\n";
        let expected = Ok(vec![
            CommentMatch { from: 0, to: 1 }
        ]);
        let actual = find_blanklines(input);
        assert_eq!(expected, actual);
    }


    
}
