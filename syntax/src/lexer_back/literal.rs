use super::{Token, TokenType};
use chumsky::{
    prelude::Simple,
    primitive::{choice, just, one_of},
    text, Parser,
};

fn base_ten() -> impl Parser<char, Token, Error = Simple<char>> {
    text::int(10)
        .from_str()
        .unwrapped()
        .map_with_span(|v, span| Token::new(TokenType::Integer(v), span))
}

const HEX_PREFIX_LOWER: &str = "0x";
const HEX_PREFIX_UPPER: &str = "0X";
const OCTAL_PREFIX_LOWER: &str = "0o";
const OCTAL_PREFIX_UPPER: &str = "0O";
const BINARY_PREFIX_LOWER: &str = "0b";
const BINARY_PREFIX_UPPER: &str = "0B";

const HEX_LIST: &str = "0123456789abcdefABCDEF";
const OCTAL_LIST: &str = "01234567";
const BINARY_LIST: &str = "01";

// 😍

fn hexadecimal() -> impl Parser<char, Token, Error = Simple<char>> {
    just(HEX_PREFIX_LOWER)
        .or(just(HEX_PREFIX_UPPER))
        .then(one_of(HEX_LIST).repeated().collect::<String>())
        .map_with_span(
            |(_, val), span| match i64::from_str_radix(val.as_str(), 16) {
                Ok(v) => Token::new(TokenType::Integer(v), span),
                Err(e) => unreachable!("Hexadecimal integer parse error: {}", e),
            },
        )
}

fn octal() -> impl Parser<char, Token, Error = Simple<char>> {
    just(OCTAL_PREFIX_LOWER)
        .or(just(OCTAL_PREFIX_UPPER))
        .then(one_of(OCTAL_LIST).repeated().collect::<String>())
        .map_with_span(
            |(_, val), span| match i64::from_str_radix(val.as_str(), 8) {
                Ok(v) => Token::new(TokenType::Integer(v), span),
                Err(e) => unreachable!("Octal integer parse error: {}", e),
            },
        )
}

fn binary() -> impl Parser<char, Token, Error = Simple<char>> {
    just(BINARY_PREFIX_LOWER)
        .or(just(BINARY_PREFIX_UPPER))
        .then(one_of(BINARY_LIST).repeated().collect::<String>())
        .map_with_span(
            |(_, val), span| match i64::from_str_radix(val.as_str(), 2) {
                Ok(v) => Token::new(TokenType::Integer(v), span),
                Err(e) => unreachable!("Binary integer parse error: {}", e),
            },
        )
}

// TODO: remove
// fn float() -> impl Parser<char, Token, Error = Simple<char>> {
//     text::int(10)
//         .then(just('.').then(text::int(10)))
//         .map_with_span(|(whole, (dot, frac)), span| {
//             let number = format!("{}{}{}", whole, dot, frac);
//             match number.parse::<f64>() {
//                 Ok(v) => Token::new(TokenType::Float(v), span),
//                 Err(e) => unreachable!("Float parsing error: {}", e),
//             }
//         })
// }

fn float_without_exponent() -> impl Parser<char, Token, Error = Simple<char>> {
    text::int(10)
        .then(just('.').then(text::int(10)))
        .map_with_span(|(whole, (dot, frac)), span| {
            let number = format!("{}{}{}", whole, dot, frac);
            match number.parse::<f64>() {
                Ok(v) => Token::new(TokenType::Float(v), span),
                Err(e) => unreachable!("Float parsing error: {}", e),
            }
        })
}

fn float_with_exponent() -> impl Parser<char, Token, Error = Simple<char>> {
    float_without_exponent()
        .then(one_of("Ee"))
        .then(
            one_of("-0123456789")
                .repeated()
                .collect::<String>()
                .map_with_span(|val, _| match val.parse::<i64>() {
                    Ok(v) => v,
                    Err(e) => unreachable!("Could not parse exponent: {}", e),
                }),
        )
        .map_with_span(|((tok, _), power), span| match tok.token_type {
            TokenType::Float(f) => Token::new(TokenType::Float(f * 10f64.powf(power as f64)), span),
            _ => unreachable!("Whole component could not be parsed"),
        })
}

fn boolean() -> impl Parser<char, Token, Error = Simple<char>> {
    just("true")
        .or(just("false"))
        .map_with_span(|val, span| match val {
            "true" => Token::new(TokenType::Boolean(true), span),
            "false" => Token::new(TokenType::Boolean(false), span),
            e => unreachable!("Invalid boolean literal: {}", e),
        })
}

fn integer() -> impl Parser<char, Token, Error = Simple<char>> {
    choice((hexadecimal(), octal(), binary(), base_ten()))
}

pub(super) fn literal() -> impl Parser<char, Token, Error = Simple<char>> {
    choice((
        boolean(),
        float_with_exponent(),
        float_without_exponent(),
        integer(),
    ))
}

#[cfg(test)]
mod tests {
    use super::literal;
    use crate::lexer_back::{Token, TokenType};
    use chumsky::{prelude::Simple, Parser};

    fn expect_parser(
        input: &str,
        parser: impl Parser<char, Token, Error = Simple<char>>,
        expect_case: TokenType,
    ) {
        let (val, _) = parser.parse_recovery(input);
        match val {
            Some(tok) => assert_eq!(tok.token_type, expect_case),
            None => panic!("parse_recovery returned a None case."),
        }
    }

    #[test]
    fn base_ten_integers() {
        expect_parser("123", literal(), TokenType::Integer(123))
    }

    #[test]
    fn hex_integers() {
        expect_parser("0x123", literal(), TokenType::Integer(291));
        expect_parser("0X123", literal(), TokenType::Integer(291));
    }

    #[test]
    fn octal_integers() {
        expect_parser("0o123", literal(), TokenType::Integer(83));
        expect_parser("0O123", literal(), TokenType::Integer(83))
    }

    #[test]
    fn binary_integers() {
        expect_parser("0b1010", literal(), TokenType::Integer(10));
        expect_parser("0B1010", literal(), TokenType::Integer(10))
    }

    #[test]
    fn floats() {
        expect_parser("12.34", literal(), TokenType::Float(12.34))
    }

    #[test]
    fn scientific_float() {
        expect_parser("1.23e2", literal(), TokenType::Float(123.0));
        expect_parser("1.23E-2", literal(), TokenType::Float(0.0123))
    }

    #[test]
    fn booleans() {
        expect_parser("true", literal(), TokenType::Boolean(true));
        expect_parser("false", literal(), TokenType::Boolean(false));
    }
}
