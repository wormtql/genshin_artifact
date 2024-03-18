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
#![feature(decl_macro)]
#[macro_use] extern crate pest_derive;

pub mod character;
pub mod attribute;
pub mod artifacts;
pub mod weapon;
pub mod common;
pub mod target_functions;
pub mod buffs;
pub mod enemies;
pub mod utils;
pub mod damage;
pub mod team;
pub mod potential_function;
pub mod upgrade_predicate;
// pub mod ast;
// pub mod compiler;
// pub mod parser;
// pub mod error;
// pub mod code;
// pub mod object;
// pub mod vm;
// pub mod builtin;

// #[cfg(not(target_family = "wasm"))]
// pub mod gen_meta;

// calculator
// pub use applications::calculator::interface_calculator::{CalculatorInterface};
// // optimize single
// pub use applications::optimize_artifacts::OptimizeSingleWasm;
// // optimize team
// pub use applications::team_optimize::interface_wasm::TeamOptimizationWasm;
// // get attribute
// pub use applications::common_interface::CommonInterface;
//
// pub use applications::bonus_per_stat::interface_wasm::BonusPerStat;

