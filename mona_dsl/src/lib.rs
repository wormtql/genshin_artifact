#![feature(decl_macro)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(clippy::approx_constant)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_must_use)]
#[macro_use] extern crate pest_derive;


use crate::compiler::compiler::CodeObject;
use crate::compiler::simple_compiler::MonaCompilerASTToCode;
use crate::error::CompileError;
use crate::parser::pest::parse_to_cst;
use crate::parser::to_ast::ToAST;

pub mod ast;
pub mod compiler;
pub mod parser;
pub mod error;
pub mod common;
pub mod code;
pub mod object;
pub mod vm;
pub mod builtin;

pub fn compile_source_to_code_object(source: &str) -> Result<CodeObject, CompileError> {
    let to_ast = ToAST {
        input: &source
    };

    let object = parse_to_cst(&source);

    let ast = to_ast.convert_program(object).unwrap();

    let compiler = MonaCompilerASTToCode::new(ast);
    compiler.compile()
}
