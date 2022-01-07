use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeGraph, AttributeName};
use crate::common::WeaponType;

pub struct GladiatorsFinaleEffect {
    weapon_type: WeaponType,
}

impl GladiatorsFinaleEffect {
    pub fn new(character: &Character) -> GladiatorsFinaleEffect {
        GladiatorsFinaleEffect {
            weapon_type: character.common_data.static_data.weapon_type
        }
    }
}

impl ArtifactEffect for GladiatorsFinaleEffect {
    fn effect2(&self, attribute: &mut AttributeGraph) {
        attribute.add_edge(
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            Box::new(|n| (String::from("角斗士的终幕礼2"), n.value() * 0.18))
        );
    }

    fn effect4(&self, attribute: &mut AttributeGraph) {
        match self.weapon_type {
            WeaponType::Sword | WeaponType::Claymore | WeaponType::Polearm => {
                attribute.add_value(AttributeName::BonusNormalAttack, "角斗士的终幕礼4", 0.35);
            },
            _ => (),
        };
    }
}