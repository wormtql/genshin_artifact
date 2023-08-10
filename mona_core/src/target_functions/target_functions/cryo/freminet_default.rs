use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Freminet;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct FreminetDefaultTargetFunction;

impl TargetFunction for FreminetDefaultTargetFunction {
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
            enemy,
        };

        type S = <Freminet as CharacterTrait>::DamageEnumType;
        let e_shatter3_cryo = Freminet::damage::<SimpleDamageBuilder>(&context, S::EShatter3Cryo, &CharacterSkillConfig::Freminet { talent2_rate: 1.0 }, None);
        let e_shatter3_physical = Freminet::damage::<SimpleDamageBuilder>(&context, S::EShatter3Physical, &CharacterSkillConfig::Freminet { talent2_rate: 1.0 }, None);

        let dmg = e_shatter3_cryo.normal.expectation + e_shatter3_physical.normal.expectation;
        dmg
    }
}

impl TargetFunctionMetaTrait for FreminetDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::FreminetDefault,
        name_locale: locale!(
            zh_cn: "菲米尼-潜怀遐梦",
            en: "Freminet-Yearning for Unseen Depths",
        ),
        description: locale!(
            zh_cn: "普通输出菲米尼，考虑物伤和冰伤",
            en: "Normal DPS Freminet"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Freminet),
        image: TargetFunctionMetaImage::Avatar
    };


    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(FreminetDefaultTargetFunction)
    }
}