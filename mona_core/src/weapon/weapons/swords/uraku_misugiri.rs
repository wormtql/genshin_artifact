use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct UrakuMisugiriEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for UrakuMisugiriEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let bonus_normal_atk = (refine * 0.04 + 0.12) * (1.0 + self.rate);
        let bonus_e = (refine * 0.06 + 0.18) * (1.0 + self.rate);
        let bonus_def = 0.05 * refine + 0.15;

        attribute.set_value_by(AttributeName::BonusNormalAttack, "有乐御簾切被动", bonus_normal_atk);
        attribute.set_value_by(AttributeName::BonusElementalSkill, "有乐御簾切被动", bonus_e);
        attribute.add_def_percentage("有乐御簾切被动", bonus_def);
    }
}

pub struct UrakuMisugiri;

impl WeaponTrait for UrakuMisugiri {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::UrakuMisugiri,
        internal_name: "IDontKnow",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "普通攻击造成的伤害提升<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>，元素战技造成的伤害提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>；队伍中附近的角色在场上造成岩元素伤害后，上述效果进一步提升100%，持续15秒。此外，装备者的防御力提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>。",
            en: "Normal Attack DMG is increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> and Elemental Skill DMG is increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>. After a nearby active character deals Geo DMG, the aforementioned effects increase by 100% for 15s. Additionally, the wielder's DEF is increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "有乐御簾切",
            en: "Uraku Misugiri"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::UrakuMisugiri { rate } => Some(Box::new(UrakuMisugiriEffect { rate })),
            _ => None
        }
    }
}
