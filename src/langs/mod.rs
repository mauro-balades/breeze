use crate::runner::Runner;
use crate::Args;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Language {
    pub name: String,
    pub functions: HashMap<String, fn(HashMap<String, String>, Args, &mut Runner) -> ()>,
}