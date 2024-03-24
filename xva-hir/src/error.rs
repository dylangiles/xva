use ariadne::{Color, Label, Report, ReportKind, Span};
use std::io::Write;
use xva_middle::typechk::error::TypeError;
use xva_span::{SourceId, SourceSpan};

#[derive(Debug, PartialEq)]
pub enum HirErrorKind {
    // UnexpectedEnd,
    // UnexpectedPattern(ErrorPattern),
    // InvalidUnicode(u32), // UnclosedDelimiter,
    // // NoEndBranch,
    // UninitedImmutable { expr_start: SourceSpan },
    TypeAnnoNeeded {
        expr_start: SourceSpan,
    },
    TypeError {
        err: TypeError,
        because_of: SourceSpan,
    },
}

#[derive(Debug)]
pub struct HirError {
    kind: HirErrorKind,
    span: SourceSpan,
    _label: Option<&'static str>,
}

impl HirError {
    pub const fn new(kind: HirErrorKind, span: SourceSpan) -> Self {
        Self {
            kind,
            span,
            _label: None,
        }
    }

    pub const fn type_anno_needed(span: SourceSpan, label: Option<&'static str>) -> Self {
        Self {
            kind: HirErrorKind::TypeAnnoNeeded { expr_start: span },
            span,
            _label: label,
        }
    }

    pub const fn type_error(span: SourceSpan, err: TypeError, because_of: SourceSpan) -> Self {
        Self {
            kind: HirErrorKind::TypeError { err, because_of },
            span,
            _label: None,
        }
    }

    pub fn write<C>(self, cache: C, writer: impl Write)
    where
        C: ariadne::Cache<SourceId>,
    {
        let msg = match &self.kind {
            HirErrorKind::TypeAnnoNeeded { .. } => {
                format!("Type annotations needed")
            }

            HirErrorKind::TypeError { err, .. } => {
                format!("{err}")
            }
        };
        // let some_rand: BingBong = 1;

        let report = Report::build(ReportKind::Error, self.span.src(), self.span.start())
            .with_code(3)
            .with_message(msg)
            .with_label(match &self.kind {
                HirErrorKind::TypeAnnoNeeded { expr_start } => Label::new(expr_start.clone())
                    .with_message("Type must be known at this point")
                    .with_color(Color::Cyan),
                HirErrorKind::TypeError { err, because_of } => match err {
                    TypeError::TypeUnknown(_) => todo!(),
                    TypeError::TypeNotFound(var) => Label::new(because_of.clone())
                        .with_message(format!("`{var}` is unknown in the current scope"))
                        .with_color(Color::Cyan),

                    TypeError::Malformed(_) => todo!(),
                    TypeError::UnitIsNotUnit => todo!(),
                    TypeError::Mismatched { expected, .. } => Label::new(because_of.clone())
                        .with_message(format!("Expected `{expected}` because of this"))
                        .with_color(Color::Cyan),
                    TypeError::NotAFunction(_) => todo!(),
                },
            });

        report.finish().write(cache, writer).unwrap();
    }
}
