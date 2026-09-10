use std::fs;
use std::path::Path;

use crate::domain::project::Project;
use crate::persistence::project::save;
use crate::core::context::Context;

pub struct ProjectUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub fn initialize_project(
    path: &Path,
    id: u64,
    name: String,
    description: String,
) -> Result<Project, Box<dyn std::error::Error>> {
    let project = Project::new(id, name, description);

    save(&project, path)?;

    let context = Context::new();
    crate::persistence::context::save(&context, path)?;

    Ok(project)
}

pub fn delete_project(
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let keepld_path = path.join(".keepld");

    if !keepld_path.exists() {
        return Err("not a Keepld project".into());
    }

    fs::remove_dir_all(keepld_path)?;

    Ok(())
}

pub fn update_project(
    path: &Path,
    update: ProjectUpdate,
) -> Result<Project, Box<dyn std::error::Error>> {
    let mut project = crate::persistence::project::load(path)?;

    if let Some(name) = update.name {
        project.name = name;
    }

    if let Some(description) = update.description {
        project.description = description;
    }

    save(&project, path)?;

    Ok(project)
}

pub fn read_project(
    path: &Path,
) -> Result<Project, Box<dyn std::error::Error>> {
    crate::persistence::project::load(path)
}

pub fn create_project(
    path: &Path,
    id: u64,
    name: String,
    description: String,
) -> Result<Project, Box<dyn std::error::Error>> {
    fs::create_dir_all(path)?;

    initialize_project(path, id, name, description)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_project_creates_project_directory() {
        let path = std::env::temp_dir().join("keepld_test_create_project");

        let project = create_project(
            &path,
            1,
            String::from("Test project"),
            String::from("Test description"),
        )
        .unwrap();

        assert_eq!(project.id, 1);
        assert_eq!(project.name, "Test project");
        assert_eq!(project.description, "Test description");

        assert!(path.exists());
        assert!(path.join(".keepld").exists());
        assert!(path.join(".keepld").join("project.toml").exists());

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn update_project_changes_only_specified_fields() {
        let path = std::env::temp_dir().join("keepld_test_update_project");

        create_project(
            &path,
            1,
            String::from("Original name"),
            String::from("Original description"),
        )
        .unwrap();

        let updated = update_project(
            &path,
            ProjectUpdate {
                name: Some(String::from("Updated name")),
                description: None,
            },
        )
        .unwrap();

        assert_eq!(updated.name, "Updated name");
        assert_eq!(updated.description, "Original description");

        let loaded = crate::persistence::project::load(&path).unwrap();

        assert_eq!(loaded.name, "Updated name");
        assert_eq!(loaded.description, "Original description");

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn delete_project_removes_keepld_directory() {
        let path = std::env::temp_dir().join("keepld_test_delete_project");

        create_project(
            &path,
            1,
            String::from("Test project"),
            String::from("Test description"),
        )
        .unwrap();

        let external_file = path.join("external.txt");
        fs::write(&external_file, "keep me").unwrap();

        delete_project(&path).unwrap();

        assert!(!path.join(".keepld").exists());
        assert!(path.exists());
        assert!(external_file.exists());

        fs::remove_dir_all(path).unwrap();
    }
}
