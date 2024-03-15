use crate::typechk::var::Variable;

/// A type in the bi-directional system.
/// ```text
/// A, B, C ::= ()        [unit]
///            | α        [variable]
///            | ∃α       [existential]
///            | ∀α. A    [quantification]
///            | A → B    [function]
///
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Unit,
    Variable(Variable),
    Existential(Variable),
    Quantification(Variable, Box<Self>),
    Function(Box<Self>, Box<Self>),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => write!(f, "()"),
            Type::Variable(var) => write!(f, "{var}"),
            Type::Existential(var) => write!(f, "∃{var}"),
            Type::Quantification(var, ty) => write!(f, "(∀{var}. {ty})"),
            Type::Function(input, output) => write!(f, "({input} -> {output})"),
        }
    }
}

impl Type {
    pub fn var<V>(v: V) -> Self
    where
        V: Into<String>,
    {
        Type::Variable(v.into().into())
    }

    pub fn existential<V>(v: V) -> Self
    where
        V: Into<String>,
    {
        Type::Existential(v.into().into())
    }

    pub fn quantification<V>(var: V, ty: Type) -> Self
    where
        V: Into<String>,
    {
        Type::Quantification(var.into().into(), Box::new(ty))
    }

    pub fn function(input: Type, output: Type) -> Self {
        Type::Function(Box::new(input), Box::new(output))
    }

    /// Well-formed-ness of types.
    ///
    /// First, lets define some axioms:
    /// ```text
    /// ------
    /// Γ ⊢ 1            Unit is always well-formed,
    ///
    /// ---------
    /// Γ[α] ⊢ α         Universal variables are well formed if they are in the context
    ///
    /// ----------
    /// Γ[∃α] ⊢ ∃α       Unsolved existential variables are well formed if they are in the context
    ///
    ///
    /// --------------   Solved existential variables are well-formed if they are in the context.
    /// Γ[∃α = τ] ⊢ ∃α

    /// ```
    ///
    ///
    /// Given the above axioms, under the context `Γ`, type `A` is well-formed if:
    /// ```text
    /// Γ ⊢ A     Γ ⊢ B
    /// ---------------  Given type A and type B are in the context, the function A -> B is well-formed.
    ///    Γ ⊢ A → B
    ///
    /// Γ, α ⊢ A
    /// ---------        Given the variable α, the universal quantification of ∀α. A is well-formed.
    /// Γ ⊢ ∀α. A
    ///
    /// ```
    // pub(crate) fn is_well_formed(&self, tcx: &TypeContext) -> bool {
    //     match self {
    //         Type::Unit => true,
    //         Type::Variable(var) => tcx.has_variable(var),
    //         _ => unreachable!(), // Type::Existential(e_var) => {
    //                              //     tcx.has_existential(e_var) || tcx.get_solved(e_var).is_some()
    //                              // }
    //                              // Type::Quantification(forall, ty) => {
    //                              //     ty.is_well_formed(&tcx.add(*forall, TypeContextElement::Variable))
    //                              // }
    //                              // Type::Function(input, output) => {
    //                              //     input.is_well_formed(tcx) && output.is_well_formed(tcx)
    //                              // }
    //     }
    // }

    pub(crate) fn is_unit(&self) -> bool {
        if let Type::Unit = self {
            true
        } else {
            false
        }
    }
}

const BUILTIN_BOOL: &str = "bool";
const BUILTIN_INT: &str = "int";
const BUILTIN_FLOAT: &str = "float";
const BUILTIN_CHAR: &str = "char";
const BUILTIN_STRING: &str = "string";

pub(crate) fn builtin_bool() -> Type {
    Type::Variable(BUILTIN_BOOL.into())
}

pub(crate) fn builtin_int() -> Type {
    Type::Variable(BUILTIN_INT.into())
}

pub(crate) fn builtin_float() -> Type {
    Type::Variable(BUILTIN_FLOAT.into())
}

pub(crate) fn builtin_char() -> Type {
    Type::Variable(BUILTIN_CHAR.into())
}

pub(crate) fn builtin_string() -> Type {
    Type::Variable(BUILTIN_STRING.into())
}

#[cfg(test)]
mod tests {
    use crate::typechk::ty::Type;

    #[test]
    fn display() {
        println!("{}", Type::Unit);
        println!("{}", Type::var("t"));
        println!("{}", Type::existential("a"));
        println!("{}", Type::quantification("a", Type::var("A")));
        println!("{}", Type::function(Type::var("t1"), Type::var("t2")));
    }
}
