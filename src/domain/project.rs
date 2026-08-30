use super::node::Node;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub nodes: Vec<Node>,
}

impl Project {
    pub fn new(id: u64, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            nodes: Vec::new(),
        }
    }
}
