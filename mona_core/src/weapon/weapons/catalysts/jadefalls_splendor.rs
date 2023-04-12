use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::{Element, WeaponType};
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct JadeFallsSplendorEffect {
    pub rate: f64,
    pub element: Element,
}

impl<A: Attribute> WeaponEffect<A> for JadeFallsSplendorEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let bonus_name = AttributeName::bonus_name_by_element(self.element);
        let refine = data.refine as f64;
        let rate = self.rate;

        attribute.add_edge1(
            AttributeName::HP,
            bonus_name,
            Box::new(move |hp, _| {
                let temp = (hp / 1000.0).floor();
                let max = 0.08 * refine + 0.04;
                let step = 0.002 * refine + 0.001;
                let value = max.min(step * temp);
                value * rate
            }),
            Box::new(|grad, x, y| (0.0, 0.0)), // todo
            "碧落之珑被动等效"
        );
    }
}

pub struct JadeFallsSplendor;

impl WeaponTrait for JadeFallsSplendor {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::JadeFallsSplendor,
        internal_name: "Catalyst_Morax",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素爆发或创造护盾后的3秒内，将产生「定土玉圭」效果：每2.5秒恢复<span style=\"color: #409EFF;\">4.5-5-5.5-6-6.5</span>点元素能量，并基于装备者的生命值上限，每1000点使其对应元素类型的元素伤害加成提高<span style=\"color: #409EFF;\">0.3%-0.5%-0.7%-0.9%-1.1%</span>，至多提高<span style=\"color: #409EFF;\">12%-20%-28%-36%-44%</span>。装备该武器的角色处于队伍后台时，依然能产生「定土玉圭」效果。",
            en: "For 3s after using an Elemental Burst or creating a shield, the equipping character can gain the Primordial Jade Regalia effect: Restore <span style=\"color: #409EFF;\">4.5-5-5.5-6-6.5</span> Energy every 2.5s, and gain <span style=\"color: #409EFF;\">0.3%-0.5%-0.7%-0.9%-1.1%</span> Elemental DMG Bonus for their corresponding Elemental Type for every 1,000 Max HP they possess, up to <span style=\"color: #409EFF;\">12%-20%-28%-36%-44%</span>. Primordial Jade Regalia will still take effect even if the equipping character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "碧落之珑",
            en: "Jadefall’s Splendor",
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::JadeFallsSplendor { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(JadeFallsSplendorEffect {
            rate,
            element: character.static_data.element
        }))
    }
}
