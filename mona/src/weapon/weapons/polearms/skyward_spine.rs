use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct SkywardSpineEffect;

impl SkywardSpineEffect {
    pub fn new() -> SkywardSpineEffect {
        SkywardSpineEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardSpineEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "天空之脊被动", data.refine as f64 * 0.02 + 0.06);
        attribute.set_value_by(AttributeName::SpeedNormalAttack, "天空之脊被动", 0.12);
    }
}

pub struct SkywardSpine;

impl WeaponTrait for SkywardSpine {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkywardSpine,
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: WeaponSubStatFamily::Recharge80,
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("斫断黑翼的利齿：暴击率提升8%/10%/12%/14%/16%， 普通攻击速度提升12%, 此外，普通攻击与重击命中敌人时，有50%概率触发真空刃，在小范围内造成额外40%/55%/70%/85%/100%攻击力的伤害。该效果每2秒至多触发一次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "天空之脊"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardSpineEffect::new()))
    }
}
