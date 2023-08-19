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

pub struct ScionOfTheBlazingSunEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for ScionOfTheBlazingSunEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusChargedAttack, "烈阳之嗣被动", self.rate * (0.07 * data.refine as f64 + 0.21));
    }
}

pub struct ScionOfTheBlazingSun;

impl WeaponTrait for ScionOfTheBlazingSun {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ScionOfTheBlazingSun,
        internal_name: "Bow_Gurabad",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate40),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "重击命中敌人后，会向命中的敌人降下阳炎矢，造成攻击力<span style=\"color: #409EFF;\">60%-75%-90%-105%-120%</span>的伤害，并为受到阳炎矢伤害的敌人施加持续10秒的灼心效果。装备者的重击对处于灼心状态下的敌人造成的伤害提升<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>。阳炎矢每10秒至多触发一次。",
            en: "After a Charged Attack hits an opponent, a Sunfire Arrow will descend upon the opponent hit, dealing <span style=\"color: #409EFF;\">60%-75%-90%-105%-120%</span> ATK as DMG, and applying the Heartsearer effect to the opponent damaged by said Arrow for 10s. Opponents affected by Heartsearer take <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span> more Charged Attack DMG from the wielder. A Sunfire Arrow can be triggered once every 10s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "烈阳之嗣",
            en: "Scion of the Blazing Sun"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::ScionOfTheBlazingSun { rate } => rate,
            _ => 0.0
        };
        Some(Box::new(ScionOfTheBlazingSunEffect {
            rate
        }))
    }
}
