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

pub struct FleuveCendreFerrymanEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for FleuveCendreFerrymanEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::CriticalElementalSkill, "灰河渡手被动", 0.02 * refine + 0.06);
        attribute.set_value_by(AttributeName::Recharge, "灰河渡手被动", (0.04 * refine + 0.12) * self.rate);
    }
}

pub struct FleuveCendreFerryman;

impl WeaponTrait for FleuveCendreFerryman {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FleuveCendreFerryman,
        internal_name: "Sword_Machination",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "元素战技暴击率提升<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>；此外，施放元素战技后的5秒内，元素充能效率提升1<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>。",
            en: "Increases Elemental Skill CRIT Rate by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>. Additionally, increases Energy Recharge by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> for 5s after using an Elemental Skill."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "灰河渡手",
            en: "Fleuve Cendre Ferryman"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::FleuveCendreFerryman { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(FleuveCendreFerrymanEffect {
            rate
        }))
    }
}
