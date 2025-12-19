pub use std::num::ParseIntError;

/// Parse a binary integer literal (base 2).
///
/// # Errors
///
/// Returns a parsing error if the value cannot be converted to i64.
pub fn try_from_binary(value: &str) -> Result<i64, ParseIntError> {
    i64::from_str_radix(&value[2..].replace('_', ""), 2)
}

/// Parse an octal integer literal (base 8).
///
/// # Errors
///
/// Returns a parsing error if the value cannot be converted to i64.
pub fn try_from_octal(value: &str) -> Result<i64, ParseIntError> {
    i64::from_str_radix(&value[2..].replace('_', ""), 8)
}

/// Parse a decimal integer literal (base 10).
///
/// # Errors
///
/// Returns a parsing error if the value cannot be converted to i64.
pub fn try_from_decimal(value: &str) -> Result<i64, ParseIntError> {
    value.replace('_', "").parse::<i64>()
}

/// Parse a hexadecimal integer literal (base 16).
///
/// # Errors
///
/// Returns a parsing error if the value cannot be converted to i64.
pub fn try_from_hexadecimal(value: &str) -> Result<i64, ParseIntError> {
    i64::from_str_radix(&value[2..].replace('_', ""), 16)
}
