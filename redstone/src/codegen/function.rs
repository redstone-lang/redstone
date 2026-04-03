use inkwell::AddressSpace;
use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use crate::ast::Type;
use crate::typecheck::{TFunction, TStmt};
use super::{Codegen, Vars};
use super::stmt::compile_stmt;
use super::expr::compile_expr;
use super::types::llvm_type;

pub fn declare_printf(cg: &Codegen) {
    let i8_ptr = cg.ctx.ptr_type(AddressSpace::default());
    let printf_t = cg.ctx.i32_type().fn_type(&[i8_ptr.into()], true);
    cg.module.add_function("printf", printf_t, None);
}

pub fn compile_fn(cg: &mut Codegen, func_def: &TFunction) {
    let param_types: Vec<BasicMetadataTypeEnum> = func_def
        .params
        .iter()
        .map(|(_, ty)| llvm_type(cg.ctx, ty).into())
        .collect();

    let func = match &func_def.ret {
        Type::Unit => {
            let void_t = cg.ctx.void_type().fn_type(&param_types, false);
            cg.module.add_function(&func_def.name, void_t, None)
        }
        ret_ty => {
            let fn_t = llvm_type(cg.ctx, ret_ty).fn_type(&param_types, false);
            cg.module.add_function(&func_def.name, fn_t, None)
        }
    };

    let entry = cg.ctx.append_basic_block(func, "entry");
    cg.builder.position_at_end(entry);

    let mut vars: Vars = Vars::new();
    for (i, (name, ty)) in func_def.params.iter().enumerate() {
        let llty = llvm_type(cg.ctx, ty);
        let slot = cg.builder.build_alloca(llty, name).unwrap();
        cg.builder.build_store(slot, func.get_nth_param(i as u32).unwrap()).unwrap();
        vars.insert(name.clone(), (slot, llty));
    }

    // If last stmt is a bare Expr and return type is non-Unit, treat it as implicit return
    let (main_stmts, implicit_ret) = match (func_def.ret != Type::Unit, func_def.body.last()) {
        (true, Some(TStmt::Expr(_))) => {
            let split = func_def.body.len() - 1;
            (&func_def.body[..split], Some(&func_def.body[split]))
        }
        _ => (&func_def.body[..], None),
    };

    for stmt in main_stmts {
        compile_stmt(cg, stmt, func, &mut vars);
    }

    if let Some(TStmt::Expr(expr)) = implicit_ret {
        if func.get_last_basic_block().unwrap().get_terminator().is_none() {
            let val = compile_expr(cg, expr, func, &mut vars);
            cg.builder.build_return(Some(&val)).unwrap();
        }
    }

    if func.get_last_basic_block().unwrap().get_terminator().is_none() {
        match &func_def.ret {
            Type::Unit => { cg.builder.build_return(None).unwrap(); }
            _ => {
                let zero = llvm_type(cg.ctx, &func_def.ret).into_int_type().const_int(0, false);
                cg.builder.build_return(Some(&zero)).unwrap();
            }
        }
    }
}
