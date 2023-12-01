use clap::Parser as CLIParser;

#[macro_use] extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;

pub mod nodes;
pub mod runner;
pub mod project;
pub mod functions;
pub mod langs;
pub mod cache;

lalrpop_mod!(pub parser); // synthesized by LALRPOP


/// Simple program to greet a person
#[derive(CLIParser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(index = 1, default_value = "$default")]
    pub method: String,

    #[arg(short, long, default_value = "./")]
    pub project: String,

    #[arg(short, long)]
    pub debug: bool,
}
