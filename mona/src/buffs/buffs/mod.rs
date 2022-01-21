pub use atk_percentage::BuffAtkPercentage;

use crate::attribute::Attribute;
use crate::buffs::{Buff, BuffType};

pub mod atk_percentage;

pub fn get_buff<T: Attribute>(t: &BuffType) -> Box<dyn Buff<T>> {
    match *t {
        BuffType::ATKPercentage(value) => Box::new(BuffAtkPercentage::new(value)),
    }
}