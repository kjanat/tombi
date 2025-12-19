#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BooleanSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub range: tombi_text::Range,
    pub default: Option<bool>,
    pub const_value: Option<bool>,
    pub enumerate: Option<Vec<bool>>,
    pub examples: Option<Vec<bool>>,
    pub deprecated: Option<bool>,
}

impl BooleanSchema {
    #[must_use]
    pub fn new(object: &tombi_json::ObjectNode) -> Self {
        Self {
            title: object
                .get("title")
                .and_then(|value| value.as_str().map(std::string::ToString::to_string)),
            description: object
                .get("description")
                .and_then(|value| value.as_str().map(std::string::ToString::to_string)),
            default: object
                .get("default")
                .and_then(tombi_json::ValueNode::as_bool),
            const_value: object.get("const").and_then(tombi_json::ValueNode::as_bool),
            enumerate: object
                .get("enum")
                .and_then(|value| value.as_array())
                .map(|array| {
                    array
                        .items
                        .iter()
                        .filter_map(tombi_json::ValueNode::as_bool)
                        .collect()
                }),
            examples: object
                .get("examples")
                .and_then(|v| v.as_array())
                .map(|array| {
                    array
                        .items
                        .iter()
                        .filter_map(tombi_json::ValueNode::as_bool)
                        .collect()
                }),
            deprecated: object
                .get("deprecated")
                .and_then(tombi_json::ValueNode::as_bool),
            range: object.range,
        }
    }

    #[must_use]
    pub const fn value_type(&self) -> crate::ValueType {
        crate::ValueType::Boolean
    }
}
