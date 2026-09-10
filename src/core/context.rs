use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reference {
    Project(u64),
    Node(u64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    selected: Option<Reference>,
    focused: HashMap<u8, Reference>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            selected: None,
            focused: HashMap::new(),
        }
    }

    pub fn selected(&self) -> Option<Reference> {
        self.selected
    }

    pub fn set_selected(&mut self, reference: Reference) {
        self.selected = Some(reference);
    }

    pub fn clear_selected(&mut self) {
        self.selected = None;
    }

    pub fn focused(&self, position: u8) -> Option<Reference> {
        self.focused.get(&position).copied()
    }

    pub fn set_focused(&mut self, position: u8, reference: Reference) {
        if position == 0 {
            self.selected = None;
        }

        self.focused.insert(position, reference);
    }

    pub fn clear_focused(&mut self, position: u8) {
        self.focused.remove(&position);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_context_is_empty() {
        let context = Context::new();

        assert_eq!(context.selected(), None);
        assert_eq!(context.focused(0), None);
    }

    #[test]
    fn selected_can_be_set_and_read() {
        let mut context = Context::new();

        context.set_selected(Reference::Node(42));

        assert_eq!(context.selected(), Some(Reference::Node(42)));
    }

    #[test]
    fn selected_persists_until_replaced_or_cleared() {
        let mut context = Context::new();

        context.set_selected(Reference::Node(42));

        assert_eq!(context.selected(), Some(Reference::Node(42)));
        assert_eq!(context.selected(), Some(Reference::Node(42)));

        context.clear_selected();

        assert_eq!(context.selected(), None);
    }

    #[test]
    fn primary_focus_clears_selected() {
        let mut context = Context::new();

        context.set_selected(Reference::Node(42));
        context.set_focused(0, Reference::Node(99));

        assert_eq!(context.selected(), None);
        assert_eq!(context.focused(0), Some(Reference::Node(99)));
    }

    #[test]
    fn non_primary_focus_does_not_clear_selected() {
        let mut context = Context::new();

        context.set_selected(Reference::Node(42));
        context.set_focused(12, Reference::Node(99));

        assert_eq!(context.selected(), Some(Reference::Node(42)));
        assert_eq!(context.focused(12), Some(Reference::Node(99)));
    }

    #[test]
    fn non_primary_focus_does_not_change_primary_focus() {
        let mut context = Context::new();

        context.set_focused(0, Reference::Node(42));
        context.set_focused(12, Reference::Node(99));

        assert_eq!(context.focused(0), Some(Reference::Node(42)));
        assert_eq!(context.focused(12), Some(Reference::Node(99)));
    }

    #[test]
    fn focused_position_can_be_cleared() {
        let mut context = Context::new();

        context.set_focused(12, Reference::Node(99));
        context.clear_focused(12);

        assert_eq!(context.focused(12), None);
    }

    #[test]
    fn clearing_primary_focus_does_not_restore_selected() {
        let mut context = Context::new();

        context.set_selected(Reference::Node(42));
        context.set_focused(0, Reference::Node(99));
        context.clear_focused(0);

        assert_eq!(context.selected(), None);
        assert_eq!(context.focused(0), None);
    }
}
