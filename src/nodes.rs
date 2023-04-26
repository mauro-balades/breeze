
#[derive(Debug)]
pub enum Node {
    Task(String, Vec<Node>)
}
