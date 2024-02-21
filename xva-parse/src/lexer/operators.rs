use chumsky::prelude::*;

use crate::token::Delimiter;

use super::{LexerExtra, TokenKind};

const VALID_CONTROL_CHARS: &str = r#"(){}:,."#;
const VALID_SINGLE_CHAR_OPERATORS: &str = r#"+-*%<>&|^"#;

pub(crate) fn control<'src>() -> impl Parser<'src, &'src str, TokenKind, LexerExtra> {
    one_of(VALID_CONTROL_CHARS).map(|c| match c {
        '(' => TokenKind::OpenDelim(Delimiter::Parentheses),
        ')' => TokenKind::CloseDelim(Delimiter::Parentheses),
        '{' => TokenKind::OpenDelim(Delimiter::Braces),
        '}' => TokenKind::CloseDelim(Delimiter::Parentheses),
        ':' => TokenKind::Colon,
        ',' => TokenKind::Comma,
        '.' => TokenKind::Dot,
        // '"' => Token::DoubleQuote,       // Quotes are handled by literals
        // '\'' => Token::SingleQuote,
        // '=' => Token::Equals,
        c => unreachable!("Not a control character: {c}"),
    })
}

pub(crate) fn operator<'src>() -> impl Parser<'src, &'src str, TokenKind, LexerExtra> {
    let single_char_op = one_of(VALID_SINGLE_CHAR_OPERATORS).map(|op| match op {
        '+' => TokenKind::Plus,
        '-' => TokenKind::Minus,
        '*' => TokenKind::Star,
        // '/' => TokenKind::Slash,
        '%' => TokenKind::Percent,
        '<' => TokenKind::LeftAngle, // Note: can also be a control character (for generics)
        '>' => TokenKind::RightAngle, // Note: can also be a control character (for generics)
        '&' => TokenKind::Ampersand,
        '|' => TokenKind::Pipe,
        '^' => TokenKind::Caret,
        _ => unreachable!("Not an operator"),
    });

    let multi_char_op = choice((
        just("**").to(TokenKind::DoubleStar),
        just("++").to(TokenKind::Increment),
        just("--").to(TokenKind::Decrement),
        just(">>").to(TokenKind::DoubleRightAngle),
        just("<<").to(TokenKind::DoubleLeftAngle),
        just(">=").to(TokenKind::GreaterThanEquals),
        just("<=").to(TokenKind::LessThanEquals),
        // just("==").to(Token::DoubleEquals),
        just("!=").to(TokenKind::NotEquals),
        just("and").to(TokenKind::And),
        just("or").to(TokenKind::Or),
        just("not").to(TokenKind::Not),
    ));

    // `=` and `==` are a weird conflicting case, they are handled specially outside
    // the parsers above.
    let double_or_single_equals = just('=')
        .then(just('=').or_not())
        .map(|(_, second)| second.map_or_else(|| TokenKind::Equals, |_| TokenKind::DoubleEquals));

    // `/` conflicts with comments
    let slash = just('/').then(just('/').not()).to(TokenKind::Slash);

    choice((
        slash,
        double_or_single_equals,
        multi_char_op,
        single_char_op,
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{tests::assert_single_no_errors, TokenKind},
        token::Delimiter,
    };

    #[test]
    fn controls() {
        assert_single_no_errors("(", &TokenKind::OpenDelim(Delimiter::Parentheses));
        assert_single_no_errors(")", &TokenKind::CloseDelim(Delimiter::Parentheses));
        assert_single_no_errors("{", &TokenKind::OpenDelim(Delimiter::Braces));
        assert_single_no_errors("}", &TokenKind::CloseDelim(Delimiter::Braces));
        assert_single_no_errors(":", &TokenKind::Colon);
        assert_single_no_errors(",", &TokenKind::Comma);
        assert_single_no_errors("=", &TokenKind::Equals);
    }

    #[test]
    fn single_char_operators() {
        assert_single_no_errors("+", &TokenKind::Plus);
        assert_single_no_errors("-", &TokenKind::Minus);
        assert_single_no_errors("*", &TokenKind::Star);
        assert_single_no_errors("/", &TokenKind::Slash);
        assert_single_no_errors("%", &TokenKind::Percent);
        assert_single_no_errors("<", &TokenKind::LeftAngle);
        assert_single_no_errors(">", &TokenKind::RightAngle);
        assert_single_no_errors("&", &TokenKind::Ampersand);
        assert_single_no_errors("|", &TokenKind::Pipe);
        assert_single_no_errors("^", &TokenKind::Caret);
    }

    #[test]
    fn multi_char_operators() {
        assert_single_no_errors("**", &TokenKind::DoubleStar);
        assert_single_no_errors("++", &TokenKind::Increment);
        assert_single_no_errors("--", &TokenKind::Decrement);
        assert_single_no_errors(">>", &TokenKind::DoubleRightAngle);
        assert_single_no_errors("<<", &TokenKind::DoubleLeftAngle);
        assert_single_no_errors(">=", &TokenKind::GreaterThanEquals);
        assert_single_no_errors("<=", &TokenKind::LessThanEquals);
        assert_single_no_errors("==", &TokenKind::DoubleEquals);
        assert_single_no_errors("!=", &TokenKind::NotEquals);
        assert_single_no_errors("and", &TokenKind::And);
        assert_single_no_errors("or", &TokenKind::Or);
        assert_single_no_errors("not", &TokenKind::Not);
    }
}
