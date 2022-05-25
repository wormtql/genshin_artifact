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
use mona::ast::expression::expression::{ASTIdentifier, ASTNumber};
use mona::ast::program::ASTProgram;
use mona::ast::traverse::{ASTTraverse, ASTTraverser};
use mona::compiler::simple_compiler::MonaCompilerASTToCode;
use mona::parser::pest::parse_to_cst;
use mona::parser::to_ast::ToAST;
use mona::vm::env::MonaEnv;

fn print_tree<T: RuleType>(pair: &Pair<T>, depth: usize) {
    println!("{}{:?}: {}", " ".repeat(depth), pair.as_rule(), pair.as_str());
    for record in pair.clone().into_inner() {
        print_tree(&record, depth + 4);
    }
}

fn init_damage_context(vm: &mut MonaEnv) {

}

fn main() {
    // let unparsed_file = fs::read_to_string("source/test.mona").expect("cannot read file");
    let unparsed_file = r#"
prop x = KamisatoAyaka.recharge
dmg d = KamisatoAyaka.Q1({ after_dash: true })
result = max(d.normal.e, 10000)
print(result)
    "#.trim();

    let to_ast = ToAST {
        input: &unparsed_file
    };

    let object = parse_to_cst(&unparsed_file);
    // println!("{:?}", object);

    // print_tree(&object, 0);

    let ast = to_ast.convert_program(object).unwrap();

    // for item in ast.borrow().statements.iter() {
    //     println!("{}", item.borrow().common.span.as_str());
    // }

    let compiler = MonaCompilerASTToCode::new(ast);
    let code = compiler.compile().unwrap();
    // println!("asdasdas {:?}", code.prop_configs.len());
    println!("{}", code);

    // println!("{}", code);
    let character: Character<SimpleAttributeGraph2> = Character::new(
        CharacterName::KamisatoAyaka,
        90,
        false,
        0,
        8, 8, 8,
        &CharacterConfig::NoConfig
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
    let context = DamageContext {
        character_common_data: &character.common_data,
        attribute: &attribute,
        enemy: &enemy
    };

    let mut vm = MonaEnv::new(code);
    vm.add_damage_context(context);
    vm.init_damage();
    vm.init_prop();
    vm.execute().unwrap();
    println!("{:?}", vm.namespace.map.keys().collect::<Vec<_>>());
}
