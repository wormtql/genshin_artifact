use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct EmeraldOrbEffect {
    pub rate: f64
}

impl EmeraldOrbEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for EmeraldOrbEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.05 + 0.15) * self.rate;
        attribute.add_atk_percentage("异世界行记被动等效", value);
    }
}

pub struct EmeraldOrb;

impl WeaponTrait for EmeraldOrb {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::EmeraldOrb,
        internal_name: "Catalyst_Jade",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM20),
        weapon_base: WeaponBaseATKFamily::ATK448,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "触发蒸发、感电、冰冻或水元素扩散反应后的12秒内，攻击力提高<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。",
            en: "Upon causing a Vaporize, Electro-Charged, Frozen, or a Hydro-infused Swirl reaction, increases ATK by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> for 12s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "翡玉法球",
            en: "Emerald Orb"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::EmeraldOrb { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(EmeraldOrbEffect::new(rate)))
    }
}
