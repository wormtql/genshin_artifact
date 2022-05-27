use lazy_static::lazy_static;
use mona::attribute::{Attribute, AttributeName, SimpleAttributeGraph2, AttributeCommon};
use mona::character::CharacterName;
use crate::error::runtime_error::RuntimeError;

lazy_static! {
    static ref LIST: Vec<&'static str> = vec![
        "recharge",
        "atk",
        "def",
        "hp",
        "em",
        "crit0",
        "cd0",
        "heal"
    ];
}

pub fn is_valid_prop_name(name: &str) -> bool {
    LIST.iter().position(|x| *x == name).is_some()
}

pub fn get_prop_value(attribute: &SimpleAttributeGraph2, name: &str) -> Result<f64, RuntimeError> {
    let v = match name {
        "recharge" => attribute.get_value(AttributeName::Recharge),
        "atk" => attribute.get_atk(),
        "def" => attribute.get_def(),
        "hp" => attribute.get_hp(),
        "em" => attribute.get_value(AttributeName::ElementalMastery),
        "crit0" => attribute.get_value(AttributeName::CriticalBase),
        "cd0" => attribute.get_value(AttributeName::CriticalDamageBase),
        "heal" => attribute.get_value(AttributeName::HealingBonus),
        _ => panic!("prop name not exist")
    };

    Ok(v)
}

#[derive(Debug)]
pub struct MonaObjectPropConfig {
    pub character_name: CharacterName,
    pub var_name: String,
    pub prop_name: String,
}
