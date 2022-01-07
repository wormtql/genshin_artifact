use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};
use crate::common::WeaponType;

pub struct WanderersTroupeEffect {
    pub weapon_type: WeaponType
}

impl WanderersTroupeEffect {
    pub fn new(weapon_type: WeaponType) -> WanderersTroupeEffect {
        WanderersTroupeEffect {
            weapon_type,
        }
    }
}

impl ArtifactEffect for WanderersTroupeEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_value(AttributeName::ElementalMastery, "流浪大地的乐团2", 80.0);
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        match self.weapon_type {
            WeaponType::Catalyst | WeaponType::Bow => {
                attribute.add_value(AttributeName::BonusChargedAttack, "流浪大地的乐团4", 0.35);
            },
            _ => (),
        }
    }
}