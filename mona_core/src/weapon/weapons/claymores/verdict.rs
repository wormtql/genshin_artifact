use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct VerdictEffect {
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for VerdictEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let atk_bonus = 0.05 * refine + 0.15;
        let e_bonus = 0.045 * refine + 0.135;

        attribute.add_atk_percentage("「裁断」被动", atk_bonus);
        attribute.set_value_by(AttributeName::BonusElementalSkill, "「裁断」被动", self.stack * e_bonus);
    }
}

pub struct Verdict;

impl WeaponTrait for Verdict {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Verdict,
        internal_name: "Claymore_GoldenVerdict",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate48),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "攻击力提升<span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>；队伍中的角色获取结晶反应产生的晶片时，会为装备者赋予1枚「约印」，使元素战技造成的伤害提升<span style=\"color: #409EFF;\">18%-22.5%-27%-31.5%-36%</span>，约印持续15秒，至多同时持有2枚。所有约印将在装备者的元素战技造成伤害后的0.2秒后移除。",
            en: "Increases ATK by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. When characters in your party obtain Elemental Shards from Crystallize reactions, the equipping character will gain 1 Seal, increasing Elemental Skill DMG by <span style=\"color: #409EFF;\">18%-22.5%-27%-31.5%-36%</span>. The Seal lasts for 15s, and the equipper may have up to 2 Seals at once. All of the equipper's Seals will disappear 0.2s after their Elemental Skill deals DMG."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "裁断",
            en: "Verdict"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "被动等效层数",
                en: "Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 2.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::Verdict { stack } => stack,
            _ => 0.0
        };
        Some(Box::new(VerdictEffect {
            stack
        }))
    }
}
