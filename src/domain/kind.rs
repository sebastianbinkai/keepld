use serde::{Deserialize, Serialize};

use super::class::{ElementClass, ModelClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    Model(ModelClass),
    Element(ElementClass),
}
