use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct HuntersPathEffect {
    rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for HuntersPathEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.add_elemental_bonus("猎人之径被动", refine * 0.03 + 0.09);

        let rate = self.rate;

        for a in [AttributeName::ElementalMastery, AttributeName::ElementalMasteryExtra] {
            attribute.add_edge1(
                a,
                AttributeName::ExtraDmgChargedAttack,
                Box::new(move |em: f64, _| (0.4 * refine + 1.2) * em * rate),
                Box::new(move |grad, x1, _x2| (grad * (0.4 * refine + 1.2) * rate, 0.0)),
                "猎人之径被动等效"
            );
        }
    }
}

pub struct HuntersPath;

impl WeaponTrait for HuntersPath {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::HuntersPath,
        internal_name: "Bow_Ayus",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate96),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "获得<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>所有元素伤害加成。重击命中敌人后，将获得「无休止的狩猎」：重击造成的伤害值提高，提高值相当于元素精通数值的<span style=\"color: #409EFF;\">160%-200%-240%-280%-320%</span>。该效果将在生效12次或10秒后消失，每12秒至多获得一次无休止的狩猎。",
            en: "Gain <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> All Elemental DMG Bonus. Obtain the Tireless Hunt effect after hitting an opponent with a Charged Attack. This effect increases Charged Attack DMG by <span style=\"color: #409EFF;\">160%-200%-240%-280%-320%</span> of Elemental Mastery. This effect will be removed after 12 Charged Attacks or 10s. Only 1 instance of Tireless Hunt can be gained every 12s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "猎人之径",
            en: "Hunter's Path"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::HuntersPath { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(HuntersPathEffect { rate }))
    }
}