use super::weapon_common_data::WeaponCommonData;
use super::weapon_effect::WeaponEffect;
use super::weapons::get_effect;
use crate::common::ChangeAttribute;
use crate::attribute::AttributeGraph;
use crate::character::Character;
use crate::weapon::weapon_name::WeaponName;
use super::weapon_config::WeaponConfig;

pub struct Weapon {
    pub common_data: WeaponCommonData,
    pub effect: Box<dyn WeaponEffect>,
}

impl Weapon {
    pub fn new(
        name: WeaponName,
        level: i32,
        ascend: bool,
        refine: i32,
        config: &WeaponConfig,
        character: &Character,
    ) -> Weapon {
        let common_data = WeaponCommonData::new(name, level, ascend, refine);
        let effect = get_effect(name, config, character);

        Weapon {
            common_data,
            effect,
        }
    }
}

impl ChangeAttribute for Weapon {
    fn change_attribute(&self, attribute: &mut AttributeGraph) {
        self.common_data.change_attribute(attribute);
        self.effect.apply(&self.common_data, attribute);
    }
}
