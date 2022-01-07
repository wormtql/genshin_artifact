use serde::{Serialize, Deserialize};

use crate::attribute::{AttributeName, AttributeGraph};

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub enum StatName {
    ATKFixed,
    ATKPercentage,
    HealingBonus,
    HPFixed,
    HPPercentage,
    DEFFixed,
    DEFPercentage,
    CriticalRate,
    CriticalDamage,
    ElementalMastery,
    Recharge,
    ElectroBonus,
    PyroBonus,
    HydroBonus,
    CryoBonus,
    AnemoBonus,
    GeoBonus,
    DendroBonus,
    PhysicalBonus,
}

impl StatName {
    pub fn apply(&self, attribute: &mut AttributeGraph, key: &str, value: f64) {
        match *self {
            StatName::ATKFixed => {
                attribute.add_value(AttributeName::ATKFixed, key, value)
            },
            StatName::ATKPercentage => {
                let temp = String::from(key);
                attribute.add_edge(
                    AttributeName::ATKBase,
                    AttributeName::ATKPercentage,
                    Box::new(move |n| (temp.clone(), n.value() * value))
                )
            },
            StatName::HealingBonus => {
                attribute.add_value(AttributeName::HealingBonus, key, value);
            },
            StatName::HPFixed => {
                attribute.add_value(AttributeName::HPFixed, key, value);
            },
            StatName::HPPercentage => {
                let temp = String::from(key);
                attribute.add_edge(
                    AttributeName::HPBase,
                    AttributeName::HPPercentage,
                    Box::new(move |n| (temp.clone(), n.value() * value)),
                );
            },
            StatName::DEFFixed => {
                attribute.add_value(AttributeName::DEFFixed, key, value);
            },
            StatName::DEFPercentage => {
                let temp = String::from(key);
                attribute.add_edge(
                    AttributeName::DEFBase,
                    AttributeName::DEFPercentage,
                    Box::new(move |n| (temp.clone(), n.value() * value))
                );
            },
            StatName::CriticalRate => {
                attribute.add_value(AttributeName::CriticalBase, key, value);
            },
            StatName::CriticalDamage => {
                attribute.add_value(AttributeName::CriticalDamageBase, key, value);
            },
            StatName::ElementalMastery => {
                attribute.add_value(AttributeName::ElementalMastery, key, value);
            },
            StatName::Recharge => {
                attribute.add_value(AttributeName::Recharge, key, value);
            },
            StatName::ElectroBonus => {
                attribute.add_value(AttributeName::BonusElectro, key, value);
            },
            StatName::PyroBonus => {
                attribute.add_value(AttributeName::BonusPyro, key, value);
            },
            StatName::HydroBonus => {
                attribute.add_value(AttributeName::BonusHydro, key, value);
            },
            StatName::CryoBonus => {
                attribute.add_value(AttributeName::BonusCryo, key, value);
            },
            StatName::AnemoBonus => {
                attribute.add_value(AttributeName::BonusAnemo, key, value);
            },
            StatName::GeoBonus => {
                attribute.add_value(AttributeName::BonusGeo, key, value);
            },
            StatName::DendroBonus => {
                attribute.add_value(AttributeName::BonusDendro, key, value);
            },
            StatName::PhysicalBonus => {
                attribute.add_value(AttributeName::BonusPhysical, key, value);
            }
        }
    }
}