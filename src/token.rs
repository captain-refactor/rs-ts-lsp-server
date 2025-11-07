use std::borrow::Cow;

/// Basic token definitions used by the lexer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Generic / sentinel
    Illegal,
    Eof,

    // ===== Trivia (scanner can emit/observe; typically skipped by parser) =====
    SingleLineCommentTrivia,
    MultiLineCommentTrivia,
    NewLineTrivia,
    WhitespaceTrivia,
    ShebangTrivia,
    ConflictMarkerTrivia,

    // ===== Identifiers =====
    Identifier(String),
    PrivateIdentifier(String), // e.g. #x

    // ===== Literals =====
    NumericLiteral(String),
    BigIntLiteral(String),
    StringLiteral(String),
    RegularExpressionLiteral(String),

    // Template literals (split into pieces in TS scanning)
    NoSubstitutionTemplateLiteral(String),
    TemplateHead(String),
    TemplateMiddle(String),
    TemplateTail(String),

    // JSX text tokens (only inside JSX)
    JsxText(String),
    JsxTextAllWhiteSpaces(String),

    // ===== Punctuators / Delimiters / Operators =====
    // Delimiters
    Comma,        // ,
    Semicolon,    // ;
    Colon,        // :
    Dot,          // .
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    DotDotDot,    // ...

    // Member access / optional chaining
    QuestionDot, // ?.

    // Relational / equality / arrow
    LessThan,                // <
    LessThanSlash,           // </ (JSX)
    GreaterThan,             // >
    LessThanEquals,          // <=
    GreaterThanEquals,       // >=
    EqualsEquals,            // ==
    ExclamationEquals,       // !=
    EqualsEqualsEquals,      // ===
    ExclamationEqualsEquals, // !==
    EqualsGreaterThan,       // =>

    // Arithmetic / unary / bitwise / logical
    Plus,                              // +
    Minus,                             // -
    Asterisk,                          // *
    AsteriskAsterisk,                  // **
    Slash,                             // /
    Percent,                           // %
    PlusPlus,                          // ++
    MinusMinus,                        // --
    LessThanLessThan,                  // <<
    GreaterThanGreaterThan,            // >>
    GreaterThanGreaterThanGreaterThan, // >>>
    Ampersand,                         // &
    Bar,                               // |
    Caret,                             // ^
    Bang,                              // !
    Tilde,                             // ~
    AmpersandAmpersand,                // &&
    BarBar,                            // ||
    Question,                          // ?
    // ColonToken removed; use Colon above
    At,               // @
    QuestionQuestion, // ??
    Hash,             // # (used e.g. in private names in certain contexts)

    // Assignment
    Equals,                                  // =
    PlusEquals,                              // +=
    MinusEquals,                             // -=
    AsteriskEquals,                          // *=
    AsteriskAsteriskEquals,                  // **=
    SlashEquals,                             // /=
    PercentEquals,                           // %=
    LessThanLessThanEquals,                  // <<=
    GreaterThanGreaterThanEquals,            // >>=
    GreaterThanGreaterThanGreaterThanEquals, // >>>=
    AmpersandEquals,                         // &=
    BarEquals,                               // |=
    CaretEquals,                             // ^=
    BarBarEquals,                            // ||= (logical OR assignment)
    AmpersandAmpersandEquals,                // &&= (logical AND assignment)
    QuestionQuestionEquals,                  // ??= (nullish coalescing assignment)

    // ===== Keywords =====
    Break,
    Case,
    Catch,
    Class,
    Const, // already used above
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else, // already used above
    Enum,
    Export,
    Extends,
    False, // already used above
    Finally,
    For,
    Function, // already used above
    If,       // already used above
    Import,
    In,
    InstanceOf,
    New,
    Null,
    Return, // already used above
    Super,
    Switch,
    This,
    Throw,
    True, // already used above
    Try,
    TypeOf,
    Var,
    Void,
    While,
    With,

    // Strict/ES keywords & TS contextual keywords
    Implements,
    Interface,
    Let, // already used above
    Package,
    Private,
    Protected,
    Public,
    Static,
    Yield,

    // TypeScript-specific/contextual keywords
    Abstract,
    As,
    Asserts,
    Any,
    Async,
    Await,
    Boolean,
    Constructor,
    Declare,
    Get,
    Infer,
    Is,
    KeyOf,
    Module,
    Namespace,
    Never,
    Readonly,
    Require,
    Number,
    Object,
    Set,
    String,
    Symbol,
    Type,
    Undefined,
    Unique,
    Unknown,
    From,
    Global,
    BigInt,
    Of,
    Satisfies,
    Override,
    Using,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedToken {
    pub value: Token,
    pub line: u32,
    pub column: u32,
}

pub fn find_match(s: &str) -> Option<Token> {
    match s {
        // Arithmetic operators
        "+" => Some(Token::Plus),
        "-" => Some(Token::Minus),
        "*" => Some(Token::Asterisk),
        "/" => Some(Token::Slash),
        "%" => Some(Token::Percent),
        "**" => Some(Token::AsteriskAsterisk),
        "++" => Some(Token::PlusPlus),
        "--" => Some(Token::MinusMinus),

        // Assignment operators
        "=" => Some(Token::Equals),
        "+=" => Some(Token::PlusEquals),
        "-=" => Some(Token::MinusEquals),
        "*=" => Some(Token::AsteriskEquals),
        "/=" => Some(Token::SlashEquals),
        "%=" => Some(Token::PercentEquals),
        "**=" => Some(Token::AsteriskAsteriskEquals),
        "&=" => Some(Token::AmpersandEquals),
        "|=" => Some(Token::BarEquals),
        "^=" => Some(Token::CaretEquals),

        // Comparison operators
        "==" => Some(Token::EqualsEquals),
        "!=" => Some(Token::ExclamationEquals),
        "===" => Some(Token::EqualsEqualsEquals),
        "!==" => Some(Token::ExclamationEqualsEquals),
        ">" => Some(Token::GreaterThan),
        "<" => Some(Token::LessThan),
        ">=" => Some(Token::GreaterThanEquals),
        "<=" => Some(Token::LessThanEquals),

        // Logical/bitwise
        "&&" => Some(Token::AmpersandAmpersand),
        "||" => Some(Token::BarBar),
        "!" => Some(Token::Bang),
        "&" => Some(Token::Ampersand),
        "|" => Some(Token::Bar),
        "^" => Some(Token::Caret),
        "~" => Some(Token::Tilde),
        "??" => Some(Token::QuestionQuestion),

        // Arrow
        "=>" => Some(Token::EqualsGreaterThan),

        // Spread/rest, member access
        "..." => Some(Token::DotDotDot),
        "?" => Some(Token::Question),
        "?." => Some(Token::QuestionDot),
        ":" => Some(Token::Colon),

        // Delimiters
        "," => Some(Token::Comma),
        ";" => Some(Token::Semicolon),
        "." => Some(Token::Dot),

        // Brackets and parenthesis
        "(" => Some(Token::OpenParen),
        ")" => Some(Token::CloseParen),
        "{" => Some(Token::OpenBrace),
        "}" => Some(Token::CloseBrace),
        "[" => Some(Token::OpenBracket),
        "]" => Some(Token::CloseBracket),

        // Keywords
        "break" => Some(Token::Break),
        "case" => Some(Token::Case),
        "catch" => Some(Token::Catch),
        "class" => Some(Token::Class),
        "const" => Some(Token::Const),
        "continue" => Some(Token::Continue),
        "debugger" => Some(Token::Debugger),
        "default" => Some(Token::Default),
        "delete" => Some(Token::Delete),
        "do" => Some(Token::Do),
        "else" => Some(Token::Else),
        "enum" => Some(Token::Enum),
        "export" => Some(Token::Export),
        "extends" => Some(Token::Extends),
        "finally" => Some(Token::Finally),
        "for" => Some(Token::For),
        "function" => Some(Token::Function),
        "if" => Some(Token::If),
        "import" => Some(Token::Import),
        "in" => Some(Token::In),
        "instanceof" => Some(Token::InstanceOf),
        "let" => Some(Token::Let),
        "new" => Some(Token::New),
        "return" => Some(Token::Return),
        "super" => Some(Token::Super),
        "switch" => Some(Token::Switch),
        "this" => Some(Token::This),
        "throw" => Some(Token::Throw),
        "try" => Some(Token::Try),
        "typeof" => Some(Token::TypeOf),
        "var" => Some(Token::Var),
        "void" => Some(Token::Void),
        "while" => Some(Token::While),
        "with" => Some(Token::With),

        // Reserved words and strict mode restricted words
        "implements" => Some(Token::Implements),
        "interface" => Some(Token::Interface),
        "package" => Some(Token::Package),
        "private" => Some(Token::Private),
        "protected" => Some(Token::Protected),
        "public" => Some(Token::Public),
        "static" => Some(Token::Static),
        "yield" => Some(Token::Yield),

        // TypeScript/contextual keywords
        "abstract" => Some(Token::Abstract),
        "as" => Some(Token::As),
        "asserts" => Some(Token::Asserts),
        "any" => Some(Token::Any),
        "async" => Some(Token::Async),
        "await" => Some(Token::Await),
        "boolean" => Some(Token::Boolean),
        "constructor" => Some(Token::Constructor),
        "declare" => Some(Token::Declare),
        "get" => Some(Token::Get),
        "infer" => Some(Token::Infer),
        "is" => Some(Token::Is),
        "keyof" => Some(Token::KeyOf),
        "module" => Some(Token::Module),
        "namespace" => Some(Token::Namespace),
        "never" => Some(Token::Never),
        "readonly" => Some(Token::Readonly),
        "require" => Some(Token::Require),
        "number" => Some(Token::Number),
        "object" => Some(Token::Object),
        "set" => Some(Token::Set),
        "string" => Some(Token::String),
        "symbol" => Some(Token::Symbol),
        "type" => Some(Token::Type),
        "undefined" => Some(Token::Undefined),
        "unique" => Some(Token::Unique),
        "unknown" => Some(Token::Unknown),
        "from" => Some(Token::From),
        "global" => Some(Token::Global),
        "bigint" => Some(Token::BigInt),
        "of" => Some(Token::Of),
        "satisfies" => Some(Token::Satisfies),
        "override" => Some(Token::Override),
        "using" => Some(Token::Using),
        _ => None,
    }
}

fn escape_string(value: &str) -> String {
    value.chars().flat_map(|c| c.escape_default()).collect()
}

fn escape_template(value: &str) -> String {
    value.replace('`', "\\`")
}

fn escape_regex_body(value: &str) -> String {
    value.replace('/', "\\/")
}

fn token_fragment(token: &Token) -> Option<Cow<'static, str>> {
    let fragment = match token {
        Token::Illegal => Cow::Borrowed("/*illegal*/"),
        Token::Eof => return None,
        Token::SingleLineCommentTrivia => Cow::Borrowed("//"),
        Token::MultiLineCommentTrivia => Cow::Borrowed("/* */"),
        Token::NewLineTrivia => Cow::Borrowed("\n"),
        Token::WhitespaceTrivia => Cow::Borrowed(" "),
        Token::ShebangTrivia => Cow::Borrowed("#!"),
        Token::ConflictMarkerTrivia => Cow::Borrowed("<<<<<<<"),
        Token::Identifier(name) => Cow::Owned(name.clone()),
        Token::PrivateIdentifier(name) => Cow::Owned(format!("#{name}")),
        Token::NumericLiteral(value) => Cow::Owned(value.clone()),
        Token::BigIntLiteral(value) => Cow::Owned(format!("{value}n")),
        Token::StringLiteral(value) => Cow::Owned(format!("\"{}\"", escape_string(value))),
        Token::RegularExpressionLiteral(body) => {
            Cow::Owned(format!("/{}/", escape_regex_body(body)))
        }
        Token::NoSubstitutionTemplateLiteral(value) => {
            Cow::Owned(format!("`{}`", escape_template(value)))
        }
        Token::TemplateHead(value) => Cow::Owned(format!("`{}${{", escape_template(value))),
        Token::TemplateMiddle(value) => Cow::Owned(format!("}}{}${{", escape_template(value))),
        Token::TemplateTail(value) => Cow::Owned(format!("}}{}`", escape_template(value))),
        Token::JsxText(value) | Token::JsxTextAllWhiteSpaces(value) => Cow::Owned(value.clone()),
        Token::DotDotDot => Cow::Borrowed("..."),
        Token::QuestionDot => Cow::Borrowed("?."),
        Token::LessThanSlash => Cow::Borrowed("</"),
        Token::Plus => Cow::Borrowed("+"),
        Token::Minus => Cow::Borrowed("-"),
        Token::Asterisk => Cow::Borrowed("*"),
        Token::AsteriskAsterisk => Cow::Borrowed("**"),
        Token::Slash => Cow::Borrowed("/"),
        Token::Percent => Cow::Borrowed("%"),
        Token::PlusPlus => Cow::Borrowed("++"),
        Token::MinusMinus => Cow::Borrowed("--"),
        Token::LessThanLessThan => Cow::Borrowed("<<"),
        Token::GreaterThanGreaterThan => Cow::Borrowed(">>"),
        Token::GreaterThanGreaterThanGreaterThan => Cow::Borrowed(">>>"),
        Token::Ampersand => Cow::Borrowed("&"),
        Token::Bar => Cow::Borrowed("|"),
        Token::Caret => Cow::Borrowed("^"),
        Token::Bang => Cow::Borrowed("!"),
        Token::Tilde => Cow::Borrowed("~"),
        Token::AmpersandAmpersand => Cow::Borrowed("&&"),
        Token::BarBar => Cow::Borrowed("||"),
        Token::Question => Cow::Borrowed("?"),
        Token::At => Cow::Borrowed("@"),
        Token::QuestionQuestion => Cow::Borrowed("??"),
        Token::Hash => Cow::Borrowed("#"),
        Token::Equals => Cow::Borrowed("="),
        Token::PlusEquals => Cow::Borrowed("+="),
        Token::MinusEquals => Cow::Borrowed("-="),
        Token::AsteriskEquals => Cow::Borrowed("*="),
        Token::AsteriskAsteriskEquals => Cow::Borrowed("**="),
        Token::SlashEquals => Cow::Borrowed("/="),
        Token::PercentEquals => Cow::Borrowed("%="),
        Token::LessThanLessThanEquals => Cow::Borrowed("<<="),
        Token::GreaterThanGreaterThanEquals => Cow::Borrowed(">>="),
        Token::GreaterThanGreaterThanGreaterThanEquals => Cow::Borrowed(">>>="),
        Token::AmpersandEquals => Cow::Borrowed("&="),
        Token::BarEquals => Cow::Borrowed("|="),
        Token::CaretEquals => Cow::Borrowed("^="),
        Token::BarBarEquals => Cow::Borrowed("||="),
        Token::AmpersandAmpersandEquals => Cow::Borrowed("&&="),
        Token::QuestionQuestionEquals => Cow::Borrowed("??="),
        Token::EqualsEquals => Cow::Borrowed("=="),
        Token::ExclamationEquals => Cow::Borrowed("!="),
        Token::EqualsEqualsEquals => Cow::Borrowed("==="),
        Token::ExclamationEqualsEquals => Cow::Borrowed("!=="),
        Token::GreaterThan => Cow::Borrowed(">"),
        Token::LessThan => Cow::Borrowed("<"),
        Token::GreaterThanEquals => Cow::Borrowed(">="),
        Token::LessThanEquals => Cow::Borrowed("<="),
        Token::EqualsGreaterThan => Cow::Borrowed("=>"),
        Token::Comma => Cow::Borrowed(","),
        Token::Semicolon => Cow::Borrowed(";"),
        Token::Colon => Cow::Borrowed(":"),
        Token::Dot => Cow::Borrowed("."),
        Token::OpenParen => Cow::Borrowed("("),
        Token::CloseParen => Cow::Borrowed(")"),
        Token::OpenBrace => Cow::Borrowed("{"),
        Token::CloseBrace => Cow::Borrowed("}"),
        Token::OpenBracket => Cow::Borrowed("["),
        Token::CloseBracket => Cow::Borrowed("]"),
        Token::Break => Cow::Borrowed("break"),
        Token::Case => Cow::Borrowed("case"),
        Token::Catch => Cow::Borrowed("catch"),
        Token::Class => Cow::Borrowed("class"),
        Token::Const => Cow::Borrowed("const"),
        Token::Continue => Cow::Borrowed("continue"),
        Token::Debugger => Cow::Borrowed("debugger"),
        Token::Default => Cow::Borrowed("default"),
        Token::Delete => Cow::Borrowed("delete"),
        Token::Do => Cow::Borrowed("do"),
        Token::Else => Cow::Borrowed("else"),
        Token::Enum => Cow::Borrowed("enum"),
        Token::Export => Cow::Borrowed("export"),
        Token::Extends => Cow::Borrowed("extends"),
        Token::False => Cow::Borrowed("false"),
        Token::Finally => Cow::Borrowed("finally"),
        Token::For => Cow::Borrowed("for"),
        Token::Function => Cow::Borrowed("function"),
        Token::If => Cow::Borrowed("if"),
        Token::Import => Cow::Borrowed("import"),
        Token::In => Cow::Borrowed("in"),
        Token::InstanceOf => Cow::Borrowed("instanceof"),
        Token::New => Cow::Borrowed("new"),
        Token::Null => Cow::Borrowed("null"),
        Token::Return => Cow::Borrowed("return"),
        Token::Super => Cow::Borrowed("super"),
        Token::Switch => Cow::Borrowed("switch"),
        Token::This => Cow::Borrowed("this"),
        Token::Throw => Cow::Borrowed("throw"),
        Token::True => Cow::Borrowed("true"),
        Token::Try => Cow::Borrowed("try"),
        Token::TypeOf => Cow::Borrowed("typeof"),
        Token::Var => Cow::Borrowed("var"),
        Token::Void => Cow::Borrowed("void"),
        Token::While => Cow::Borrowed("while"),
        Token::With => Cow::Borrowed("with"),
        Token::Implements => Cow::Borrowed("implements"),
        Token::Interface => Cow::Borrowed("interface"),
        Token::Let => Cow::Borrowed("let"),
        Token::Package => Cow::Borrowed("package"),
        Token::Private => Cow::Borrowed("private"),
        Token::Protected => Cow::Borrowed("protected"),
        Token::Public => Cow::Borrowed("public"),
        Token::Static => Cow::Borrowed("static"),
        Token::Yield => Cow::Borrowed("yield"),
        Token::Abstract => Cow::Borrowed("abstract"),
        Token::As => Cow::Borrowed("as"),
        Token::Asserts => Cow::Borrowed("asserts"),
        Token::Any => Cow::Borrowed("any"),
        Token::Async => Cow::Borrowed("async"),
        Token::Await => Cow::Borrowed("await"),
        Token::Boolean => Cow::Borrowed("boolean"),
        Token::Constructor => Cow::Borrowed("constructor"),
        Token::Declare => Cow::Borrowed("declare"),
        Token::Get => Cow::Borrowed("get"),
        Token::Infer => Cow::Borrowed("infer"),
        Token::Is => Cow::Borrowed("is"),
        Token::KeyOf => Cow::Borrowed("keyof"),
        Token::Module => Cow::Borrowed("module"),
        Token::Namespace => Cow::Borrowed("namespace"),
        Token::Never => Cow::Borrowed("never"),
        Token::Readonly => Cow::Borrowed("readonly"),
        Token::Require => Cow::Borrowed("require"),
        Token::Number => Cow::Borrowed("number"),
        Token::Object => Cow::Borrowed("object"),
        Token::Set => Cow::Borrowed("set"),
        Token::String => Cow::Borrowed("string"),
        Token::Symbol => Cow::Borrowed("symbol"),
        Token::Type => Cow::Borrowed("type"),
        Token::Undefined => Cow::Borrowed("undefined"),
        Token::Unique => Cow::Borrowed("unique"),
        Token::Unknown => Cow::Borrowed("unknown"),
        Token::From => Cow::Borrowed("from"),
        Token::Global => Cow::Borrowed("global"),
        Token::BigInt => Cow::Borrowed("bigint"),
        Token::Of => Cow::Borrowed("of"),
        Token::Satisfies => Cow::Borrowed("satisfies"),
        Token::Override => Cow::Borrowed("override"),
        Token::Using => Cow::Borrowed("using"),
    };

    Some(fragment)
}

fn is_identifier_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '$')
}

fn needs_separator(prev_fragment: &str, next_fragment: &str) -> bool {
    if prev_fragment
        .chars()
        .last()
        .map(|c| c.is_whitespace())
        .unwrap_or(false)
    {
        return false;
    }

    if next_fragment
        .chars()
        .next()
        .map(|c| c.is_whitespace())
        .unwrap_or(false)
    {
        return false;
    }

    let prev_significant = prev_fragment.chars().rev().find(|c| !c.is_whitespace());
    let next_significant = next_fragment.chars().find(|c| !c.is_whitespace());

    match (prev_significant, next_significant) {
        (Some(prev), Some(next)) => is_identifier_char(prev) && is_identifier_char(next),
        _ => false,
    }
}

/// Render a sequence of tokens back into a source string suitable for tests.
///
/// This is a best-effort reconstruction. Trivia tokens that do not retain their
/// original text (e.g. comments) are emitted in a minimal canonical form.
pub fn tokens_to_source<'a, I>(tokens: I) -> String
where
    I: IntoIterator<Item = &'a SpannedToken>,
{
    tokens.into_iter().fold(String::new(), |mut acc, token| {
        if let Some(fragment) = token_fragment(&token.value) {
            if !acc.is_empty() && needs_separator(&acc, &fragment) {
                acc.push(' ');
            }
            acc.push_str(&fragment);
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_identifier_numeric_and_operator() {
        let tokens = vec![
            SpannedToken {
                value: Token::Identifier("foo".into()),
                line: 1,
                column: 1,
            },
            SpannedToken {
                value: Token::Plus,
                line: 1,
                column: 4,
            },
            SpannedToken {
                value: Token::NumericLiteral("42".into()),
                line: 1,
                column: 5,
            },
            SpannedToken {
                value: Token::Eof,
                line: 1,
                column: 7,
            },
        ];

        assert_eq!(tokens_to_source(&tokens), "foo+42");
    }

    #[test]
    fn emits_whitespace_token_verbatim() {
        let tokens = vec![
            SpannedToken {
                value: Token::Identifier("let".into()),
                line: 1,
                column: 1,
            },
            SpannedToken {
                value: Token::WhitespaceTrivia,
                line: 1,
                column: 4,
            },
            SpannedToken {
                value: Token::Identifier("x".into()),
                line: 1,
                column: 5,
            },
            SpannedToken {
                value: Token::Equals,
                line: 1,
                column: 6,
            },
            SpannedToken {
                value: Token::NumericLiteral("1".into()),
                line: 1,
                column: 7,
            },
        ];

        assert_eq!(tokens_to_source(&tokens), "let x=1");
    }

    #[test]
    fn separates_adjacent_identifiers() {
        let tokens = vec![
            SpannedToken {
                value: Token::Identifier("foo".into()),
                line: 1,
                column: 1,
            },
            SpannedToken {
                value: Token::Identifier("bar".into()),
                line: 1,
                column: 5,
            },
        ];

        assert_eq!(tokens_to_source(&tokens), "foo bar");
    }
}
