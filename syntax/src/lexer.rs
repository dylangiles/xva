use chumsky::{
    prelude::Simple,
    primitive::{choice, end, filter, just, one_of, take_until},
    recovery::skip_then_retry_until,
    text, Parser,
};
use std::ops::Range;

mod cursor;
mod keyword;
mod literal;

/// A representation of a location within written code,
/// with a start and end.
///
/// This type is an alias of [`std::ops::Range`].
pub type Span = Range<usize>;

/// A single unit of tokenisation, containing its variant and its location within the given input.
#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.span == other.span
    }
}

/// Contains definitions for all possible tokens that can be produced by the lexer.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Operator(String),

    // *** Multi character tokens ***
    /// "// comment"
    LineComment,

    /// "/* comment */"
    DocComment,

    /// Any whitespace sequence, see [`is_whitespace`].
    Whitespace,

    /// A valid identifier
    Identifier,

    /// An invalid identifier (contains Unicode characters or something)
    InvalidIdentifier,

    /// See [`LiteralKind`]s
    Literal {
        kind: LiteralKind,
    },

    /// `==`
    DoubleEquals,

    /// `..`
    Ellipsis,

    /// `!=`
    NotEquals,

    /// `=>`
    FatArrow,

    /// `<=`
    LowerThanEquals,

    /// `>=`
    GreaterThanEquals,

    // *** Single character tokens ***
    /// `;`
    Semicolon,

    /// `,`
    Comma,

    /// `.`
    Dot,

    /// `(`
    OpenParenthesis,

    /// `)`
    CloseParenthesis,

    /// `[`
    OpenSquare,

    /// `]`
    CloseSquare,

    /// `@`
    At,

    /// `#`
    Hash,

    /// `~`
    Tilde,

    /// `?`
    Question,

    /// `:`
    Colon,

    /// `=`
    Equals,

    /// `!`
    Bang,

    /// `<`
    LowerThan,

    /// `>`
    GreaterThan,

    /// `-`
    Minus,

    /// `&`
    Ampersand,

    /// `|`
    Bar,

    /// `+`
    Plus,

    /// `*`
    Star,

    /// `/`
    Slash,

    /// `^`
    Caret,

    /// `%`
    Percent,

    Newline,
    EmptyLine,

    LetKeyword,
    DefKeyword,
    EndKeyword,
    MutableKeyword,

    /// Unknown token that wasn't expected by the lexer
    Unknown,

    /// End of the input
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    /// 1, 0xAC, 0o33, 0b1100
    Integer { base: Base },

    /// 1.23, 1e2, etc.
    Float { base: Base },

    /// 'c'
    Character { terminated: bool },

    /// b'c'
    ByteChar { terminated: bool },

    /// "string"
    String { terminated: bool },

    /// b"string"
    ByteString { terminated: bool },

    /// `r"string"`, `r#"string"#`, `r##"string"##` etc.
    ///
    /// `num_hashes = None` signals an invalid literal.
    RawString { num_hashes: Option<u8> },

    /// `br"string"`, `br#"string"#`, `br##"string"##` etc.
    ///
    /// `num_hashes = None` signals an invalid literal.
    RawByteString { num_hashes: Option<u8> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Literal starts with `0b`
    Binary = 2,

    /// Literal starts with `0o`
    Octal = 8,

    /// Literal doesn't start with anything
    Decimal = 10,

    /// Literal starts with `0x`
    Hexadecimal,
}

pub(super) fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    choice((literal::literal(), keyword::keywords()))
        .recover_with(skip_then_retry_until([]))
        .repeated()
        .then(end())
        .map(|(res, _)| res)
}

/// Produces a tuple containing a stream of [`Token`]s (or `None`, if the tokeniser failed) and a stream of
/// errors.
///
/// # Arguments
/// * `input` - The input to tokenise
pub fn lex(input: &str) -> (Option<Vec<Token>>, Vec<Simple<char>>) {
    lexer().parse_recovery(input)
}

#[allow(dead_code)]
pub(crate) fn expect_lexer(
    input: &str,
    parser: impl Parser<char, Token, Error = Simple<char>>,
    expect_case: TokenKind,
) {
    let (val, _) = parser.parse_recovery(input);
    match val {
        Some(tok) => assert_eq!(tok.kind, expect_case),
        None => panic!("parse_recovery returned a None case."),
    }
}

const TOKEN_DOUBLE_EQUALS: &str = "==";
const TOKEN_ELLIPSIS: &str = "..";
const TOKEN_NOT_EQUALS: &str = "!=";
const TOKEN_FAT_ARROW: &str = "=>";
const TOKEN_LOWER_THAN_EQUALS: &str = "<=";
const TOKEN_GREATER_THAN_EQUALS: &str = ">=";
const TOKEN_SEMICOLON: &str = ";";
const TOKEN_COMMA: &str = ",";
const TOKEN_DOT: &str = ".";
const TOKEN_OPEN_PAREN: &str = "(";
const TOKEN_CLOSE_PAREN: &str = ")";
const TOKEN_OPEN_SQUARE: &str = "[";
const TOKEN_CLOSE_SQUARE: &str = "]";
const TOKEN_AT: &str = "@";
const TOKEN_HASH: &str = "#";
const TOKEN_TILDE: &str = "~";
const TOKEN_QUESTION: &str = "?";
const TOKEN_COLON: &str = ":";
const TOKEN_EQUALS: &str = "=";
const TOKEN_BANG: &str = "!";
const TOKEN_LOWER_THAN: &str = "<";
const TOKEN_GREATER_THAN: &str = ">";
const TOKEN_MINUS: &str = "-";
const TOKEN_AMPERSAND: &str = "&";
const TOKEN_BAR: &str = "|";
const TOKEN_PLUS: &str = "+";
const TOKEN_STAR: &str = "*";
const TOKEN_SLASH: &str = "/";
const TOKEN_CARET: &str = "^";
const TOKEN_PERCENT: &str = "%";

const CHAR_DOUBLE_QUOTE: &str = "\"";
const CHAR_BACKSLASH: &str = "\\";
const CHAR_LETTER_B: &str = "b";

const TOKEN_KEYWORD_LET: &str = "let";
const TOKEN_KEYWORD_DEF: &str = "def";
const TOKEN_KEYWORD_END: &str = "end";
const TOKEN_KEYWORD_MUTABLE: &str = "mutable";

pub fn new_lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let literal = literal::literal();
    let operator: chumsky::primitive::Choice<(chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>, chumsky::combinator::To<chumsky::primitive::Just<_, &str, _>, &str, TokenKind>), _> = choice((
        just(TOKEN_DOUBLE_EQUALS).to(TokenKind::DoubleEquals),
        just(TOKEN_ELLIPSIS).to(TokenKind::Ellipsis),
        just(TOKEN_NOT_EQUALS).to(TokenKind::NotEquals),
        just(TOKEN_FAT_ARROW).to(TokenKind::FatArrow),
        just(TOKEN_LOWER_THAN_EQUALS).to(TokenKind::LowerThanEquals),
        just(TOKEN_GREATER_THAN_EQUALS).to(TokenKind::GreaterThanEquals),
        just(TOKEN_SEMICOLON).to(TokenKind::Semicolon),
        just(TOKEN_COMMA).to(TokenKind::Comma),
        just(TOKEN_DOT).to(TokenKind::Dot),
        just(TOKEN_AT).to(TokenKind::At),
        just(TOKEN_HASH).to(TokenKind::Hash),
        just(TOKEN_TILDE).to(TokenKind::Tilde),
        just(TOKEN_QUESTION).to(TokenKind::Question),
        just(TOKEN_COLON).to(TokenKind::Colon),
        just(TOKEN_EQUALS).to(TokenKind::Equals),
        just(TOKEN_BANG).to(TokenKind::Bang),
        just(TOKEN_LOWER_THAN).to(TokenKind::LowerThan),
        just(TOKEN_GREATER_THAN).to(TokenKind::GreaterThan),
        just(TOKEN_MINUS).to(TokenKind::Minus),
        just(TOKEN_AMPERSAND).to(TokenKind::Ampersand),
        just(TOKEN_BAR).to(TokenKind::Bar),
        just(TOKEN_PLUS).to(TokenKind::Plus),
        just(TOKEN_STAR).to(TokenKind::Star),
        just(TOKEN_SLASH).to(TokenKind::Slash),
        just(TOKEN_CARET).to(TokenKind::Caret),
        just(TOKEN_PERCENT).to(TokenKind::Percent),
    ));

    let grouping = choice((
        just(TOKEN_OPEN_PAREN).to(TokenKind::OpenParenthesis),
        just(TOKEN_CLOSE_PAREN).to(TokenKind::CloseParenthesis),
        just(TOKEN_OPEN_SQUARE).to(TokenKind::OpenSquare),
        just(TOKEN_CLOSE_SQUARE).to(TokenKind::CloseSquare),
    ));

    let string = just(CHAR_DOUBLE_QUOTE)
        .ignore_then(filter(|c| true))
        .repeated()
        .then_ignore(just(CHAR_DOUBLE_QUOTE))
        .collect::<String>()
        .map(|v| TokenKind::String(v));

    // let byte_string = just(CHAR_LETTER_B)
    //     .then(just(CHAR_DOUBLE_QUOTE))
    //     .ignore_then(filter(|c| true))
    //     .repeated()
    //     .then_ignore(just(CHAR_DOUBLE_QUOTE))
    //     .collect::<String>()
    //     .map(|v| TokenKind::String(v));

    let keyword = text::ident().map(|s: String| match s.as_str() {
        TOKEN_KEYWORD_DEF => TokenKind::DefKeyword,
        TOKEN_KEYWORD_END => TokenKind::EndKeyword,
        TOKEN_KEYWORD_LET => TokenKind::LetKeyword,
        TOKEN_KEYWORD_MUTABLE => TokenKind::MutableKeyword,
        _ => TokenKind::Identifier,
    });

    fn comments(kind: TokenKind) -> impl Parser<char, Token, Error = Simple<char>> {
        let slashes = match kind {
            TokenKind::LineComment => 2,
            TokenKind::DocComment => 3,
            _ => unreachable!(),
        };

        choice((
            just("/".repeat(slashes))
                .ignore_then(text::newline().rewind())
                .to(kind.clone())
                .map_with_span(|t, s: Span| Token::new(t, s)),
            just("/".repeat(slashes)).ignore_then(
                take_until(text::newline().rewind())
                    .to(kind)
                    .map_with_span(|t, s| Token::new(t, s)),
            ),
        ))
    }

    // let newlines = choice((
    //     just("\n\n").or(just("\r\n\r\n")).to(TokenKind::EmptyLine),
    //     // choice((just("\n"), just("\r\n"))).to(TokenKind::Newline),
    // ));

    let newlines = just("\n\n").or(just("\r\n\r\n")).to(TokenKind::EmptyLine);
    choice((
        comments(TokenKind::LineComment),
        comments(TokenKind::DocComment),
        choice((
            keyword,
            // operator,
            // newlines,
            // grouping,
            // byte_string,
            // string,
        ))
        .map_with_span(|t, s| Token::new(t, s)),
    ))
    .padded_by(one_of(" \t").ignored().repeated())
    .recover_with(skip_then_retry_until([]))
    .repeated()
    .padded_by(one_of(" \t").ignored().repeated())
    .then_ignore(end())
}
