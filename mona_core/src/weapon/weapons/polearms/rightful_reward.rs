use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct RightfulReward;

impl WeaponTrait for RightfulReward {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::RightfulReward,
        internal_name: "Pole_Vorpal",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "受到治疗时，恢复<span style=\"color: #409EFF;\">8-10-12-14-16</span>点能量，该效果每10秒至多触发一次，角色处于队伍后台时也能触发。",
            en: "When the wielder is healed, restore <span style=\"color: #409EFF;\">8-10-12-14-16</span> Energy. This effect can be triggered once every 10s, and can occur even when the character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "公义的酬报",
            en: "Rightful Reward"
        )
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
