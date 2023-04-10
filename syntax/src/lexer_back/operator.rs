use super::{Token, TokenType};
use chumsky::{
    prelude::Simple,
    primitive::{choice, just, one_of},
    text::{self, whitespace},
    Parser,
};

pub(super) fn operators() -> impl Parser<char, Token, Error = Simple<char>> {
    let ops = choice((
        just('+').map_with_span(|_, span| Token::new(TokenType::AdditionOperator, span)),
        just('-').map_with_span(|_, span| Token::new(TokenType::SubtractionOperator, span)),
        // Exponentiation must be higher precedence than multiplication or lexer will produce multiplication instead
        just("**").map_with_span(|_, span| Token::new(TokenType::ExponentiationOperator, span)),
        just('*').map_with_span(|_, span| Token::new(TokenType::MultiplicationOperator, span)),
        just('/').map_with_span(|_, span| Token::new(TokenType::DivisionOperator, span)),
        just('%').map_with_span(|_, span| Token::new(TokenType::ModuloOperator, span)),
    ));

    ops.or(whitespace().ignore_then(ops).then_ignore(whitespace()))
}

#[cfg(test)]
mod tests {
    use super::operators;
    use crate::lexer_back::{expect_lexer, TokenType};

    #[test]
    fn addition_operator() {
        expect_lexer("+", operators(), TokenType::AdditionOperator);
        expect_lexer("  +", operators(), TokenType::AdditionOperator);
        expect_lexer("+  ", operators(), TokenType::AdditionOperator);
    }

    #[test]
    fn subtraction_operator() {
        expect_lexer("-", operators(), TokenType::SubtractionOperator);
        expect_lexer("  -", operators(), TokenType::SubtractionOperator);
        expect_lexer("-  ", operators(), TokenType::SubtractionOperator);
    }

    #[test]
    fn multiplication_oeprator() {
        expect_lexer("*", operators(), TokenType::MultiplicationOperator);
        expect_lexer("  *", operators(), TokenType::MultiplicationOperator);
        expect_lexer("*  ", operators(), TokenType::MultiplicationOperator);
    }

    #[test]
    fn division_operator() {
        expect_lexer("/", operators(), TokenType::DivisionOperator);
        expect_lexer("  /", operators(), TokenType::DivisionOperator);
        expect_lexer("/  ", operators(), TokenType::DivisionOperator);
    }

    #[test]
    fn modulo_operator() {
        expect_lexer("%", operators(), TokenType::ModuloOperator);
        expect_lexer("  %", operators(), TokenType::ModuloOperator);
        expect_lexer("%  ", operators(), TokenType::ModuloOperator);
    }

    #[test]
    fn exponentiation_operator() {
        expect_lexer("**", operators(), TokenType::ExponentiationOperator);
        expect_lexer("  **", operators(), TokenType::ExponentiationOperator);
        expect_lexer("**  ", operators(), TokenType::ExponentiationOperator);
    }
}
