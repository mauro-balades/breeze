use crate::runner::Runner;
use crate::Args;
use std::collections::HashMap;

#[derive(Debug)]
struct Language {
    name: String,
    functions: HashMap<String, fn(HashMap<String, String>, Args, &mut Runner) -> ()>,
}