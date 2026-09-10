use std::fs;
use std::path::Path;

use crate::domain::project::Project;

use super::path::project_file;

pub fn save(project: &Project, project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path = project_file(project_path);
    let content = toml::to_string_pretty(project)?;

    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, content)?;

    Ok(())
}

pub fn load(project_path: &Path) -> Result<Project, Box<dyn std::error::Error>> {
    let path = project_file(project_path);
    let content = fs::read_to_string(path)?;
    let project = toml::from_str(&content)?;

    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::project::Project;

    #[test]
    fn save_and_load_project() {
        let project = Project::new(
            1,
            String::from("Test project"),
            String::from("Test description"),
        );

        let project_path = std::env::temp_dir().join("keepld_test_project");

        save(&project, &project_path).unwrap();

        let loaded = load(&project_path).unwrap();

        assert_eq!(project, loaded);

        fs::remove_dir_all(project_path).unwrap();
    }

    #[test]
    fn save_project_writes_expected_toml() {
        let project = Project::new(
            1,
            String::from("Test project"),
            String::from("Test description"),
        );

        let project_path = std::env::temp_dir().join("keepld_test_project_toml");

        save(&project, &project_path).unwrap();

        let content = fs::read_to_string(
            project_path.join(".keepld").join("project.toml")
        ).unwrap();

        assert!(content.contains("id = 1"));
        assert!(content.contains("name = \"Test project\""));
        assert!(content.contains("description = \"Test description\""));
        assert!(content.contains("nodes = []"));

        fs::remove_dir_all(project_path).unwrap();
    }
}


