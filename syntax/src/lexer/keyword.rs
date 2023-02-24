use super::{Token, TokenKind};
use chumsky::{
    prelude::Simple,
    primitive::{choice, just, one_of},
    text::{self, whitespace},
    Parser,
};

const KEYWORD_LET: &str = "let";
const KEYWORD_DEF: &str = "def";
const KEYWORD_END: &str = "end";

pub(super) fn keywords() -> impl Parser<char, Token, Error = Simple<char>> {
    let kws = choice((
        just(KEYWORD_LET).map_with_span(|_, span| Token::new(TokenKind::LetKeyword, span)),
        just(KEYWORD_DEF).map_with_span(|_, span| Token::new(TokenKind::DefKeyword, span)),
        just(KEYWORD_END).map_with_span(|_, span| Token::new(TokenKind::EndKeyword, span)),
    ));

    kws.or(whitespace().ignore_then(kws).then_ignore(whitespace()))
}

#[cfg(test)]
mod tests {
    use super::keywords;
    use crate::lexer::{expect_lexer, TokenKind};

    #[test]
    fn let_keyword() {
        expect_lexer("let", keywords(), TokenKind::LetKeyword);
        expect_lexer("  let", keywords(), TokenKind::LetKeyword);
        expect_lexer("let  ", keywords(), TokenKind::LetKeyword);
    }

    #[test]
    fn def_keyword() {
        expect_lexer("def", keywords(), TokenKind::DefKeyword);
        expect_lexer("  def", keywords(), TokenKind::DefKeyword);
        expect_lexer("def  ", keywords(), TokenKind::DefKeyword);
    }

    #[test]
    fn end_keyword() {
        expect_lexer("end", keywords(), TokenKind::EndKeyword);
        expect_lexer("  end", keywords(), TokenKind::EndKeyword);
        expect_lexer("end  ", keywords(), TokenKind::EndKeyword);
    }
}
