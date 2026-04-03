use inkwell::values::{BasicValueEnum, FunctionValue};
use crate::ast::BinOp;
use crate::typecheck::TExpr;
use super::{Codegen, Vars};
use super::types::llvm_type;

pub fn compile_expr<'ctx>(
    cg: &mut Codegen<'ctx>,
    expr: &TExpr,
    func: FunctionValue<'ctx>,
    vars: &mut Vars<'ctx>,
) -> BasicValueEnum<'ctx> {
    match expr {
        TExpr::Int(n, ty) => {
            llvm_type(cg.ctx, ty).into_int_type().const_int(*n as u64, ty.is_signed()).into()
        }
        TExpr::Float(f, ty) => {
            llvm_type(cg.ctx, ty).into_float_type().const_float(*f).into()
        }
        TExpr::Bool(b) => cg.ctx.bool_type().const_int(*b as u64, false).into(),
        TExpr::Char(c) => cg.ctx.i32_type().const_int(*c as u64, false).into(),
        TExpr::Unit    => cg.ctx.i8_type().const_int(0, false).into(),

        TExpr::Var(name, _) => {
            let (ptr, ty) = &vars[name];
            cg.builder.build_load(*ty, *ptr, name).unwrap()
        }

        TExpr::BinOp(lhs, op, rhs, _) => {
            let l = compile_expr(cg, lhs, func, vars);
            let r = compile_expr(cg, rhs, func, vars);
            match (l, r) {
                (BasicValueEnum::IntValue(lv), BasicValueEnum::IntValue(rv)) => match op {
                    BinOp::Add => cg.builder.build_int_add(lv, rv, "add").unwrap().into(),
                    BinOp::Sub => cg.builder.build_int_sub(lv, rv, "sub").unwrap().into(),
                    BinOp::Mul => cg.builder.build_int_mul(lv, rv, "mul").unwrap().into(),
                    BinOp::Div => cg.builder.build_int_signed_div(lv, rv, "div").unwrap().into(),
                    BinOp::Lt  => cg.builder.build_int_compare(inkwell::IntPredicate::SLT, lv, rv, "lt").unwrap().into(),
                    BinOp::Gt  => cg.builder.build_int_compare(inkwell::IntPredicate::SGT, lv, rv, "gt").unwrap().into(),
                    BinOp::Le  => cg.builder.build_int_compare(inkwell::IntPredicate::SLE, lv, rv, "le").unwrap().into(),
                    BinOp::Ge  => cg.builder.build_int_compare(inkwell::IntPredicate::SGE, lv, rv, "ge").unwrap().into(),
                    BinOp::Eq  => cg.builder.build_int_compare(inkwell::IntPredicate::EQ,  lv, rv, "eq").unwrap().into(),
                    BinOp::Ne  => cg.builder.build_int_compare(inkwell::IntPredicate::NE,  lv, rv, "ne").unwrap().into(),
                },
                (BasicValueEnum::FloatValue(lv), BasicValueEnum::FloatValue(rv)) => match op {
                    BinOp::Add => cg.builder.build_float_add(lv, rv, "fadd").unwrap().into(),
                    BinOp::Sub => cg.builder.build_float_sub(lv, rv, "fsub").unwrap().into(),
                    BinOp::Mul => cg.builder.build_float_mul(lv, rv, "fmul").unwrap().into(),
                    BinOp::Div => cg.builder.build_float_div(lv, rv, "fdiv").unwrap().into(),
                    BinOp::Lt  => cg.builder.build_float_compare(inkwell::FloatPredicate::OLT, lv, rv, "flt").unwrap().into(),
                    BinOp::Gt  => cg.builder.build_float_compare(inkwell::FloatPredicate::OGT, lv, rv, "fgt").unwrap().into(),
                    BinOp::Le  => cg.builder.build_float_compare(inkwell::FloatPredicate::OLE, lv, rv, "fle").unwrap().into(),
                    BinOp::Ge  => cg.builder.build_float_compare(inkwell::FloatPredicate::OGE, lv, rv, "fge").unwrap().into(),
                    BinOp::Eq  => cg.builder.build_float_compare(inkwell::FloatPredicate::OEQ, lv, rv, "feq").unwrap().into(),
                    BinOp::Ne  => cg.builder.build_float_compare(inkwell::FloatPredicate::ONE, lv, rv, "fne").unwrap().into(),
                },
                _ => unreachable!("type mismatch should have been caught by typecheck"),
            }
        }

        TExpr::Call(name, args, _) => {
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
        }
    }
}
