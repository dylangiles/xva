use std::sync::atomic::{AtomicU64, Ordering};

use im::{OrdMap, OrdSet};
use internment::Intern;

use super::expr::Variable;

// pub struct Substitution<'tcx>(OrdMap<Intern<String>, &'tcx Type<'tcx>>);

pub(self) static TYPE_VAR_SEED: AtomicU64 = AtomicU64::new(0);

fn next_type_var() -> TypeVariable {
    TYPE_VAR_SEED.fetch_add(1, Ordering::SeqCst).into()
}

/// A type variable - an unknown type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeVariable(u64);

impl TypeVariable {
    pub fn fresh() -> Self {
        next_type_var()
    }
}

impl From<u64> for TypeVariable {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

/// Monotypes - fully instantiated, unquantified types
pub enum Type<'tcx> {
    /// A fully constructed type: `int`, `List<T>`, etc.
    Full(Variable, &'tcx [Type<'tcx>]),

    /// A type variable
    Variable(TypeVariable),
}

/// Polytypes, or type schemes - uninstantiated types with universal quantification.
/// ```text
/// σ = τ            [monotype]
///   | ∀α. σ        [quantifier]
/// ```
pub enum TypeScheme<'tcx> {
    /// Types that are not polymorphic, such as `int -> boolean`
    Monotype(&'tcx Type<'tcx>),

    /// Types that are polymorphic,
    Polytype {
        variable: TypeVariable,
        body: &'tcx Self,
    },
}

pub struct TypeContext<'tcx> {
    map: OrdMap<TypeVariable, &'tcx Type<'tcx>>,
    names: OrdMap<TypeVariable, Intern<String>>,
}

// impl<'tcx> Substitution<'tcx> {
//     /// Compose `self` with `other` by performing a union with `self` and the substitution created by
//     /// the application of `self` to every type in `other`.
//     fn compose(&self, other: &Self) -> Self {
//         let (Self(sub1), Self(sub2)) = (self, other);
//         let sub2 = sub2
//             .into_iter()
//             .map(|(k, v)| (k.clone(), &v.apply(self)))
//             .collect();
//         let sub3 = sub1.clone().union(sub2);
//         Self(sub3)
//     }

//     #[inline]
//     pub(super) fn lookup(&self, ident: &Intern<String>) -> Option<&Type<'tcx>> {
//         let Self(map) = self;
//         map.get(ident).map(|ty| *ty)
//     }

//     fn remove(&self, vars: &OrdSet<Intern<String>>) -> Self {
//         let Self(map) = self;
//         Self(
//             map.iter()
//                 .filter(|(var, ty)| vars.contains(var))
//                 .map(|(var, ty)| (var.clone(), *ty))
//                 .collect(),
//         )
//     }
// }

// impl Subst {
//     /// Compose `self` with `s2` by unioning `self` with the substitution created by the application of `self` to every type in `other`
//     fn compose(&self, s2: Self) -> Self {
//         let (Self(s1), Self(s2)) = (self, s2);
//         let s2 = s2.into_iter().map(|(k, v)| (k, v.apply(self))).collect();
//         let s3 = s1.clone().union(s2);
//         Subst(s3)
//     }

//     /// Lookup `var` in `self`
//     #[inline]
//     fn lookup(&self, var: &Var) -> Option<Type> {
//         self.0.get(var).map(Clone::clone)
//     }

//     // Return a clone of `self` with `vars` removed
//     fn remove(&self, vars: &OrdSet<Var>) -> Self {
//         Self(
//             self.0
//                 .iter()
//                 .filter(|(var, _)| vars.contains(*var))
//                 .map(|(var, ty)| (var.clone(), ty.clone()))
//                 .collect(),
//         )
//     }
// }
