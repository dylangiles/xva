//! The Xva type checker.
//!
//! The Xva type system is based on the
//! [Hindley-Milner type system](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system),
//! using Algorithm W for type inference. Hindley-Milner itself has its roots in lambda calculus.
//!
//! # Abstractions
//! Lambda calculus is entirely functions. A function definition itself is referred to as an **abstraction**, notated
//! with a `λ`. For example, the identity function, which is a function with one input that returns its output, is
//! declared with the following abstraction:
//! ```text
//! λx -> x
//! ```
//!
//! The `x` in the above example is a variable, with is how we refer to the input of the function in lambda calculus.
//!
//! # Applications
//! "Calling" a function in lambda calculus is referred to as an **application** of that function.
//!
//! For example, abstracting and applying the identity function from above to the value `42` would look like this:
//! ```text
//! (λx -> x) 42
//! ```
//!
//! A more complex example:
//! ```text
//! (λx -> (λy -> x)) 1 2
//! ```
//!
//! In a language such as JavaScript, the above could be written as `(x => (y => x))(1)(2)`.
//!
//! To evaluate it:
//! 1. Pass the first argument, `1` as the first variable, `x`: `(λy -> 1) 2`
//! 2. Pass the second argument, `2` as  the second variable, `y`, which just comes out to `1`.
//!
//! # Lets
//! A let expression binds a name to something, inside an expression.
//!
//! For example:
//! ```text
//! let x = 3 in func x
//! ```
//! binds the value of 3 to the variable x in the application of `func x`, so we'd end up with an application: `func 3`.
//!
//! # Grammar
//! So far, we have the following grammar for lambda calculus:
//! ```text
//! e = x            [variable]
//!   | e₁ e₂        [application]
//!   | λx -> e      [abstraction]
//!   | let x = e₁ in e₂
//! ```
//! where `e` is an expression in lambda calculus, defined as a variable, an application, or an abstraction. The `|`
//! symbol means "or".
//!
//! # Inference
//! Type inference is the process of automatically identifying the types of expressions in a formal language.
//!
//! For example:
//! - `3` is inferred to be of the type `int`
//! - `true` is inferred to be of the type `bool`
//! - `isOdd` is an abstraction that takes an `int` and returns a `bool`
//! - `isOdd 3` is an application of `isOdd` to `3`. The whole expression would be inferred to be of the type `bool`.
//! - `isOdd true` is an application of `isOdd` to `true`, but `isOdd` is not defined for a input of type `bool`. We
//! would say that this expression is **not well-typed**.
//! - `λx -> x` is an abstraction that takes some type `α` and produces a value of type `α`.
//!
//! # Type functions
//! A type function is a function in lambda calculus that takes a type and produces a type.
//!
//! For example:
//! - `bool` is a type function that doesn't take any input, and produces the type `bool`. We can call this
//!    a type constant.
//! - `List bool` is a application of a function that takes `bool` and produces a list of booleans.
//! - `int -> bool` is an abstraction of a function that takes `int` and produces `bool`. This would be the type of the
//!   `isOdd` function that we discussed previously.
//! - `λα -> α` is an abstraction that takes some type `α` and produces some type `α`.
//!
//! # Monotypes
//! Monotypes are one possible type in the type inference system.
//!
//! They are defined as:
//! ```text
//! τ = α            [variable]
//!   | 𝐶 τ₁ ... τₙ  [application]
//!```
//! which means the type `τ` is equal to the type variable `α`, or the application of the type function
//! `𝐶` with arguments `τ₁ ... τₙ`. `𝐶` is in the set of type functions that must contain functions (`->`)
//!
//! So `𝐶` could be `𝐶 = { ->, int, bool, List<T>, Map<K, V> }` etc.
//!
//! # Polytypes
//! Polytypes are types with "for-all" quantifiers on them.
//!
//! An example for the identity function:
//! ```text
//! ∀α. α -> α
//! ```
//!
//! which is read as "for all types `α`, an abstraction that takes an α and returns an α".
//!
//! # Summary
//! Types in Hindley-Milner are defined as:
//! ```text
//! // Monotypes
//! τ = α            [variable]
//!   | 𝐶 τ₁ ... τₙ  [application]
//!
//! // Polytypes
//! σ = τ            [monotype]
//!   | ∀α. σ        [quantifier]
//! ```

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use internment::Intern;

pub enum Literal {
    Unit,
    Bool,
    Char,
    Int,
    Float,
    String,
}

impl Literal {
    pub fn infer(&self) -> Type {
        match self {
            Literal::Unit => Type::Unit,
            Literal::Bool => Type::Bool,
            Literal::Char => Type::Char,
            Literal::Int => Type::Int,
            Literal::Float => Type::Float,
            Literal::String => Type::String,
        }
    }
}

pub enum Type<'tcx> {
    Unit,
    Bool,
    Char,
    Int,
    Float,
    String,
    Variable(Intern<String>),
    Function(&'tcx Self, &'tcx Self),
}

/// The basis for our expressions in the typed lambda calculus.
/// ```text
/// e = x            [variable]
///   | e₁ e₂        [application]
///   | λx -> e      [abstraction]
/// ```
pub enum Expression<'tcx> {
    Literal(Literal),
    Variable(Intern<String>),
    Application(&'tcx Self, &'tcx Self),
    Abstraction(Intern<String>, &'tcx Self),
    Let(Intern<String>, &'tcx Self, &'tcx Self),
}

impl<'tcx> Expression<'tcx> {}

pub struct Polytype<'tcx> {
    quantifiers: &'tcx [Type<'tcx>],
    ty: Type<'tcx>,
}
// pub struct Environment(HashMap<)

pub struct Substitution();
trait Types {
    /// Returns the set of free type variables in `self`.
    fn frees(&self) -> HashSet<Intern<String>>;

    /// Applies the given substitution
    fn apply(&self, subst: &Substitution) -> Self;
}

#[cfg(test)]
mod tests {
    use internment::Intern;

    // use super::{Monotype, Polytype, TypeContext, Variable};

    // #[test]
    // fn test() {
    //     let mut context = TypeContext {
    //         map: Default::default(),
    //     };

    //     let var = Variable(Intern::new("x".into()));

    //     let ty = Polytype::Monotype(Monotype::Application()
    //     context.map.insert()
    // }
}
