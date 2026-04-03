use inkwell::values::{BasicValueEnum, FunctionValue};
use crate::typecheck::TStmt;
use super::{Codegen, Vars};
use super::expr::compile_expr;

pub fn compile_stmt<'ctx>(
    cg: &mut Codegen<'ctx>,
    stmt: &TStmt,
    func: FunctionValue<'ctx>,
    vars: &mut Vars<'ctx>,
) {
    match stmt {
        TStmt::Let(name, expr) => {
            let val = compile_expr(cg, expr, func, vars);
            let ty = val.get_type();
            let slot = cg.builder.build_alloca(ty, name).unwrap();
            cg.builder.build_store(slot, val).unwrap();
            vars.insert(name.clone(), (slot, ty));
        }
        TStmt::Assign(name, expr) => {
            let val = compile_expr(cg, expr, func, vars);
            let (ptr, _) = vars[name];
            cg.builder.build_store(ptr, val).unwrap();
        }
        TStmt::While(cond, body) => {
            let cond_bb = cg.ctx.append_basic_block(func, "while.cond");
            let body_bb = cg.ctx.append_basic_block(func, "while.body");
            let exit_bb = cg.ctx.append_basic_block(func, "while.exit");

            cg.builder.build_unconditional_branch(cond_bb).unwrap();
            cg.builder.position_at_end(cond_bb);
            let cond_val = compile_expr(cg, cond, func, vars).into_int_value();
            cg.builder.build_conditional_branch(cond_val, body_bb, exit_bb).unwrap();

            cg.builder.position_at_end(body_bb);
            for s in body {
                compile_stmt(cg, s, func, vars);
            }
            cg.builder.build_unconditional_branch(cond_bb).unwrap();

            cg.builder.position_at_end(exit_bb);
        }
        TStmt::Return(expr) => {
            let val = compile_expr(cg, expr, func, vars);
            cg.builder.build_return(Some(&val)).unwrap();
        }
        TStmt::Print(expr) => {
            let val = compile_expr(cg, expr, func, vars);
            let printf = cg.module.get_function("printf").unwrap();
            let fmt_str = print_format_str(&val);
            let fmt_ptr = cg.builder.build_global_string_ptr(fmt_str, "fmt").unwrap();
            let args: Vec<inkwell::values::BasicMetadataValueEnum> = match val {
                BasicValueEnum::FloatValue(f) => {
                    let f64_val = if f.get_type() == cg.ctx.f32_type() {
                        cg.builder.build_float_ext(f, cg.ctx.f64_type(), "fpext").unwrap()
                    } else {
                        f
                    };
                    vec![fmt_ptr.as_pointer_value().into(), f64_val.into()]
                }
                other => vec![fmt_ptr.as_pointer_value().into(), other.into()],
            };
            cg.builder.build_call(printf, &args, "").unwrap();
        }
        TStmt::Expr(expr) => {
            compile_expr(cg, expr, func, vars);
        }
    }
}

fn print_format_str(val: &BasicValueEnum) -> &'static str {
    match val {
        BasicValueEnum::IntValue(iv) => match iv.get_type().get_bit_width() {
            1 | 8 | 16 | 32 => "%d\n",
            _                => "%lld\n",
        },
        BasicValueEnum::FloatValue(_) => "%g\n",
        _ => "%d\n",
    }
}
