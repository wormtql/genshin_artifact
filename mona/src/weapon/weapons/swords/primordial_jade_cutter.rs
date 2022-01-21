use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const PRIMORDIAL_JADE_CUTTER_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::CriticalRate96,
    weapon_base: WeaponBaseATKFamily::ATK542,
    star: 5
};

pub struct PrimordialJadeCutterEffect {}

impl PrimordialJadeCutterEffect {
    pub fn new() -> PrimordialJadeCutterEffect {
        PrimordialJadeCutterEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for PrimordialJadeCutterEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine;

        let hp_bonus = refine as f64 * 0.05 + 0.15;
        attribute.add_hp_percentage("磐岩结绿被动", hp_bonus);

        let atk_bonus = refine as f64 * 0.003 + 0.009;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ATKFixed,
            Box::new(move |x, _| x * atk_bonus),
            Box::new(move |grad, _x1, _x2| (grad * atk_bonus, 0.0)),
            "磐岩结绿被动"
        );
    }
}