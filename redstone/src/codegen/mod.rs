mod expr;
mod stmt;
mod function;

use std::collections::HashMap;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::PointerValue;
use crate::ast::Function;

pub struct Codegen<'ctx> {
    pub(crate) ctx: &'ctx Context,
    pub(crate) module: Module<'ctx>,
    pub(crate) builder: Builder<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        Self {
            module: ctx.create_module("redstone"),
            builder: ctx.create_builder(),
            ctx,
        }
    }

    pub fn compile(&mut self, fns: &[Function]) {
        function::declare_printf(self);
        for f in fns {
            function::compile_fn(self, f);
        }
    }

    pub fn emit_object(&self, path: &str) {
        use inkwell::targets::{
            CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
        };
        use inkwell::OptimizationLevel;

        Target::initialize_native(&InitializationConfig::default()).unwrap();
        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple).unwrap();
        let machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                OptimizationLevel::Default,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .unwrap();
        machine
            .write_to_file(&self.module, FileType::Object, path.as_ref())
            .unwrap();
    }
}

pub(crate) type Vars<'ctx> = HashMap<String, PointerValue<'ctx>>;
