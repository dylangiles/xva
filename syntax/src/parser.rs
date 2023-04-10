use std::iter::Peekable;

use rowan::{GreenNode, GreenNodeBuilder, SyntaxNode};

use crate::{
    lexer::{Lexer, TokenKind},
    parser::language::SyntaxKind,
    Token,
};

use self::event::Event;

mod event;
mod language;
mod location;
mod sink;

pub(crate) struct ConcreteSyntaxTree {
    green_node: GreenNode,

    #[allow(unused)]
    errors: Vec<String>,
}

struct Parser<'parse> {
    lexer: Lexer<'parse>,
    events: Vec<Event<'parse>>,
    errors: Vec<String>,
}

impl<'parse> Parser<'parse> {
    pub(crate) fn new(lexer: Lexer<'parse>) -> Self {
        Self {
            lexer,
            events: vec![],
            errors: vec![],
        }
    }

    fn parse(mut self) -> ConcreteSyntaxTree {
        self.start_node(SyntaxKind::Root.into());

        self.finish_node();

        // TODO
        ConcreteSyntaxTree {
            green_node: GreenNode::new(SyntaxKind::Root.into(), vec![].into_iter()),
            errors: vec![],
        }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        // self.builder.start_node(kind.into())
    }

    fn finish_node(&mut self) {
        // self.builder.finish_node()
    }

    fn bump(&mut self) {
        // SAFETY: this unwrap() call is kinda fine, if we hit a None it's our fault and it's unrecoverable.
        let token = self.lexer.bump();
    }

    // fn peek(&self) -> Option<&TokenKind> {
    //     self.lexer.peek()
    // }
}

// pub(self) fn check_parse(input: &str, expected_tree: Expect) {
//     let parse = Parser::new(input).parse();
//     let syntax_node = SyntaxNode::new_root(parse.green_node);

//     let actual_tree = format!("{:#?}", syntax_node);

//     // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
//     expected_tree.assert_eq(&actual_tree[0..actual_tree.len() - 1]);
// }
