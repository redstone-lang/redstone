#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    I8, I16, I32, I64, I128, Isize,
    U8, U16, U32, U64, U128, Usize,
    F32, F64,
    Bool,
    Char,
    Unit,
}

impl Type {
    pub fn is_signed(&self) -> bool {
        matches!(self, Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128 | Type::Isize)
    }
}
