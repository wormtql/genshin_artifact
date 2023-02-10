use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::traits::CharacterTrait;
use crate::character::characters::anemo::jean::Jean;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct JeanDefaultTargetFunction {
    pub damage_weight: f64,
    pub recharge_demand: f64
}

impl JeanDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> JeanDefaultTargetFunction {
        match *config {
            TargetFunctionConfig::JeanDefault { recharge_demand, damage_weight } => JeanDefaultTargetFunction {
                damage_weight, recharge_demand
            },
            _ => JeanDefaultTargetFunction {
                damage_weight: 0.8,
                recharge_demand: 1.6
            }
        }
    }
}

impl TargetFunctionMetaTrait for JeanDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::JeanDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "琴-蒲公英骑士",
            en: "Jean-Dandelion Knight"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通六边形琴",
            en: "Jean Default"
        ),
        tags: "副C,治疗",
        four: TargetFunctionFor::SomeWho(CharacterName::Jean),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "damage_weight",
            title: locale!(
                zh_cn: "治疗-伤害比重（0：纯治疗，1：纯伤害）",
                en: "Heal-DMG Weight(0: Pure Heal, 1: Pure DMG）",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 },
        },
        ItemConfig {
            name: "recharge_demand",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.0 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(JeanDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for JeanDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Jean as CharacterTrait>::DamageEnumType;
        let dmg_q = Jean::damage::<SimpleDamageBuilder>(&context, S::Q1, &CharacterSkillConfig::NoConfig, None).normal.expectation;
        let dmg_q = dmg_q * 1.8;
        let heal_q = Jean::damage::<SimpleDamageBuilder>(&context, S::QHeal1, &CharacterSkillConfig::NoConfig, None).normal.expectation;
        let heal_q = heal_q * 1.3;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(self.recharge_demand);

        (self.damage_weight * dmg_q + (1.0 - self.damage_weight) * heal_q) * recharge_ratio
    }
}
