use std::collections::HashMap;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, IntValue, PointerValue};
use inkwell::AddressSpace;
use crate::parser::{BinOp, Expr, FnDef, Stmt};

pub struct Codegen<'ctx> {
    ctx: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Self {
        Self {
            module: ctx.create_module("redstone"),
            builder: ctx.create_builder(),
            ctx,
        }
    }

    pub fn compile(&mut self, fns: &[FnDef]) {
        self.declare_printf();
        for f in fns {
            self.compile_fn(f);
        }
    }

    fn declare_printf(&self) {
        let i8_ptr = self.ctx.ptr_type(AddressSpace::default());
        let i32_t = self.ctx.i32_type();
        let printf_t = i32_t.fn_type(&[i8_ptr.into()], true);
        self.module.add_function("printf", printf_t, None);
    }

    fn compile_fn(&mut self, fn_def: &FnDef) {
        let i64_t = self.ctx.i64_type();
        let param_types: Vec<_> = fn_def.params.iter().map(|_| i64_t.into()).collect();
        let fn_type = i64_t.fn_type(&param_types, false);
        let func = self.module.add_function(&fn_def.name, fn_type, None);
        let entry = self.ctx.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);

        let mut vars: HashMap<String, PointerValue> = HashMap::new();

        // Bind parameters to alloca slots
        for (i, param) in fn_def.params.iter().enumerate() {
            let slot = self.builder.build_alloca(i64_t, param).unwrap();
            self.builder.build_store(slot, func.get_nth_param(i as u32).unwrap()).unwrap();
            vars.insert(param.clone(), slot);
        }

        for stmt in &fn_def.body {
            self.compile_stmt(stmt, func, &mut vars);
        }

        // Implicit return 0 if no explicit return
        if func.get_last_basic_block().unwrap().get_terminator().is_none() {
            self.builder.build_return(Some(&i64_t.const_int(0, false))).unwrap();
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt, func: FunctionValue<'ctx>, vars: &mut HashMap<String, PointerValue<'ctx>>) {
        let i64_t = self.ctx.i64_type();
        match stmt {
            Stmt::Let(name, expr) => {
                let val = self.compile_expr(expr, func, vars);
                let slot = self.builder.build_alloca(i64_t, name).unwrap();
                self.builder.build_store(slot, val).unwrap();
                vars.insert(name.clone(), slot);
            }
            Stmt::Return(expr) => {
                let val = self.compile_expr(expr, func, vars);
                self.builder.build_return(Some(&val)).unwrap();
            }
            Stmt::Print(expr) => {
                let val = self.compile_expr(expr, func, vars);
                let fmt = self.builder.build_global_string_ptr("%lld\n", "fmt").unwrap();
                let printf = self.module.get_function("printf").unwrap();
                self.builder.build_call(printf, &[fmt.as_pointer_value().into(), val.into()], "").unwrap();
            }
            Stmt::Expr(expr) => {
                self.compile_expr(expr, func, vars);
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr, func: FunctionValue<'ctx>, vars: &mut HashMap<String, PointerValue<'ctx>>) -> IntValue<'ctx> {
        let i64_t = self.ctx.i64_type();
        match expr {
            Expr::Int(n) => i64_t.const_int(*n as u64, false),
            Expr::Var(name) => {
                let slot = vars[name];
                self.builder.build_load(i64_t, slot, name).unwrap().into_int_value()
            }
            Expr::BinOp(lhs, op, rhs) => {
                let l = self.compile_expr(lhs, func, vars);
                let r = self.compile_expr(rhs, func, vars);
                match op {
                    BinOp::Add => self.builder.build_int_add(l, r, "add").unwrap(),
                    BinOp::Sub => self.builder.build_int_sub(l, r, "sub").unwrap(),
                    BinOp::Mul => self.builder.build_int_mul(l, r, "mul").unwrap(),
                    BinOp::Div => self.builder.build_int_signed_div(l, r, "div").unwrap(),
                }
            }
            Expr::Call(name, args) => {
                let callee = self.module.get_function(name).expect("unknown function");
                let compiled_args: Vec<_> = args.iter()
                    .map(|a| self.compile_expr(a, func, vars).into())
                    .collect();
                self.builder.build_call(callee, &compiled_args, "call").unwrap()
                    .try_as_basic_value().unwrap_basic().into_int_value()
            }
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
        let machine = target.create_target_machine(
            &triple,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::PIC,
            CodeModel::Default,
        ).unwrap();
        machine.write_to_file(&self.module, FileType::Object, path.as_ref()).unwrap();
    }
}
