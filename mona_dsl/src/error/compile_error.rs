use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::common::Span;


#[derive(Copy, Clone)]
pub enum CompileErrorType {
    DivByZero,
    TypeError,
    InternalFatal,
    CharacterNameNotFound,
    CharacterSkillNotFound,
    SkillConfigNotConstant,
    SkillConfigNotExist,
    DamageNameDuplicate,
    PropNameNotFound,
    PropNameDup,
    ElementNotFound,
}

pub struct CompileError {
    pub span: Span,
    pub desc: String,
    pub t: CompileErrorType,
}

impl Debug for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "at {}:{} : {}", self.span.start_row, self.span.start_col, self.desc)
    }
}

impl Error for CompileError {

}

impl CompileError {
    pub fn new_line_col(row: usize, col: usize, desc: &str, t: CompileErrorType) -> CompileError {
        CompileError {
            span: Span {
                start_row: row,
                start_col: col,
                end_row: row,
                end_col: col
            },
            desc: String::from(desc),
            t
        }
    }

    pub fn new_no_span(t: CompileErrorType, desc: &str) -> CompileError {
        CompileError {
            span: Span {
                start_row: 0,
                start_col: 0,
                end_col: 0,
                end_row: 0
            },
            desc: String::from(desc),
            t
        }
    }
}

