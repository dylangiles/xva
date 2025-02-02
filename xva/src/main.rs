#![deny(unused_crate_dependencies)]

use clap::Parser;
use std::io::{BufRead, Write};
use xva_compiler::Compiler;

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

    loop {
        let _stdout_lock = stdout.lock();
        stdout.write_all("> ".as_bytes())?;
        stdout.flush()?;

        let line = stdin.lock().lines().next().unwrap()?;
        let mut compiler = Compiler::default();
        let src_id = compiler.load_virtual_file(REPL_SOURCE_NAME.into(), line);

        let (tree, errors) = xva_parse::parser::parse(
            compiler.get_file_content(src_id).unwrap().as_ref(),
            src_id,
            pretty_lex,
        );

        if pretty_ast {
            println!("{tree:#?}")
        }

        if errors.len() != 0 {
            for error in errors {
                let writer = stdout.lock();
                compiler.write_syntax_error(error, writer);
            }
        }
    }
}
