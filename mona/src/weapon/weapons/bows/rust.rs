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

pub struct RustEffect;

impl RustEffect {
    pub fn new() -> RustEffect {
        RustEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for RustEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "弓藏被动", data.refine as f64 * 0.1 + 0.3);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "弓藏被动", -0.1);
    }
}

pub struct Rust;

impl WeaponTrait for Rust {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Rust,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("速射弓斗：普通攻击伤害提升40%/50%/60%/70%/80%，重击伤害下降10%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "弓藏"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(RustEffect::new()))
    }
}
