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

            // Handle whitespace and newlines (emit as trivia)
            if ch.is_whitespace() {
                if ch == '\n' {
                    // Emit newline trivia
                    tokens.push(SpannedToken {
                        value: Token::NewLineTrivia,
                        line: start_line,
                        column: start_column,
                    });
                    chars.next();
                    line += 1;
                    column = 1;
                    continue;
                } else {
                    // Collect consecutive whitespace (spaces, tabs, etc.)
                    let mut whitespace = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '\n' || !c.is_whitespace() {
                            break;
                        }
                        whitespace.push(c);
                        chars.next();
                        column += 1;
                    }
                    tokens.push(SpannedToken {
                        value: Token::WhitespaceTrivia(whitespace),
                        line: start_line,
                        column: start_column,
                    });
                    continue;
                }
            }

            // Handle comments
            if ch == '/' {
                let next = chars.clone().nth(1);
                if next == Some('/') {
                    // Single-line comment - store full comment including "//" marker
                    let mut comment = String::from("//");
                    chars.next(); // consume first '/'
                    chars.next(); // consume second '/'
                    column += 2;
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        comment.push(c);
                        chars.next();
                        column += 1;
                    }
                    tokens.push(SpannedToken {
                        value: Token::SingleLineCommentTrivia(comment),
                        line: start_line,
                        column: start_column,
                    });
                    continue;
                } else if next == Some('*') {
                    // Multi-line comment - store full comment including "/* */" markers
                    let mut comment = String::from("/*");
                    chars.next(); // consume '/'
                    chars.next(); // consume '*'
                    column += 2;
                    let mut depth = 1;
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            line += 1;
                            column = 1;
                            comment.push(c);
                            chars.next();
                        } else {
                            column += 1;
                            if c == '*' && chars.clone().nth(1) == Some('/') {
                                comment.push('*');
                                comment.push('/');
                                chars.next(); // consume '*'
                                chars.next(); // consume '/'
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            } else if c == '/' && chars.clone().nth(1) == Some('*') {
                                comment.push('/');
                                comment.push('*');
                                chars.next(); // consume '/'
                                chars.next(); // consume '*'
                                depth += 1;
                            } else {
                                comment.push(c);
                                chars.next();
                            }
                        }
                    }
                    tokens.push(SpannedToken {
                        value: Token::MultiLineCommentTrivia(comment),
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
                let token =
                    crate::token::find_match(&ident).unwrap_or_else(|| Token::Identifier(ident));
                tokens.push(SpannedToken {
                    value: token,
                    line: start_line,
                    column: start_column,
                });
                continue;
            }

            // Handle operators and punctuation (try longest match first)
            let mut matched = false;
            // Build up potential operator strings (up to 4 chars)
            let mut op_chars = Vec::new();
            let mut peek_iter = chars.clone();
            for _ in 0..4 {
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
    use crate::token::{Token, tokens_to_source};

    fn render(tokens: &[SpannedToken]) -> String {
        tokens_to_source(tokens.iter())
    }

    fn lex(input: &str) -> Vec<SpannedToken> {
        Lexer::new(input).lex()
    }

    #[test]
    fn lexes_identifier() {
        let input = "foo";
        let tokens = lex(input);

        // Verify exact round-trip rendering
        assert_eq!(render(&tokens), input);
    }

    #[test]
    fn lexes_bigint_literal() {
        let input = "123n";
        let tokens = lex(input);

        // Verify exact round-trip rendering
        assert_eq!(render(&tokens), input);
    }

    #[test]
    fn skips_whitespace_and_tracks_position() {
        let input = " \nfoo";
        let tokens = lex(input);

        // Verify exact round-trip rendering
        assert_eq!(render(&tokens), input);

        // Verify tokens include whitespace and newline trivia
        assert_eq!(
            tokens,
            [
                SpannedToken {
                    value: Token::WhitespaceTrivia(" ".into()),
                    line: 1,
                    column: 1
                },
                SpannedToken {
                    value: Token::NewLineTrivia,
                    line: 1,
                    column: 2
                },
                SpannedToken {
                    value: Token::Identifier("foo".into()),
                    line: 2,
                    column: 1
                },
                SpannedToken {
                    value: Token::Eof,
                    line: 2,
                    column: 4
                },
            ]
        );
    }

    #[test]
    fn lexes_equals_token() {
        let input = r#"var x = "string" + 5;"#;
        let tokens = lex(input);

        // Verify exact round-trip rendering
        assert_eq!(render(&tokens), input);
    }

    #[test]
    fn lexes_typescript_imports() {
        let input = r#"
import foo from "bar";
import { baz, qux as quux } from "mod";
import * as ns from "pkg";
import "side-effect";
console.log("hello world");
"#;
        let tokens = lex(input);

        // Verify exact round-trip rendering
        assert_eq!(render(&tokens), input);
    }

    #[test]
    fn test_typescript_with_trivias() {
        let src = r#"
// This is a single-line comment
import foo from "bar"; /* Block comment before import */

import {
    /* multi-line 
       trivia */ baz, 
    // oneline
    qux as quux
} from "mod";

/*
    Another block comment 
*/
import * as ns from "pkg";
// Simple import
import "side-effect";
// After import

// String literal with // inside
let s = "test // not a comment";
/* String with /* nested */ content */

console.log("hello /* not comment */ world"); // Trailing trivia
"#;

        // new Lexer, lex
        let mut lexer = super::Lexer::new(src);
        let tokens: Vec<SpannedToken> = lexer.lex();

        // Verify exact round-trip rendering
        assert_eq!(render(&tokens), src);
    }

    #[test]
    fn lexes_boolean_null_and_undefined_keywords() {
        let input = "true false null undefined";
        let tokens = lex(input);

        assert_eq!(render(&tokens), input);

        assert!(matches!(tokens[0].value, Token::True));
        assert!(matches!(tokens[1].value, Token::WhitespaceTrivia(_)));
        assert!(matches!(tokens[2].value, Token::False));
        assert!(matches!(tokens[3].value, Token::WhitespaceTrivia(_)));
        assert!(matches!(tokens[4].value, Token::Null));
        assert!(matches!(tokens[5].value, Token::WhitespaceTrivia(_)));
        assert!(matches!(tokens[6].value, Token::Undefined));
    }

    #[test]
    fn lexes_basic_jsx_tags() {
        let input = "<div>true</div>";
        let tokens = lex(input);

        assert_eq!(render(&tokens), input);

        assert!(matches!(tokens[0].value, Token::LessThan));
        assert!(matches!(tokens[1].value, Token::Identifier(ref name) if name == "div"));
        assert!(matches!(tokens[2].value, Token::GreaterThan));
        assert!(matches!(tokens[3].value, Token::True));
        assert!(matches!(tokens[4].value, Token::LessThanSlash));
        assert!(matches!(tokens[5].value, Token::Identifier(ref name) if name == "div"));
        assert!(matches!(tokens[6].value, Token::GreaterThan));
    }
}
