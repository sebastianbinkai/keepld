use serde::{Deserialize, Serialize};

/// Type alias for uniquely identifying objects within Keepld.
pub type ObjectId = u64;

/// States of an object representing its operational or lifecycle phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    None,
    Active,
    Waiting,
    Paused,
    Done,
}

/// Availability profiles for objects, useful for scheduling or resource status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Availability {
    None,
    Free,
    Busy,
    Tentative,
}

/// The atomic leaf node of information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Element {
    pub content: String,
}

/// A hierarchical container nesting other objects (Elements and Models).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    pub children: Vec<KeepldObject>,
}

/// A high-level Model acting as a boundary for synchronization and permissions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub inner_model: Model,
    pub sync_enabled: bool,
}

/// A flat semantic aggregator (like comments or tags) referencing objects without altering hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Set {
    pub referenced_ids: Vec<ObjectId>,
}

/// Specific data payload corresponding to the concrete type of KeepldObject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ObjectData {
    Element(Element),
    Model(Model),
    Project(Project),
    Set(Set),
}

/// The unified core domain object representing any entity in Keepld.
/// Everything in the Keepld graph is derived from this base.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeepldObject {
    pub id: ObjectId,
    pub title: String,
    pub description: Option<String>,
    pub type_class: Option<String>,
    pub state: State,
    pub availability: Availability,
    pub data: ObjectData,
}

impl KeepldObject {
    /// Creates a new `KeepldObject` containing an `Element`.
    pub fn new_element(id: ObjectId, title: String, content: String) -> Self {
        Self {
            id,
            title,
            description: None,
            type_class: None,
            state: State::None,
            availability: Availability::None,
            data: ObjectData::Element(Element { content }),
        }
    }

    /// Creates a new `KeepldObject` containing an empty `Model`.
    pub fn new_model(id: ObjectId, title: String) -> Self {
        Self {
            id,
            title,
            description: None,
            type_class: None,
            state: State::None,
            availability: Availability::None,
            data: ObjectData::Model(Model { children: Vec::new() }),
        }
    }

    /// Creates a new `KeepldObject` containing a `Project`.
    pub fn new_project(id: ObjectId, title: String) -> Self {
        Self {
            id,
            title,
            description: None,
            type_class: None,
            state: State::None,
            availability: Availability::None,
            data: ObjectData::Project(Project {
                inner_model: Model { children: Vec::new() },
                sync_enabled: false,
            }),
        }
    }

    /// Creates a new `KeepldObject` containing a `Set`.
    pub fn new_set(id: ObjectId, title: String) -> Self {
        Self {
            id,
            title,
            description: None,
            type_class: None,
            state: State::None,
            availability: Availability::None,
            data: ObjectData::Set(Set { referenced_ids: Vec::new() }),
        }
    }

    /// Fluent builder for setting the description metadata.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Fluent builder for setting the type class metadata.
    pub fn with_type_class(mut self, type_class: String) -> Self {
        self.type_class = Some(type_class);
        self
    }

    /// Fluent builder for setting the state metadata.
    pub fn with_state(mut self, state: State) -> Self {
        self.state = state;
        self
    }

    /// Fluent builder for setting the availability metadata.
    pub fn with_availability(mut self, availability: Availability) -> Self {
        self.availability = availability;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_element() {
        let obj = KeepldObject::new_element(1, "Note".to_string(), "Hello world".to_string())
            .with_description("My test note".to_string())
            .with_type_class("text/markdown".to_string())
            .with_state(State::Active)
            .with_availability(Availability::Free);

        assert_eq!(obj.id, 1);
        assert_eq!(obj.title, "Note");
        assert_eq!(obj.description, Some("My test note".to_string()));
        assert_eq!(obj.type_class, Some("text/markdown".to_string()));
        assert_eq!(obj.state, State::Active);
        assert_eq!(obj.availability, Availability::Free);

        if let ObjectData::Element(el) = obj.data {
            assert_eq!(el.content, "Hello world");
        } else {
            panic!("Expected ObjectData::Element");
        }
    }

    #[test]
    fn test_create_model() {
        let mut obj = KeepldObject::new_model(2, "Workspace".to_string());
        
        let child = KeepldObject::new_element(3, "Sub-item".to_string(), "Inner details".to_string());
        if let ObjectData::Model(ref mut m) = obj.data {
            m.children.push(child);
        } else {
            panic!("Expected ObjectData::Model");
        }

        assert_eq!(obj.id, 2);
        assert_eq!(obj.title, "Workspace");
        assert_eq!(obj.state, State::None);
        assert_eq!(obj.availability, Availability::None);

        if let ObjectData::Model(m) = obj.data {
            assert_eq!(m.children.len(), 1);
            assert_eq!(m.children[0].id, 3);
        } else {
            panic!("Expected ObjectData::Model");
        }
    }

    #[test]
    fn test_serde_serialization() {
        let obj = KeepldObject::new_element(10, "Tag".to_string(), "Metadata content".to_string())
            .with_state(State::Done)
            .with_availability(Availability::Busy);

        let serialized = toml::to_string(&obj).unwrap();
        // Check that state and availability are serialized in lowercase
        assert!(serialized.contains("state = \"done\""));
        assert!(serialized.contains("availability = \"busy\""));
        // Check tag "type" for Element
        assert!(serialized.contains("type = \"element\""));

        let deserialized: KeepldObject = toml::from_str(&serialized).unwrap();
        assert_eq!(obj, deserialized);
    }
}
