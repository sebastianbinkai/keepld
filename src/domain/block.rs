use uuid::Uuid;

/// Stable internal identity of a Keepld Block.
///
/// A BlockId contains no semantic information about the object.
/// It does not encode type, class, alias, scope, or permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(Uuid);

impl BlockId {
    /// Creates a new random BlockId.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a BlockId from an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Returns the underlying UUID.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    /// Generates the default 12-character Keepld ID.
    ///
    /// The KID is derived deterministically from the UUID.
    /// It is a human-facing alias, not an identity.
    pub fn default_alias(&self) -> crate::domain::alias::Alias {
        crate::domain::alias::Alias::from_kid(self)
    }
}

impl Default for BlockId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_id_is_stable() {
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

        let first = BlockId::from_uuid(uuid);
        let second = BlockId::from_uuid(uuid);

        assert_eq!(first, second);
        assert_eq!(first.as_uuid(), uuid);
    }

    #[test]
    fn new_block_ids_are_128_bit_uuids() {
        let id = BlockId::new();

        assert_ne!(id.as_uuid(), Uuid::nil());
    }
}
