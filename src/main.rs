use clap::Parser as CLIParser;
use std::path::Path;
use breeze::{nodes::AST, project::ProjectInfo, runner::execute_ast};
use label_logger::{info, log, success};

use std::fs;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser); // synthesized by LALRPOP

/// Simple program to greet a person
#[derive(CLIParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1, default_value = "$default")]
    method: String,

    #[arg(short, long, default_value = "./")]
    project: String,
}

pub fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file = args.project;

    let breeze_file = Path::new(&file).join("project.breeze");
    let program_text = fs::read_to_string(breeze_file).expect("Unable to read the program file");

    let (project, v) = parser::BreezeParser::new()
        .parse(&program_text)
        .unwrap();

    let ast = AST::new(project, v);
    success!(label: "Building", "Project {} with version '{}'", ast.project.name, ast.project.version);


    Runner::new(ast, args.method).execute_ast();
    Ok(())
}
