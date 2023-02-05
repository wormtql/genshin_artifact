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
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技造成伤害时，有<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>的概率重置该技能的冷却时间，该效果每<span style=\"color: #409EFF;\">30-26-22-19-16</span>秒只能触发一次。",
            en: "After damaging an opponent with an Elemental Skill, the skill has a <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> chance to end its own CD. Can only occur once every <span style=\"color: #409EFF;\">30-26-22s-19-16s</span>s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "祭礼残章",
            en: "Sacrificial Fragments"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
