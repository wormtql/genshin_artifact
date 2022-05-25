use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use strum_macros::Display;

#[derive(Display)]
pub enum CodeErrorType {
    ParseError
}

pub struct CodeError {
    pub t: CodeErrorType,
    pub desc: String,
}

impl Debug for CodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for CodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Code Error: {}: {}", self.t, self.desc)
    }
}

impl Error for CodeError {

}