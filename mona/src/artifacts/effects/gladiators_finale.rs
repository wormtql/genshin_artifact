use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;

pub struct GladiatorsFinaleEffect {
    weapon_type: WeaponType,
}

impl GladiatorsFinaleEffect {
    pub fn new<T: Attribute>(character: &Character<T>) -> GladiatorsFinaleEffect {
        GladiatorsFinaleEffect {
            weapon_type: character.common_data.static_data.weapon_type
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for GladiatorsFinaleEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("角斗士的终幕礼2", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        match self.weapon_type {
            WeaponType::Sword | WeaponType::Claymore | WeaponType::Polearm => {
                attribute.set_value_by(AttributeName::BonusNormalAttack, "角斗士的终幕礼4", 0.35);
            },
            _ => (),
        };
    }
}