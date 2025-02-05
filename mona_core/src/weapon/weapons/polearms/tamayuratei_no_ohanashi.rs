use crate::attribute::{Attribute, AttributeCommon};
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

struct TamayurateiNoOhanashiEffect {
    pub rate: f64
}

impl<A: Attribute> WeaponEffect<A> for TamayurateiNoOhanashiEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let bonus = 0.15 + 0.05 * refine;
        attribute.add_atk_percentage("且住亭御咄被动", bonus * self.rate);
    }
}

pub struct TamayurateiNoOhanashi;

impl WeaponTrait for TamayurateiNoOhanashi {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TamayurateiNoOhanashi,
        internal_name: "Pole_Aoandon",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技时，提高<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>攻击力和10%移动速度，持续10秒。",
            en: "Increase ATK by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> and Movement SPD by 10% for 10s when using an Elemental Skill."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "且住亭御咄",
            en: "Tamayuratei no Ohanashi"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::TamayurateiNoOhanashi { rate } => Some(Box::new(TamayurateiNoOhanashiEffect {
                rate
            })),
            _ => None
        }
    }
}
