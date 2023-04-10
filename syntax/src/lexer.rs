use std::ops::Range;

use logos::Logos;

#[derive(Debug, Clone)]
pub(crate) struct Token<'text> {
    kind: TokenKind,
    absolute_span: Range<usize>,

    // Ugly, but reduces allocations ¯\ (ツ) /¯
    text: &'text str,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} @ [{}..{}] \"{}\"",
            self.kind, self.absolute_span.start, self.absolute_span.end, self.text
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub(crate) enum TokenKind {
    #[regex(" +")]
    Whitespace,

    #[regex(r"\r?\n")]
    Newline,

    #[regex("0[xX][a-fA-F0-9](?:_?[a-fA-F0-9])")]
    HexLiteral,

    #[regex("0[oO][0-7](?:_?[0-7])")]
    OctalLiteral,

    #[regex("0[bB][01](?:_?[01])")]
    BinaryLiteral,

    #[regex("(?:0|[1-9](?:_*[0-9])*)")]
    DecimalLiteral,

    #[regex("true|false")]
    BooleanLiteral,

    #[regex(r"\\x[0-9a-fA-F][0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?")]
    HexEscapeSequence,

    #[regex(r"\\u[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]")]
    UnicodeEscapeSequence,

    #[token("'")]
    SingleQuote,

    #[token("\"")]
    DoubleQuote,

    #[error]
    Error,
}

#[derive(Debug)]
pub(crate) struct Lexer<'input> {
    tokens: Vec<Token<'input>>,
    current: usize,
}

impl<'input> Lexer<'input> {
    pub(crate) fn new(input: &'input str) -> Self {
        // match self.inner.next() {
        //     Some(kind) => Some(Self::Item {
        //         kind,
        //         text: self.inner.slice(),
        //         absolute_span: self.inner.span(),
        //     }),
        //     None => None,
        // }
        // let inner = TokenKind::lexer(input);
        let mut inner = TokenKind::lexer(input);
        let mut tokens = vec![];
        while let Some(kind) = inner.next() {
            tokens.push(Token {
                kind,
                absolute_span: inner.span(),
                text: inner.slice(),
            });
        }

        Self {
            tokens: tokens,
            current: 0,
        }
    }

    pub(crate) fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_at(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }

    pub(crate) fn bump(&mut self) -> Option<Token> {
        self.current += 1;

        // TODO remove the .clone(), probably expensive in the long run
        if self.current < self.tokens.len() {
            Some(self.tokens[self.current].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn print() {
        let lexer = Lexer::new("1+  2+3-4");
        println!("{:#?}", lexer);
    }
}
