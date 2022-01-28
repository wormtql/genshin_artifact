use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::Character;
use crate::common::StatName;
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct KaedeharaKazuhaDefaultTargetFunction;

impl TargetFunction for KaedeharaKazuhaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 0.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 1.0,
            elemental_mastery: 1.0,
            critical: 0.0,
            critical_damage: 0.0,
            bonus_electro: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ElementalMastery,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::ElementalMastery,
            ],
            head_main_stats: vec![
                StatName::ElementalMastery,
            ],
            set_names: Some(vec![
                ArtifactSetName::ViridescentVenerer,
                ArtifactSetName::WanderersTroupe,
                ArtifactSetName::Instructor
            ]),
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
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

    fn target(&self, attribute: &SimpleAttributeGraph2, _character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &Vec<&Artifact>, enemy: &Enemy) -> f64 {
        let em = attribute.get_value(AttributeName::ElementalMastery);
        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(1.8);
        recharge_ratio * em
    }
}
