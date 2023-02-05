use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct KatsuragikiriNagamasaEffect;

impl KatsuragikiriNagamasaEffect {
    pub fn new() -> KatsuragikiriNagamasaEffect {
        KatsuragikiriNagamasaEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for KatsuragikiriNagamasaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.015 + 0.045;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "桂木斩长正被动", value);
    }
}

pub struct KatsuragikiriNagamasa;

impl WeaponTrait for KatsuragikiriNagamasa {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::KatsuragikiriNagamasa,
        internal_name: "Claymore_Bakufu",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技造成的伤害提升<span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span>。元素战技命中后，角色流失3点元素能量，并在此后的6秒内，每2秒恢复<span style=\"color: #409EFF;\">3-3.5-4-4.5-5</span>点元素能量。该效果每10秒至多触发一次，角色处于队伍后台也能触发。",
            en: "Increases Elemental Skill DMG by <span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span>. After Elemental Skill hits an opponent, the character loses 3 Energy but regenerates <span style=\"color: #409EFF;\">3-3.5-4-4.5-5</span> Energy every 2s for the next 6s. This effect can occur once every 10s. Can be triggered even when the character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "桂木斩长正",
            en: "Katsuragikiri Nagamasa"
        )
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(KatsuragikiriNagamasaEffect::new()))
    }
}
