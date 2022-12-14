#![allow(clippy::upper_case_acronyms, clippy::unusual_byte_groupings)]

use anyhow::Result;
use clap::Parser;
use compile::to_blob;
use parsing::parse_to_ast;
use std::{path::PathBuf, process::exit};

mod ast;
mod compile;
mod parsing;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    output: Option<PathBuf>,
    path: PathBuf,
}

fn main() -> Result<()> {
    let (path, output) = {
        let args = Cli::parse();
        let path = match args.path.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
        };
        let output = args
            .output
            .unwrap_or_else(|| path.file_stem().unwrap().into());
        (path, output)
    };

    let ast = parse_to_ast(&path)?;
    to_blob(&ast, &output)?;
    Ok(())
}
