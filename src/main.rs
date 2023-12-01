
use std::path::Path;
use breeze::{nodes::AST, runner::Runner, parser, Args};
use clap::Parser;
use label_logger::{success};
use std::env;
use std::fs;

pub fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let binding = args.project.clone();
    let file = Path::new(&binding);

    let breeze_file = file.join("project.breeze");
    let program_text = fs::read_to_string(breeze_file).expect("Unable to read the program file");
    assert!(env::set_current_dir(&file).is_ok());

    let (project, v) = parser::BreezeParser::new()
        .parse(&program_text)
        .unwrap();

    let ast = AST::new(project, v);
    success!(label: "Building", "Project {} with version '{}'", ast.project.name, ast.project.version);

    let m = args.method.clone();
    let mut r = Runner::new(ast, m, args);
    r.initialize_data();
    r.load_data();
    r.execute_ast();
    Ok(())
}
