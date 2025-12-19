use nu_ansi_term::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Level {
    ERROR,
    WARNING,
}

impl Level {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ERROR => "Error",
            Self::WARNING => "Warning",
        }
    }

    #[must_use]
    pub const fn as_padded_str(&self) -> &'static str {
        match self {
            Self::ERROR => "  Error",
            Self::WARNING => "Warning",
        }
    }

    #[must_use]
    pub const fn color(&self) -> Color {
        match self {
            Self::ERROR => Color::Red,
            Self::WARNING => Color::Yellow,
        }
    }
}

impl From<Level> for Style {
    fn from(val: Level) -> Self {
        match val {
            Level::ERROR => Self::new().bold().fg(Color::Red),
            Level::WARNING => Self::new().bold().fg(Color::Yellow),
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color().bold().paint(self.as_str()))
    }
}
