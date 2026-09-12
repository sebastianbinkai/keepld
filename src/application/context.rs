use std::path::Path;

use crate::core::context::Reference;
use crate::persistence::context as context_persistence;

pub fn focus(
    path: &Path,
    position: u8,
    reference: Reference,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = context_persistence::load(path)?;

    context.set_focused(position, reference);

    context_persistence::save(&context, path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::core::context::Reference;
    use crate::persistence::context as context_persistence;

    fn temporary_path(name: &str) -> std::path::PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir()
            .join(format!("keepld-context-test-{}-{}", name, timestamp))
    }

    fn cleanup(path: &std::path::Path) {
        let _ = fs::remove_dir_all(path);
    }

    fn initialize_context(path: &std::path::Path) {
        fs::create_dir_all(path).unwrap();

        let context = crate::core::context::Context::new();

        context_persistence::save(&context, path).unwrap();
    }

    #[test]
    fn focus_sets_primary_focus() {
        let path = temporary_path("sets-primary-focus");

        initialize_context(&path);

        focus(&path, 0, Reference::Project(1)).unwrap();

        let context = context_persistence::load(&path).unwrap();

        assert_eq!(
            context.focused(0),
            Some(Reference::Project(1))
        );

        cleanup(&path);
    }

    #[test]
    fn focus_sets_non_primary_focus() {
        let path = temporary_path("sets-non-primary-focus");

        initialize_context(&path);

        focus(&path, 1, Reference::Node(2)).unwrap();

        let context = context_persistence::load(&path).unwrap();

        assert_eq!(
            context.focused(1),
            Some(Reference::Node(2))
        );

        cleanup(&path);
    }

    #[test]
    fn focus_replaces_existing_position() {
        let path = temporary_path("replaces-focus");

        initialize_context(&path);

        focus(&path, 1, Reference::Node(2)).unwrap();
        focus(&path, 1, Reference::Node(3)).unwrap();

        let context = context_persistence::load(&path).unwrap();

        assert_eq!(
            context.focused(1),
            Some(Reference::Node(3))
        );

        cleanup(&path);
    }

    #[test]
    fn primary_focus_clears_selected_reference() {
        let path = temporary_path("primary-clears-selected");

        initialize_context(&path);

        let mut context = context_persistence::load(&path).unwrap();

        context.set_selected(Reference::Project(10));

        context_persistence::save(&context, &path).unwrap();

        focus(&path, 0, Reference::Node(20)).unwrap();

        let context = context_persistence::load(&path).unwrap();

        assert_eq!(
            context.selected(),
            None
        );

        assert_eq!(
            context.focused(0),
            Some(Reference::Node(20))
        );

        cleanup(&path);
    }

    #[test]
    fn non_primary_focus_does_not_change_primary_focus() {
        let path = temporary_path("non-primary-keeps-primary");

        initialize_context(&path);

        focus(&path, 0, Reference::Project(1)).unwrap();
        focus(&path, 1, Reference::Node(2)).unwrap();

        let context = context_persistence::load(&path).unwrap();

        assert_eq!(
            context.focused(0),
            Some(Reference::Project(1))
        );

        assert_eq!(
            context.focused(1),
            Some(Reference::Node(2))
        );

        cleanup(&path);
    }
}
