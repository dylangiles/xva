use crate::typechk::{expr::TypeExpr, ty::Type, var::Variable};

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug, PartialEq, Eq)]
pub enum TypeError {
    TypeUnknown(Variable),
    TypeNotFound(Variable),
    Malformed(Type),
    UnitIsNotUnit,
    Mismatched {
        expr: TypeExpr,
        expected: Type,
        found: Type,
    },
    NotAFunction(TypeExpr),
}

impl std::error::Error for TypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::TypeUnknown(var) => write!(f, "The type of {var} is not known."),
            TypeError::TypeNotFound(var) => {
                write!(f, "The type `{var}` was not found in the current scope.")
            }
            TypeError::Malformed(ty) => write!(f, "The type {ty} is not well-formed."),
            TypeError::UnitIsNotUnit => write!(f, "The expression does not evaluate to ()."),
            TypeError::Mismatched {
                expected, found, ..
            } => write!(
                f,
                "Mismatched types: expected `{expected}`, found `{found}`"
            ),
            TypeError::NotAFunction(expr) => write!(f, "The expression {expr} is not a function."),
        }
    }
}
