use super::{Token, TokenType};
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
        just(KEYWORD_LET).map_with_span(|_, span| Token::new(TokenType::LetKeyword, span)),
        just(KEYWORD_DEF).map_with_span(|_, span| Token::new(TokenType::DefKeyword, span)),
        just(KEYWORD_END).map_with_span(|_, span| Token::new(TokenType::EndKeyword, span)),
    ));

    kws.or(whitespace().ignore_then(kws).then_ignore(whitespace()))
}

#[cfg(test)]
mod tests {
    use super::keywords;
    use crate::lexer_back::{expect_lexer, TokenType};

    #[test]
    fn let_keyword() {
        expect_lexer("let", keywords(), TokenType::LetKeyword);
        expect_lexer("  let", keywords(), TokenType::LetKeyword);
        expect_lexer("let  ", keywords(), TokenType::LetKeyword);
    }

    #[test]
    fn def_keyword() {
        expect_lexer("def", keywords(), TokenType::DefKeyword);
        expect_lexer("  def", keywords(), TokenType::DefKeyword);
        expect_lexer("def  ", keywords(), TokenType::DefKeyword);
    }

    #[test]
    fn end_keyword() {
        expect_lexer("end", keywords(), TokenType::EndKeyword);
        expect_lexer("  end", keywords(), TokenType::EndKeyword);
        expect_lexer("end  ", keywords(), TokenType::EndKeyword);
    }
}
