use crate::attribute::{Attribute, AttributeName};
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

pub struct BlackTasselEffect {
    pub rate: f64
}

impl BlackTasselEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for BlackTasselEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.1 + 0.3) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "黑缨枪被动等效", value);
    }
}

pub struct BlackTassel;

impl WeaponTrait for BlackTassel {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BlackTassel,
        internal_name: "Pole_Noire",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP102),
        weapon_base: WeaponBaseATKFamily::ATK354,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "对史莱姆类敌人造成的伤害增加<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。",
            en: "Increases DMG against slimes by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "黑缨枪",
            en: "Black Tassel"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::BlackTassel { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(BlackTasselEffect::new(rate)))
    }
}
