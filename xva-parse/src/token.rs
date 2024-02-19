use internment::Intern;
use xva_span::SourceSpan;

/// A single token produced by the lexer.
///
/// Contains a [`TokenKind`] as the variant, a [`SourceSpan`] indicating its span in the input text,
/// and a reference to the original text, with full-fidelity.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
    pub original: Intern<String>,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn start(&self) -> usize {
        self.span.range().start()
    }

    pub fn end(&self) -> usize {
        self.span.range().start()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)] // Manual PartialEq impl
pub enum TokenKind {
    // Literals
    Boolean(bool),
    Char(char),
    String(Intern<String>),
    Integer(i128),
    Float(f64),

    // Comments
    Comment(Intern<String>),
    DocComment(Intern<String>),

    // Symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Colon,
    Comma,
    DoubleQuote,
    SingleQuote,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    DoubleStar,
    Equals,
    LeftAngle,
    RightAngle,
    Increment,
    Decrement,
    Dot,
    Ampersand,
    Pipe,
    Caret,
    DoubleRightAngle,
    DoubleLeftAngle,
    GreaterThanEquals,
    LessThanEquals,
    DoubleEquals,
    NotEquals,
    And,
    Or,
    Not,

    /// Identifier
    Identifier(Intern<String>),

    /// Error token.
    ///
    /// Only contains a single character, error tokens should be collected at a later stage.
    CharError(char),

    /// Error token
    ///
    /// Contains multiple characters.
    Error(Intern<String>),
}

impl<'src> std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Boolean(b) => write!(f, "'{b}'"),
            TokenKind::Char(c) => write!(f, "'{c}'"),
            TokenKind::String(s) => write!(f, "'{s}'"),
            TokenKind::Integer(i) => write!(f, "'{i}'"),
            TokenKind::Float(fl) => write!(f, "'{fl}'"),
            TokenKind::Comment(co) => write!(f, "'{co}'"),
            TokenKind::DocComment(dco) => write!(f, "'{dco}'"),
            TokenKind::OpenParen => write!(f, "'('"),
            TokenKind::CloseParen => write!(f, "')'"),
            TokenKind::OpenBrace => write!(f, "'{{'"),
            TokenKind::CloseBrace => write!(f, "'}}'"),
            TokenKind::Colon => write!(f, "':'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::DoubleQuote => write!(f, "'\"'"),
            TokenKind::SingleQuote => write!(f, "'\''"),
            TokenKind::Plus => write!(f, "'+'"),
            TokenKind::Minus => write!(f, "'-'"),
            TokenKind::Star => write!(f, "'*'"),
            TokenKind::Slash => write!(f, "'/'"),
            TokenKind::Percent => write!(f, "'%'"),
            TokenKind::DoubleStar => write!(f, "'**'"),
            TokenKind::Equals => write!(f, "'='"),
            TokenKind::LeftAngle => write!(f, "'<'"),
            TokenKind::RightAngle => write!(f, "'>'"),
            TokenKind::Increment => write!(f, "'++'"),
            TokenKind::Decrement => write!(f, "'--'"),
            TokenKind::Dot => write!(f, "'.'"),
            TokenKind::Ampersand => write!(f, "'&'"),
            TokenKind::Pipe => write!(f, "'|'"),
            TokenKind::Caret => write!(f, "'^'"),
            TokenKind::DoubleRightAngle => write!(f, "'>>'"),
            TokenKind::DoubleLeftAngle => write!(f, "'<<'"),
            TokenKind::GreaterThanEquals => write!(f, "'>='"),
            TokenKind::LessThanEquals => write!(f, "'<='"),
            TokenKind::DoubleEquals => write!(f, "'=='"),
            TokenKind::NotEquals => write!(f, "'!='"),
            TokenKind::And => write!(f, "'and'"),
            TokenKind::Or => write!(f, "'or'"),
            TokenKind::Not => write!(f, "'not'"),

            TokenKind::Identifier(i) => write!(f, "'{i}'"),
            TokenKind::CharError(err) => write!(f, "'{err}'"),
            TokenKind::Error(err) => write!(f, "'{err}'"),
        }
    }
}

// Manual implementation of PartialEq because of floats
impl PartialEq for TokenKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float(f_self), Self::Float(f_other)) => f_self.to_bits() == f_other.to_bits(),
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Comment(l0), Self::Comment(r0)) => l0 == r0,
            (Self::DocComment(l0), Self::DocComment(r0)) => l0 == r0,
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::CharError(l0), Self::CharError(r0)) => l0 == r0,
            (Self::Error(l0), Self::Error(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
