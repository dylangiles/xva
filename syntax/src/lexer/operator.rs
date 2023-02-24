use super::{Token, TokenKind};
use chumsky::{
    prelude::Simple,
    primitive::{choice, just, one_of},
    text, Parser,
};


pub(crate) fn operator() -> impl Parser<char, Token, Error = Simple