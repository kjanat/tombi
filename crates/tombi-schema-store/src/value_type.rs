use indexmap::IndexSet;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
    OffsetDateTime,
    LocalDateTime,
    LocalDate,
    LocalTime,
    Array,
    Table,
    OneOf(Vec<ValueType>),
    AnyOf(Vec<ValueType>),
    AllOf(Vec<ValueType>),
}

impl ValueType {
    pub fn set_nullable(&mut self) {
        let value_type = self.clone();
        *self = match value_type {
            Self::Null => Self::Null,
            Self::Boolean
            | Self::Integer
            | Self::Float
            | Self::String
            | Self::OffsetDateTime
            | Self::LocalDateTime
            | Self::LocalDate
            | Self::LocalTime
            | Self::Array
            | Self::Table => Self::AnyOf(vec![value_type, Self::Null]),
            Self::OneOf(mut types) => {
                if !types.iter().any(Self::is_nullable) {
                    types.push(Self::Null);
                }
                Self::OneOf(types)
            }
            Self::AnyOf(mut types) => {
                if !types.iter().all(Self::is_nullable) {
                    types.push(Self::Null);
                }
                Self::AnyOf(types)
            }
            Self::AllOf(types) => {
                if types.iter().all(|t| !t.is_nullable()) {
                    Self::AnyOf(vec![Self::AllOf(types), Self::Null])
                } else {
                    Self::AllOf(types)
                }
            }
        }
    }

    #[must_use]
    pub fn is_nullable(&self) -> bool {
        match self {
            Self::Null => true,
            Self::Boolean
            | Self::Integer
            | Self::Float
            | Self::String
            | Self::OffsetDateTime
            | Self::LocalDateTime
            | Self::LocalDate
            | Self::LocalTime
            | Self::Array
            | Self::Table => false,
            Self::OneOf(types) | Self::AnyOf(types) => types.iter().any(Self::is_nullable),
            Self::AllOf(types) => types.iter().all(Self::is_nullable),
        }
    }

    fn to_display(&self, is_root: bool) -> String {
        match self {
            Self::Null => {
                // NOTE: If this representation appears in the Hover of the Language Server, it is a bug.
                "Null".to_string()
            }
            Self::Boolean => "Boolean".to_string(),
            Self::Integer => "Integer".to_string(),
            Self::Float => "Float".to_string(),
            Self::String => "String".to_string(),
            Self::OffsetDateTime => "OffsetDateTime".to_string(),
            Self::LocalDateTime => "LocalDateTime".to_string(),
            Self::LocalDate => "LocalDate".to_string(),
            Self::LocalTime => "LocalTime".to_string(),
            Self::Array => "Array".to_string(),
            Self::Table => "Table".to_string(),
            Self::OneOf(types) => fmt_composit_types(types, '^', is_root),
            Self::AnyOf(types) => fmt_composit_types(types, '|', is_root),
            Self::AllOf(types) => fmt_composit_types(types, '&', is_root),
        }
    }

    /// Simplify the type by removing unnecessary nesting.
    ///
    /// For example, `OneOf([OneOf([A, B]), C])` will be simplified to `OneOf([A, B, C])`.
    /// Also, if `Null` is included, it is taken out at the end of the outermost. This always displays `? at the end of type display.
    #[must_use]
    pub fn simplify(&self) -> Self {
        // Macro to handle the common pattern of simplifying composite types (OneOf, AnyOf, AllOf)
        macro_rules! simplify_composite {
            ($value_types:expr, $current_variant:ident, $($other_variant:ident)|+) => {{
                let mut flattened = IndexSet::new();
                let mut has_null = false;

                for value_type in $value_types {
                    match value_type.simplify() {
                        ValueType::Null => has_null = true,
                        // Flatten nested types of the same variant
                        ValueType::$current_variant(nested_value_types) => {
                            for nested_value_type in nested_value_types {
                                if matches!(nested_value_type, ValueType::Null) {
                                    has_null = true;
                                } else {
                                    flattened.insert(nested_value_type);
                                }
                            }
                        }
                        // Handle nested types of other composite variants (one match arm per variant)
                        $(
                            ValueType::$other_variant(nested_value_types) => {
                                let non_nulls = nested_value_types
                                    .into_iter()
                                    .filter_map(|nested_value_type| {
                                        if matches!(nested_value_type, ValueType::Null) {
                                            has_null = true;
                                            None
                                        } else {
                                            Some(nested_value_type)
                                        }
                                    })
                                    .collect_vec();

                                if non_nulls.len() == 1 {
                                    flattened.insert(non_nulls.into_iter().next().unwrap());
                                } else if !non_nulls.is_empty() {
                                    flattened.insert(ValueType::$other_variant(non_nulls));
                                }
                            }
                        )+
                        other => {
                            flattened.insert(other);
                        }
                    }
                }

                if has_null {
                    flattened.insert(ValueType::Null);
                }

                ValueType::$current_variant(flattened.into_iter().collect())
            }};
        }

        let simplified = match self {
            Self::OneOf(value_types) => {
                simplify_composite!(value_types, OneOf, AnyOf | AllOf)
            }
            Self::AnyOf(value_types) => {
                simplify_composite!(value_types, AnyOf, AllOf | OneOf)
            }
            Self::AllOf(value_types) => {
                simplify_composite!(value_types, AllOf, OneOf | AnyOf)
            }
            other => other.to_owned(),
        };

        // Further simplify single-element composite types
        match simplified {
            Self::OneOf(value_types) | Self::AnyOf(value_types) | Self::AllOf(value_types)
                if value_types.len() == 1 =>
            {
                value_types.into_iter().next().unwrap()
            }
            _ => simplified,
        }
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simplify().to_display(true))
    }
}

#[cfg(feature = "document-tree")]
impl From<tombi_document_tree::ValueType> for ValueType {
    fn from(value_type: tombi_document_tree::ValueType) -> Self {
        match value_type {
            tombi_document_tree::ValueType::Boolean => Self::Boolean,
            tombi_document_tree::ValueType::Integer => Self::Integer,
            tombi_document_tree::ValueType::Float => Self::Float,
            tombi_document_tree::ValueType::String => Self::String,
            tombi_document_tree::ValueType::OffsetDateTime => Self::OffsetDateTime,
            tombi_document_tree::ValueType::LocalDateTime => Self::LocalDateTime,
            tombi_document_tree::ValueType::LocalDate => Self::LocalDate,
            tombi_document_tree::ValueType::LocalTime => Self::LocalTime,
            tombi_document_tree::ValueType::Array => Self::Array,
            tombi_document_tree::ValueType::Table => Self::Table,
            tombi_document_tree::ValueType::Incomplete => unreachable!("incomplete value"),
        }
    }
}

fn fmt_composit_types(types: &[ValueType], separator: char, is_root: bool) -> String {
    let mut nullable = false;
    let non_null_types = types
        .iter()
        .filter(|t| {
            if matches!(t, ValueType::Null) {
                nullable = true;
                false
            } else {
                true
            }
        })
        .collect_vec();

    if nullable {
        if non_null_types.len() == 1 {
            format!("{}?", non_null_types[0].to_display(false))
        } else {
            format!(
                "({})?",
                non_null_types
                    .iter()
                    .map(|t| t.to_display(false))
                    .join(&format!(" {separator} ")),
            )
        }
    } else if is_root {
        non_null_types
            .iter()
            .map(|t| t.to_display(false))
            .join(&format!(" {separator} "))
    } else if non_null_types.len() == 1 {
        non_null_types[0].to_display(false)
    } else {
        format!(
            "({})",
            non_null_types
                .iter()
                .map(|t| t.to_display(false))
                .join(&format!(" {separator} ")),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn any_of_array_null() {
        let value_type = ValueType::AnyOf(
            vec![ValueType::Array, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Array?");
    }

    #[test]
    fn one_of_array_null() {
        let value_type = ValueType::OneOf(
            vec![ValueType::Array, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Array?");
    }

    #[test]
    fn all_of_array_null() {
        let value_type = ValueType::AllOf(
            vec![ValueType::Array, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Array?");
    }

    #[test]
    fn nullable_one_of() {
        let value_type = ValueType::OneOf(
            vec![ValueType::Array, ValueType::Table, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "(Array ^ Table)?");
    }

    #[test]
    fn nullable_any_of() {
        let value_type = ValueType::AnyOf(
            vec![ValueType::Array, ValueType::Table, ValueType::Null]
                .into_iter()
                .collect(),
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "(Array | Table)?");
    }

    #[test]
    fn nullable_all_of() {
        let value_type =
            ValueType::AllOf(vec![ValueType::Array, ValueType::Table, ValueType::Null]);
        pretty_assertions::assert_eq!(value_type.to_string(), "(Array & Table)?");
    }

    #[test]
    fn same_type_one_of() {
        let value_type = ValueType::OneOf(vec![
            ValueType::OneOf(vec![ValueType::Boolean, ValueType::Null]),
            ValueType::Boolean,
        ]);
        pretty_assertions::assert_eq!(value_type.to_string(), "Boolean?");
    }

    #[test]
    fn same_type_any_of() {
        let value_type = ValueType::AnyOf(vec![
            ValueType::OneOf(vec![ValueType::Boolean, ValueType::Null]),
            ValueType::OneOf(vec![ValueType::Boolean, ValueType::Null]),
        ]);
        pretty_assertions::assert_eq!(value_type.to_string(), "Boolean?");
    }

    #[test]
    fn same_type_all_of() {
        let value_type = ValueType::AllOf(vec![
            ValueType::OneOf(vec![ValueType::Boolean, ValueType::Null]),
            ValueType::Boolean,
            ValueType::Null,
        ]);
        pretty_assertions::assert_eq!(value_type.to_string(), "Boolean?");
    }

    #[test]
    fn nested_one_of() {
        let value_type = ValueType::OneOf(
            vec![
                ValueType::OneOf(vec![ValueType::Boolean, ValueType::String]),
                ValueType::Array,
                ValueType::Table,
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "(Boolean ^ String) ^ Array ^ Table"
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Boolean ^ String ^ Array ^ Table");
    }

    #[test]
    fn nested_any_of() {
        let value_type = ValueType::AnyOf(
            vec![
                ValueType::AnyOf(vec![ValueType::Boolean, ValueType::String]),
                ValueType::Array,
                ValueType::Table,
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "(Boolean | String) | Array | Table"
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Boolean | String | Array | Table");
    }

    #[test]
    fn nested_all_of() {
        let value_type = ValueType::AllOf(
            vec![
                ValueType::AllOf(vec![ValueType::Boolean, ValueType::String]),
                ValueType::Array,
                ValueType::Table,
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "(Boolean & String) & Array & Table"
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Boolean & String & Array & Table");
    }

    #[test]
    fn nested_one_of_withnullable() {
        let value_type = ValueType::OneOf(
            vec![
                ValueType::OneOf(vec![ValueType::Boolean, ValueType::String]),
                ValueType::Array,
                ValueType::Table,
                ValueType::Null,
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "((Boolean ^ String) ^ Array ^ Table)?"
        );
        pretty_assertions::assert_eq!(
            value_type.to_string(),
            "(Boolean ^ String ^ Array ^ Table)?"
        );
    }

    #[test]
    fn nested_one_of_with_nested_nullable() {
        let value_type = ValueType::OneOf(
            vec![
                ValueType::OneOf(vec![ValueType::Boolean, ValueType::String, ValueType::Null]),
                ValueType::Array,
                ValueType::Table,
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "(Boolean ^ String)? ^ Array ^ Table"
        );
        pretty_assertions::assert_eq!(
            value_type.to_string(),
            "(Boolean ^ String ^ Array ^ Table)?"
        );
    }

    #[test]
    fn nested_any_of_with_nested_nullable() {
        let value_type = ValueType::AnyOf(
            vec![
                ValueType::AnyOf(vec![ValueType::Boolean, ValueType::String, ValueType::Null]),
                ValueType::Array,
                ValueType::Table,
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "(Boolean | String)? | Array | Table"
        );
        pretty_assertions::assert_eq!(
            value_type.to_string(),
            "(Boolean | String | Array | Table)?"
        );
    }

    #[test]
    fn nested_all_of_with_nested_nullable() {
        let value_type = ValueType::AllOf(
            vec![
                ValueType::AllOf(vec![ValueType::Boolean, ValueType::String, ValueType::Null]),
                ValueType::Array,
                ValueType::Table,
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "(Boolean & String)? & Array & Table"
        );
        pretty_assertions::assert_eq!(
            value_type.to_string(),
            "(Boolean & String & Array & Table)?"
        );
    }

    #[test]
    fn nested_one_of_any_of() {
        let value_type = ValueType::OneOf(
            vec![
                ValueType::OneOf(vec![ValueType::Boolean, ValueType::String]),
                ValueType::AnyOf(vec![ValueType::Array, ValueType::Table]),
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "(Boolean ^ String) ^ (Array | Table)"
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Boolean ^ String ^ (Array | Table)");
    }

    #[test]
    fn nested_one_of_any_of_with_nullable() {
        let value_type = ValueType::OneOf(
            vec![
                ValueType::OneOf(vec![ValueType::Boolean, ValueType::String]),
                ValueType::AnyOf(vec![ValueType::Array, ValueType::Table, ValueType::Null]),
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "(Boolean ^ String) ^ (Array | Table)?"
        );
        pretty_assertions::assert_eq!(
            value_type.to_string(),
            "(Boolean ^ String ^ (Array | Table))?"
        );
    }

    #[test]
    fn slim_same_type() {
        let value_type = ValueType::OneOf(
            vec![
                ValueType::OneOf(vec![ValueType::Boolean, ValueType::Array]),
                ValueType::Boolean,
                ValueType::Array,
            ]
            .into_iter()
            .collect(),
        );
        pretty_assertions::assert_eq!(
            value_type.to_display(true),
            "(Boolean ^ Array) ^ Boolean ^ Array"
        );
        pretty_assertions::assert_eq!(value_type.to_string(), "Boolean ^ Array");
    }
}
