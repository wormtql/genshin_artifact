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

pub struct SkywardBladeEffect;

impl SkywardBladeEffect {
    pub fn new() -> SkywardBladeEffect {
        SkywardBladeEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardBladeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let crit = data.refine as f64 * 0.01 + 0.03;
        attribute.set_value_by(AttributeName::CriticalBase, "天空之刃被动", crit);
    }
}

pub struct SkywardBlade;

impl WeaponTrait for SkywardBlade {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkywardBlade,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge120),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("穿刺高天的利齿：暴击率提升4%/5%/6%/7%/8%；施放元素爆发时，获得破空之势：移动速度提升10%，攻击速度提升10%，普通攻击与重击命中时，额外造成20%/25%/30%/35%/40%攻击力的伤害，持续12秒。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "天空之刃"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardBladeEffect::new()))
    }
}
