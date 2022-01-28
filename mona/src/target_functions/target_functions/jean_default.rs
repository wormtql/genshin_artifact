use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::Character;
use crate::character::traits::CharacterTrait;
use crate::character::characters::jean::Jean;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

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
                damage_weight: 1.0,
                recharge_demand: 1.4
            }
        }
    }
}

impl TargetFunction for JeanDefaultTargetFunction {
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
            bonus_cryo: 0.0,
            bonus_geo: 2.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::AnemoBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
            ],
            set_names: Some(vec![
                ArtifactSetName::ViridescentVenerer,
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence
            ]),
        }
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfig {
            config_archaic_petra: Default::default(),
            config_berserker: Default::default(),
            config_blizzard_strayer: Default::default(),
            config_bloodstained_chivalry: Default::default(),
            config_brave_heart: Default::default(),
            config_crimson_witch_of_flames: Default::default(),
            config_heart_of_depth: Default::default(),
            config_husk_of_opulent_dreams: Default::default(),
            config_instructor: Default::default(),
            config_lavawalker: Default::default(),
            config_martial_artist: Default::default(),
            config_noblesse_oblige: Default::default(),
            config_pale_flame: Default::default(),
            config_retracing_bolide: Default::default(),
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

        type S = <Jean as CharacterTrait>::DamageEnumType;
        let dmg_q = Jean::damage::<SimpleDamageBuilder>(&context, S::Q1, &CharacterSkillConfig::NoConfig).normal.expectation;
        let dmg_q = dmg_q * 1.8;
        let heal_q = Jean::damage::<SimpleDamageBuilder>(&context, S::QHeal1, &CharacterSkillConfig::NoConfig).normal.expectation;
        let heal_q = heal_q * 1.3;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(self.recharge_demand);

        (self.damage_weight * dmg_q + (1.0 - self.damage_weight) * heal_q) * recharge_ratio
    }
}
