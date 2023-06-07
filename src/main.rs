
use std::path::Path;
use breeze::{nodes::AST, runner::Runner, parser, Args};
use clap::Parser;
use label_logger::{success};

use std::fs;

pub fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file = args.project.clone();

    let breeze_file = Path::new(&file).join("project.breeze");
    let program_text = fs::read_to_string(breeze_file).expect("Unable to read the program file");

    let (project, v) = parser::BreezeParser::new()
        .parse(&program_text)
        .unwrap();

    let ast = AST::new(project, v);
    success!(label: "Building", "Project {} with version '{}'", ast.project.name, ast.project.version);

    let m = args.method.clone();
    let _ = Runner::new(ast, m, args).execute_ast();
    Ok(())
}
