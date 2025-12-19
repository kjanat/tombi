use std::iter;

use super::{node::RedNode, token::RedToken};
use crate::{Language, NodeOrToken, cursor};

pub type RedElement<L> = NodeOrToken<RedNode<L>, RedToken<L>>;

impl<L: Language> From<RedNode<L>> for RedElement<L> {
    fn from(node: RedNode<L>) -> Self {
        Self::Node(node)
    }
}

impl<L: Language> From<RedToken<L>> for RedElement<L> {
    fn from(token: RedToken<L>) -> Self {
        Self::Token(token)
    }
}

impl<L: Language> RedElement<L> {
    #[must_use]
    pub fn span(&self) -> tombi_text::Span {
        match self {
            Self::Node(it) => it.span(),
            Self::Token(it) => it.span(),
        }
    }

    #[must_use]
    pub fn range(&self) -> tombi_text::Range {
        match self {
            Self::Node(it) => it.range(),
            Self::Token(it) => it.range(),
        }
    }

    #[must_use]
    pub fn index(&self) -> usize {
        match self {
            Self::Node(it) => it.index(),
            Self::Token(it) => it.index(),
        }
    }

    #[must_use]
    pub fn kind(&self) -> L::Kind {
        match self {
            Self::Node(it) => it.kind(),
            Self::Token(it) => it.kind(),
        }
    }

    #[must_use]
    pub fn parent(&self) -> Option<RedNode<L>> {
        match self {
            Self::Node(it) => it.parent(),
            Self::Token(it) => it.parent(),
        }
    }

    pub fn ancestors(&self) -> impl Iterator<Item = RedNode<L>> {
        let first = match self {
            Self::Node(it) => Some(it.clone()),
            Self::Token(it) => it.parent(),
        };
        iter::successors(first, RedNode::parent)
    }

    #[must_use]
    pub fn next_sibling_or_token(&self) -> Option<Self> {
        match self {
            Self::Node(it) => it.next_sibling_or_token(),
            Self::Token(it) => it.next_sibling_or_token(),
        }
    }
    #[must_use]
    pub fn prev_sibling_or_token(&self) -> Option<Self> {
        match self {
            Self::Node(it) => it.prev_sibling_or_token(),
            Self::Token(it) => it.prev_sibling_or_token(),
        }
    }
    pub fn detach(&self) {
        match self {
            Self::Node(it) => it.detach(),
            Self::Token(it) => it.detach(),
        }
    }
}

impl<L: Language> From<cursor::SyntaxElement> for RedElement<L> {
    fn from(raw: cursor::SyntaxElement) -> Self {
        match raw {
            NodeOrToken::Node(it) => Self::Node(it.into()),
            NodeOrToken::Token(it) => Self::Token(it.into()),
        }
    }
}

impl<L: Language> From<RedElement<L>> for cursor::SyntaxElement {
    fn from(element: RedElement<L>) -> Self {
        match element {
            NodeOrToken::Node(it) => Self::Node(it.into()),
            NodeOrToken::Token(it) => Self::Token(it.into()),
        }
    }
}
