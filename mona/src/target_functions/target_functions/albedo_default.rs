use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::{ArtifactEffectConfig, ConfigLevel, ConfigRate};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::character::traits::{CharacterConstant, CharacterDamage, CharacterTrait};
use crate::character::characters::Albedo;
use crate::character::characters::albedo::AlbedoDamageEnum;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::damage_result::SimpleDamageResult;
use crate::enemies::Enemy;
use crate::target_functions::target_function::{DefaultArtifactConfig, GetTargetFunctionOptConfig};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct AlbedoDefaultTargetFunction;

impl AlbedoDefaultTargetFunction {
    pub fn new() -> AlbedoDefaultTargetFunction {
        AlbedoDefaultTargetFunction {}
    }
}

impl GetTargetFunctionOptConfig for AlbedoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.2,
            elemental_mastery: 0.3,
            critical: 1.0,
            critical_damage: 1.0,
            bonus_electro: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 2.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::DEFPercentage,
            ],
            goblet_main_stats: vec![
                StatName::GeoBonus,
                StatName::ATKPercentage,
                StatName::DEFPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::DEFPercentage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::HuskOfOpulentDreams,
                ArtifactSetName::DefendersWill,
                ArtifactSetName::Gambler,
                ArtifactSetName::ArchaicPetra,
            ]),
        }
    }
}

impl DefaultArtifactConfig for AlbedoDefaultTargetFunction {
    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfig {
            config_archaic_petra: Default::default(),
            config_berserker: Default::default(),
            config_blizzard_strayer: Default::default(),
            config_bloodstained_chivalry: Default::default(),
            config_brave_heart: Default::default(),
            config_crimson_witch_of_flames: Default::default(),
            config_heart_of_depth: Default::default(),
            config_husk_of_opulent_dreams: ConfigLevel {
                level: 4.0
            },
            config_instructor: Default::default(),
            config_lavawalker: Default::default(),
            config_martial_artist: Default::default(),
            config_noblesse_oblige: Default::default(),
            config_pale_flame: Default::default(),
            config_retracing_bolide: ConfigRate {
                rate: team_config.shield_coverage
            },
            config_shimenawas_reminiscence: Default::default(),
            config_tenacity_of_the_millelith: Default::default(),
            config_thundersoother: Default::default()
        }
    }
}

impl TargetFunction for AlbedoDefaultTargetFunction {
    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, enemy: &Enemy) -> f64 {
        let context = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        type SkillEnum = <Albedo as CharacterConstant>::DamageEnumType;

        let config = CharacterSkillConfig::Albedo { fatal_count: 4 };

        let damage_transient_blossom: SimpleDamageResult = <Albedo as CharacterDamage<SimpleDamageBuilder>>::damage(&context, SkillEnum::ETransientBlossom, &config);
        let damage_q_blossom: SimpleDamageResult = <Albedo as CharacterDamage<SimpleDamageBuilder>>::damage(&context, SkillEnum::QFatalBlossom, &config);
        let damage_q1: SimpleDamageResult = <Albedo as CharacterDamage<SimpleDamageBuilder>>::damage(&context, SkillEnum::Q1, &config);

        damage_transient_blossom.normal.expectation * 14.0
        + damage_q1.normal.expectation * 1.0
        + damage_q_blossom.normal.expectation * 7.0
    }
}