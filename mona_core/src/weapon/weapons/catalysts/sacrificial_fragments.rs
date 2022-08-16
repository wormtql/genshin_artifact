use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct SacrificialFragments;

impl WeaponTrait for SacrificialFragments {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SacrificialFragments,
        internal_name: "Catalyst_Fossil",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM48),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("气定神闲：元素战技造成伤害时，有40%/50%/60%/70%/80%的概率重置该技能的冷却时间，该效果每30/26/22/19/16秒只能触发1次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "祭礼残章"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
