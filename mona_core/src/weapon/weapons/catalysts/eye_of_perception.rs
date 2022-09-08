use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct EyeOfPerception;

impl WeaponTrait for EyeOfPerception {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::EyeOfPerception,
        internal_name: "Catalyst_Truelens",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK120),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("回响：普通攻击与重击命中时，有50%几率发射一枚昭心法球，造成240%/270%/300%/330%/360%攻击力伤害，至多在敌人之间弹射4次。该效果每12/11/10/9/8秒至多触发一次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "昭心"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
