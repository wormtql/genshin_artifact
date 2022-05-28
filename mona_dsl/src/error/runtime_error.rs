use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use strum_macros::Display;

#[derive(Display)]
pub enum RuntimeErrorEnum {
    TypeError,
    CodeError,
    NameNotFound,
    NotSupported,
    DamageNotFound,
    CharacterContextNotFound,
    DivByZero,
    ParamError,
}

pub struct RuntimeError {
    t: RuntimeErrorEnum,
    desc: String,
    // todo
    line: usize,
}

impl RuntimeError {
    pub fn new(t: RuntimeErrorEnum, desc: &str) -> RuntimeError {
        RuntimeError {
            t,
            desc: String::from(desc),
            line: 0
        }
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime Error: {}, {}", self.t, self.desc)
    }
}

impl Error for RuntimeError {

}