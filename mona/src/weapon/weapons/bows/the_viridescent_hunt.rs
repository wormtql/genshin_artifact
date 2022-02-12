use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct TheViridescentHunt;

impl WeaponTrait for TheViridescentHunt {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheViridescentHunt,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: WeaponSubStatFamily::CriticalRate60,
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        effect: Some("苍翠之风：普通攻击与重击命中时，有50%几率生成一个风之眼，持续吸引周围敌人，并对其中的敌人每0.5秒造成40%/50%/60%/70%/80%攻击力的伤害。该效果持续4秒，每14/13/12/11/10秒至多触发一次。"),
        chs: "苍翠猎弓"
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
