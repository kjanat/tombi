use std::marker::PhantomData;

use tombi_ast::AstNode;
use tombi_syntax::SyntaxNode;

#[derive(Debug, PartialEq, Eq)]
pub struct Parsed<T> {
    green_tree: tombi_rg_tree::GreenNode,
    pub errors: Vec<crate::Error>,
    _ty: PhantomData<fn() -> T>,
}

impl<T> Parsed<T> {
    #[must_use]
    pub fn new(green_tree: tombi_rg_tree::GreenNode, errors: Vec<crate::Error>) -> Self {
        Self {
            green_tree,
            errors,
            _ty: PhantomData,
        }
    }

    #[must_use]
    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_tree.clone())
    }

    #[must_use]
    pub fn into_syntax_node(self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_tree)
    }

    #[must_use]
    pub fn into_syntax_node_mut(self) -> SyntaxNode {
        SyntaxNode::new_root_mut(self.green_tree)
    }
}

impl<T> Clone for Parsed<T> {
    fn clone(&self) -> Self {
        Self {
            green_tree: self.green_tree.clone(),
            errors: self.errors.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T: AstNode> Parsed<T> {
    /// Converts this parse result into a parse result for an untyped syntax tree.
    #[must_use]
    pub fn into_syntax(self) -> Parsed<SyntaxNode> {
        Parsed {
            green_tree: self.green_tree,
            errors: self.errors,
            _ty: PhantomData,
        }
    }

    /// Gets the parsed syntax tree as a typed ast node.
    ///
    /// # Panics
    ///
    /// Panics if the root node cannot be casted into the typed ast node
    /// (e.g. if it's an `ERROR` node).
    #[must_use]
    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }
}

impl Parsed<SyntaxNode> {
    #[must_use]
    pub fn cast<N: AstNode>(self) -> Option<Parsed<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parsed {
                green_tree: self.green_tree,
                errors: self.errors,
                _ty: PhantomData,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub fn into_node_and_errors<N: AstNode>(self) -> (N, Vec<crate::Error>) {
        let Some(parsed) = self.cast::<N>() else {
            unreachable!("TOML node is always a valid AST node even if source is empty.")
        };

        (parsed.tree(), parsed.errors)
    }

    #[inline]
    #[must_use]
    pub fn into_root_and_errors(self) -> (tombi_ast::Root, Vec<crate::Error>) {
        self.into_node_and_errors::<tombi_ast::Root>()
    }

    pub fn try_into_node<N: AstNode>(self) -> Result<N, Vec<crate::Error>> {
        let (root, errors) = self.into_node_and_errors::<N>();

        if errors.is_empty() {
            Ok(root)
        } else {
            Err(errors)
        }
    }

    #[inline]
    pub fn try_into_root(self) -> Result<tombi_ast::Root, Vec<crate::Error>> {
        self.try_into_node::<tombi_ast::Root>()
    }
}
