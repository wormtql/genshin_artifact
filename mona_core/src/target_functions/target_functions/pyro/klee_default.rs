use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigArchaicPetra, ConfigLevel, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::pyro::klee::Klee;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct KleeDefaultTargetFunction {
    pub recharge_demand: f64,
}

impl TargetFunctionMetaTrait for KleeDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::KleeDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "可莉-逃跑的太阳",
            en: "Klee-Fleeing Sunlight"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "可莉火伤输出",
            en: "Pyro DPS Klee"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Klee),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: crate::common::i18n::locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement",
            ),
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.0 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let recharge_demand = match *config {
            TargetFunctionConfig::KleeDefault { recharge_demand } => recharge_demand,
            _ => 1.0
        };
        Box::new(KleeDefaultTargetFunction {
            recharge_demand
        })
    }
}

impl TargetFunction for KleeDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.2,
        //     elemental_mastery: 0.0,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 2.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::PyroBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::Lavawalker,
        //         ArtifactSetName::CrimsonWitchOfFlames,
        //         ArtifactSetName::EmblemOfSeveredFate,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .crimson_witch_of_flames(1.0)
            .echoes_of_an_offering_avg()
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        let config = CharacterSkillConfig::NoConfig;
        type S = <Klee as CharacterTrait>::DamageEnumType;
        let dmg_q = Klee::damage::<SimpleDamageBuilder>(&context, S::Q1, &config, None).normal.expectation;
        let dmg_charged = if character.common_data.has_talent1 {
            Klee::damage::<SimpleDamageBuilder>(&context, S::ChargedWithTalent, &config, None).normal.expectation
        } else {
            Klee::damage::<SimpleDamageBuilder>(&context, S::Charged, &config, None).normal.expectation
        };

        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand);

        r * dmg_q * 24.0 * 0.1 + dmg_charged * 0.9
    }
}
