pub use simple_damage_builder::SimpleDamageBuilder;
pub use damage_analysis::DamageAnalysis;
pub use complicated_damage_builder::ComplicatedDamageBuilder;
pub use damage_context::DamageContext;

mod simple_damage_builder;
mod complicated_damage_builder;
mod damage_analysis;
mod damage_context;
pub mod reaction;
pub mod damage_type;
pub mod damage_builder;
pub mod damage_result;
pub mod transformative_damage;
pub mod level_coefficient;
