
#![cfg(feature = "alloc")]

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
  combinator::{cut, map, map_res, opt},
  error::{context, VerboseError},
  multi::many,
  sequence::{delimited, preceded, terminated},
  IResult, Parser,
};



/// Defining the types that define the shape of data that we want.
/// In this case, we want something tree-like

/// Starting from the most basic, we define some built-in functions that our lisp has
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BuiltIn {
  Plus,
  Minus,
  Times,
  Divide,
  Equal,
  Not,
}

/// We now wrap this type and a few other primitives into our Atom type.
/// Remember from before that Atoms form one half of our language.

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Atom {
  Num(i32),
  Keyword(String),
  Boolean(bool),
  BuiltIn(BuiltIn),
}
