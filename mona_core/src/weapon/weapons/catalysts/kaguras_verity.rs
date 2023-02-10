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

pub struct KagurasVerityEffect {
    pub stack: f64,
    pub full_rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for KagurasVerityEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = refine * 0.03 + 0.09;

        attribute.set_value_by(AttributeName::BonusElementalSkill, "神乐之真意被动等效", value * self.stack);
        attribute.add_elemental_bonus("神乐之真意被动等效", value * self.full_rate);
    }
}

pub struct KagurasVerity;

impl WeaponTrait for KagurasVerity {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::KagurasVerity,
        internal_name: "Catalyst_Narukami",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技时，将获得「神乐舞」的效果，使装备该武器的角色的元素战技造成的伤害提高<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>，该效果持续16秒，至多叠加3层。持有3层时，该角色获得<span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span>所有元素伤害加成。",
            en: "Gains the Kagura Dance effect when using an Elemental Skill, causing the Elemental Skill DMG of the character wielding this weapon to increase by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> for 16s. Max 3 stacks. This character will gain <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> All Elemental DMG Bonus when they possess 3 stacks."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "神乐之真意",
            en: "Kagura's Verity"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: ItemConfig::DEFAULT_STACK_TITLE,
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 3.0 }
        },
        ItemConfig {
            name: "full_rate",
            title: locale!(
                zh_cn: "满层比例",
                en: "Full Stack Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (stack, full_rate) = match *config {
            WeaponConfig::KagurasVerity { stack, full_rate } => (stack, full_rate),
            _ => (0.0, 0.0)
        };

        Some(Box::new(KagurasVerityEffect {
            stack, full_rate
        }))
    }
}

