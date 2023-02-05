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
        internal_name: "Sword_Sashimi",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK77),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "攻击命中时，有50%的概率对单个敌人造成<span style=\"color: #409EFF;\">240%-280%-320%-360%-400%</span>攻击力的伤害。该效果每<span style=\"color: #409EFF;\">15-14-13-12-11</span>秒只能触发一次。",
            en: "On hit, has a 50% chance to deal <span style=\"color: #409EFF;\">240%-280%-320%-360%-400%</span> ATK DMG to a single opponent. Can only occur once every <span style=\"color: #409EFF;\">15-14-13-12-11</span>s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "吃虎鱼刀",
            en: "Fillet Blade"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
