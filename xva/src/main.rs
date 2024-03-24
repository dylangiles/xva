#![deny(unused_crate_dependencies)]

use clap::Parser;
use std::io::{BufRead, Write};
use xva_compiler::Compiler;
use xva_hir::{HirContext, Item};

mod opts;

use opts::Options;

const BUILD_INFO: &str = include_str!("../.buildinfo");

fn main() -> Result<(), std::io::Error> {
    println!("{BUILD_INFO}");

    let opts = Options::parse();
    run_repl(&opts)?;

    Ok(())
}

const REPL_SOURCE_NAME: &str = "<repl>";
fn run_repl(opts: &Options) -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let pretty_lex = opts.unstable_option_contains("pretty", "lex");
    let pretty_ast = opts.unstable_option_contains("pretty", "ast");
    let pretty_hir = opts.unstable_option_contains("pretty", "hir");

    loop {
        let _stdout_lock = stdout.lock();
        stdout.write_all("> ".as_bytes())?;
        stdout.flush()?;

        let line = stdin.lock().lines().next().unwrap()?;
        let mut compiler = Compiler::default();
        let src_id = compiler.load_virtual_file(REPL_SOURCE_NAME.into(), line);

        let ast = compiler.parse(src_id, pretty_lex, pretty_ast);

        let mut hcx = HirContext::new();
        let hir = ast
            .into_iter()
            .map(|item| match hcx.lower(item) {
                Ok(item) => Some(item),
                Err(error) => {
                    let writer = stdout.lock();
                    compiler.write_hir_error(error, writer);
                    None
                }
            })
            .filter_map(|item| item)
            .collect::<Vec<_>>();

        if pretty_hir && hir.len() > 0 {
            println!("{hir:#?}");
        }
    }
}
