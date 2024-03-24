use xva_span::LiteralKind;

use crate::typechk::{context::TypeContext, ty::Type, var::Variable};

/// An expression in the bi-directional system.
///
/// ```text
/// e ::= x        [variable]
///    | ()        [unit]
///    | λx. e     [abstraction]
///    | e e       [application]
///    | (e : A)   [annotation]
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpr {
    /// A variable, suchg as `x`
    Variable(Variable),

    /// The unit type, `()`
    Unit,

    /// A literal expression
    Literal(LiteralKind),

    /// A function abstraction, `λx. e`
    Abstraction(Variable, Box<Self>),

    /// The application of a function, `e e`
    Application(Box<Self>, Box<Self>),

    /// An expression annotated with a particular type, `(e : A)`
    Annotation(Box<Self>, Type),
}

impl std::fmt::Display for TypeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeExpr::Variable(var) => write!(f, "{var}"),
            TypeExpr::Unit => write!(f, "()"),
            TypeExpr::Literal(lit) => write!(f, "{lit}"),
            TypeExpr::Abstraction(param, expr) => write!(f, "(λ{param}. {expr})"),
            TypeExpr::Application(e1, e2) => write!(f, "({e1} {e2})"),
            TypeExpr::Annotation(expr, ty) => write!(f, "({expr}: {ty})"),
        }
    }
}

impl TypeExpr {
    pub fn var<V>(v: V) -> Self
    where
        V: Into<String>,
    {
        TypeExpr::Variable(v.into().into())
    }

    pub fn abstraction<V>(param: V, expr: TypeExpr) -> Self
    where
        V: Into<String>,
    {
        TypeExpr::Abstraction(param.into().into(), Box::new(expr))
    }

    pub fn application(abs: TypeExpr, input: TypeExpr) -> Self {
        TypeExpr::Application(Box::new(abs), Box::new(input))
    }

    pub fn annotation(expr: TypeExpr, ty: Type, tcx: &mut TypeContext) -> Self {
        let var = match expr {
            TypeExpr::Variable(var) => var,
            TypeExpr::Unit | TypeExpr::Literal(_) => unreachable!(),
            _ => tcx.fresh_type_var(),
        };

        let _ = tcx.annotate(var, ty.clone());

        TypeExpr::Annotation(Box::new(expr), ty)
    }
}

// #[derive(Debug, Clone, Copy)]
// pub enum Literal {
//     Bool(bool),
//     Int(i128),
//     Float(f64),
//     Char(char),
//     String(Intern<String>),
// }

// impl std::fmt::Display for Literal {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Literal::Bool(b) => write!(f, "{b}"),
//             Literal::Int(i) => write!(f, "{i}"),
//             Literal::Float(fl) => write!(f, "{fl}"),
//             Literal::Char(c) => write!(f, "'{c}'"),
//             Literal::String(s) => write!(f, "\"{s}\""),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::TypeExpr;
    use crate::typechk::{context::TypeContext, ty::Type};

    #[test]
    fn display() {
        let mut tcx = TypeContext::default();
        println!("{}", TypeExpr::Unit);
        println!("{}", TypeExpr::var("x"));
        println!("{}", TypeExpr::abstraction("id", TypeExpr::var("id")));
        println!(
            "{}",
            TypeExpr::application(
                TypeExpr::abstraction("id", TypeExpr::var("id")),
                TypeExpr::var("x")
            )
        );
        println!(
            "{}",
            TypeExpr::annotation(TypeExpr::var("x"), Type::var("int"), &mut tcx)
        );
    }
}
