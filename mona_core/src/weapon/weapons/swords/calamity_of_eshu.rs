use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct CalamityOfEshuEffect {
    rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for CalamityOfEshuEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let bonus1 = 0.15 + refine * 0.05;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "厄水之祸被动", bonus1 * self.rate);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "厄水之祸被动", bonus1 * self.rate);

        let bonus2 = 0.06 + 0.02 * refine;
        attribute.set_value_by(AttributeName::CriticalNormalAttack, "厄水之祸被动", bonus2 * self.rate);
        attribute.set_value_by(AttributeName::CriticalChargedAttack, "厄水之祸被动", bonus2 * self.rate);
    }
}

pub struct CalamityOfEshu;

impl WeaponTrait for CalamityOfEshu {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CalamityOfEshu,
        internal_name: "Sword_SacrificialNgombe",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "角色处于护盾庇护下时，普通攻击和重击造成的伤害提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>，普通攻击和重击的暴击率提升<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>。",
            en: "While characters are protected by a Shield, DMG dealt by Normal and Charged Attacks is increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>, and Normal and Charged Attack CRIT Rate is increased by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "厄水之祸",
            en: "Calamity of Eshu"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::CalamityOfEshu { rate } => Some(Box::new(CalamityOfEshuEffect { rate })),
            _ => None
        }
    }
}
