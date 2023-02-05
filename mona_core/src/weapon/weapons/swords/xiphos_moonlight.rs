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

pub struct XiphosMoonlightEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for XiphosMoonlightEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.00009 + 0.00027) * self.rate;
        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::Recharge,
            Box::new(move |x, _| value * x),
            Box::new(move |x, y, grad| (value * grad, 0.0)),
            "西福斯的月光被动",
        );
    }
}

pub struct XiphosMoonlight;

impl WeaponTrait for XiphosMoonlight {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::XiphosMoonlight,
        internal_name: "Sword_Pleroma",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM36),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "每10秒，产生如下效果：装备者的每点元素精通，都会为该角色提升<span style=\"color: #409EFF;\">0.036%-0.045%-0.054%-0.063%-0.072%</span>元素充能效率，并基于该提升的30%为队伍中附近的其他角色提升元素充能效率，持续12秒，多件同名武器产生的此效果可以叠加。角色处于队伍后台时也能触发效果。",
            en: "The following effect will trigger every 10s: The equipping character will gain <span style=\"color: #409EFF;\">0.036%-0.045%-0.054%-0.063%-0.072%</span> Energy Recharge for each point of Elemental Mastery they possess for 12s, with nearby party members gaining 30% of this buff for the same duration. Multiple instances of this weapon can allow this buff to stack. This effect will still trigger even if the character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "西福斯的月光",
            en: "Xiphos' Moonlight"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::XiphosMoonlight { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(XiphosMoonlightEffect { rate }))
    }
}
