use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ConfigArchaicPetra, ConfigLevel, ConfigRate};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::character::characters::Diluc;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct DilucDefaultTargetFunction;

impl TargetFunction for DilucDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.0,
            elemental_mastery: 0.5,
            critical: 1.0,
            critical_damage: 1.0,
            bonus_electro: 0.0,
            bonus_pyro: 2.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::ElementalMastery
            ],
            goblet_main_stats: vec![
                StatName::PyroBonus,
                StatName::ATKPercentage
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::CrimsonWitchOfFlames,
            ]),
        }
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfig {
            config_archaic_petra: ConfigArchaicPetra {
                element: Element::Pyro,
                rate: team_config.shield_coverage
            },
            config_berserker: Default::default(),
            config_blizzard_strayer: Default::default(),
            config_bloodstained_chivalry: Default::default(),
            config_brave_heart: Default::default(),
            config_crimson_witch_of_flames: ConfigLevel {
                level: 3.0
            },
            config_heart_of_depth: Default::default(),
            config_husk_of_opulent_dreams: Default::default(),
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

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &Vec<&Artifact>, enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        const CONFIG: CharacterSkillConfig = CharacterSkillConfig::Diluc { pyro: true };
        type S = <Diluc as CharacterTrait>::DamageEnumType;
        let dmg_e = Diluc::damage::<SimpleDamageBuilder>(
            &context, S::E1, &CONFIG
        ).normal.expectation;
        let dmg_q = Diluc::damage::<SimpleDamageBuilder>(
            &context, S::Q1, &CONFIG
        ).normal.expectation;

        dmg_e * 3.4 + dmg_q * 2.5
    }
}
