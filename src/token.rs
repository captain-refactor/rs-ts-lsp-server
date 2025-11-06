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
    Comma,          // ,
    Semicolon,      // ;
    Colon,          // :
    Dot,            // .
    OpenParen,      // (
    CloseParen,     // )
    OpenBrace,      // {
    CloseBrace,     // }
    OpenBracket,    // [
    CloseBracket,   // ]
    DotDotDot, // ...

    // Member access / optional chaining
    QuestionDot, // ?.

    // Relational / equality / arrow
    LessThan,        // <
    LessThanSlash,   // </ (JSX)
    GreaterThan,     // >
    LessThanEquals,  // <=
    GreaterThanEquals, // >=
    EqualsEquals,    // ==
    ExclamationEquals,        // !=
    EqualsEqualsEquals,       // ===
    ExclamationEqualsEquals,  // !==
    EqualsGreaterThan,        // =>

    // Arithmetic / unary / bitwise / logical
    Plus,                 // +
    Minus,                // -
    Asterisk,             // *
    AsteriskAsterisk, // **
    Slash,                // /
    Percent,         // %
    PlusPlus,        // ++
    MinusMinus,      // --
    LessThanLessThan,            // <<
    GreaterThanGreaterThan,      // >>
    GreaterThanGreaterThanGreaterThan, // >>>
    Ampersand,       // &
    Bar,             // |
    Caret,           // ^
    Bang,                 // !
    Tilde,           // ~
    AmpersandAmpersand, // &&
    BarBar,          // ||
    Question,        // ?
    // ColonToken removed; use Colon above
    At,              // @
    QuestionQuestion, // ??
    Hash,            // # (used e.g. in private names in certain contexts)

    // Assignment
    Assign,               // =
    Equals,                     // ==
    PlusEquals,                  // +=
    MinusEquals,                 // -=
    AsteriskEquals,              // *=
    AsteriskAsteriskEquals,      // **=
    SlashEquals,                 // /=
    PercentEquals,               // %=
    LessThanLessThanEquals,      // <<=
    GreaterThanGreaterThanEquals, // >>=
    GreaterThanGreaterThanGreaterThanEquals, // >>>=
    AmpersandEquals,             // &=
    BarEquals,                   // |=
    CaretEquals,                 // ^=
    BarBarEquals,                // ||= (logical OR assignment)
    AmpersandAmpersandEquals,    // &&= (logical AND assignment)
    QuestionQuestionEquals,      // ??= (nullish coalescing assignment)

    // ===== Keywords =====
    Break,
    Case,
    Catch,
    Class,
    Const,           // already used above
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,            // already used above
    Enum,
    Export,
    Extends,
    False,           // already used above
    Finally,
    For,
    Function,        // already used above
    If,              // already used above
    Import,
    In,
    InstanceOf,
    New,
    Null,
    Return,          // already used above
    Super,
    Switch,
    This,
    Throw,
    True,            // already used above
    Try,
    TypeOf,
    Var,
    Void,
    While,
    With,

    // Strict/ES keywords & TS contextual keywords
    Implements,
    Interface,
    Let,             // already used above
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
    pub line: usize,
    pub column: usize,
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

