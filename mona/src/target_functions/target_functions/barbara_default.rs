use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::Character;
use crate::character::characters::Barbara;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterConstant, CharacterDamage};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::{DefaultArtifactConfig, GetTargetFunctionOptConfig};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct BarbaraDefaultTargetFunction;

impl GetTargetFunctionOptConfig for BarbaraDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 0.0,
            hp_fixed: 0.1,
            hp_percentage: 1.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.8,
            elemental_mastery: 0.0,
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
                StatName::HPPercentage,
                StatName::Recharge
            ],
            goblet_main_stats: vec![
                StatName::HPPercentage
            ],
            head_main_stats: vec![
                StatName::HealingBonus,
                StatName::HPPercentage
            ],
            set_names: Some(vec![
                ArtifactSetName::MaidenBeloved,
                ArtifactSetName::OceanHuedClam,
            ]),
        }
    }
}

impl DefaultArtifactConfig for BarbaraDefaultTargetFunction {
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
}

impl TargetFunction for BarbaraDefaultTargetFunction {
    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        type S = <Barbara as CharacterConstant>::DamageEnumType;
        let heal_e1 = <Barbara as CharacterDamage<SimpleDamageBuilder>>::damage(&context, S::EHeal1, &CharacterSkillConfig::NoConfig);
        const ENV_CHARGE: f64 = 2.3;
        const E1_COUNT: f64 = 10.0;
        const E2_COUNT: f64 = 3.0;

        let heal_e = E1_COUNT * heal_e1.normal.expectation + E2_COUNT * heal_e1.normal.expectation * 5.33;
        let heal_q = heal_e1.normal.expectation * 23.467;

        let recharge = attribute.get_value(AttributeName::Recharge);

        heal_q / 20.0_f64.max(80.0 / (ENV_CHARGE * recharge)) + heal_e / 32.0
    }
}