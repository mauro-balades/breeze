use clap::Parser as CLIParser;
use std::path::Path;

use std::fs;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser); // synthesized by LALRPOP

/// Simple program to greet a person
#[derive(CLIParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    file: String,
}

pub fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file = args.file;

    let breeze_file = Path::new(&file).join("project.breeze");
    let program_text = fs::read_to_string(breeze_file).expect("Unable to read the program file");

    let expr = parser::BreezeParser::new()
        .parse(&program_text)
        .unwrap();
    Ok(())
}
