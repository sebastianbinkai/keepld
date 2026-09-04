use crate::core::context::{Context, Reference};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceQuery {
    Explicit(Reference),
    Focus(u8),
    Current,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResolveError {
    FocusNotFound(u8),
    NoCurrentReference,
}

impl std::fmt::Display for ResolveError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ResolveError::FocusNotFound(position) => {
                write!(f, "Focus position {} is not set", position)
            }
            ResolveError::NoCurrentReference => {
                write!(f, "No current reference is available")
            }
        }
    }
}

impl std::error::Error for ResolveError {}

pub fn resolve(
    query: ReferenceQuery,
    context: &mut Context,
) -> Result<Reference, ResolveError> {
    match query {
        ReferenceQuery::Explicit(reference) => Ok(reference),

        ReferenceQuery::Focus(position) => {
            context
                .focused(position)
                .ok_or(ResolveError::FocusNotFound(position))
        }

        ReferenceQuery::Current => {
            if let Some(selected) = context.selected() {
                if context.focused(0).is_some() {
                    context.clear_selected();
                }

                return Ok(selected);
            }

            if let Some(focused) = context.focused(0) {
                return Ok(focused);
            }

            Err(ResolveError::NoCurrentReference)
        }
    }
}

#[test]
fn selected_persists_across_current_resolution_without_primary_focus() {
    let mut context = Context::new();

    context.set_selected(Reference::Node(10));

    let first = resolve(
        ReferenceQuery::Current,
        &mut context,
    )
    .unwrap();

    let second = resolve(
        ReferenceQuery::Current,
        &mut context,
    )
    .unwrap();

    assert_eq!(first, Reference::Node(10));
    assert_eq!(second, Reference::Node(10));
    assert_eq!(context.selected(), Some(Reference::Node(10)));
}

#[test]
fn selected_has_priority_over_primary_focus() {
    let mut context = Context::new();

    context.set_focused(0, Reference::Node(20));
    context.set_selected(Reference::Node(10));

    let resolved = resolve(
        ReferenceQuery::Current,
        &mut context,
    )
    .unwrap();

    assert_eq!(resolved, Reference::Node(10));
    assert_eq!(context.focused(0), Some(Reference::Node(20)));
    assert_eq!(context.selected(), None);
}

#[test]
fn selected_is_consumed_when_primary_focus_exists() {
    let mut context = Context::new();

    context.set_focused(0, Reference::Node(20));
    context.set_selected(Reference::Node(10));

    let resolved = resolve(
        ReferenceQuery::Current,
        &mut context,
    )
    .unwrap();

    assert_eq!(resolved, Reference::Node(10));
    assert_eq!(context.selected(), None);
    assert_eq!(context.focused(0), Some(Reference::Node(20)));
}

#[test]
fn current_falls_back_to_primary_focus_after_selected_is_consumed() {
    let mut context = Context::new();

    context.set_focused(0, Reference::Node(20));
    context.set_selected(Reference::Node(10));

    let first = resolve(
        ReferenceQuery::Current,
        &mut context,
    )
    .unwrap();

    let second = resolve(
        ReferenceQuery::Current,
        &mut context,
    )
    .unwrap();

    assert_eq!(first, Reference::Node(10));
    assert_eq!(second, Reference::Node(20));
    assert_eq!(context.focused(0), Some(Reference::Node(20)));
}

#[test]
fn secondary_focus_does_not_interfere_with_current_resolution() {
    let mut context = Context::new();

    context.set_focused(12, Reference::Node(30));
    context.set_focused(0, Reference::Node(20));
    context.set_selected(Reference::Node(10));

    let first = resolve(
        ReferenceQuery::Current,
        &mut context,
    )
    .unwrap();

    let second = resolve(
        ReferenceQuery::Current,
        &mut context,
    )
    .unwrap();

    assert_eq!(first, Reference::Node(10));
    assert_eq!(second, Reference::Node(20));
    assert_eq!(context.focused(0), Some(Reference::Node(20)));
    assert_eq!(context.focused(12), Some(Reference::Node(30)));
}

#[test]
fn explicit_reference_does_not_modify_context() {
    let mut context = Context::new();

    context.set_focused(0, Reference::Node(20));
    context.set_focused(12, Reference::Node(30));
    context.set_selected(Reference::Node(10));

    let resolved = resolve(
        ReferenceQuery::Explicit(Reference::Node(40)),
        &mut context,
    )
    .unwrap();

    assert_eq!(resolved, Reference::Node(40));
    assert_eq!(context.selected(), Some(Reference::Node(10)));
    assert_eq!(context.focused(0), Some(Reference::Node(20)));
    assert_eq!(context.focused(12), Some(Reference::Node(30)));
}

#[test]
fn explicit_focus_does_not_modify_context() {
    let mut context = Context::new();

    context.set_focused(0, Reference::Node(20));
    context.set_focused(12, Reference::Node(30));
    context.set_selected(Reference::Node(10));

    let resolved = resolve(
        ReferenceQuery::Focus(12),
        &mut context,
    )
    .unwrap();

    assert_eq!(resolved, Reference::Node(30));
    assert_eq!(context.selected(), Some(Reference::Node(10)));
    assert_eq!(context.focused(0), Some(Reference::Node(20)));
    assert_eq!(context.focused(12), Some(Reference::Node(30)));
}

#[test]
fn current_without_selected_or_primary_focus_fails() {
    let mut context = Context::new();

    let result = resolve(
        ReferenceQuery::Current,
        &mut context,
    );

    assert_eq!(
        result,
        Err(ResolveError::NoCurrentReference)
    );
}

#[test]
fn current_does_not_promote_secondary_focus() {
    let mut context = Context::new();

    context.set_focused(12, Reference::Node(30));

    let result = resolve(
        ReferenceQuery::Current,
        &mut context,
    );

    assert_eq!(
        result,
        Err(ResolveError::NoCurrentReference)
    );

    assert_eq!(
        context.focused(12),
        Some(Reference::Node(30))
    );
}

/*
 * Anteriores tests
 *

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_reference_is_returned_directly() {
        let mut context = Context::new();

        let result = resolve(
            ReferenceQuery::Explicit(Reference::Node(42)),
            &mut context,
        )
        .unwrap();

        assert_eq!(result, Reference::Node(42));
        assert_eq!(context.selected(), None);
        assert_eq!(context.focused(0), None);
    }

    #[test]
    fn focus_reference_is_returned() {
        let mut context = Context::new();

        context.set_focused(12, Reference::Node(42));

        let result = resolve(
            ReferenceQuery::Focus(12),
            &mut context,
        )
        .unwrap();

        assert_eq!(result, Reference::Node(42));
        assert_eq!(
            context.focused(12),
            Some(Reference::Node(42))
        );
    }

    #[test]
    fn missing_focus_returns_error() {
        let mut context = Context::new();

        let result = resolve(
            ReferenceQuery::Focus(12),
            &mut context,
        );

        assert_eq!(
            result,
            Err(ResolveError::FocusNotFound(12))
        );
    }

    #[test]
    fn current_uses_selected_when_no_primary_focus_exists() {
        let mut context = Context::new();

        context.set_selected(Reference::Node(42));

        let result = resolve(
            ReferenceQuery::Current,
            &mut context,
        )
        .unwrap();

        assert_eq!(result, Reference::Node(42));
        assert_eq!(
            context.selected(),
            Some(Reference::Node(42))
        );
    }

    #[test]
    fn current_uses_primary_focus_when_selected_is_absent() {
        let mut context = Context::new();

        context.set_focused(0, Reference::Node(42));

        let result = resolve(
            ReferenceQuery::Current,
            &mut context,
        )
        .unwrap();

        assert_eq!(result, Reference::Node(42));
        assert_eq!(
            context.focused(0),
            Some(Reference::Node(42))
        );
    }

    #[test]
    fn selected_has_priority_over_primary_focus() {
        let mut context = Context::new();

        context.set_selected(Reference::Node(10));
        context.set_focused(0, Reference::Node(20));

        let result = resolve(
            ReferenceQuery::Current,
            &mut context,
        )
        .unwrap();

        assert_eq!(result, Reference::Node(10));
        assert_eq!(context.selected(), None);
        assert_eq!(
            context.focused(0),
            Some(Reference::Node(20))
        );
    }

    #[test]
    fn selected_is_consumed_when_primary_focus_exists() {
        let mut context = Context::new();

        context.set_selected(Reference::Node(42));
        context.set_focused(0, Reference::Node(42));

        let result = resolve(
            ReferenceQuery::Current,
            &mut context,
        )
        .unwrap();

        assert_eq!(result, Reference::Node(42));
        assert_eq!(context.selected(), None);
        assert_eq!(
            context.focused(0),
            Some(Reference::Node(42))
        );
    }

    #[test]
    fn consuming_selected_does_not_affect_secondary_focus() {
        let mut context = Context::new();

        context.set_selected(Reference::Node(10));
        context.set_focused(0, Reference::Node(20));
        context.set_focused(12, Reference::Node(30));

        let result = resolve(
            ReferenceQuery::Current,
            &mut context,
        )
        .unwrap();

        assert_eq!(result, Reference::Node(10));
        assert_eq!(context.selected(), None);
        assert_eq!(
            context.focused(0),
            Some(Reference::Node(20))
        );
        assert_eq!(
            context.focused(12),
            Some(Reference::Node(30))
        );
    }

    #[test]
    fn secondary_focus_does_not_become_primary_automatically() {
        let mut context = Context::new();

        context.set_focused(12, Reference::Node(42));

        let result = resolve(
            ReferenceQuery::Current,
            &mut context,
        );

        assert_eq!(
            result,
            Err(ResolveError::NoCurrentReference)
        );

        assert_eq!(context.focused(0), None);
        assert_eq!(
            context.focused(12),
            Some(Reference::Node(42))
        );
    }

    #[test]
    fn current_without_any_reference_returns_error() {
        let mut context = Context::new();

        let result = resolve(
            ReferenceQuery::Current,
            &mut context,
        );

        assert_eq!(
            result,
            Err(ResolveError::NoCurrentReference)
        );
    }
}

*/

