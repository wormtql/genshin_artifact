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
pub struct DialoguesOfTheDesertSages;

impl WeaponTrait for DialoguesOfTheDesertSages {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::DialoguesOfTheDesertSages,
        internal_name: "",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "进行治疗时，恢复<span style=\"color: #409EFF;\">8-10-12-14-16</span>点能量，该效果每10秒至多触发一次，角色处于队伍后台时也能触发。",
            en: "When the wielder performs healing, restore <span style=\"color: #409EFF;\">8-10-12-14-16</span> Energy. This effect can be triggered once every 10s and can occur even when the character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "沙中伟贤的对答",
            en: "Dialogues of the Desert Sages"
        ),
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
