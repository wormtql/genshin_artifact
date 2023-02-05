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
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "普通攻击与重击命中时，有50%几率发射一枚昭心法球，造成<span style=\"color: #409EFF;\">240%-270%-300%-330%-360%</span>攻击力伤害，至多在敌人之间弹射4次。该效果每<span style=\"color: #409EFF;\">12-11-10-9-8</span>秒至多触发一次。",
            en: "Normal and Charged Attacks have a 50% chance to fire a Bolt of Perception, dealing <span style=\"color: #409EFF;\">240%-270%-300%-330%-360%</span> ATK as DMG. This bolt can bounce between opponents a maximum of 4 times. This effect can occur once every <span style=\"color: #409EFF;\">12-11-10-9-8</span>s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "昭心",
            en: "Eye of Perception"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
