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

struct AshGravenDrinkingHornEffect {

}

pub struct AshGravenDrinkingHorn;

impl WeaponTrait for AshGravenDrinkingHorn {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AshGravenDrinkingHorn,
        internal_name: "Catalyst_ConchSprayer",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "攻击命中敌人时，在目标位置基于生命值上限的<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>，造成范围伤害。该效果每15秒至多触发一次。",
            en: "When an attack hits an opponent, deal AoE DMG equal to <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> of Max HP at the target location. This effect can be triggered once every 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "苍纹角杯",
            en: "Ash-Graven Drinking Horn"
        ),
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
