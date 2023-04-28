use std::collections::HashMap;
use crate::nodes::{{ AST, Node }};

use label_logger::{{ error }};

fn throw_error(error: String /* TODO: line, col, etc */) {
    error!(label: "Execution error", "{}", error);
}

pub fn execute_ast(ast: AST, mut method: String) {
    let nodes = ast.nodes;
    let mut tasks = HashMap::<String, Node>::new();
    let mut variables = vec![HashMap::<String, String>::new()];

    let mut defaultTask: String = "".to_string();

    for node in nodes {
        match node {
            Node::Task(ref name, _) => {
                if tasks.contains_key(name) {
                    throw_error(format!("Task '{}' has already been defined!", name));
                }

                tasks.insert(name.to_string(), node);
            },

            Node::DefaultTask(ref name) => {
                if !defaultTask.is_empty() {
                    throw_error("Can't defined the default task twice!".to_owned());
                }

                if !tasks.contains_key(name) {
                    throw_error(format!("Can't use task '{}' because it's not defined!", name));
                }

                defaultTask = name.to_string();
            },

            _ => panic!("what")
        }
    }

    if (method == "$default") {
        if defaultTask.is_empty() {
            throw_error("No defaunt task has been set for project!".to_owned());
        }

        method = defaultTask;
    }


}
