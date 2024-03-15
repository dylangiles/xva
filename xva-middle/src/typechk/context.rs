use im::OrdMap;
use xva_ast::ast::LiteralKind;

use crate::typechk::{
    error::{TypeError, TypeResult},
    expr::TypeExpr,
    ty::{builtin_bool, builtin_char, builtin_float, builtin_int, builtin_string, Type},
    var::Variable,
};

#[derive(Debug, Clone)]
pub struct TypeContext {
    elems: OrdMap<Variable, Type>,
    type_var: usize,
}

impl TypeContext {
    pub(crate) fn check(&self, expr: &TypeExpr, against: &Type) -> TypeResult<Type> {
        // Under the context Γ, expression e checks against type A:

        match expr {
            TypeExpr::Unit => {
                if against.is_unit() {
                    Ok(Type::Unit)
                } else {
                    Err(TypeError::UnitIsNotUnit)
                }
            }

            TypeExpr::Literal(lit) => {
                let ty_bool = builtin_bool();
                let ty_int = builtin_int();
                let ty_float = builtin_float();
                let ty_char = builtin_char();
                let ty_string = builtin_string();

                let lit_check = |ty: &Type| {
                    if against == ty {
                        Ok(ty.clone())
                    } else {
                        Err(TypeError::Incompatible(expr.clone(), against.clone()))
                    }
                };

                match lit {
                    LiteralKind::Boolean(_) => lit_check(&ty_bool),
                    LiteralKind::Integer(_) => lit_check(&ty_int),
                    LiteralKind::Float(_) => lit_check(&ty_float),
                    LiteralKind::Char(_) => lit_check(&ty_char),
                    LiteralKind::String(_) => lit_check(&ty_string),

                    _ => Err(TypeError::Incompatible(expr.clone(), against.clone())),
                }
            }

            TypeExpr::Variable(var) => self
                .elems
                .get(var)
                .map_or_else(|| Err(TypeError::TypeUnknown(*var)), |ty| Ok(ty.clone())),

            TypeExpr::Abstraction(_, _) => todo!(),
            TypeExpr::Application(_, _) => todo!(),
            TypeExpr::Annotation(expr, ty) => {
                // Check that `expr` is indeed of type `ty` - that the annotation is well formed.
                let expr_ty = self.synthesise(expr)?;
                if &expr_ty == ty {
                    Ok(ty.clone())
                } else {
                    Err(TypeError::Incompatible(*expr.clone(), ty.clone()))
                }
            }
        }
    }

    pub(crate) fn synthesise(&self, expr: &TypeExpr) -> TypeResult<Type> {
        match expr {
            TypeExpr::Unit => Ok(Type::Unit),

            TypeExpr::Variable(var) => match self.elems.get(var) {
                Some(ty) => Ok(ty.clone()),
                None => Err(TypeError::TypeUnknown(*var)),
            },

            TypeExpr::Literal(lit) => match lit {
                LiteralKind::Boolean(_) => Ok(builtin_bool()),
                LiteralKind::Integer(_) => Ok(builtin_int()),
                LiteralKind::Float(_) => Ok(builtin_float()),
                LiteralKind::Char(_) => Ok(builtin_char()),
                LiteralKind::String(_) => Ok(builtin_string()),
            },

            TypeExpr::Annotation(_, _) => todo!(),
            TypeExpr::Abstraction(param, expr) => {
                let param_type = self
                    .elems
                    .get(param)
                    .map_or_else(|| Err(TypeError::TypeUnknown(*param)), |ty| Ok(ty.clone()))?;
                let expr_type = self.synthesise(expr)?;
                Ok(Type::Function(param_type.into(), expr_type.into()))
            }

            TypeExpr::Application(func, arg) => {
                // Find the type of the function
                let func_type = self.synthesise(func)?;

                // If it is indeed a function, then check the input type is the same type as the arg type.
                if let Type::Function(input, output) = func_type {
                    let arg_type = self.synthesise(arg)?;

                    // If they are compatible, then return the output type.
                    if *input == arg_type {
                        Ok(*output)
                    } else {
                        Err(TypeError::Incompatible(*arg.clone(), *input))
                    }
                } else {
                    // If the func type is not a function, raise a type error.
                    Err(TypeError::NotAFunction(*func.clone()))
                }
            }
        }
    }

    pub(crate) fn annotate<V>(&mut self, var: V, ty: Type) -> Option<Type>
    where
        V: Into<Variable>,
    {
        self.elems.insert(var.into(), ty)
    }

    pub(crate) fn fresh_type_var(&mut self) -> Variable {
        self.type_var += 1;
        format!("{}", self.type_var).into()
    }
}

impl Default for TypeContext {
    fn default() -> Self {
        Self {
            elems: {
                let mut temp: OrdMap<Variable, Type> = Default::default();

                temp.insert("bool".into(), builtin_bool());
                temp.insert("int".into(), builtin_int());
                temp.insert("char".into(), builtin_char());
                temp.insert("float".into(), builtin_float());
                temp.insert("string".into(), builtin_string());

                temp
            },
            type_var: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use internment::Intern;
    use xva_ast::ast::LiteralKind;

    use crate::typechk::{
        error::TypeResult,
        expr::TypeExpr,
        ty::{builtin_bool, builtin_char, builtin_float, builtin_int, builtin_string},
    };

    use super::TypeContext;

    #[test]
    fn synth_literal() -> TypeResult<()> {
        let tcx = TypeContext::default();

        assert_eq!(
            tcx.synthesise(&TypeExpr::Literal(LiteralKind::Boolean(false)))?,
            builtin_bool()
        );

        assert_eq!(
            tcx.synthesise(&TypeExpr::Literal(LiteralKind::Integer(1)))?,
            builtin_int()
        );

        assert_eq!(
            tcx.synthesise(&TypeExpr::Literal(LiteralKind::Float(1.0)))?,
            builtin_float()
        );

        assert_eq!(
            tcx.synthesise(&TypeExpr::Literal(LiteralKind::Char('c')))?,
            builtin_char()
        );
        assert_eq!(
            tcx.synthesise(&TypeExpr::Literal(LiteralKind::String(Intern::new(
                "hello".into()
            ))))?,
            builtin_string()
        );

        Ok(())
    }

    #[test]
    fn test() {
        let mut tcx = TypeContext::default();
        // let x: int
        let x_type = builtin_int();
        tcx.annotate("x", x_type.clone());

        // We know x is annotated as int, so the expr must be int
        let expr = TypeExpr::Literal(LiteralKind::Boolean(false));

        match tcx.check(&expr, &x_type) {
            Ok(t) => println!("{t}"),
            Err(e) => println!("{e}"),
        }
    }
}
