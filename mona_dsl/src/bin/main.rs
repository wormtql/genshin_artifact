use std::cell::RefCell;
use std::fmt::Display;
use std::fs;
use std::rc::Rc;
use pest::iterators::Pair;
use pest::RuleType;
use mona_dsl::ast::expression::expression::{ASTIdentifier, ASTNumber};
use mona_dsl::ast::program::ASTProgram;
use mona_dsl::ast::traverse::{ASTTraverse, ASTTraverser};
use mona_dsl::compiler::simple_compiler::MonaCompilerASTToCode;
use mona_dsl::parser::pest::parse_to_cst;
use mona_dsl::parser::to_ast::ToAST;
use mona_dsl::vm::env::MonaEnv;

fn print_tree<T: RuleType>(pair: &Pair<T>, depth: usize) {
    println!("{}{:?}: {}", " ".repeat(depth), pair.as_rule(), pair.as_str());
    for record in pair.clone().into_inner() {
        print_tree(&record, depth + 4);
    }
}

fn main() {
    let unparsed_file = fs::read_to_string("source/test.mona").expect("cannot read file");

    let to_ast = ToAST {
        input: &unparsed_file
    };

    let object = parse_to_cst(&unparsed_file);
    println!("{:?}", object);

    print_tree(&object, 0);

    let ast = to_ast.convert_program(object).unwrap();

    for item in ast.borrow().statements.iter() {
        println!("{}", item.borrow().common.span.as_str());
    }

    let compiler = MonaCompilerASTToCode::new(ast);
    let code = compiler.compile().unwrap();
    println!("{:?}", code.damage_configs);

    println!("{}", code);

    let mut vm = MonaEnv::new(code);
    vm.execute().unwrap();
    println!("{:?}", vm.namespace.map.keys().collect::<Vec<_>>());
}
