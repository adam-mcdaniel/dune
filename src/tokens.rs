use std::fmt;

use nom::{InputLength, InputTake};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
}

impl<'a> Token<'a> {
    #[inline]
    pub fn new(kind: TokenKind, text: &'a str) -> Self {
        Token { kind, text }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Punctuation,
    Operator,
    Keyword,
    StringLiteral,
    IntegerLiteral,
    FloatLiteral,
    BooleanLiteral,
    Symbol,
    Whitespace,
    Comment,
    Other,
}

impl fmt::Debug for Token<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({})", self.kind, self.text)
    }
}

impl fmt::Display for Token<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tokens<'a>(pub &'a [Token<'a>]);


const MAX_TOKENS_TO_DISPLAY: usize = 10;
impl fmt::Display for Tokens<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for t in self.0.iter().take(MAX_TOKENS_TO_DISPLAY) {
            write!(f, "{} ", t)?;
        }
        Ok(())
    }
}

impl<'a> std::ops::Deref for Tokens<'a> {
    type Target = [Token<'a>];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl InputLength for Tokens<'_> {
    #[inline]
    fn input_len(&self) -> usize {
        self.0.len()
    }
}

impl InputTake for Tokens<'_> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        Tokens(&self.0[count..])
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (a, b) = self.0.split_at(count);
        (Tokens(a), Tokens(b))
    }
}

impl<'a> Tokens<'a> {
    #[inline]
    pub fn skip_n(self, count: usize) -> Self {
        Tokens(&self.0[count..])
    }
}
