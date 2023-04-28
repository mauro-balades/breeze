
use crate::project::ProjectInfo;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Node {
    Task(String, Vec<Node>),
    DefaultTask(String)
}
