#[macro_use] extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;

pub mod nodes;
pub mod runner;
pub mod project;

lalrpop_mod!(pub parser); // synthesized by LALRPOP
