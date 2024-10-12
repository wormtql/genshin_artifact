use crate::attribute::{Attribute, AttributeCommon};
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

struct MountainBracingBoltEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for MountainBracingBoltEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        let bonus = 0.09 + 0.03 * refine;

        attribute.add_elemental_bonus("镇山之钉被动", bonus + bonus * self.rate);
    }
}

pub struct MountainBracingBolt;

impl WeaponTrait for MountainBracingBolt {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MountainBracingBolt,
        internal_name: "Pole_Umpakati",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "攀爬消耗的体力降低15%，元素战技造成的伤害提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>；此外，队伍中附近的其他角色施放元素战技后，装备者的元素战技造成的伤害还会提升<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>，持续8秒。",
            en: "Decreases Climbing Stamina Consumption by 15% and increases Elemental Skill DMG by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>. Also, after other nearby party members use Elemental Skills, the equipping character's Elemental Skill DMG will also increase by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> for 8s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "镇山之钉",
            en: "Mountain-Bracing Bolt"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::MountainBracingBolt { rate } => Some(Box::new(MountainBracingBoltEffect {
                rate
            })),
            _ => None
        }
    }
}
