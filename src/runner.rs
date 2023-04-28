use std::{collections::HashMap, process::exit};
use crate::nodes::{{ AST, Node }, Expr};

use lazy_static;

use label_logger::{{ error }};
use regex::Regex;

lazy_static! {
    static ref STRING_IDENTIFIER_REGEX: Regex = Regex::new(r"[^$]\$\{[a-zA-Z][a-zA-Z0-9_-]*\}").unwrap();
}

#[derive(Debug, Clone)]
pub struct Runner {
    ast: AST,
    method: String,

    scopes: Vec<HashMap::<String, String>>,
    tasks: HashMap<String, Node>
}

impl Runner {
    fn throw_error(error: String /* TODO: line, col, etc */) {
        error!(label: "Execution error", "{}", error);
        exit(0);
    }

    pub fn new(ast: AST, method: String) -> Runner {
        Self {
            ast,
            method,
            scopes: vec![HashMap::<String, String>::new()],
            tasks: HashMap::<String, Node>::new(),
        }
    }

    pub fn execute_ast(&mut self) {
        let nodes = &self.ast.nodes;

        let mut defaultTask: String = "".to_string();
        for node in nodes {
            match node {
                Node::Task(ref name, _) => {
                    if self.tasks.contains_key(name) {
                        Self::throw_error(format!("Task '{}' has already been defined!", name));
                    }

                    self.tasks.insert(name.to_string(), node.clone());
                },
    
                Node::DefaultTask(ref name) => {
                    if !defaultTask.is_empty() {
                        Self::throw_error("Can't defined the default task twice!".to_owned());
                    }
    
                    if !self.tasks.contains_key(name) {
                        Self::throw_error(format!("Can't use task '{}' because it's not defined!", name));
                    }
    
                    defaultTask = name.to_string();
                },
    
                _ => panic!("what")
            }
        }
    
        if self.method == "$default" {
            if defaultTask.is_empty() {
                Self::throw_error("No default task has been set for project!".to_owned());
            }
    
            self.method = defaultTask;
        }
    
        if !self.tasks.contains_key(&self.method) {
            Self::throw_error(format!("Can't use task '{}' because it's not defined!", self.method));
        }
    
        let executed_task = self.tasks.get(&self.method).unwrap();
        self.clone().execute_task(executed_task);
    }

    fn get_variable(&self, var: String) -> String {
        let mut value: Option<String> = Option::None;
        for scope in &self.scopes {
            if (scope.contains_key(&var)) {
                value = scope.get(&var).cloned();
            }
        }


        if value.is_none() {
            Self::throw_error(format!("No variable with name '{}' has been found!", var));
        }

        value.unwrap()
    }

    fn execute_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::String(s) => {
                // STRING_IDENTIFIER_REGEX.
                let output = STRING_IDENTIFIER_REGEX.replace_all(s, |captures: &regex::Captures| {
                    let matched_word = &mut captures[0].to_string(); // get the matched word

                    matched_word.remove(0);
                    matched_word.remove(0);
                    matched_word.remove(0);

                    matched_word.pop();

                    self.get_variable(matched_word.to_string())
                });

                println!("s: {}", output);
                output.to_string()
            },

            _ => panic!("Invalid expression given!")
        }
    }

    fn execute_block(&mut self, block: &Vec<Node>) {
        self.scopes.insert(0, HashMap::new());

        for node in block {
            match node {
                Node::Command(expr) => {
                    let val = self.execute_expr(expr);
                },

                _ => panic!("Invalid node parsed!")
            }
        }

        self.scopes.remove(0);
    }

    fn execute_task(mut self, executed_task: &Node) {
        match executed_task {
            // It should only be this task
            Node::Task(ref name, ref nodes) => {
                self.execute_block(nodes)
            },

            _ => panic!("Invalid task given!")
        }
    }

}
