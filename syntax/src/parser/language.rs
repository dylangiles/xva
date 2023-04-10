#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum SyntaxKind {
    Root = 0,
    Unknown,
    Error,

    Whitespace,
    LineComment,

    // Symbols
    Plus,
    Minus,
    Star,
    Slash,
    DoubleStar,
    OpenParenthesis,
    CloseParenthesis,

    // Composite nodes
    BinaryExpression,
    Literal,
    ParenthesisedExpression,
    UnaryExpression,
}

/// Converts our `SyntaxKind` into a Rowan-friendly one
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// Teaches rowan to convert between the two SyntaxKind types, allowing for a nicer SyntaxNode API where
/// "kinds" are values from our `enum SyntaxKind`, instead of plain u16 values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum XvaLanguage {}
impl rowan::Language for XvaLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Root as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
