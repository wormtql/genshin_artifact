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

pub struct SkirkETargetFunction;

impl TargetFunction for SkirkETargetFunction {
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
        let dmg1_1 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal1, 
            &CharacterSkillConfig::Skirk { after_q: true }, 
            None
        ).normal.expectation;

        let dmg1_2 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal2, 
            &CharacterSkillConfig::Skirk { after_q: true }, 
            None
        ).normal.expectation;

        let dmg1_3 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal3, 
            &CharacterSkillConfig::Skirk { after_q: true }, 
            None
        ).normal.expectation;

        let dmg1_4 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal4, 
            &CharacterSkillConfig::Skirk { after_q: true }, 
            None
        ).normal.expectation;

        let dmg1_5 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal5, 
            &CharacterSkillConfig::Skirk { after_q: true }, 
            None
        ).normal.expectation;

        let dmg2_1 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal1, 
            &CharacterSkillConfig::NoConfig, 
            None
        ).normal.expectation;

        let dmg2_2 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal2, 
            &CharacterSkillConfig::NoConfig, 
            None
        ).normal.expectation;

        let dmg2_3 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal3, 
            &CharacterSkillConfig::NoConfig, 
            None
        ).normal.expectation;

        let dmg2_4 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal4, 
            &CharacterSkillConfig::NoConfig, 
            None
        ).normal.expectation;

        let dmg2_5 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::ENormal5, 
            &CharacterSkillConfig::NoConfig, 
            None
        ).normal.expectation;

        let dmg1_c6 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::C6_E, 
            &CharacterSkillConfig::Skirk { after_q: true }, 
            None
        ).normal.expectation;

        let dmg2_c6 = Skirk::damage::<SimpleDamageBuilder>(
            &context, 
            S::C6_E, 
            &CharacterSkillConfig::NoConfig, 
            None
        ).normal.expectation;

        dmg1_1 + dmg1_2 + dmg1_3 + dmg1_4 + dmg1_5 + dmg1_c6 * 3.0 + (dmg2_1 + dmg2_2 + dmg2_3 + dmg2_4 + dmg2_5) * 3.0 + dmg2_c6 * 3.0
    }
}

impl TargetFunctionMetaTrait for SkirkETargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::SkirkE,
        name_locale: locale!(
            zh_cn: "丝柯克-极恶技·闪",
            en: "Skirk-Havoc: Warp"
        ),
        description: locale!(
            zh_cn: "4轮完整普攻的伤害 默认开头开q",
            en: "4 rounds of full attack damage. Default to Q at the beginning"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Skirk),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(SkirkETargetFunction)
    }
}