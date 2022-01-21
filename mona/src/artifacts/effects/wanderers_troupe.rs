use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
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

impl<T: Attribute> ArtifactEffect<T> for WanderersTroupeEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "流浪大地的乐团2", 80.0);
    }

    fn effect4(&self, attribute: &mut T) {
        match self.weapon_type {
            WeaponType::Catalyst | WeaponType::Bow => {
                attribute.set_value_by(AttributeName::BonusChargedAttack, "流浪大地的乐团4", 0.35);
            },
            _ => (),
        }
    }
}