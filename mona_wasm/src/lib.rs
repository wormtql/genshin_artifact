#![allow(unused_imports)]
#![allow(clippy::approx_constant)]

// pub mod character;
// pub mod attribute;
// pub mod artifacts;
// pub mod weapon;
// pub mod common;
// pub mod target_functions;
// pub mod buffs;
// pub mod enemies;
pub mod applications;
// pub mod utils;
// pub mod damage;
// pub mod team;
// pub mod team_target;
// pub mod potential_function;


// calculator
pub use applications::calculator::interface_calculator::{CalculatorInterface};
// optimize single
pub use applications::optimize_artifacts::OptimizeSingleWasm;
// optimize team
pub use applications::team_optimize::interface_wasm::TeamOptimizationWasm;
// get attribute
pub use applications::common_interface::CommonInterface;

pub use applications::bonus_per_stat::interface_wasm::BonusPerStat;

