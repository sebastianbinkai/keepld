use std::fmt;

/// Maximum number of characters allowed in a Tag.
pub const MAX_TAG_LENGTH: usize = 32;

/// A user-defined semantic tag attached to an Object.
///
/// Tags are intentionally restricted to a simple ASCII identifier syntax so
/// they remain predictable across the CLI, serialization, APIs and future
/// filtering mechanisms.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagError {
    Empty,
    TooLong,
    InvalidCharacter(char),
}

impl Tag {
    /// Creates a Tag after validating its syntax.
    pub fn new(value: impl Into<String>) -> Result<Self, TagError> {
        let value = value.into();

        if value.is_empty() {
            return Err(TagError::Empty);
        }

        if value.chars().count() > MAX_TAG_LENGTH {
            return Err(TagError::TooLong);
        }

        for character in value.chars() {
            if !character.is_ascii_alphanumeric() && character != '_' && character != '-' {
                return Err(TagError::InvalidCharacter(character));
            }
        }

        Ok(Self(value))
    }

    /// Returns the Tag as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_simple_tag() {
        let tag = Tag::new("backend").unwrap();

        assert_eq!(tag.as_str(), "backend");
    }

    #[test]
    fn accepts_hyphen_and_underscore() {
        assert!(Tag::new("backend-api").is_ok());
        assert!(Tag::new("rust_2024").is_ok());
    }

    #[test]
    fn accepts_uppercase_characters() {
        assert!(Tag::new("API").is_ok());
    }

    #[test]
    fn preserves_case() {
        let lower = Tag::new("api").unwrap();
        let upper = Tag::new("API").unwrap();

        assert_ne!(lower, upper);
    }

    #[test]
    fn rejects_empty_tag() {
        assert_eq!(Tag::new(""), Err(TagError::Empty));
    }

    #[test]
    fn rejects_tag_longer_than_maximum() {
        let value = "a".repeat(MAX_TAG_LENGTH + 1);

        assert_eq!(Tag::new(value), Err(TagError::TooLong));
    }

    #[test]
    fn accepts_tag_at_maximum_length() {
        let value = "a".repeat(MAX_TAG_LENGTH);

        assert!(Tag::new(value).is_ok());
    }

    #[test]
    fn rejects_spaces() {
        assert_eq!(
            Tag::new("backend api"),
            Err(TagError::InvalidCharacter(' '))
        );
    }

    #[test]
    fn rejects_path_separator() {
        assert_eq!(
            Tag::new("backend/api"),
            Err(TagError::InvalidCharacter('/'))
        );

        assert_eq!(
            Tag::new("backend\\api"),
            Err(TagError::InvalidCharacter('\\'))
        );
    }

    #[test]
    fn rejects_punctuation() {
        assert_eq!(
            Tag::new("backend.api"),
            Err(TagError::InvalidCharacter('.'))
        );

        assert_eq!(
            Tag::new("backend:api"),
            Err(TagError::InvalidCharacter(':'))
        );

        assert_eq!(
            Tag::new("backend@api"),
            Err(TagError::InvalidCharacter('@'))
        );
    }

    #[test]
    fn rejects_unicode() {
        assert_eq!(Tag::new("backend-é"), Err(TagError::InvalidCharacter('é')));
    }

    #[test]
    fn rejects_control_characters() {
        assert_eq!(
            Tag::new("backend\napi"),
            Err(TagError::InvalidCharacter('\n'))
        );
    }

    #[test]
    fn supports_hashing_and_equality() {
        use std::collections::HashSet;

        let mut tags = HashSet::new();
        tags.insert(Tag::new("rust").unwrap());

        assert!(tags.contains(&Tag::new("rust").unwrap()));
        assert!(!tags.contains(&Tag::new("Rust").unwrap()));
    }

    #[test]
    fn displays_original_value() {
        let tag = Tag::new("backend-api").unwrap();

        assert_eq!(tag.to_string(), "backend-api");
    }
}
