use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Skirk;
use crate::character::prelude::CharacterTrait;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::i18n::locale;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct SkirkQTargetFunction;

impl TargetFunction for SkirkQTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        type S = <Skirk as CharacterTrait>::DamageEnumType;
        let dmg1 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::Q1, 
            &CharacterSkillConfig::NoConfig, // 因为是q的伤害所以after_q 不需要配置
            None
        ).normal.expectation;

        let dmg2 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::Q2, 
            &CharacterSkillConfig::NoConfig,
            None
        ).normal.expectation;

        let dmg3 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::C6_Q, 
            &CharacterSkillConfig::NoConfig,
            None
        ).normal.expectation;

        dmg1 + dmg2 + dmg3
    }
}

impl TargetFunctionMetaTrait for SkirkQTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::SkirkQ,
        name_locale: locale!(
            zh_cn: "丝柯克-极恶技·灭",
            en: "Skirk-Havoc: Ruin"
        ),
        description: locale!(
            zh_cn: "最大化极恶技·灭伤害",
            en: "Maximize the damage of Skirk-Havoc: Ruin"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Skirk),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(SkirkQTargetFunction)
    }
}