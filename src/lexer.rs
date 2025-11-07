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
        let mut tokens = Vec::new();
        let mut chars = self.source.chars().peekable();
        let mut line = 1;
        let mut column = 1;

        while let Some(&ch) = chars.peek() {
            let start_line = line;
            let start_column = column;

            // Skip whitespace (or emit as trivia if needed)
            if ch.is_whitespace() {
                if ch == '\n' {
                    line += 1;
                    column = 1;
                } else {
                    column += 1;
                }
                chars.next();
                continue;
            }

            // Handle comments
            if ch == '/' {
                let next = chars.clone().nth(1);
                if next == Some('/') {
                    // Single-line comment
                    let mut comment = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        comment.push(c);
                        chars.next();
                        column += 1;
                    }
                    tokens.push(SpannedToken {
                        value: Token::SingleLineCommentTrivia,
                        line: start_line,
                        column: start_column,
                    });
                    continue;
                } else if next == Some('*') {
                    // Multi-line comment
                    chars.next(); // consume '/'
                    chars.next(); // consume '*'
                    column += 2;
                    let mut depth = 1;
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            line += 1;
                            column = 1;
                        } else {
                            column += 1;
                        }
                        if c == '*' && chars.clone().nth(1) == Some('/') {
                            chars.next(); // consume '*'
                            chars.next(); // consume '/'
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        } else if c == '/' && chars.clone().nth(1) == Some('*') {
                            chars.next(); // consume '/'
                            chars.next(); // consume '*'
                            depth += 1;
                        } else {
                            chars.next();
                        }
                    }
                    tokens.push(SpannedToken {
                        value: Token::MultiLineCommentTrivia,
                        line: start_line,
                        column: start_column,
                    });
                    continue;
                }
            }

            // Handle string literals
            if ch == '"' || ch == '\'' {
                let quote = ch;
                chars.next();
                column += 1;
                let mut value = String::new();
                let mut escaped = false;

                while let Some(&c) = chars.peek() {
                    if escaped {
                        match c {
                            'n' => value.push('\n'),
                            't' => value.push('\t'),
                            'r' => value.push('\r'),
                            '\\' => value.push('\\'),
                            '"' => value.push('"'),
                            '\'' => value.push('\''),
                            _ => value.push(c),
                        }
                        escaped = false;
                        chars.next();
                        column += 1;
                    } else if c == '\\' {
                        escaped = true;
                        chars.next();
                        column += 1;
                    } else if c == quote {
                        chars.next();
                        column += 1;
                        break;
                    } else {
                        if c == '\n' {
                            line += 1;
                            column = 1;
                        } else {
                            column += 1;
                        }
                        value.push(c);
                        chars.next();
                    }
                }

                tokens.push(SpannedToken {
                    value: Token::StringLiteral(value),
                    line: start_line,
                    column: start_column,
                });
                continue;
            }

            // Handle numeric literals
            if ch.is_ascii_digit() {
                let mut num_str = String::new();
                let mut has_dot = false;
                let mut is_bigint = false;

                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num_str.push(c);
                        chars.next();
                        column += 1;
                    } else if c == '.' && !has_dot {
                        num_str.push(c);
                        chars.next();
                        column += 1;
                        has_dot = true;
                    } else if c == 'n' && !has_dot {
                        // BigInt literal
                        is_bigint = true;
                        chars.next();
                        column += 1;
                        break;
                    } else {
                        break;
                    }
                }

                let token = if is_bigint {
                    Token::BigIntLiteral(num_str)
                } else {
                    Token::NumericLiteral(num_str)
                };
                tokens.push(SpannedToken {
                    value: token,
                    line: start_line,
                    column: start_column,
                });
                continue;
            }

            // Handle identifiers and keywords
            if ch.is_ascii_alphabetic() || ch == '_' || ch == '$' {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
                        ident.push(c);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                // Check if it's a keyword
                let token = crate::token::find_match(&ident)
                    .unwrap_or_else(|| Token::Identifier(ident));
                tokens.push(SpannedToken {
                    value: token,
                    line: start_line,
                    column: start_column,
                });
                continue;
            }

            // Handle operators and punctuation (try longest match first)
            let mut matched = false;
            // Build up potential operator strings (up to 3 chars)
            let mut op_chars = Vec::new();
            let mut peek_iter = chars.clone();
            for _ in 0..3 {
                if let Some(&c) = peek_iter.peek() {
                    op_chars.push(c);
                    peek_iter.next();
                } else {
                    break;
                }
            }

            // Try matching from longest to shortest
            for len in (1..=op_chars.len()).rev() {
                let op_str: String = op_chars[..len].iter().collect();
                if let Some(token) = crate::token::find_match(&op_str) {
                    tokens.push(SpannedToken {
                        value: token,
                        line: start_line,
                        column: start_column,
                    });
                    // Consume the matched characters
                    for _ in 0..len {
                        if let Some(c) = chars.next() {
                            if c == '\n' {
                                line += 1;
                                column = 1;
                            } else {
                                column += 1;
                            }
                        }
                    }
                    matched = true;
                    break;
                }
            }

            if !matched {
                // Unknown character
                tokens.push(SpannedToken {
                    value: Token::Illegal,
                    line: start_line,
                    column: start_column,
                });
                chars.next();
                column += 1;
            }
        }

        // Add EOF token
        tokens.push(SpannedToken {
            value: Token::Eof,
            line,
            column,
        });

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<SpannedToken> {
        Lexer::new(input).lex()
    }

    #[test]
    fn lexes_identifier() {
        let tokens = lex("foo");

        assert_eq!(
            tokens,
            vec![
                SpannedToken {
                    value: Token::Identifier("foo".into()),
                    line: 1,
                    column: 1,
                },
                SpannedToken {
                    value: Token::Eof,
                    line: 1,
                    column: 4,
                },
            ],
        );
    }

    #[test]
    fn lexes_bigint_literal() {
        let tokens = lex("123n");

        assert_eq!(
            tokens,
            vec![
                SpannedToken {
                    value: Token::BigIntLiteral("123".into()),
                    line: 1,
                    column: 1,
                },
                SpannedToken {
                    value: Token::Eof,
                    line: 1,
                    column: 5,
                },
            ],
        );
    }

    #[test]
    fn skips_whitespace_and_tracks_position() {
        let tokens = lex(" \nfoo");

        assert_eq!(
            tokens,
            vec![
                SpannedToken {
                    value: Token::Identifier("foo".into()),
                    line: 2,
                    column: 1,
                },
                SpannedToken {
                    value: Token::Eof,
                    line: 2,
                    column: 4,
                },
            ],
        );
    }
}

