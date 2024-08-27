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

struct FangOfTheMountainKingEffect {
    stack: f64
}

impl<A: Attribute> WeaponEffect<A> for FangOfTheMountainKingEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let bonus = 0.075 + 0.025 * refine;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "山王长牙被动", self.stack * bonus);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "山王长牙被动", self.stack * bonus);
    }
}

pub struct FangOfTheMountainKing;

impl WeaponTrait for FangOfTheMountainKing {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FangOfTheMountainKing,
        internal_name: "Claymore_EmeraldSword",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate24),
        weapon_base: WeaponBaseATKFamily::ATK741,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "元素战技命中敌人后，会获得1层「悬木祝赐」，该效果每0.5秒至多触发1次；队伍中附近的角色触发了燃烧或烈绽放反应后，装备者会获得3层悬木祝赐，该效果每2秒至多触发1次，队伍中的角色处于队伍后台时也能触发。悬木祝赐：元素战技伤害和元素爆发伤害提升<span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span>，持续6秒，至多叠加6层，每层持续时间独立计算。",
            en: "Gain 1 stack of Canopy's Favor after hitting an opponent with an Elemental Skill. This can be triggered once every 0.5s. After a nearby party member triggers a Burning or Burgeon reaction, the equipping character will gain 3 stacks. This effect can be triggered once every 2s and can be triggered even when the triggering party member is off-field. Canopy's Favor: Elemental Skill and Burst DMG is increased by <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span> for 6s. Max 6 stacks. Each stack is counted independently."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "山王长牙",
            en: "Fang of the Mountain King"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK06
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::FangOfTheMountainKing { stack } => Some(Box::new(FangOfTheMountainKingEffect {
                stack
            })),
            _ => None
        }
    }
}
