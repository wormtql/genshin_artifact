use crate::compiler::compiler::CodeObject;
use crate::error::CodeError;
use crate::error::runtime_error::RuntimeError;
use crate::vm::env::MonaEnv;

pub trait ByteCode {
    fn to_string(&self, ctx: &CodeObject) -> String;

    fn get_bits(&self) -> u64;

    fn from_bits(bits: u64) -> Result<Self, CodeError> where Self: Sized;

    fn execute(&self, env: &mut MonaEnv) -> Result<(), RuntimeError>;
}

pub enum ByteCodeOp {
    Noop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    SaveName,
    LoadNumber,
    LoadName,
    Call,
    LoadBool,
    Access,
    LoadString,
}