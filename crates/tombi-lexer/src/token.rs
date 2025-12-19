use tombi_syntax::SyntaxKind;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Token {
    kind: SyntaxKind,
    span: tombi_text::Span,
    range: tombi_text::Range,
}

impl Token {
    #[must_use]
    pub const fn new(
        kind: SyntaxKind,
        (span, range): (tombi_text::Span, tombi_text::Range),
    ) -> Self {
        Self { kind, span, range }
    }

    #[must_use]
    pub const fn eof() -> Self {
        Self {
            kind: SyntaxKind::EOF,
            span: tombi_text::Span::MAX,
            range: tombi_text::Range::MAX,
        }
    }

    #[inline]
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.kind == SyntaxKind::EOF
    }

    #[inline]
    #[must_use]
    pub const fn kind(&self) -> SyntaxKind {
        self.kind
    }

    #[inline]
    #[must_use]
    pub const fn span(&self) -> tombi_text::Span {
        self.span
    }

    #[inline]
    #[must_use]
    pub const fn range(&self) -> tombi_text::Range {
        self.range
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @{} @{}", self.kind, self.span, self.range)
    }
}
