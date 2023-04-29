
use crate::project::ProjectInfo;

#[derive(Debug, Clone)]
pub struct AST {
    pub nodes: Vec<Node>,
    pub project: ProjectInfo
}

impl AST {
    pub fn new(project: ProjectInfo, nodes: Vec<Node>) -> AST {
        Self {
            nodes,
            project,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Task(String, Vec<Node>),
    DefaultTask(String),
    VariableDecl(String, Expr),
    FunctionCall(String, Vec<(String, Expr)>),
    Message(Expr)
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String)
}
