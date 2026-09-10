use super::project_reference::ProjectReference;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub projects: Vec<ProjectReference>,
}

impl Profile {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            projects: Vec::new(),
        }
    }
}
