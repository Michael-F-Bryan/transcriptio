#[rustfmt::skip]
pub mod ast;
#[rustfmt::skip]
mod syntax_kind;

pub use self::syntax_kind::SyntaxKind;

/// A tag type used by [`rowan`] to represent the WebVTT language.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WebVTT;

impl rowan::Language for WebVTT {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from_code(raw.0).unwrap()
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

/// A [`rowan::SyntaxToken`] specialised for use with [`WebVTT`].
pub type SyntaxToken = rowan::SyntaxToken<WebVTT>;

/// A [`rowan::SyntaxNode`] specialised for use with [`WebVTT`].
pub type SyntaxNode = rowan::SyntaxNode<WebVTT>;
