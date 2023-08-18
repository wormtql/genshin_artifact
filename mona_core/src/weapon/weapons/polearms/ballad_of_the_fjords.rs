use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct BalladOfTheFjordsEffect {
    pub use_effect: bool
}

impl<A: Attribute> WeaponEffect<A> for BalladOfTheFjordsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        if self.use_effect {
            let value = 30 * data.refine + 90;
            attribute.set_value_by(AttributeName::ElementalMastery, "峡湾长歌被动", value as f64);
        }
    }
}

pub struct BalladOfTheFjords;

impl WeaponTrait for BalladOfTheFjords {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BalladOfTheFjords,
        internal_name: "Pole_Shanty",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate60),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "队伍中存在至少三种不同元素类型的角色时，元素精通提升<span style=\"color: #409EFF;\">120-150-180-210-240</span>点。",
            en: "When there are at least 3 different Elemental Types in your party, Elemental Mastery will be increased by <span style=\"color: #409EFF;\">120-150-180-210-240</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "峡湾长歌",
            en: "Ballad of the Fjords"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "use_effect",
            title: locale!(
                zh_cn: "开启被动",
                en: "Use Effect"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let b = match *config {
            WeaponConfig::BalladOfTheFjords { use_effect } => use_effect,
            _ => false
        };
        Some(Box::new(BalladOfTheFjordsEffect {
            use_effect: b
        }))
    }
}
