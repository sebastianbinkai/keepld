use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectReference {
    pub name: String,
    pub path: PathBuf,
}
