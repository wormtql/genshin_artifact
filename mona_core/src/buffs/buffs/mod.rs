use crate::attribute::Attribute;
use crate::buffs::{Buff, BuffConfig};

pub use common::*;
pub use resonance::*;
pub use character::*;
pub use weapon::*;
pub use artifact::*;

use crate::buffs::buff_name::BuffName;

pub mod common;
pub mod character;
pub mod weapon;
pub mod resonance;
pub mod artifact;

pub fn get_buff<T: Attribute>(name: &BuffName, b: &BuffConfig) -> Box<dyn Buff<T>> {
    name.create(b)
}
