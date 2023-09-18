use crate::runner::Runner;
use crate::Args;
use std::collections::HashMap;

pub(crate) mod python;
use crate::langs::python::get_python;

#[derive(Debug, Clone, PartialEq)]
pub struct Language {
    pub name: String,
    pub functions: HashMap<String, fn(HashMap<String, String>, Args, &mut Runner) -> ()>,
}

pub fn get_core_langs() -> Vec<Language> {
    let mut langs: Vec<Language> = vec![];

    langs.push(get_python());

    return langs;
}
