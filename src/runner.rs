use std::{collections::{HashMap, HashSet}, process::exit, hash::Hash, fmt};
use crate::{nodes::{{ AST, Node }, Expr}, Args};
use crate::functions::get_std_functions;

use lazy_static;

use label_logger::{{ error }, log};
use regex::Regex;

lazy_static! {
    static ref STRING_IDENTIFIER_REGEX: Regex = Regex::new(r"[^$]\$\{[a-zA-Z][a-zA-Z0-9_-]*\}").unwrap();
}


#[derive(Clone)]
pub struct Runner {
    ast: AST,
    method: String,

    scopes: Vec<HashMap::<String, String>>,
    tasks: HashMap<String, Node>,

    args: Args,
    functions: HashMap<String, fn(HashMap<String, String>, Args, &mut Runner) -> ()>
}

impl Runner {
    pub fn throw_error(error: String /* TODO: line, col, etc */) {
        error!(label: "Execution error", "{}", error);
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
        }
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
        name.remove(0);

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
            println!("{:?}", self.scopes);
            Self::throw_error(format!("No variable with name '{}' has been found!", var));
        }

        value.unwrap()
    }

    fn execute_expr(&mut self, expr: &Expr) -> Result<String, &'static str>  {
        match expr {
            Expr::String(s) => {
                // THIS IS NOT BEING DETECTED!
                let output = STRING_IDENTIFIER_REGEX.replace_all(s, |captures: &regex::Captures| {
                    let matched_word = &mut captures[0].to_string(); // get the matched word

                    matched_word.remove(0);
                    matched_word.remove(0);
                    matched_word.remove(0);

                    matched_word.pop();

                    self.get_variable(matched_word.to_string())
                });

                Ok(output.to_string())
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

                Node::Message(ref e) => {
                    let expr = self.execute_expr(e).unwrap();
                    log!("{}", expr);
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
            Node::Task(ref name, ref nodes) => {
                self.execute_block(nodes).unwrap()
            },

            _ => panic!("Invalid task given!")
        }

        Ok(())
    }

}
