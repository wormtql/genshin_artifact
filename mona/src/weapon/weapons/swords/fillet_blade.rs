use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct FilletBlade;

impl WeaponTrait for FilletBlade {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FilletBlade,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK77),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("决：攻击命中时，有50%的概率对单个敌人造成240%/280%/320%/360%/400%攻击力的伤害。该效果每15/14/13/12/11秒只能触发一次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "吃虎鱼刀"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
