#![deny(unused_crate_dependencies)]
use std::{
    io::{stdout, Stdout, Write},
    sync::Arc,
};

use xva_hir::error::HirError;
use xva_parse::SyntaxError;
use xva_span::{SourceId, SourceMap};
pub struct Compiler {
    pub source_map: SourceMap,
    stdout: Stdout,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            source_map: Default::default(),
            stdout: stdout(),
        }
    }
}

impl Compiler {
    pub fn load_virtual_file(&mut self, name: String, src: String) -> SourceId {
        self.source_map.load_virtual(name, src)
    }

    pub fn get_file_content(&self, id: SourceId) -> Option<Arc<str>> {
        self.source_map.get_raw(&id)
    }

    pub fn write_syntax_error(&self, error: SyntaxError, writer: impl Write) {
        error.write(&self.source_map, writer);
    }

    pub fn write_hir_error(&self, error: HirError, writer: impl Write) {
        error.write(&self.source_map, writer);
    }

    pub fn parse(
        &self,
        src_id: SourceId,
        pretty_lex: bool,
        pretty_ast: bool,
    ) -> Vec<xva_ast::ast::Item> {
        let (tree, errors) = xva_parse::parser::parse(
            self.get_file_content(src_id).unwrap().as_ref(),
            src_id,
            pretty_lex,
        );

        if pretty_ast {
            println!("{tree:#?}")
        }

        if errors.len() != 0 {
            for error in errors {
                let writer = self.stdout.lock();
                self.write_syntax_error(error, writer);
            }
        }

        tree
    }
}

#[cfg(test)]
mod tests {}
