#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompletionKind {
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
    Key,
    MagicTrigger,
    CommentDirective,
}

impl CompletionKind {
    #[must_use]
    pub const fn is_literal(&self) -> bool {
        matches!(
            self,
            Self::Boolean
                | Self::Integer
                | Self::Float
                | Self::String
                | Self::OffsetDateTime
                | Self::LocalDateTime
                | Self::LocalDate
                | Self::LocalTime
        )
    }
}

impl From<CompletionKind> for tower_lsp::lsp_types::CompletionItemKind {
    fn from(kind: CompletionKind) -> Self {
        // NOTE: All TOML completions are CompletionItemKind::VALUE,
        //       but some are assigned different types to make it easier to distinguish by symbols.
        match kind {
            CompletionKind::Boolean => Self::ENUM_MEMBER,
            CompletionKind::Integer => Self::VALUE,
            CompletionKind::Float => Self::VALUE,
            CompletionKind::String => Self::TEXT,
            // NOTE: Event is related to time
            CompletionKind::OffsetDateTime => Self::EVENT,
            CompletionKind::LocalDateTime => Self::EVENT,
            CompletionKind::LocalDate => Self::EVENT,
            CompletionKind::LocalTime => Self::EVENT,
            CompletionKind::Array => Self::STRUCT,
            CompletionKind::Table => Self::STRUCT,
            CompletionKind::Key => Self::FIELD,
            // NOTE: To give a writing taste close to method chaining
            CompletionKind::MagicTrigger => Self::METHOD,
            CompletionKind::CommentDirective => Self::KEYWORD,
        }
    }
}
