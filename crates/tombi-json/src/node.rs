use std::io::Read;

use itertools::Itertools;
use tombi_json_value::{Number, Object, Value};
use tombi_text::Range;

/// A JSON value with source code position information
#[derive(Debug, Clone, PartialEq)]
pub enum ValueNode {
    /// A JSON null value
    Null(NullNode),
    /// A JSON boolean value
    Bool(BoolNode),
    /// A JSON number value
    Number(NumberNode),
    /// A JSON string value
    String(StringNode),
    /// A JSON array value
    Array(ArrayNode),
    /// A JSON object value
    Object(ObjectNode),
}

impl ValueNode {
    #[must_use]
    pub const fn range(&self) -> Range {
        match self {
            Self::Null(node) => node.range,
            Self::Bool(node) => node.range,
            Self::Number(node) => node.range,
            Self::String(node) => node.range,
            Self::Array(node) => node.range,
            Self::Object(node) => node.range,
        }
    }

    /// Check if the node is null
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null(_))
    }

    /// Check if the node is a boolean
    #[must_use]
    pub const fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Check if the node is a number
    #[must_use]
    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Check if the node is a string
    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    /// Check if the node is an array
    #[must_use]
    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    /// Check if the node is an object
    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }

    /// Get as boolean value
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(node) => Some(node.value),
            _ => None,
        }
    }

    /// Get as float value
    #[must_use]
    pub const fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(node) => node.value.as_f64(),
            _ => None,
        }
    }

    /// Get as unsigned integer value
    #[must_use]
    pub const fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Number(node) => node.value.as_u64(),
            _ => None,
        }
    }

    /// Get as integer number value
    #[must_use]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Number(node) => node.value.as_i64(),
            _ => None,
        }
    }

    /// Get as string reference
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(node) => Some(&node.value),
            _ => None,
        }
    }

    /// Get as array reference
    #[must_use]
    pub const fn as_array(&self) -> Option<&ArrayNode> {
        match self {
            Self::Array(node) => Some(node),
            _ => None,
        }
    }

    /// Get as mutable array reference
    pub const fn as_array_mut(&mut self) -> Option<&mut ArrayNode> {
        match self {
            Self::Array(node) => Some(node),
            _ => None,
        }
    }

    /// Get as `ObjectNode` if this node contains an object
    #[must_use]
    pub const fn as_object(&self) -> Option<&ObjectNode> {
        match self {
            Self::Object(node) => Some(node),
            _ => None,
        }
    }

    pub const fn as_object_mut(&mut self) -> Option<&mut ObjectNode> {
        match self {
            Self::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn from_reader<R>(reader: R) -> Result<Self, crate::Error>
    where
        R: std::io::Read,
    {
        let mut reader = std::io::BufReader::new(reader);
        let mut s = String::new();
        reader.read_to_string(&mut s)?;
        Ok(crate::parser::parse(&s)?)
    }
}

impl std::str::FromStr for ValueNode {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(crate::parser::parse(s)?)
    }
}

/// A JSON null value with source code position information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullNode {
    /// The position of the null value in the source code
    pub range: Range,
}

impl std::fmt::Display for NullNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

/// A JSON boolean value with source code position information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolNode {
    /// The boolean value
    pub value: bool,
    /// The position of the boolean value in the source code
    pub range: Range,
}

impl std::fmt::Display for BoolNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// A JSON number value with source code position information
#[derive(Debug, Clone, PartialEq)]
pub struct NumberNode {
    /// The number value
    pub value: Number,
    /// The position of the number value in the source code
    pub range: Range,
}

impl std::fmt::Display for NumberNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// A JSON string value with source code position information
#[derive(Debug, Clone)]
pub struct StringNode {
    /// The string value
    pub value: String,
    /// The position of the string value in the source code
    pub range: Range,
}

impl std::fmt::Display for StringNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl PartialEq for StringNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for StringNode {}

impl indexmap::Equivalent<String> for StringNode {
    fn equivalent(&self, other: &String) -> bool {
        self.value == *other
    }
}

impl std::borrow::Borrow<str> for StringNode {
    fn borrow(&self) -> &str {
        &self.value
    }
}

impl std::hash::Hash for StringNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// A JSON array with source code position information
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayNode {
    /// The array elements
    pub items: Vec<ValueNode>,
    /// The position of the entire array in the source code
    pub range: Range,
}

impl ArrayNode {
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<&ValueNode> {
        self.items.get(index)
    }
}

impl std::fmt::Display for ArrayNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.items
                .iter()
                .map(std::string::ToString::to_string)
                .join(", ")
        )
    }
}

/// A JSON object with source code position information
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectNode {
    /// The object properties
    pub properties: tombi_json_value::Map<StringNode, ValueNode>,
    /// The position of the entire object in the source code
    pub range: Range,
}

impl ObjectNode {
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&ValueNode> {
        self.properties.get(key)
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

impl From<ObjectNode> for Object {
    fn from(node: ObjectNode) -> Self {
        node.properties
            .into_iter()
            .map(|(k, v)| (k.value, v.into()))
            .collect()
    }
}

impl From<&ObjectNode> for Object {
    fn from(node: &ObjectNode) -> Self {
        node.properties
            .iter()
            .map(|(k, v)| (k.value.clone(), v.into()))
            .collect()
    }
}
impl std::fmt::Display for ObjectNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.properties
                .iter()
                .map(|(k, v)| format!("{}: {}", k.value, v))
                .join(", ")
        )
    }
}

impl From<ValueNode> for Value {
    fn from(node: ValueNode) -> Self {
        match node {
            ValueNode::Null(_) => Self::Null,
            ValueNode::Bool(node) => Self::Bool(node.value),
            ValueNode::Number(node) => Self::Number(node.value),
            ValueNode::String(node) => Self::String(node.value),
            ValueNode::Array(node) => Self::Array(node.items.into_iter().map(Into::into).collect()),
            ValueNode::Object(node) => Self::Object(
                node.properties
                    .into_iter()
                    .map(|(k, v)| (k.value, v.into()))
                    .collect(),
            ),
        }
    }
}

impl std::fmt::Display for ValueNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null(_) => write!(f, "null"),
            Self::Bool(node) => write!(f, "{}", node.value),
            Self::Number(node) => write!(f, "{}", node.value),
            Self::String(node) => write!(f, "\"{}\"", node.value),
            Self::Array(node) => write!(
                f,
                "[{}]",
                node.items
                    .iter()
                    .map(std::string::ToString::to_string)
                    .join(", ")
            ),
            Self::Object(node) => write!(
                f,
                "{{{}}}",
                node.properties
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k.value, v))
                    .join(", ")
            ),
        }
    }
}

impl From<&ValueNode> for Value {
    fn from(node: &ValueNode) -> Self {
        match node {
            ValueNode::Null(_) => Self::Null,
            ValueNode::Bool(node) => Self::Bool(node.value),
            ValueNode::Number(node) => Self::Number(node.value.clone()),
            ValueNode::String(node) => Self::String(node.value.clone()),
            ValueNode::Array(node) => Self::Array(node.items.iter().map(Into::into).collect()),
            ValueNode::Object(node) => Self::Object(
                node.properties
                    .iter()
                    .map(|(k, v)| (k.value.clone(), v.into()))
                    .collect(),
            ),
        }
    }
}

impl From<ArrayNode> for Value {
    fn from(node: ArrayNode) -> Self {
        let values: Vec<Self> = node.items.into_iter().map(Into::into).collect();
        Self::Array(values)
    }
}

impl From<&ArrayNode> for Value {
    fn from(node: &ArrayNode) -> Self {
        let values: Vec<Self> = node.items.iter().map(Into::into).collect();
        Self::Array(values)
    }
}

impl From<ObjectNode> for Value {
    fn from(node: ObjectNode) -> Self {
        // Use IndexMap as an intermediate step
        let mut map = Object::new();
        for (key, value_node) in node.properties {
            map.insert(key.value, Self::from(value_node));
        }
        // Convert IndexMap to Value
        Self::Object(map)
    }
}

impl From<&ObjectNode> for Value {
    fn from(node: &ObjectNode) -> Self {
        // Use IndexMap as an intermediate step
        let mut map = Object::new();
        for (key, value_node) in &node.properties {
            map.insert(key.value.clone(), Self::from(value_node));
        }
        // Convert IndexMap to Value
        Self::Object(map)
    }
}
