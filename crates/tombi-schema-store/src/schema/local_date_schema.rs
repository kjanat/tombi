#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LocalDateSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub range: tombi_text::Range,
    pub enumerate: Option<Vec<String>>,
    pub default: Option<String>,
    pub const_value: Option<String>,
    pub examples: Option<Vec<String>>,
    pub deprecated: Option<bool>,
}

impl LocalDateSchema {
    #[must_use]
    pub fn new(object: &tombi_json::ObjectNode) -> Self {
        Self {
            title: object
                .get("title")
                .and_then(|v| v.as_str().map(std::string::ToString::to_string)),
            description: object
                .get("description")
                .and_then(|v| v.as_str().map(std::string::ToString::to_string)),
            enumerate: object.get("enum").and_then(|v| v.as_array()).map(|a| {
                a.items
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(ToString::to_string)
                    .collect()
            }),
            default: object
                .get("default")
                .and_then(|v| v.as_str().map(std::string::ToString::to_string)),
            const_value: object
                .get("const")
                .and_then(|v| v.as_str().map(std::string::ToString::to_string)),
            examples: object.get("examples").and_then(|v| v.as_array()).map(|v| {
                v.items
                    .iter()
                    .filter_map(|v| v.as_str().map(ToString::to_string))
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
        crate::ValueType::LocalDate
    }
}
