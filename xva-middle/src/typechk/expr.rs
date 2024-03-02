use internment::Intern;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Expression<'tcx> {
    Variable(Variable),

    Literal(Literal),

    Application {
        function: &'tcx Self,
        arg: &'tcx Self,
    },

    Abstraction {
        param: Variable,
        expr: &'tcx Self,
    },

    Let {
        ident: Variable,
        value: &'tcx Self,
        in_expr: &'tcx Self,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Variable(Intern<String>);

impl Variable {
    #[inline]
    pub fn new(ident: String) -> Self {
        Self(Intern::new(ident))
    }

    pub(crate) fn identifier(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier())
    }
}

impl From<String> for Variable {
    #[inline]
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Variable {
    fn from(value: &str) -> Self {
        Self::new(value.to_string())
    }
}

impl std::fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Variable(var) => write!(f, "{var}"),
            Expression::Literal(lit) => write!(f, "{lit}"),
            Expression::Application { function, arg } => write!(f, "({function}) -> {arg}"),
            Expression::Abstraction { param, expr } => write!(f, "λ{param} -> {expr}"),
            Expression::Let {
                ident,
                value,
                in_expr,
            } => write!(f, "let {ident} = {value} in {in_expr}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Literal {
    Unit,
    Boolean,
    Integer,
    Float,
    Char,
    String,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::Unit => "()",
                Literal::Boolean => "boolean",
                Literal::Integer => "int",
                Literal::Float => "float",
                Literal::Char => "char",
                Literal::String => "string",
            }
        )
    }
}

const UNIT: Expression<'_> = Expression::Literal(Literal::Unit);

#[cfg(test)]
mod tests {

    use super::{Expression, Variable, UNIT};

    #[test]
    fn identity() {
        let param: Variable = "x".into();
        let identity = Expression::Abstraction {
            param: param.clone(),
            expr: &Expression::Variable(param),
        };

        let let_ = Expression::Let {
            ident: "identity".into(),
            value: &identity,
            in_expr: &UNIT,
        };

        println!("{let_}");
    }
}
