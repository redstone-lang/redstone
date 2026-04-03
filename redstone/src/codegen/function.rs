use inkwell::AddressSpace;
use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use crate::ast::Type;
use crate::typecheck::TFunction;
use super::{Codegen, Vars};
use super::stmt::compile_stmt;
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

    for stmt in &func_def.body {
        compile_stmt(cg, stmt, func, &mut vars);
    }

    if func.get_last_basic_block().unwrap().get_terminator().is_none() {
        match &func_def.ret {
            Type::Unit => { cg.builder.build_return(None).unwrap(); }
            ret_ty => {
                let zero = llvm_type(cg.ctx, ret_ty).into_int_type().const_int(0, false);
                cg.builder.build_return(Some(&zero)).unwrap();
            }
        }
    }
}
