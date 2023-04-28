
use crate::project::ProjectInfo;

#[derive(Debug)]
pub struct AST {
    nodes: Vec<Node>,
    project: ProjectInfo
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
    Task(String, Vec<Node>)
}
