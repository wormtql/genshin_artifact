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
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: WeaponSubStatFamily::EM24,
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        effect: Some("注能之卷：触发元素反应后的10秒内，获得8%/10%/12%/14%/16%元素伤害加成，该效果最多可叠加2层"),
        chs: "万国诸海图谱"
    };

    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK02
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MappaMareEffect::new(config)))
    }
}
