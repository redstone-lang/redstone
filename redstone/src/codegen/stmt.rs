use inkwell::values::FunctionValue;
use crate::ast::Stmt;
use super::{Codegen, Vars};
use super::expr::compile_expr;

pub fn compile_stmt<'ctx>(
    cg: &mut Codegen<'ctx>,
    stmt: &Stmt,
    func: FunctionValue<'ctx>,
    vars: &mut Vars<'ctx>,
) {
    let i64_t = cg.ctx.i64_type();
    match stmt {
        Stmt::Let(name, expr) => {
            let val = compile_expr(cg, expr, func, vars);
            let slot = cg.builder.build_alloca(i64_t, name).unwrap();
            cg.builder.build_store(slot, val).unwrap();
            vars.insert(name.clone(), slot);
        }
        Stmt::Return(expr) => {
            let val = compile_expr(cg, expr, func, vars);
            cg.builder.build_return(Some(&val)).unwrap();
        }
        Stmt::Print(expr) => {
            let val = compile_expr(cg, expr, func, vars);
            let fmt = cg
                .builder
                .build_global_string_ptr("%lld\n", "fmt")
                .unwrap();
            let printf = cg.module.get_function("printf").unwrap();
            cg.builder
                .build_call(printf, &[fmt.as_pointer_value().into(), val.into()], "")
                .unwrap();
        }
        Stmt::Expr(expr) => {
            compile_expr(cg, expr, func, vars);
        }
    }
}
