use std::{collections::{HashMap}, process::exit};
use crate::{nodes::{{ AST, Node }, Expr}, Args};
use crate::functions::get_std_functions;
use crate::langs::{ Language, get_core_langs };
use crate::{cache};

use lazy_static;

use label_logger::{{ error }, info};
use regex::Regex;

lazy_static! {
    static ref STRING_IDENTIFIER_REGEX: Regex = Regex::new(r#"(^$|^)\$\{[a-zA-Z][a-zA-Z0-9_-]*\}"#).unwrap();
}

#[derive(Clone)]
pub struct Runner {
    ast: AST,
    method: String,

    scopes: Vec<HashMap::<String, String>>,
    tasks: HashMap<String, Node>,

    args: Args,
    functions: HashMap<String, fn(HashMap<String, String>, Args, &mut Runner) -> ()>,

    languages: Vec<Language>,
    pub cache: cache::Cache,
}

impl Runner {
    pub fn throw_error(error: String /* TODO: line, col, etc */) {
        for line in error.lines() {
            error!(label: "Error", "{}", line);
        }
        exit(0);
    }

    pub fn new(ast: AST, method: String, args: Args) -> Runner {
        Self {
            ast,
            method,
            scopes: vec![HashMap::<String, String>::new()],
            tasks: HashMap::<String, Node>::new(),
            args,
            functions: get_std_functions(),
            languages: get_core_langs(),
            cache: cache::Cache::new()
        }
    }

    pub fn initialize_data(&mut self) {
        // create .breeze folder if it doesn't exist
        let breeze_folder = ".breeze";
        if !std::path::Path::new(breeze_folder).exists() {
            std::fs::create_dir(breeze_folder).unwrap();
        }

        // create cache file if it doesn't exist
        let cache_file = format!("{}/cache.json", breeze_folder);
        if !std::path::Path::new(&cache_file).exists() {
            std::fs::write(&cache_file, serde_json::to_string(&self.cache).unwrap()).unwrap();
        }
    }

    pub fn save_data(&mut self) {
        let breeze_folder = ".breeze";
        let cache_file = format!("{}/cache.json", breeze_folder);
        std::fs::write(&cache_file, serde_json::to_string(&self.cache).unwrap()).unwrap();
    }

    pub fn load_data(&mut self) {
        let breeze_folder = ".breeze";
        let cache_file = format!("{}/cache.json", breeze_folder);
        let cache_file = std::fs::read_to_string(&cache_file).unwrap();
        self.cache = serde_json::from_str(&cache_file).unwrap();
    }

    pub fn execute_ast(&mut self) -> Result<(), &'static str> {

        let mut default_task: String = "".to_string();
        let mut clone = self.clone();
        for node in &mut self.ast.nodes {
            match node {
                Node::Task(ref name, _) => {
                    if self.tasks.contains_key(name) {
                        Self::throw_error(format!("Task '{}' has already been defined!", name));
                    }

                    self.tasks.insert(name.to_string(), node.clone());
                },
    
                Node::DefaultTask(ref name) => {
                    if !default_task.is_empty() {
                        Self::throw_error("Can't defined the default task twice!".to_owned());
                    }
    
                    if !self.tasks.contains_key(name) {
                        Self::throw_error(format!("Can't use task '{}' because it's not defined!", name));
                    }
    
                    default_task = name.to_string();
                },

                Node::VariableDecl(ref n, ref e) => {
                    let expr = clone.execute_expr(e).unwrap();
                    clone.generate_variable(n.to_string(), &expr);
                },
    
                _ => panic!("what")
            }
        }
    
        if self.method == "$default" {
            if default_task.is_empty() {
                Self::throw_error("No default task has been set for project!".to_owned());
            }
    
            self.method = default_task;
        }
    
        if !self.tasks.contains_key(&self.method) {
            Self::throw_error(format!("Can't use task '{}' because it's not defined!", self.method));
        }
    
        let executed_task = self.tasks.get(&self.method).unwrap();
        return clone.execute_task(executed_task);
    }

    pub fn generate_variable(&mut self, n: String, e: &String) {
        let scope = &mut self.scopes[0];
        let mut name = n.clone().to_string();

        if name.starts_with("$") {
            name.remove(0);
        }

        if scope.contains_key(&n) {
            Self::throw_error(format!("Variable with name '{}' has already been defined on the same scope!", n));
        }

        scope.insert(name, e.to_string());
    }

    pub fn get_variable(&self, var: String) -> String {
        let mut value: Option<String> = Option::None;
        for scope in &self.scopes {
            if scope.contains_key(&var) {
                value = scope.get(&var).cloned();
            }
        }

        if value.is_none() {
            Self::throw_error(format!("No variable with name '{}' has been found!", var));
        }

        value.unwrap()
    }

    fn execute_expr(&mut self, expr: &Expr) -> Result<String, &'static str>  {
        match expr {
            Expr::String(s) => {
                // THIS IS NOT BEING DETECTED!
                let output = STRING_IDENTIFIER_REGEX.replace_all(&s, |captures: &regex::Captures| {
                    let matched_word = &mut captures[0].to_string(); // get the matched word

                    matched_word.remove(0);
                    matched_word.remove(0);

                    matched_word.pop();

                    self.get_variable(matched_word.to_string())
                }).to_string();

                Ok(output)
            },

            _ => panic!("Invalid expression given!")
        }
    }

    fn execute_block(&mut self, block: &Vec<Node>) -> Result<(), &'static str> {
        self.scopes.insert(0, HashMap::new());

        for node in block {
            match node {
                Node::FunctionCall(n, args) => {
                    let mut argv = HashMap::new();

                    for a in args {
                        if argv.contains_key(&a.0) {
                            Self::throw_error("Arguments has been repeated on this call!".to_owned());
                        }

                        argv.insert(a.0.clone(), self.execute_expr(&a.1).unwrap());
                    }

                    if self.functions.contains_key(n) {
                        let f = self.functions.get(n).unwrap();
                        f(argv, self.args.clone(), self);
                    } else {
                        Self::throw_error(format!("Coudn't find function with name '{}'", n))
                    }
                },

                Node::LangCall(ref lang, ref ident, ref args) => {
                    let name = format!("@{}.{}", lang, ident);
                    
                    let mut found_lang = Option::None;
                    for lang_dir in &self.languages {
                        if lang_dir.name == lang.to_string() {
                            found_lang = Some(lang_dir);
                        }
                    }

                    if found_lang == Option::None {
                        Self::throw_error(format!("Language directory '{}' not found!", lang));
                    }

                    let lang = found_lang.unwrap();
                    if !lang.functions.contains_key(ident) {
                        Self::throw_error(format!("Function '{}' not found in language '{}'!", ident, lang.name));
                    }

                    let f = lang.functions.get(ident).unwrap();
                    let mut argv = HashMap::new();
                    for a in args {
                        if argv.contains_key(&a.0) {
                            Self::throw_error("Arguments has been repeated on this call!".to_owned());
                        }

                        argv.insert(a.0.clone(), self.clone().execute_expr(&a.1).unwrap());
                    }
                    f(argv, self.args.clone(), self);
                }   

                Node::Message(ref e) => {
                    let expr = self.execute_expr(e).unwrap();
                    let lines = expr.lines();

                    if expr.contains("\n") {
                        for l in lines {
                            info!(label: "=>", "{}", l);
                        }
                    } else {
                        info!(label: "==>", "{}", expr);
                    }
                },

                Node::VariableDecl(ref n, ref e) => {
                    let expr = self.execute_expr(e).unwrap();
                    self.generate_variable(n.to_string(), &expr);
                },

                _ => panic!("Invalid node parsed!")
            }
        }

        self.scopes.remove(0);
        Ok(())
    }

    fn execute_task(mut self, executed_task: &Node) -> Result<(), &'static str> {
        match executed_task {
            // It should only be this task
            Node::Task(ref _name, ref nodes) => {
                let res = self.execute_block(nodes).unwrap();
                self.save_data();
                res
            },

            _ => panic!("Invalid task given!")
        }

        Ok(())
    }

    pub fn check_arguments(givenArgs: &HashMap<String, String>, requiredArgs: HashMap<&str, bool>, called_method: &str) {
        for (name, required) in givenArgs {
            if !requiredArgs.contains_key(name.as_str()) {
                Self::throw_error(format!("Argument '{}' is not registered for method '{}'!", name, called_method));
            }
        }

        for (name, required) in requiredArgs {
            if required && !givenArgs.contains_key(name) {
                Self::throw_error(format!("Argument '{}' is required for method '{}'!", name, called_method));
            }
        }
    }
}
