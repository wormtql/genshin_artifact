#![allow(unused_imports)]
#![allow(clippy::approx_constant)]
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
#![allow(noop_method_call)]

// pub mod character;
// pub mod attribute;
// pub mod artifacts;
// pub mod weapon;
// pub mod common;
// pub mod target_functions;
// pub mod buffs;
// pub mod enemies;
pub mod applications;
pub mod target_function;
// pub mod utils;
// pub mod damage;
// pub mod team;
// pub mod team_target;
// pub mod potential_function;
pub mod utils;


// calculator
pub use applications::calculator::interface_calculator::{CalculatorInterface};
// optimize single
pub use applications::optimize_artifacts::OptimizeSingleWasm;
// optimize team
pub use applications::team_optimize::interface_wasm::TeamOptimizationWasm;
// get attribute
pub use applications::common_interface::CommonInterface;

pub use applications::bonus_per_stat::interface_wasm::BonusPerStat;

pub use applications::dsl::dsl_interface::DSLInterface;
// calc best artifact set and main stats
pub use applications::artifact_best_set::wasm_interface::CalcArtifactBestSet;
