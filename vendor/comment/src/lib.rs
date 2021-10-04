#[macro_use] 
extern crate quick_error;

pub mod c;
pub mod atom;
pub mod shell;
pub mod xml;
pub mod blanklines;

use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum AppError {
        Io(err: io::Error) {
            from()
            cause(err)
            description(err.description())
        }
        Other(s: &'static str) {
            from()
            description(s)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentStyle {
    C,
    XML,
    Shell,
    Atom
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommentMatch {
    pub from: usize,
    pub to: usize
}

pub trait Start {
    fn start() -> Self;
}

pub trait End {
    fn end() -> Self;
}

pub fn find_comments_impl<P, A, C, FT, FA>(input: &str, state_transition: FT, do_action: FA) 
    -> Result<Vec<CommentMatch>, &'static str> 
    where   P: Start + End + Copy + Eq,
            A: Copy + Eq,
            C: Start + Copy + Eq,
            FT: Fn(P, Option<char>) -> (P, A),
            FA: Fn(A, C, usize, Vec<CommentMatch>) 
                -> Result<(C, Vec<CommentMatch>), &'static str> {
    let mut matches = Vec::new();
    let mut current_parse_state = P::start();
    let mut current_comment_state = C::start();
    let mut chars = input.chars();
    let mut position = 0;
    while current_parse_state != P::end() {
        let current_char = chars.next();
        let (next_parse_state, action) = 
            state_transition(current_parse_state, current_char);
        let (next_comment_state, next_matches) = 
            do_action(action, current_comment_state, position, matches)?;
        current_parse_state = next_parse_state;
        current_comment_state = next_comment_state;
        matches = next_matches;
        position += 1;
    }
    Ok(matches)
}

fn find_comments(input: &str, style: &CommentStyle) -> Result<Vec<CommentMatch>, &'static str> {
    match style {
        &CommentStyle::C => c::find_comments(input),
        &CommentStyle::Atom => atom::find_comments(input),
        &CommentStyle::Shell => shell::find_comments(input),
        &CommentStyle::XML => xml::find_comments(input)
    }
}

fn remove_matches(input: String, matches: Vec<CommentMatch>) -> Result<String, &'static str> {
    let mut input = input;
    let mut matches = matches;
    matches.sort_by_key(|m| m.from);
    /* must come before reversing */
    check_sorted_matches(input.as_str(), &matches)?;
    matches.reverse();
    for m in matches {
        input.drain((m.from)..(m.to));
    }
    Ok(input.to_owned())
}

fn check_sorted_matches(input: &str, matches: &Vec<CommentMatch>) -> Result<(), &'static str> {
    if matches.iter().any(|m| m.from >= input.len() || m.to > input.len()) {
        return Err("match out of range");
    }
    if matches.iter().zip(matches.iter().skip(1)).any(|(m, n)| m.to > n.from) {
        return Err("matches overlapping");
    }
    Ok(())
}

pub fn strip_comments(data: String, style: CommentStyle, remove_blanks: bool) -> Result<String, &'static str> {
    let comment_matches = find_comments(data.as_str(), &style)?;
    let mut stripped = remove_matches(data, comment_matches)?;
    if remove_blanks {
        let blank_matches = blanklines::find_blanklines(stripped.as_str())?;
        stripped = remove_matches(stripped, blank_matches)?;
    }
    Ok(stripped)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_correctly() {
        let s = "012345#789\n#abcd\nefghi#jkl\n".to_owned();
        let matches = vec![
            CommentMatch{from:6, to:10},
            CommentMatch{from:11, to:16},
            CommentMatch{from:22, to:26}];
        let stripped = remove_matches(s, matches);
        assert_eq!(Ok("012345\n\nefghi\n".to_owned()), stripped);
    }

    #[test]
    fn remove_finds_overlapping() {
        let s = "1234567890".to_owned();
        let matches = vec![
            CommentMatch{from:0, to:5},
            CommentMatch{from:3, to:7}];
        let checked = check_sorted_matches(s.as_str(), &matches);
        assert!(checked.is_err());
        let stripped = remove_matches(s, matches);
        assert!(stripped.is_err());
    }

    #[test]
    fn remove_finds_out_of_range() {
        let s = "12345".to_owned();
        let matches = vec![
            CommentMatch{from:3, to:10},
            CommentMatch{from:11, to:16}];
        let checked = check_sorted_matches(s.as_str(), &matches);
        assert!(checked.is_err());
        let stripped = remove_matches(s, matches);
        assert!(stripped.is_err());
    }

}

pub mod cpp {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::C, false)
    }
}

pub mod rust {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::C, false)
    }
}

pub mod python {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::Shell, false)
    }
}

pub mod html {
    use super::*;
    pub fn strip(script: impl ToString) -> Result<String, &'static str> {
        strip_comments(script.to_string(), CommentStyle::XML, false)
    }
}
