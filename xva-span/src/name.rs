use internment::Intern;

use crate::SourceSpan;

/// A Xva identifier. This may be made up by a series of [`NameSegment`]s that further qualify a path
/// to a declaration.
#[derive(Debug, Clone)]
pub struct Name {
    pub span: SourceSpan,
    pub segments: Vec<NameSegment>,
}

impl Name {
    pub fn normalise(&self) -> Intern<String> {
        let seg_count = self.segments.len();
        let result = self
            .segments
            .iter()
            .enumerate()
            .fold(String::new(), |mut acc, (idx, seg)| {
                let NameSegment(ident) = seg;
                let Identifier { name, .. } = ident;
                if seg_count == 1 {
                    acc.push_str(name);
                } else if seg_count > 1 {
                    acc.push_str(
                        format!("{name}{}", if idx < (seg_count - 1) { "." } else { "" }).as_str(),
                    )
                }

                acc
            });

        Intern::new(result)
    }
}

/// A segment of a Xva identifier. For example, the name `brick.module.function` would contain three [`NameSegment`]s.
#[derive(Debug, Clone)]
pub struct NameSegment(pub Identifier);

impl NameSegment {
    pub fn ident(&self) -> Identifier {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Identifier {
    pub name: Intern<String>,
    pub span: SourceSpan,
}
