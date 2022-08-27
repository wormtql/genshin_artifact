use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct StaffOfScarletSandsEffect {
    stack: f64
}

impl StaffOfScarletSandsEffect {
    pub fn new(config: &WeaponConfig) -> StaffOfScarletSandsEffect {
        match *config {
            WeaponConfig::StaffOfScarletSands { stack } => StaffOfScarletSandsEffect {
                stack
            },
            _ => StaffOfScarletSandsEffect {
                stack: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for StaffOfScarletSandsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let atk_bonus_1 = refine * 0.13 + 0.39;
        let atk_bonus_2 = (refine * 0.07 + 0.21)*self.stack;
        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::ATKFixed,
            Box::new(move |x, _| x * (atk_bonus_1+atk_bonus_2)),
            Box::new(move |grad, _x1, _x2| (atk_bonus_1+atk_bonus_2, 0.0)),
            "赤沙之杖被动等效"
        );
    }
}

pub struct StaffOfScarletSands;

impl WeaponTrait for StaffOfScarletSands {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::StaffOfScarletSands,
        internal_name: "Pole_ScarletSands",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate96),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("基于装备者元素精通的52%，获得攻击力加成。元素战技命中敌人时，将产生持续10秒的「赤沙之梦」效果：基于装备者元素精通的28%，获得攻击力加成，该效果至多叠加3层。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "赤沙之杖"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "w27",
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 4.0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(StaffOfScarletSandsEffect::new(config)))
    }
}
