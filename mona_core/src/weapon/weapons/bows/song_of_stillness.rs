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

pub struct SongOfStillnessEffect {
    pub rate: f64
}

impl<A: Attribute> WeaponEffect<A> for SongOfStillnessEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::BonusBase, "静谧之曲被动", self.rate * (0.04 * refine + 0.12));
    }
}

pub struct SongOfStillness;

impl WeaponTrait for SongOfStillness {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SongOfStillness,
        internal_name: "Bow_Vorpal",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "受到治疗后，造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，持续8秒。角色处于队伍后台也能触发。",
            en: "After the wielder is healed, they will deal <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> more DMG for 8s. This can be triggered even when the character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "静谧之曲",
            en: "Song of Stillness"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::SongOfStillness { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(SongOfStillnessEffect {
            rate
        }))
    }
}
