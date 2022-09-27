use serde::{Serialize, Deserialize};
use rand::{Rng, thread_rng};
use num_derive::FromPrimitive;
use strum_macros::{EnumCount as EnumCountMacro};

use crate::artifacts::ArtifactSlotName;
use crate::attribute::{AttributeName, Attribute, AttributeCommon};
use super::element::Element;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[derive(Serialize, Deserialize, FromPrimitive, EnumCountMacro)]
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
    pub fn apply<T: Attribute>(&self, attribute: &mut T, key: &str, value: f64) {
        match *self {
            StatName::ATKFixed => {
                attribute.set_value_by(AttributeName::ATKFixed, key, value)
            },
            StatName::ATKPercentage => {
                let temp = String::from(key);
                attribute.add_atk_percentage(key, value);
            },
            StatName::HealingBonus => {
                attribute.set_value_by(AttributeName::HealingBonus, key, value);
            },
            StatName::HPFixed => {
                attribute.set_value_by(AttributeName::HPFixed, key, value);
            },
            StatName::HPPercentage => {
                attribute.add_hp_percentage(key, value);
            },
            StatName::DEFFixed => {
                attribute.set_value_by(AttributeName::DEFFixed, key, value);
            },
            StatName::DEFPercentage => {
                attribute.add_def_percentage(key, value);
            },
            StatName::CriticalRate => {
                attribute.set_value_by(AttributeName::CriticalBase, key, value);
            },
            StatName::CriticalDamage => {
                attribute.set_value_by(AttributeName::CriticalDamageBase, key, value);
            },
            StatName::ElementalMastery => {
                attribute.set_value_by(AttributeName::ElementalMastery, key, value);
            },
            StatName::Recharge => {
                attribute.set_value_by(AttributeName::Recharge, key, value);
            },
            StatName::ElectroBonus => {
                attribute.set_value_by(AttributeName::BonusElectro, key, value);
            },
            StatName::PyroBonus => {
                attribute.set_value_by(AttributeName::BonusPyro, key, value);
            },
            StatName::HydroBonus => {
                attribute.set_value_by(AttributeName::BonusHydro, key, value);
            },
            StatName::CryoBonus => {
                attribute.set_value_by(AttributeName::BonusCryo, key, value);
            },
            StatName::AnemoBonus => {
                attribute.set_value_by(AttributeName::BonusAnemo, key, value);
            },
            StatName::GeoBonus => {
                attribute.set_value_by(AttributeName::BonusGeo, key, value);
            },
            StatName::DendroBonus => {
                attribute.set_value_by(AttributeName::BonusDendro, key, value);
            },
            StatName::PhysicalBonus => {
                attribute.set_value_by(AttributeName::BonusPhysical, key, value);
            }
        }
    }

    pub fn stat_name_bonus_from_element(element: Element) -> StatName {
        match element {
            Element::Electro => StatName::ElectroBonus,
            Element::Pyro => StatName::PyroBonus,
            Element::Anemo => StatName::AnemoBonus,
            Element::Hydro => StatName::HydroBonus,
            Element::Cryo => StatName::CryoBonus,
            Element::Geo => StatName::GeoBonus,
            Element::Dendro => StatName::DendroBonus,
            Element::Physical => StatName::PhysicalBonus,
        }
    }

    pub fn get_slot_main_stats() -> [Vec<StatName>; 5] {
        use StatName::*;
        [
            vec![HPFixed],
            vec![ATKFixed],
            vec![ATKPercentage, DEFPercentage, HPPercentage, ElementalMastery, Recharge],
            vec![ATKPercentage, DEFPercentage, HPPercentage, ElementalMastery, DendroBonus, PyroBonus, ElectroBonus, HydroBonus, CryoBonus, AnemoBonus, GeoBonus, PhysicalBonus],
            vec![ATKPercentage, DEFPercentage, HPPercentage, ElementalMastery, CriticalRate, CriticalDamage, HealingBonus]
        ]
    }

    pub fn random_artifact_main_stat(slot: ArtifactSlotName) -> StatName {
        use StatName::*;
        let v = match slot {
            ArtifactSlotName::Flower => return HPFixed,
            ArtifactSlotName::Feather => return ATKFixed,
            ArtifactSlotName::Sand => vec![ATKPercentage, DEFPercentage, HPPercentage, ElementalMastery, Recharge],
            ArtifactSlotName::Goblet => vec![ATKPercentage, DEFPercentage, HPPercentage, ElementalMastery, DendroBonus, PyroBonus, ElectroBonus, HydroBonus, CryoBonus, AnemoBonus, GeoBonus, PhysicalBonus],
            ArtifactSlotName::Head => vec![ATKPercentage, DEFPercentage, HPPercentage, ElementalMastery, CriticalRate, CriticalDamage, HealingBonus]
        };

        let len = v.len();
        let index: usize = thread_rng().gen::<usize>() % len;

        v[index]
    }

    #[inline]
    pub fn artifact_main_stat_max_value(name: StatName) -> f64 {
        use StatName::*;
        match name {
            HPFixed => 4780.0,
            ATKFixed => 311.0,
            Recharge => 0.518,
            CriticalRate => 0.311,
            HPPercentage | ATKPercentage => 0.466,
            DEFPercentage => 0.583,
            CriticalDamage => 0.622,
            HealingBonus => 0.359,
            ElementalMastery => 187.0,
            PyroBonus | ElectroBonus | HydroBonus | CryoBonus | AnemoBonus | GeoBonus | DendroBonus => 0.466,
            PhysicalBonus => 0.583,
            _ => unreachable!()
        }
    }
}

pub struct StatValueStruct {
    pub atk_fixed: [f64; 4],
    pub atk_percentage: [f64; 4],
    pub hp_fixed: [f64; 4],
    pub hp_percentage: [f64; 4],
    pub def_fixed: [f64; 4],
    pub def_percentage: [f64; 4],
    pub critical_rate: [f64; 4],
    pub critical_damage: [f64; 4],
    pub elemental_mastery: [f64; 4],
    pub recharge: [f64; 4],
    pub healing_bonus: [f64; 4],
    pub elemental_bonus: [f64; 4],
    pub physical_bonus: [f64; 4],
}

pub const SUB_STAT_VALUE_5: StatValueStruct = StatValueStruct {
    atk_fixed: [14.0, 16.0, 18.0, 19.0],
    atk_percentage: [0.041, 0.047, 0.053, 0.058],
    hp_fixed: [209.0, 239.0, 269.0, 299.0],
    hp_percentage: [0.041, 0.047, 0.053, 0.058],
    def_fixed: [16.0, 19.0, 21.0, 23.0],
    def_percentage: [0.051, 0.058, 0.066, 0.073],
    critical_rate: [0.027, 0.031, 0.035, 0.039],
    critical_damage: [0.054, 0.062, 0.07, 0.078],
    elemental_mastery: [16.0, 19.0, 21.0, 23.0],
    recharge: [0.045, 0.052, 0.058, 0.065],

    healing_bonus: [0.031, 0.036, 0.04, 0.045],
    elemental_bonus: [0.041, 0.047, 0.053, 0.058],
    physical_bonus: [0.051, 0.059, 0.066, 0.0725],
};