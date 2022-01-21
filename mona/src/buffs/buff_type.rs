use serde::{Serialize, Deserialize};
use crate::attribute::Attribute;
use crate::buffs::Buff;
use crate::buffs::buffs::get_buff;

#[derive(Serialize, Deserialize)]
pub enum BuffType {
    ATKPercentage(f64)
}

impl<T: Attribute> Into<Box<dyn Buff<T>>> for &BuffType {
    fn into(self) -> Box<dyn Buff<T>> {
        get_buff(self)
    }
}
