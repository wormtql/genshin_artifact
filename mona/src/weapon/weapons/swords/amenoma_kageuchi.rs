use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::{WeaponType};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct AmenomaKageuchi;

impl WeaponTrait for AmenomaKageuchi {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AmenomaKageuchi,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: WeaponSubStatFamily::ATK120,
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        effect: Some("施放元素战技后，获得1个胤种，该效果每5秒至多触发一次。胤种持续30秒，至多同时存在3个。施放元素爆发后，会清除持有的所有胤种，并在2秒之后，基于消耗的胤种数量，每个为该角色恢复6/7.5/9/10.5/12点元素能量"),
        chs: "天目影打刀"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
