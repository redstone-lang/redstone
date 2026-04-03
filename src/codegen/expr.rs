use inkwell::values::{FunctionValue, IntValue};
use crate::ast::{BinOp, Expr};
use super::{Codegen, Vars};

pub fn compile_expr<'ctx>(
    cg: &mut Codegen<'ctx>,
    expr: &Expr,
    func: FunctionValue<'ctx>,
    vars: &mut Vars<'ctx>,
) -> IntValue<'ctx> {
    let i64_t = cg.ctx.i64_type();
    match expr {
        Expr::Int(n) => i64_t.const_int(*n as u64, false),
        Expr::Var(name) => cg
            .builder
            .build_load(i64_t, vars[name], name)
            .unwrap()
            .into_int_value(),
        Expr::BinOp(lhs, op, rhs) => {
            let l = compile_expr(cg, lhs, func, vars);
            let r = compile_expr(cg, rhs, func, vars);
            match op {
                BinOp::Add => cg.builder.build_int_add(l, r, "add").unwrap(),
                BinOp::Sub => cg.builder.build_int_sub(l, r, "sub").unwrap(),
                BinOp::Mul => cg.builder.build_int_mul(l, r, "mul").unwrap(),
                BinOp::Div => cg.builder.build_int_signed_div(l, r, "div").unwrap(),
            }
        }
        Expr::Call(name, args) => {
            let callee = cg.module.get_function(name).expect("unknown function");
            let compiled_args: Vec<_> = args
                .iter()
                .map(|a| compile_expr(cg, a, func, vars).into())
                .collect();
            cg.builder
                .build_call(callee, &compiled_args, "call")
                .unwrap()
                .try_as_basic_value()
                .unwrap_basic()
                .into_int_value()
        }
    }
}
