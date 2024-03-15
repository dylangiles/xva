use crate::typechk::{expr::TypeExpr, ty::Type, var::Variable};

pub type TypeResult<T> = Result<T, TypeError>;

#[derive(Debug)]
pub enum TypeError {
    TypeUnknown(Variable),
    Malformed(Type),
    UnitIsNotUnit,
    Incompatible(TypeExpr, Type),
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
            TypeError::Malformed(ty) => write!(f, "The type {ty} is not well-formed."),
            TypeError::UnitIsNotUnit => write!(f, "The expression does not evaluate to ()."),
            TypeError::Incompatible(expr, ty) => write!(
                f,
                "Incompatible types: the given expression is not of the type {ty}."
            ),
            TypeError::NotAFunction(expr) => write!(f, "The expression {expr} is not a function."),
        }
    }
}
