use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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

pub struct FluteOfEzpitzalEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for FluteOfEzpitzalEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let bonus = 0.12 + 0.04 * refine;
        attribute.add_def_percentage("息燧之笛被动", bonus * self.rate);
    }
}

pub struct FluteOfEzpitzal;

impl WeaponTrait for FluteOfEzpitzal {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FluteOfEzpitzal,
        internal_name: "Sword_Isikhulu",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::DEF150),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技时，防御力提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，持续15秒。",
            en: "Using an Elemental Skill increases DEF by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> for 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "息燧之笛",
            en: "Flute of Ezpitzal"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::FluteOfEzpitzal { rate } => Some(Box::new(FluteOfEzpitzalEffect { rate })),
            _ => None
        }
    }
}
