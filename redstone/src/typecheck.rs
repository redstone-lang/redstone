use std::collections::HashMap;
use crate::ast::{BinOp, Expr, Function, Stmt, Type};

#[derive(Debug)]
pub struct TypeError {
    pub message: String,
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "type error: {}", self.message)
    }
}

/// Typed IR — same structure as AST but every node carries a resolved Type.
#[derive(Debug, Clone)]
pub enum TExpr {
    Int(i64, Type),
    Float(f64, Type),
    Bool(bool),
    Char(char),
    Unit,
    Var(String, Type),
    BinOp(Box<TExpr>, BinOp, Box<TExpr>, Type),
    Call(String, Vec<TExpr>, Type),
}

impl TExpr {
    pub fn ty(&self) -> Type {
        match self {
            TExpr::Int(_, t)       => t.clone(),
            TExpr::Float(_, t)     => t.clone(),
            TExpr::Bool(_)         => Type::Bool,
            TExpr::Char(_)         => Type::Char,
            TExpr::Unit            => Type::Unit,
            TExpr::Var(_, t)       => t.clone(),
            TExpr::BinOp(_, _, _, t) => t.clone(),
            TExpr::Call(_, _, t)   => t.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TStmt {
    Let(String, TExpr),
    Return(TExpr),
    Print(TExpr),
    Expr(TExpr),
}

#[derive(Debug, Clone)]
pub struct TFunction {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub ret: Type,
    pub body: Vec<TStmt>,
}

// Maps function name → (param types, return type)
type FnSigs = HashMap<String, (Vec<Type>, Type)>;
// Maps variable name → type
type Env = HashMap<String, Type>;

pub fn check_program(fns: &[Function]) -> Result<Vec<TFunction>, TypeError> {
    // First pass: collect all function signatures so calls can be resolved.
    let mut sigs: FnSigs = HashMap::new();
    for f in fns {
        let params = f.params.iter().map(|p| {
            p.ty.clone().ok_or_else(|| TypeError {
                message: format!("parameter `{}` in function `{}` needs a type annotation", p.name, f.name),
            })
        }).collect::<Result<Vec<_>, _>>()?;

        let ret = f.ret.clone().unwrap_or(Type::Unit);
        sigs.insert(f.name.clone(), (params, ret));
    }

    fns.iter().map(|f| check_fn(f, &sigs)).collect()
}

fn check_fn(f: &Function, sigs: &FnSigs) -> Result<TFunction, TypeError> {
    let mut env: Env = HashMap::new();

    let (param_types, ret) = sigs.get(&f.name).unwrap();
    let params: Vec<(String, Type)> = f.params.iter().zip(param_types)
        .map(|(p, ty)| {
            env.insert(p.name.clone(), ty.clone());
            (p.name.clone(), ty.clone())
        })
        .collect();

    let body = f.body.iter()
        .map(|s| check_stmt(s, &mut env, sigs, ret))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TFunction { name: f.name.clone(), params, ret: ret.clone(), body })
}

fn check_stmt(stmt: &Stmt, env: &mut Env, sigs: &FnSigs, ret_ty: &Type) -> Result<TStmt, TypeError> {
    match stmt {
        Stmt::Let(name, ann, expr) => {
            let texpr = infer_expr(expr, ann.as_ref(), env, sigs)?;
            env.insert(name.clone(), texpr.ty());
            Ok(TStmt::Let(name.clone(), texpr))
        }
        Stmt::Return(expr) => {
            let texpr = infer_expr(expr, Some(ret_ty), env, sigs)?;
            Ok(TStmt::Return(texpr))
        }
        Stmt::Print(expr) => {
            let texpr = infer_expr(expr, None, env, sigs)?;
            Ok(TStmt::Print(texpr))
        }
        Stmt::Expr(expr) => {
            let texpr = infer_expr(expr, None, env, sigs)?;
            Ok(TStmt::Expr(texpr))
        }
    }
}

/// Infer the type of an expression.
/// `hint` is the expected type from context (annotation or return type).
fn infer_expr(expr: &Expr, hint: Option<&Type>, env: &Env, sigs: &FnSigs) -> Result<TExpr, TypeError> {
    match expr {
        Expr::Int(n) => {
            let ty = hint
                .filter(|t| t.is_numeric())
                .cloned()
                .ok_or_else(|| TypeError {
                    message: format!(
                        "integer literal `{n}` has ambiguous type; add a type annotation (e.g. `let x: i32 = {n};`)"
                    ),
                })?;
            Ok(TExpr::Int(*n, ty))
        }
        Expr::Float(f) => {
            let ty = match hint {
                Some(Type::F32) => Type::F32,
                _ => Type::F64, // float literals default to f64, like Rust
            };
            Ok(TExpr::Float(*f, ty))
        }
        Expr::Bool(b) => Ok(TExpr::Bool(*b)),
        Expr::Char(c) => Ok(TExpr::Char(*c)),
        Expr::Unit    => Ok(TExpr::Unit),

        Expr::Var(name) => {
            let ty = env.get(name).cloned().ok_or_else(|| TypeError {
                message: format!("undefined variable `{name}`"),
            })?;
            Ok(TExpr::Var(name.clone(), ty))
        }

        Expr::BinOp(lhs, op, rhs) => {
            let tlhs = infer_expr(lhs, hint, env, sigs)?;
            let trhs = infer_expr(rhs, Some(&tlhs.ty()), env, sigs)?;
            let ty = tlhs.ty();
            if ty != trhs.ty() {
                return Err(TypeError {
                    message: format!("type mismatch in binary operation: `{:?}` vs `{:?}`", ty, trhs.ty()),
                });
            }
            Ok(TExpr::BinOp(Box::new(tlhs), op.clone(), Box::new(trhs), ty))
        }

        Expr::Call(name, args) => {
            let (param_types, ret) = sigs.get(name).ok_or_else(|| TypeError {
                message: format!("undefined function `{name}`"),
            })?;
            if args.len() != param_types.len() {
                return Err(TypeError {
                    message: format!(
                        "function `{name}` expects {} arguments, got {}",
                        param_types.len(), args.len()
                    ),
                });
            }
            let targs = args.iter().zip(param_types)
                .map(|(a, pt)| infer_expr(a, Some(pt), env, sigs))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(TExpr::Call(name.clone(), targs, ret.clone()))
        }
    }
}

impl Type {
    fn is_numeric(&self) -> bool {
        matches!(self,
            Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::Isize |
            Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128 | Type::Usize |
            Type::F32 | Type::F64
        )
    }
}
