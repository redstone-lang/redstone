use inkwell::AddressSpace;
use crate::ast::Function;
use super::{Codegen, Vars};
use super::stmt::compile_stmt;

pub fn declare_printf(cg: &Codegen) {
    let i8_ptr = cg.ctx.ptr_type(AddressSpace::default());
    let printf_t = cg.ctx.i32_type().fn_type(&[i8_ptr.into()], true);
    cg.module.add_function("printf", printf_t, None);
}

pub fn compile_fn(cg: &mut Codegen, func_def: &Function) {
    let i64_t = cg.ctx.i64_type();
    let param_types: Vec<_> = func_def.params.iter().map(|_| i64_t.into()).collect();
    let func = cg
        .module
        .add_function(&func_def.name, i64_t.fn_type(&param_types, false), None);
    let entry = cg.ctx.append_basic_block(func, "entry");
    cg.builder.position_at_end(entry);

    let mut vars: Vars = Vars::new();
    for (i, param) in func_def.params.iter().enumerate() {
        let slot = cg.builder.build_alloca(i64_t, param).unwrap();
        cg.builder
            .build_store(slot, func.get_nth_param(i as u32).unwrap())
            .unwrap();
        vars.insert(param.clone(), slot);
    }

    for stmt in &func_def.body {
        compile_stmt(cg, stmt, func, &mut vars);
    }

    if func.get_last_basic_block().unwrap().get_terminator().is_none() {
        cg.builder
            .build_return(Some(&i64_t.const_int(0, false)))
            .unwrap();
    }
}
