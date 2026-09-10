use std::path::Path;

use crate::domain::node::Node;
use crate::persistence::project::{load, save};

pub struct NodeUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub fn delete_node(
    path: &Path,
    id: u64,
) -> Result<Node, Box<dyn std::error::Error>> {
    let mut project = crate::persistence::project::load(path)?;

    let index = project
        .nodes
        .iter()
        .position(|node| node.id == id)
        .ok_or_else(|| format!("Node with id {} not found", id))?;

    let node = project.nodes.remove(index);

    crate::persistence::project::save(&project, path)?;

    Ok(node)
}

pub fn update_node(
    path: &Path,
    id: u64,
    update: NodeUpdate,
) -> Result<Node, Box<dyn std::error::Error>> {
    let mut project = crate::persistence::project::load(path)?;

    let node = project
        .nodes
        .iter_mut()
        .find(|node| node.id == id)
        .ok_or_else(|| format!("Node with id {} not found", id))?;

    if let Some(name) = update.name {
        node.name = name;
    }

    if let Some(description) = update.description {
        node.description = description;
    }

    let updated = node.clone();

    crate::persistence::project::save(&project, path)?;

    Ok(updated)
}

pub fn read_node(
    path: &Path,
    id: u64,
) -> Result<Node, Box<dyn std::error::Error>> {
    let project = crate::persistence::project::load(path)?;

    project
        .nodes
        .into_iter()
        .find(|node| node.id == id)
        .ok_or_else(|| format!("Node with id {} not found", id).into())
}

pub fn create_node(
    path: &Path,
    id: u64,
    name: String,
    description: String,
) -> Result<Node, Box<dyn std::error::Error>> {
    let mut project = load(path)?;

    let node = Node::new(id, name, description);

    project.nodes.push(node.clone());

    save(&project, path)?;

    Ok(node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::project::create_project;
    use crate::persistence::project::load;

    use std::fs;

    #[test]
    fn create_node_adds_node_to_project() {
        let path = std::env::temp_dir().join("keepld_test_create_node");

        create_project(
            &path,
            1,
            String::from("Test project"),
            String::from("Test description"),
        )
        .unwrap();

        let node = create_node(
            &path,
            1,
            String::from("Test node"),
            String::from("Test node description"),
        )
        .unwrap();

        assert_eq!(node.id, 1);
        assert_eq!(node.name, "Test node");
        assert_eq!(node.description, "Test node description");

        let project = load(&path).unwrap();

        assert_eq!(project.nodes.len(), 1);
        assert_eq!(project.nodes[0], node);

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn read_node_returns_node_by_id() {
        let path = std::env::temp_dir().join("keepld_test_read_node");

        create_project(
            &path,
            1,
            String::from("Test project"),
            String::from("Test description"),
        )
        .unwrap();

        let created = create_node(
            &path,
            10,
            String::from("Test node"),
            String::from("Test node description"),
        )
        .unwrap();

        let node = read_node(&path, 10).unwrap();

        assert_eq!(node, created);

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn update_node_changes_only_specified_fields() {
        let path = std::env::temp_dir().join("keepld_test_update_node");

        create_project(
            &path,
            1,
            String::from("Test project"),
            String::from("Test description"),
        )
        .unwrap();

        create_node(
            &path,
            10,
            String::from("Original name"),
            String::from("Original description"),
        )
        .unwrap();

        let updated = update_node(
            &path,
            10,
            NodeUpdate {
                name: Some(String::from("Updated name")),
                description: None,
            },
        )
        .unwrap();

        assert_eq!(updated.id, 10);
        assert_eq!(updated.name, "Updated name");
        assert_eq!(updated.description, "Original description");

        let loaded = load(&path).unwrap();

        assert_eq!(loaded.nodes.len(), 1);
        assert_eq!(loaded.nodes[0].name, "Updated name");
        assert_eq!(loaded.nodes[0].description, "Original description");

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn delete_node_removes_node_from_project() {
        let path = std::env::temp_dir().join("keepld_test_delete_node");

        create_project(
            &path,
            1,
            String::from("Test project"),
            String::from("Test description"),
        )
        .unwrap();

        create_node(
            &path,
            10,
            String::from("Test node"),
            String::from("Test node description"),
        )
        .unwrap();

        let deleted = delete_node(&path, 10).unwrap();

        assert_eq!(deleted.id, 10);
        assert_eq!(deleted.name, "Test node");
        assert_eq!(deleted.description, "Test node description");

        let project = load(&path).unwrap();

        assert!(project.nodes.is_empty());

        fs::remove_dir_all(path).unwrap();
    }
}
