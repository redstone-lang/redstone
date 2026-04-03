use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use crate::ast::Type;

pub fn llvm_type<'ctx>(ctx: &'ctx Context, ty: &Type) -> BasicTypeEnum<'ctx> {
    match ty {
        Type::I8  | Type::U8  => ctx.i8_type().into(),
        Type::I16 | Type::U16 => ctx.i16_type().into(),
        Type::I32 | Type::U32 => ctx.i32_type().into(),
        Type::I64 | Type::U64 => ctx.i64_type().into(),
        Type::I128 | Type::U128 => ctx.i128_type().into(),
        Type::Isize | Type::Usize => ctx.i64_type().into(), // pointer-sized on 64-bit
        Type::F32 => ctx.f32_type().into(),
        Type::F64 => ctx.f64_type().into(),
        Type::Bool => ctx.bool_type().into(),
        Type::Char => ctx.i32_type().into(), // Unicode scalar = u32
        Type::Unit => ctx.i8_type().into(),  // unit represented as i8(0), void handled at call site
    }
}
