//! Lexer module — converts source code into tokens.

use crate::token::{SpannedToken, Token};

/// A very small, byte-oriented lexer suitable for ASCII-oriented languages.
/// For simplicity, we treat input as bytes and only support ASCII. You can
/// later switch to a UTF-8 character iterator if you need full Unicode.
pub struct Lexer {
    source: String,
}

impl Lexer {
    /// Create a new lexer from a source string.
    pub fn new<S: Into<String>>(source: S) -> Self { 
        Self {
            source: source.into(),
        }
    }

    pub fn lex(&mut self) -> Vec<SpannedToken> {
        Vec::new()
    }
}
