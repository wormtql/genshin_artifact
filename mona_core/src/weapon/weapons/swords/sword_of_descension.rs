use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct SwordOfDescension;

impl WeaponTrait for SwordOfDescension {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SwordOfDescension,
        internal_name: "Sword_Psalmus",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK77),
        weapon_base: WeaponBaseATKFamily::ATK440,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "<span style=\"color: #409EFF;\">仅在以下平台生效：</span><br><span style=\"color: #409EFF;\">\"PlayStation Network\"</span><br>普通攻击与重击命中敌人后有<span style=\"color: #409EFF;\">50%</span>概率在小范围内造成<span style=\"color: #409EFF;\">200%</span>攻击力的伤害。该效果每10秒只能触发一次；此外，旅行者装备降临之剑时，攻击力提升<span style=\"color: #409EFF;\">66</span>点。",
            en: "<span style=\"color: #409EFF;\">Effective only on the following platform: </span><br><span style=\"color: #409EFF;\">\"PlayStation Network\"</span><br>Hitting opponents with Normal and Charged Attacks grants a <span style=\"color: #409EFF;\">50%</span> chance to deal <span style=\"color: #409EFF;\">200%</span> ATK as DMG in a small AoE. This effect can only occur once every 10s. Additionally, if the Traveler equips the Sword of Descension, their ATK is increased by <span style=\"color: #409EFF;\">66</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "降临之剑",
            en: "Sword of Descension"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
