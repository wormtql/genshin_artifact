use std::cell::RefCell;
use std::fmt::Display;
use std::fs;
use std::rc::Rc;
use mona::attribute::{AttributeUtils, SimpleAttributeGraph2};
use mona::character::{Character, CharacterConfig, CharacterName};
use mona::damage::DamageContext;
use mona::enemies::Enemy;
use mona::weapon::{Weapon, WeaponConfig, WeaponName};
use pest::iterators::Pair;
use pest::RuleType;
use mona_dsl::ast::expression::expression::{ASTIdentifier, ASTNumber};
use mona_dsl::ast::program::ASTProgram;
use mona_dsl::ast::traverse::{ASTTraverse, ASTTraverser};
use mona_dsl::common::UnsafeDamageContext;
use mona_dsl::compiler::simple_compiler::MonaCompilerASTToCode;
use mona_dsl::parser::pest::parse_to_cst;
use mona_dsl::parser::to_ast::ToAST;
use mona_dsl::vm::env::MonaEnv;
use mona_dsl::vm::stream::{OutputStream, StringOutputStream};

fn print_tree<T: RuleType>(pair: &Pair<T>, depth: usize) {
    println!("{}{:?}: {}", " ".repeat(depth), pair.as_rule(), pair.as_str());
    for record in pair.clone().into_inner() {
        print_tree(&record, depth + 4);
    }
}

fn init_damage_context(vm: &mut MonaEnv) {

}

fn main() {
    let unparsed_file = fs::read_to_string("source/test.mona").expect("cannot read file");

    let to_ast = ToAST {
        input: &unparsed_file
    };

    let object = parse_to_cst(&unparsed_file);
    // println!("{:?}", object);

    // print_tree(&object, 0);

    let ast = to_ast.convert_program(object).unwrap();

    for item in ast.borrow().statements.iter() {
        println!("{}", item.borrow().common.span.as_str());
    }

    let compiler = MonaCompilerASTToCode::new(ast);
    let code = compiler.compile().unwrap();
    // println!("asdasdas {:?}", code.prop_configs.len());
    println!("{}", code);

    // println!("{}", code);
    let character: Character<SimpleAttributeGraph2> = Character::new(
        CharacterName::Yelan,
        90,
        false,
        0,
        8, 8, 8,
        &CharacterConfig::NoConfig,
        // &CharacterConfig::Neuvillette {current_hp: 100},
    );
    let weapon = Weapon::new(
        WeaponName::MistsplitterReforged,
        90,
        false,
        1,
        &WeaponConfig::MistsplitterReforged { emblem_level: 2 },
        &character
    );
    let attribute = AttributeUtils::create_attribute_from_c_w_bs(&character, &weapon, &Vec::new());
    let enemy = Enemy::default();
    let context = UnsafeDamageContext {
        character_common_data: &character.common_data,
        attribute: &attribute,
        enemy: &enemy
    };

    let os = Box::new(StringOutputStream::new());

    let mut vm = MonaEnv::new(code);
    vm.set_ostream(os);
    vm.add_damage_context(context);
    vm.init_damage();
    vm.init_prop();
    vm.execute().unwrap();

    // let output = unsafe { (*osp).get_string() };
    let os = vm.ostream.as_any().downcast_ref::<StringOutputStream>().unwrap();
    println!("{}", os.get_string());
    println!("{:?}", vm.namespace.map.keys().collect::<Vec<_>>());
}
