use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct PolarStarEffect {
    stack: usize
}

impl PolarStarEffect {
    pub fn new(config: &WeaponConfig) -> PolarStarEffect {
        match *config {
            WeaponConfig::PolarStar { stack } => PolarStarEffect {
                stack
            },
            _ => PolarStarEffect {
                stack: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PolarStarEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let bonus1 = refine * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "冬极白星被动", bonus1);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "冬极白星被动", bonus1);

        let atk_bonus = if self.stack == 1 {
            refine * 0.025 + 0.075
        } else if self.stack == 2 {
            refine * 0.05 + 0.15
        } else if self.stack == 3 {
            refine * 0.075 + 0.225
        } else if self.stack == 4 {
            refine * 0.12 + 0.36
        } else {
            0.0
        };

        attribute.add_atk_percentage("冬极白星被动等效", atk_bonus);
    }
}

pub struct PolarStar;

impl WeaponTrait for PolarStar {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PolarStar,
        internal_name: "Bow_Worldbane",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate72),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技和元素爆发造成的伤害提高<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>；普通攻击、重击、元素战技或元素爆发命中敌人后，将产生1层持续12秒的「白夜极星」效果。处于1/2/3/4层「白夜极星」效果下时，攻击力将提高<span style=\"color: #409EFF;\">10/20/30/48%-12.5/25/37.5/60%-15/30/45/72%-17.5/35/52.5/84%-20/40/60/96%</span>。由普通攻击、重击、元素战技或元素爆发产生的「白夜极星」将分别独立存在。",
            en: "Elemental Skill and Elemental Burst DMG increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>. After a Normal Attack, Charged Attack, Elemental Skill or Elemental Burst hits an opponent, 1 stack of Ashen Nightstar will be gained for 12s. When 1/2/3/4 stacks of Ashen Nightstar are present, ATK is increased by <span style=\"color: #409EFF;\">10/20/30/48%-12.5/25/37.5/60%-15/30/45/72%-17.5/35/52.5/84%-20/40/60/96%</span>. The stack of Ashen Nightstar created by the Normal Attack, Charged Attack, Elemental Skill or Elemental Burst will be counted independently of the others."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "冬极白星",
            en: "Polar Star"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "「白夜极星」层数",
                en: "「Nightstar」Stack"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(PolarStarEffect::new(config)))
    }
}
