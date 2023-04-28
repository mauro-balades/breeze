use std::collections::HashMap;
use crate::nodes::{{ AST, Node }};

use label_logger::{{ error }};

struct Runner {
    ast: AST,
    method: String,

    scopes: Vec<HashMap::<String, String>>,
    tasks: HashMap<String, Node>
}

impl Runner {
    fn throw_error(error: String /* TODO: line, col, etc */) {
        error!(label: "Execution error", "{}", error);
    }

    pub fn new(ast: AST, method: String) -> Runner {
        Self {
            ast,
            method,
            scopes: vec![HashMap::<String, String>::new()],
            tasks: HashMap::<String, Node>::new(),
        }
    }

    fn execute_ast(&mut self) {
        let nodes = self.ast.nodes;

        let mut defaultTask: String = "".to_string();
        for node in nodes {
            match node {
                Node::Task(ref name, _) => {
                    if self.tasks.contains_key(name) {
                        Self::throw_error(format!("Task '{}' has already been defined!", name));
                    }

                    self.tasks.insert(name.to_string(), node);
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
        self.execute_task(executed_task);
    }

    // fn execute_expr(self, )

    fn execute_block(self, block: &Vec<Node>) {
        for node in block {
            match node {
                _ => panic!("Invalid node parsed!")
            }
        }
    }

    fn execute_task(self, executed_task: &Node) {
        match executed_task {
            // It should only be this task
            Node::Task(ref name, ref nodes) => {
                self.execute_block(nodes)
            },

            _ => panic!("Invalid task given!")
        }
    }

}
