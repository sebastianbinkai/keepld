use super::project::Project;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub projects: Vec<Project>,
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
