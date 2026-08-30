use super::availability::Availability;
use super::class::Class;
use super::state::State;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub availability: Availability,
    pub class: Class,
    pub state: State,
}

impl Node {
    pub fn new(id: u64, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            availability: Availability::Enabled,
            class: Class::Default,
            state: State::Active,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_has_default_values() {
        let node = Node::new(
            1,
            String::from("Test node"),
            String::from("Test description"),
        );

        assert_eq!(node.id, 1);
        assert_eq!(node.name, "Test node");
        assert_eq!(node.description, "Test description");
        assert_eq!(node.availability, Availability::Enabled);
        assert_eq!(node.class, Class::Default);
        assert_eq!(node.state, State::Active);
    }
}
