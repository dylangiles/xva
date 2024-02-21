#![deny(unused_crate_dependencies)]
use std::{io::Write, sync::Arc};

use xva_parse::SyntaxError;
use xva_span::{SourceId, SourceMap};
pub struct Compiler {
    pub source_map: SourceMap,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            source_map: Default::default(),
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
}

#[cfg(test)]
mod tests {}
