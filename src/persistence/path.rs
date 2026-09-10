use std::path::{Path, PathBuf};

pub fn metadata_dir(project_path: &Path) -> PathBuf {
    project_path.join(".keepld")
}

pub fn project_file(project_path: &Path) -> PathBuf {
    metadata_dir(project_path).join("project.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_file_points_to_keepld_directory() {
        let project_path = Path::new("/tmp/my-project");

        assert_eq!(
            project_file(project_path),
            PathBuf::from("/tmp/my-project/.keepld/project.toml")
        );
    }
}
