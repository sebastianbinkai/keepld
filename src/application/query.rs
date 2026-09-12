use std::path::Path;

use crate::application::project::read_project;
use crate::application::node::read_node;
use crate::core::context::Reference;
use crate::core::resolver::resolve;
use crate::core::resolver::ReferenceQuery;
use crate::domain::project::Project;
use crate::domain::node::Node;

/// Describe qué objeto quiere consultar la aplicación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    pub target: QueryTarget,
    pub scope: QueryScope,
    pub detail: QueryDetail,
}

/// Indica dónde debe buscarse el objeto consultado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryTarget {
    /// Utiliza el objeto determinado por el contexto actual.
    Current,

    /// Utiliza una posición concreta del foco.
    Focus(u8),

    /// Utiliza una referencia proporcionada directamente.
    Explicit(Reference),
}

/// Indica qué parte del objeto debe devolver la consulta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryScope {
    Object,
    Contents,
    Tree,
}

/// Indica cuánto detalle debe contener el resultado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryDetail {
    Normal,
    Verbose,
}

impl Query {
    /// Crea una consulta sobre el objeto del contexto actual.
    pub fn current() -> Self {
        Self {
            target: QueryTarget::Current,
            scope: QueryScope::Object,
            detail: QueryDetail::Normal,
        }
    }

    /// Crea una consulta sobre una posición concreta del foco.
    pub fn focus(position: u8) -> Self {
        Self {
            target: QueryTarget::Focus(position),
            scope: QueryScope::Object,
            detail: QueryDetail::Normal,
        }
    }

    /// Crea una consulta sobre una referencia concreta.
    pub fn explicit(reference: Reference) -> Self {
        Self {
            target: QueryTarget::Explicit(reference),
            scope: QueryScope::Object,
            detail: QueryDetail::Normal,
        }
    }

    /// Convierte el destino de la consulta en una petición
    /// que puede interpretar el Resolver.
    pub fn reference_query(&self) -> ReferenceQuery {
        match &self.target {
            QueryTarget::Current => ReferenceQuery::Current,

            QueryTarget::Focus(position) => {
                ReferenceQuery::Focus(*position)
            }

            QueryTarget::Explicit(reference) => {
                ReferenceQuery::Explicit(*reference)
            }
        }
    }
}

pub fn execute(
    query: Query,
    path: &Path,
) -> Result<QueryResult, Box<dyn std::error::Error>> {
    let mut context = crate::persistence::context::load(path)?;

    let reference = crate::core::resolver::resolve(
        query.reference_query(),
        &mut context,
    )?;

    match reference {
        Reference::Project(id) => {
            let project = crate::application::project::read_project(path)?;

            if project.id != id {
                return Err(format!(
                    "Project {} not found",
                    id
                ).into());
            }

            Ok(QueryResult::Project(project))
        }

        Reference::Node(id) => {
            let node = crate::application::node::read_node(path, id)?;

            Ok(QueryResult::Node(node))
        }
    }
}

/// Resultado estructurado de una consulta.
///
/// La aplicación devuelve datos.
/// La CLI decide cómo representarlos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryResult {
    Project(Project),
    Node(Node),
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::application::node::create_node;
    use crate::application::project::initialize_project;
    use crate::core::context::Context;
    use crate::core::context::Reference;
    use crate::persistence::context as context_persistence;

    fn temporary_path(name: &str) -> std::path::PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!(
            "keepld-query-test-{}-{}",
            name, timestamp
        ))
    }

fn cleanup(path: &std::path::Path) {
    let _ = fs::remove_dir_all(path);
}

    #[test]
    fn current_query_uses_current_target() {
        let query = Query::current();

        assert_eq!(query.target, QueryTarget::Current);
        assert_eq!(query.scope, QueryScope::Object);
        assert_eq!(query.detail, QueryDetail::Normal);
    }

    #[test]
    fn current_query_becomes_current_reference_query() {
        let query = Query::current();

        assert_eq!(
            query.reference_query(),
            ReferenceQuery::Current
        );
    }

    #[test]
    fn focus_query_uses_focus_target() {
        let query = Query::focus(0);

        assert_eq!(
            query.target,
            QueryTarget::Focus(0)
        );
    }

    #[test]
    fn focus_query_becomes_focus_reference_query() {
        let query = Query::focus(0);

        assert_eq!(
            query.reference_query(),
            ReferenceQuery::Focus(0)
        );
    }

    #[test]
    fn explicit_query_keeps_reference() {
        let reference = Reference::Project(42);
        let query = Query::explicit(reference);

        assert_eq!(
            query.reference_query(),
            ReferenceQuery::Explicit(reference)
        );
    }

    #[test]
    fn execute_current_returns_selected_project() {
        let path = temporary_path("current-project");

        initialize_project(
            &path,
            1,
            String::from("Test Project"),
            String::from("Description"),
        )
        .unwrap();

        let mut context = Context::new();
        context.set_selected(Reference::Project(1));

        context_persistence::save(&context, &path).unwrap();

        let result = execute(
            Query::current(),
            &path,
        )
        .unwrap();

        match result {
            QueryResult::Project(project) => {
                assert_eq!(project.id, 1);
                assert_eq!(project.name, "Test Project");
                assert_eq!(project.description, "Description");
            }

            QueryResult::Node(_) => {
                panic!("expected project");
            }
        }

        cleanup(&path);
    }

    #[test]
    fn execute_focus_returns_focused_project() {
        let path = temporary_path("focus-project");

        initialize_project(
           &path,
            1,
            String::from("Focused Project"),
            String::new(),
        )
        .unwrap();

        let mut context = Context::new();
        context.set_focused(0, Reference::Project(1));

        context_persistence::save(&context, &path).unwrap();

        let result = execute(
            Query::focus(0),
            &path,
        )
        .unwrap();

        match result {
            QueryResult::Project(project) => {
                assert_eq!(project.id, 1);
                assert_eq!(project.name, "Focused Project");
            }

            QueryResult::Node(_) => {
                panic!("expected project");
            }
        }

        cleanup(&path);
    }

    #[test]
    fn execute_explicit_project_does_not_require_context() {
        let path = temporary_path("explicit-project");

        initialize_project(
            &path,
            1,
            String::from("Explicit Project"),
            String::new(),
        )
        .unwrap();

        let result = execute(
            Query::explicit(Reference::Project(1)),
            &path,
        )
        .unwrap();

        match result {
            QueryResult::Project(project) => {
                assert_eq!(project.id, 1);
                assert_eq!(project.name, "Explicit Project");
            }

            QueryResult::Node(_) => {
                panic!("expected project");
            }
        }

        cleanup(&path);
    }

    #[test]
    fn execute_current_without_reference_fails() {
        let path = temporary_path("no-current");

        initialize_project(
            &path,
            1,
            String::from("Test Project"),
            String::new(),
        )
        .unwrap();

        let result = execute(
            Query::current(),
            &path,
        );

        assert!(result.is_err());

        cleanup(&path);
    }

    #[test]
    fn execute_explicit_node_returns_node() {
        let path = temporary_path("explicit-node");

        initialize_project(
            &path,
            1,
            String::from("Test Project"),
            String::new(),
        )
        .unwrap();

        create_node(
            &path,
            2,
            String::from("Test Node"),
            String::from("Node description"),
        )
        .unwrap();

        let result = execute(
            Query::explicit(Reference::Node(2)),
            &path,
        )
        .unwrap();

        match result {
            QueryResult::Node(node) => {
                assert_eq!(node.id, 2);
                assert_eq!(node.name, "Test Node");
                assert_eq!(node.description, "Node description");
            }

            QueryResult::Project(_) => {
                panic!("expected node");
            }
        }

        cleanup(&path);
    }

    #[test]
    fn execute_current_without_selected_uses_primary_focus() {
        let path = temporary_path("current_primary_focus");
        

        let project = crate::application::project::create_project(
            &path,
            1,
            "Focus Project".to_string(),
            String::new(),
        )
        .unwrap();

        let mut context = crate::persistence::context::load(&path).unwrap();

        context.set_focused(
            0,
            crate::core::context::Reference::Project(project.id),
        );

        crate::persistence::context::save(&context, &path).unwrap();

        let result = execute(Query::current(), &path).unwrap();

        assert_eq!(
            result,
            QueryResult::Project(project)
        );

        std::fs::remove_dir_all(&path).unwrap();
    }

    #[test]
    fn execute_current_prefers_primary_focus_over_selected() {
        let path = temporary_path("current_primary_focus_priority");

        let selected_project = crate::application::project::create_project(
            &path,
            1,
            "Selected Project".to_string(),
            String::new(),
        )
        .unwrap();

        let focused_project = crate::domain::project::Project::new(
            2,
            "Focused Project".to_string(),
            String::new(),
        );

        crate::persistence::project::save(
            &focused_project,
            &path,
        )
        .unwrap();

        let mut context = crate::persistence::context::load(&path).unwrap();

        context.set_selected(
            crate::core::context::Reference::Project(
                selected_project.id,
            ),
        );

        context.set_focused(
            0,
            crate::core::context::Reference::Project(
                focused_project.id,
            ),
        );

        crate::persistence::context::save(
            &context,
            &path,
        )
        .unwrap();
    
        let result = execute(Query::current(), &path).unwrap();

        assert_eq!(
            result,
            QueryResult::Project(focused_project)
        );

        std::fs::remove_dir_all(&path).unwrap();
    }

    #[test]
    fn execute_focus_not_found_fails() {
        let path = temporary_path("focus-not-found");

        initialize_project(
            &path,
            1,
            String::from("Test Project"),
            String::new(),
        )
        .unwrap();

        let result = execute(
            Query::focus(5),
            &path,
        );

        assert!(result.is_err());

        cleanup(&path);
    }

    #[test]
    fn execute_focus_can_use_non_primary_position() {
        let path = temporary_path("focus-non-primary");

        let project = crate::application::project::create_project(
            &path,
            1,
            String::from("Focused Project"),
            String::new(),
        )
        .unwrap();

        let mut context =
            context_persistence::load(&path).unwrap();

        context.set_focused(
            3,
            Reference::Project(project.id),
        );

        context_persistence::save(
            &context,
            &path,
        )
        .unwrap();

        let result = execute(
            Query::focus(3),
            &path,
        )
        .unwrap();

        assert_eq!(
            result,
            QueryResult::Project(project)
        );

        cleanup(&path);
    }

    #[test]
    fn execute_verbose_query_returns_same_object() {
        let path = temporary_path("verbose");

        let project = crate::application::project::create_project(
            &path,
            1,
            String::from("Verbose Project"),
            String::from("Description"),
        )
        .unwrap();

        let mut context =
            context_persistence::load(&path).unwrap();

        context.set_selected(
            Reference::Project(project.id),
        );

        context_persistence::save(
            &context,
            &path,
        )
        .unwrap();

        let normal = execute(
            Query::current(),
            &path,
        )
        .unwrap();

        let mut verbose_query = Query::current();
        verbose_query.detail = QueryDetail::Verbose;

        let verbose = execute(
            verbose_query,
            &path,
        )
        .unwrap();

        assert_eq!(normal, verbose);

        cleanup(&path);
    }
}
