use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct AquilaFavoniaEffect;

impl AquilaFavoniaEffect {
    pub fn new() -> AquilaFavoniaEffect {
        AquilaFavoniaEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for AquilaFavoniaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.05 + 0.15;
        attribute.add_atk_percentage("风鹰剑被动", value);
    }
}

pub struct AquilaFavonia;

impl WeaponTrait for AquilaFavonia {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AquilaFavonia,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: WeaponSubStatFamily::PhysicalBonus90,
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        chs: "风鹰剑",
        #[cfg(not(target_family = "wasm"))]
        effect: Some("西风之鹰的抗争：攻击力提高20%/25%/30%/35%/40%；受到伤害时触发：高扬抗争旗号的西风鹰之魂苏醒，恢复同等与攻击力的100%/115%/130%/145%/160%生命值，并对周围的敌人造成200%/230%/260%/290%/320%攻击力的伤害。该效果每15秒只能触发一次。")
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(AquilaFavoniaEffect::new()))
    }
}
