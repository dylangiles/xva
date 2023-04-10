use super::language::SyntaxKind;

#[derive(Debug)]
pub(super) enum Event<'text> {
    StartNode { kind: SyntaxKind },
    StartNodeAt { kind: SyntaxKind, checkpoint: usize },
    AddToken { kind: SyntaxKind, text: &'text str },
    FinishNode,
}
