#[derive(thiserror::Error, Default, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[default]
    #[error("Invalid token")]
    InvalidToken,
}

impl Error {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::InvalidToken => "Invalid token",
        }
    }
}
