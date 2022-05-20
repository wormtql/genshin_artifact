use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct FavoniusSword;

impl WeaponTrait for FavoniusSword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FavoniusSword,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge133),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("顺风而行：攻击造成暴击时，有60%/70%/80%/90%/100%的几率产生少量元素微粒，能为角色恢复6点元素能量。该效果每12/10.5/9/7.5/6秒只能触发一次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "西风剑"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
