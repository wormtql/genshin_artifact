use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct SacrificialGreatsword;

impl WeaponTrait for SacrificialGreatsword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SacrificialGreatsword,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: WeaponSubStatFamily::Recharge67,
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        effect: Some("气定神闲：元素战技造成伤害时，有40%/50%/60%/70%/80%概率重置该技能的冷却时间，该效果每30/26/22/19/16秒只能触发一次。"),
        chs: "祭礼大剑"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
