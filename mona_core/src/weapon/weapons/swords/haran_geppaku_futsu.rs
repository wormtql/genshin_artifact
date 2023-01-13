use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct HaranGeppakuFutsuEffect {
    pub stack: f64
}

impl<A: Attribute> WeaponEffect<A> for HaranGeppakuFutsuEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let bonus1 = refine * 0.03 + 0.09;
        attribute.add_elemental_bonus("波乱月白经津被动", bonus1);

        let bonus2 = (refine * 0.05 + 0.15) * self.stack;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "波乱月白经津被动等效", bonus2);
    }
}

pub struct HaranGeppakuFutsu;

impl WeaponTrait for HaranGeppakuFutsu {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::HaranGeppakuFutsu,
        internal_name: "Sword_Amenoma",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate72),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("白刃流转：获得12/15/18/21/24%所有元素伤害加成；队伍中附近的其他角色在施放元素战技时，会为装备该武器的角色产生1层「波穗」效果，至多叠加2层，每0.3秒最多触发1次。装备该武器的角色施放元素战技时，如果有积累的「波穗」效果，则将消耗已有的「波穗」，获得「波乱」：根据消耗的层数，每层提升20/25/30/35/40%普通攻击伤害，持续8秒。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "波乱月白经津"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "w25",
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 2.0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::HaranGeppakuFutsu { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(HaranGeppakuFutsuEffect {
            stack
        }))
    }
}
