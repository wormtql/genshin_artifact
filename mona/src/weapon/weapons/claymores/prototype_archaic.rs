use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct PrototypeArchaic;

impl WeaponTrait for PrototypeArchaic {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrototypeArchaic,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: WeaponSubStatFamily::ATK60,
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        effect: Some("粉碎：普通攻击和重击命中时，有50%的概率对小范围内的敌人造成240%/300%/360%/420%/480%攻击力的额外伤害。该效果每15秒只能触发一次。"),
        chs: "试作古华"
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
