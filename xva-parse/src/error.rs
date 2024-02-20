use ariadne::{Color, Fmt, Label, Report, ReportKind, Span};
use chumsky::util::MaybeRef;
use std::io::Write;

use xva_span::{CheapRange, SourceId, SourceSpan, TokenSpan};

use crate::token::{Token, TokenKind};

#[derive(Debug, PartialEq)]
pub enum SyntaxErrorKind {
    UnexpectedEnd,
    UnexpectedPattern(ErrorPattern),
    InvalidUnicode(u32), // UnclosedDelimiter,
                         // NoEndBranch,
}
#[derive(Debug)]
pub struct SyntaxError {
    kind: SyntaxErrorKind,
    span: SourceSpan,
    label: Option<&'static str>,
}

impl SyntaxError {
    pub const fn new(kind: SyntaxErrorKind, span: SourceSpan) -> Self {
        Self {
            kind,
            span,
            label: None,
        }
    }

    pub const fn unexpected_pattern(
        pat: ErrorPattern,
        span: SourceSpan,
        label: Option<&'static str>,
    ) -> Self {
        Self {
            kind: SyntaxErrorKind::UnexpectedPattern(pat),
            span,
            label,
        }
    }

    pub fn write<C>(self, cache: C, writer: impl Write)
    where
        C: ariadne::Cache<SourceId>,
    {
        let msg = match &self.kind {
            SyntaxErrorKind::InvalidUnicode(uc) => {
                format!("Invalid Unicode: the value {uc:#x} is not a valid Unicode scalar value.")
            }
            error_kind => format!(
                "{}{}, expected {}",
                match error_kind {
                    // SyntaxErrorKind::NoEndBranch => "No end branch",
                    SyntaxErrorKind::UnexpectedEnd => "Unexpected end of input".to_string(),
                    SyntaxErrorKind::UnexpectedPattern(pat) =>
                        format!("Unexpected pattern: {}", pat.fg(Color::Red)),

                    _ => unreachable!(),
                },
                self.label.map_or("".into(), |label| format!(
                    " while parsing {}",
                    label.fg(Color::Cyan)
                )),
                "something else",
            ),
        };

        let report = Report::build(ReportKind::Error, self.span.src(), self.span.start())
            .with_code(3)
            .with_message(msg)
            .with_label(
                // BUG: labels attaches to both the text and trailing whitespace, instead of just the text
                Label::new(self.span)
                    .with_message(match &self.kind {
                        SyntaxErrorKind::UnexpectedEnd => "End of input".to_string(),
                        SyntaxErrorKind::UnexpectedPattern(pat) => {
                            format!("Unexpected pattern: {} ", pat.fg(Color::Red))
                        }
                        SyntaxErrorKind::InvalidUnicode(_) => "Invalid Unicode value here".into(),
                    })
                    .with_color(Color::Red),
            );

        report.finish().write(cache, writer).unwrap();
    }
}

// Lexer error implementation: the input type is &'src str
impl<'src> chumsky::error::Error<'src, &'src str> for SyntaxError {
    /// `&'a str`'s implementation of [`chumsky::input::Input`] has the following associated types:
    /// ```ignore
    /// type Offset = usize;
    /// type Token = char;
    /// type Span = SimpleSpan<usize>;
    /// ```
    fn expected_found<Iter: IntoIterator<Item = Option<MaybeRef<'src, char>>>>(
        _expected: Iter,
        found: Option<MaybeRef<'src, char>>,
        span: TokenSpan,
    ) -> Self {
        Self {
            kind: found
                .map(|c| ErrorPattern::Char(*c))
                .map(SyntaxErrorKind::UnexpectedPattern)
                .unwrap_or(SyntaxErrorKind::UnexpectedEnd),
            span: SourceSpan::new(0.into(), {
                // TODO don't use 0 for SrcId
                let TokenSpan { start, end, .. } = span;
                CheapRange::new(start, end)
            }),
            label: None,
        }
    }
}

// Parser error implementation: the input type is &'src [Token]
impl<'src> chumsky::error::Error<'src, &'src [Token]> for SyntaxError {
    /// `&'src [Token]`'s implementation of [`chumsky::input::Input`] has the following associated types:
    /// ```ignore
    /// type Offset = usize;
    /// type Token = Token;
    /// type Span = TokenSpan;
    /// ```
    fn expected_found<E: IntoIterator<Item = Option<MaybeRef<'src, Token>>>>(
        _expected: E,
        found: Option<MaybeRef<'src, Token>>,
        span: TokenSpan,
    ) -> Self {
        Self {
            kind: found
                .map(|tok| ErrorPattern::Token(tok.kind))
                .map(SyntaxErrorKind::UnexpectedPattern)
                .unwrap_or(SyntaxErrorKind::UnexpectedEnd),
            span: SourceSpan::new(0.into(), {
                let TokenSpan { start, end, .. } = span;
                CheapRange::new(start, end)
            }),
            label: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorPattern {
    Char(char),
    Token(TokenKind),
    EndOfInput,
}

// Chumsky's `Maybe` contains either a reference to a value or an owned value. These From<> implementations
// assist with mapping Maybe<> values in chumsky's error handling to these ErrorPattern<'_>'s.

// impl From<Maybe<char, &char>> for ErrorPattern<'_> {
//     fn from(value: Maybe<char, &char>) -> Self {
//         match value {
//             Maybe::Ref(ref_c) => Self::Char(*ref_c),
//             Maybe::Val(c) => Self::Char(c),
//         }
//     }
// }

// impl<'src> From<Maybe<TokenKind<'src>, &TokenKind<'src>>> for ErrorPattern<'src> {
//     fn from(value: Maybe<TokenKind<'src>, &TokenKind<'src>>) -> Self {
//         match value {
//             Maybe::Ref(ref_t) => Self::Token(*ref_t),
//             Maybe::Val(t) => Self::Token(t),
//         }
//     }
// }

impl std::fmt::Display for ErrorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorPattern::Char(c) => write!(f, "{c:?}"),
            ErrorPattern::Token(t) => write!(f, "{t}"),
            ErrorPattern::EndOfInput => write!(f, "end of input"),
        }
    }
}
