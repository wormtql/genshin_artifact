use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct MappaMareEffect {
    stack: f64
}

impl MappaMareEffect {
    pub fn new(config: &WeaponConfig) -> MappaMareEffect {
        match *config {
            WeaponConfig::MappaMare { stack } => MappaMareEffect {
                stack
            },
            _ => MappaMareEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for MappaMareEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.02 + 0.06) * self.stack;
        attribute.add_elemental_bonus("万国诸海图谱被动等效", value);
    }
}

pub struct MappaMare;

impl WeaponTrait for MappaMare {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MappaMare,
        internal_name: "Catalyst_Exotic",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM24),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "触发元素反应后的10秒内，获得<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>元素伤害加成，该效果最多可以叠加2层。",
            en: "Triggering an Elemental reaction grants a <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> Elemental DMG Bonus for 10s. Max 2 stacks."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "万国诸海图谱",
            en: "Mappa Mare"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK02
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MappaMareEffect::new(config)))
    }
}
