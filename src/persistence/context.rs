use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::context::{Context, Reference};

#[derive(Debug, Serialize, Deserialize)]
struct StatusFile {
    selected: Option<PersistedReference>,
    #[serde(default)]
    focused: Vec<PersistedFocus>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistedReference {
    kind: String,
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistedFocus {
    position: u8,
    kind: String,
    id: u64,
}

pub fn save(
    context: &Context,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let status = StatusFile {
        selected: context.selected().map(reference_to_persisted),
        focused: collect_focused(context),
    };

    let content = toml::to_string_pretty(&status)?;

    let keepld_dir = path.join(".keepld");
    fs::create_dir_all(&keepld_dir)?;

    fs::write(keepld_dir.join("context.toml"), content)?;

    Ok(())
}

pub fn load(
    path: &Path,
) -> Result<Context, Box<dyn std::error::Error>> {
    let status_path = path.join(".keepld").join("context.toml");

    if !status_path.exists() {
        return Ok(Context::new());
    }

    let content = fs::read_to_string(status_path)?;
    let status: StatusFile = toml::from_str(&content)?;

    let mut context = Context::new();

    if let Some(reference) = status.selected {
        context.set_selected(persisted_to_reference(reference)?);
    }

    for focus in status.focused {
        context.set_focused(
            focus.position,
            persisted_to_reference(PersistedReference {
                kind: focus.kind,
                id: focus.id,
            })?,
        );
    }

    Ok(context)
}

fn collect_focused(context: &Context) -> Vec<PersistedFocus> {
    let mut focused = Vec::new();

    for position in 0..=u8::MAX {
        if let Some(reference) = context.focused(position) {
            let persisted = reference_to_persisted(reference);

            focused.push(PersistedFocus {
                position,
                kind: persisted.kind,
                id: persisted.id,
            });
        }
    }

    focused
}

fn reference_to_persisted(reference: Reference) -> PersistedReference {
    match reference {
        Reference::Project(id) => PersistedReference {
            kind: String::from("project"),
            id,
        },
        Reference::Node(id) => PersistedReference {
            kind: String::from("node"),
            id,
        },
    }
}

fn persisted_to_reference(
    reference: PersistedReference,
) -> Result<Reference, Box<dyn std::error::Error>> {
    match reference.kind.as_str() {
        "project" => Ok(Reference::Project(reference.id)),
        "node" => Ok(Reference::Node(reference.id)),
        _ => Err(format!(
            "Unknown reference kind: {}",
            reference.kind
        )
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load_selected() {
        let path = std::env::temp_dir()
            .join("keepld_test_context_selected");

        let mut context = Context::new();
        context.set_selected(Reference::Node(42));

        save(&context, &path).unwrap();

        let loaded = load(&path).unwrap();

        assert_eq!(
            loaded.selected(),
            Some(Reference::Node(42))
        );

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn save_and_load_focused() {
        let path = std::env::temp_dir()
            .join("keepld_test_context_focused");

        let mut context = Context::new();
        context.set_focused(0, Reference::Node(17));
        context.set_focused(12, Reference::Project(99));

        save(&context, &path).unwrap();

        let loaded = load(&path).unwrap();

        assert_eq!(
            loaded.focused(0),
            Some(Reference::Node(17))
        );

        assert_eq!(
            loaded.focused(12),
            Some(Reference::Project(99))
        );

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn save_and_load_complete_context() {
        let path = std::env::temp_dir()
            .join("keepld_test_context_complete");

        let mut context = Context::new();

        context.set_selected(Reference::Node(42));
        context.set_focused(12, Reference::Project(99));

        save(&context, &path).unwrap();

        let loaded = load(&path).unwrap();

        assert_eq!(
            loaded.selected(),
            Some(Reference::Node(42))
        );

        assert_eq!(
            loaded.focused(12),
            Some(Reference::Project(99))
        );

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn save_and_load_primary_focus() {
        let path = std::env::temp_dir()
            .join("keepld_test_context_primary_focus");

        let mut context = Context::new();

        context.set_focused(0, Reference::Node(17));

        save(&context, &path).unwrap();

        let loaded = load(&path).unwrap();

        assert_eq!(
            loaded.selected(),
            None
        );

        assert_eq!(
            loaded.focused(0),
            Some(Reference::Node(17))
        );

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn loading_missing_status_creates_empty_context() {
        let path = std::env::temp_dir()
            .join("keepld_test_context_missing");

        fs::create_dir_all(&path).unwrap();

        let context = load(&path).unwrap();

        assert_eq!(context.selected(), None);
        assert_eq!(context.focused(0), None);

        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn unknown_reference_kind_is_rejected() {
        let path = std::env::temp_dir()
            .join("keepld_test_context_invalid");

        let keepld_dir = path.join(".keepld");
        fs::create_dir_all(&keepld_dir).unwrap();

        let status = r#"
[selected]
kind = "unknown"
id = 42
"#;

        fs::write(
            keepld_dir.join("context.toml"),
            status,
        )
        .unwrap();

        assert!(load(&path).is_err());

        fs::remove_dir_all(path).unwrap();
    }
}
