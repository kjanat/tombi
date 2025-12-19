use crate::SchemaAccessor;

/// Represents an accessor to a value in a TOML-like structure.
/// It can either be a key (for objects) or an index (for arrays).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Accessor {
    Key(String),
    Index(usize),
}

impl Accessor {
    #[inline]
    #[must_use]
    pub const fn is_key(&self) -> bool {
        matches!(self, Self::Key(_))
    }

    #[inline]
    #[must_use]
    pub fn as_key(&self) -> Option<&str> {
        match self {
            Self::Key(key) => Some(key),
            _ => None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_index(&self) -> bool {
        matches!(self, Self::Index(_))
    }

    #[inline]
    #[must_use]
    pub const fn as_index(&self) -> Option<usize> {
        match self {
            Self::Index(index) => Some(*index),
            _ => None,
        }
    }
}

impl std::fmt::Display for Accessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(key) => write!(f, "{key}"),
            Self::Index(index) => write!(f, "[{index}]"),
        }
    }
}

impl PartialEq<SchemaAccessor> for Accessor {
    fn eq(&self, other: &SchemaAccessor) -> bool {
        match (self, other) {
            (Self::Key(key), SchemaAccessor::Key(other_key)) => key == other_key,
            (Self::Index(_), SchemaAccessor::Index) => true,
            _ => false,
        }
    }
}

impl PartialEq<&str> for Accessor {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Self::Key(key) => key == *other,
            _ => false,
        }
    }
}

impl PartialOrd<Self> for Accessor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Key(key1), Self::Key(key2)) => key1.partial_cmp(key2),
            (Self::Index(index1), Self::Index(index2)) => index1.partial_cmp(index2),
            _ => None,
        }
    }
}

impl indexmap::Equivalent<SchemaAccessor> for Accessor {
    fn equivalent(&self, other: &SchemaAccessor) -> bool {
        match (self, other) {
            (Self::Key(key1), SchemaAccessor::Key(key2)) => key1 == key2,
            (Self::Index(_), SchemaAccessor::Index) => true,
            _ => false,
        }
    }
}

/// A collection of `Accessor`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Accessors(Vec<Accessor>);

impl Accessors {
    #[inline]
    #[must_use]
    pub fn first(&self) -> Option<&Accessor> {
        self.0.first()
    }

    #[inline]
    #[must_use]
    pub fn last(&self) -> Option<&Accessor> {
        self.0.last()
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<Accessor>> for Accessors {
    fn from(accessors: Vec<Accessor>) -> Self {
        Self(accessors)
    }
}

impl AsRef<[Accessor]> for Accessors {
    fn as_ref(&self) -> &[Accessor] {
        &self.0
    }
}

impl std::fmt::Display for Accessors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(accessor) = iter.next() {
            write!(f, "{accessor}")?;
            for accessor in iter {
                match accessor {
                    Accessor::Key(_) => write!(f, ".{accessor}")?,
                    Accessor::Index(_) => write!(f, "{accessor}")?,
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessorKeyKind {
    Header,
    Dotted,
    KeyValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyContext {
    pub kind: AccessorKeyKind,
    pub range: tombi_text::Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessorContext {
    Key(KeyContext),
    Index,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accessors_display() {
        let accessors = Accessors::from(vec![
            Accessor::Key("root".to_string()),
            Accessor::Key("child".to_string()),
            Accessor::Index(1),
            Accessor::Key("item".to_string()),
        ]);
        assert_eq!(format!("{accessors}"), "root.child[1].item");
    }

    #[test]
    fn test_accessors_display_consecutive_indices() {
        let accessors = Accessors::from(vec![
            Accessor::Key("array".to_string()),
            Accessor::Index(0),
            Accessor::Index(1),
            Accessor::Index(2),
        ]);
        assert_eq!(format!("{accessors}"), "array[0][1][2]");
    }
}
