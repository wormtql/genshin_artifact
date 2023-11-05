use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct SplendorOfTranquilWatersEffect {
    pub stack1: f64,
    pub stack2: f64
}

impl<A: Attribute> WeaponEffect<A> for SplendorOfTranquilWatersEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "静水流涌之辉被动", ((0.02 * refine) + 0.06) * self.stack1);
        attribute.add_hp_percentage("静水流涌之辉被动", (0.035 * refine + 0.105) * self.stack2);
    }
}

pub struct SplendorOfTranquilWaters;

impl WeaponTrait for SplendorOfTranquilWaters {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SplendorOfTranquilWaters,
        internal_name: "Sword_Regalis",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "装备者的当前生命值提升或降低时，元素战技造成的伤害提升<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>，该效果持续6秒，至多叠加3次，每0.2秒至多触发一次；队伍中其他角色的当前生命值提升或降低时，装备者的生命值上限提升<span style=\"color: #409EFF;\">14%-17.5%-21%-24.5%-28%</span>，该效果持续6秒，至多叠加2次，每0.2秒至多触发一次。装备者处于队伍后台时，依然能触发上述效果。",
            en: "When the equipping character's current HP increases or decreases, Elemental Skill DMG dealt will be increased by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> for 6s. Max 3 stacks. This effect can be triggered once every 0.2s. When other party members' current HP increases or decreases, the equipping character's Max HP will be increased by <span style=\"color: #409EFF;\">14%-17.5%-21%-24.5%-28%</span> for 6s. Max 2 stacks. This effect can be triggered once every 0.2s, and can be triggered even if the wielder is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "静水流涌之辉",
            en: "Splendor of Tranquil Waters"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack1",
            title: locale!(
                zh_cn: "被动1层数",
                en: "Stack 1",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 2.0 },
        },
        ItemConfig {
            name: "stack2",
            title: locale!(
                zh_cn: "被动2层数",
                en: "Stack 2"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 2.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (s1, s2) = match *config {
            WeaponConfig::SplendorOfTranquilWaters { stack1, stack2 } => (stack1, stack2),
            _ => (0.0, 0.0)
        };
        Some(Box::new(SplendorOfTranquilWatersEffect {
            stack1: s1,
            stack2: s2
        }))
    }
}
