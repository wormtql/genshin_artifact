use crate::artifacts::Artifact;
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{Character, CharacterName};
use crate::character::characters::Furina;
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

pub struct FurinaDefaultTargetFunction;

impl TargetFunctionMetaTrait for FurinaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::FurinaDefault,
        name_locale: locale!(
            zh_cn: "芙宁娜-不休独舞",
            en: "Furina-Endless Solo of Solitude"
        ),
        description: locale!(
            zh_cn: "最大化E伤害",
            en: "Maximize E Damage"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Furina),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(FurinaDefaultTargetFunction)
    }
}

impl TargetFunction for FurinaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .golden_troupe(1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Furina as CharacterTrait>::DamageEnumType;
        let dmg_e2 = Furina::damage::<SimpleDamageBuilder>(
            &context,
            S::E2,
            &CharacterSkillConfig::Furina { hp_above50_count: 3, c6_after_e: true, c6_pneuma: true },
            None
        );

        dmg_e2.normal.expectation
    }
}
