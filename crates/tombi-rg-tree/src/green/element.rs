use std::borrow::Cow;

use super::GreenTokenData;
use crate::{
    GreenNodeData, NodeOrToken,
    green::{GreenNode, GreenToken, SyntaxKind},
};

pub(super) type GreenElement = NodeOrToken<GreenNode, GreenToken>;
pub type GreenElementRef<'a> = NodeOrToken<&'a GreenNodeData, &'a GreenTokenData>;

impl From<GreenNode> for GreenElement {
    #[inline]
    fn from(node: GreenNode) -> Self {
        Self::Node(node)
    }
}

impl<'a> From<&'a GreenNode> for GreenElementRef<'a> {
    #[inline]
    fn from(node: &'a GreenNode) -> Self {
        NodeOrToken::Node(node)
    }
}

impl From<GreenToken> for GreenElement {
    #[inline]
    fn from(token: GreenToken) -> Self {
        Self::Token(token)
    }
}

impl From<Cow<'_, GreenNodeData>> for GreenElement {
    #[inline]
    fn from(cow: Cow<'_, GreenNodeData>) -> Self {
        Self::Node(cow.into_owned())
    }
}

impl<'a> From<&'a GreenToken> for GreenElementRef<'a> {
    #[inline]
    fn from(token: &'a GreenToken) -> Self {
        NodeOrToken::Token(token)
    }
}

impl GreenElementRef<'_> {
    #[must_use]
    pub fn to_owned(self) -> GreenElement {
        match self {
            NodeOrToken::Node(it) => NodeOrToken::Node(it.to_owned()),
            NodeOrToken::Token(it) => NodeOrToken::Token(it.to_owned()),
        }
    }
}

impl GreenElement {
    /// Returns kind of this element.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> SyntaxKind {
        self.as_deref().kind()
    }

    /// Returns the length of the text covered by this element.
    #[inline]
    #[must_use]
    pub fn text_len(&self) -> tombi_text::RelativeOffset {
        self.as_deref().text_len()
    }

    #[inline]
    #[must_use]
    pub fn text_relative_position(&self) -> tombi_text::RelativePosition {
        self.as_deref().text_relative_position()
    }
}

impl GreenElementRef<'_> {
    /// Returns kind of this element.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> SyntaxKind {
        match self {
            NodeOrToken::Node(it) => it.kind(),
            NodeOrToken::Token(it) => it.kind(),
        }
    }

    /// Returns the length of the text covered by this element.
    #[inline]
    #[must_use]
    pub fn text_len(self) -> tombi_text::RawOffset {
        match self {
            NodeOrToken::Node(it) => it.text_len(),
            NodeOrToken::Token(it) => it.text_len(),
        }
    }

    #[inline]
    #[must_use]
    pub fn text_relative_position(self) -> tombi_text::RelativePosition {
        match self {
            NodeOrToken::Node(it) => it.text_relative_position(),
            NodeOrToken::Token(it) => it.text_relative_position(),
        }
    }
}
