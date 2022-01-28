use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ConfigArchaicPetra, ConfigBlizzardStrayer, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::Character;
use crate::character::characters::kamisato_ayaka::KamisatoAyaka;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct KamisatoAyakaDefaultTargetFunction;

impl TargetFunction for KamisatoAyakaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.3,
            elemental_mastery: 0.0,
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
            ],
            goblet_main_stats: vec![
                StatName::CryoBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::BlizzardStrayer,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::EmblemOfSeveredFate,
                ArtifactSetName::GladiatorsFinale,
            ]),
        }
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfig {
            config_archaic_petra: ConfigArchaicPetra {
                element: Element::Cryo,
                rate: team_config.shield_coverage
            },
            config_berserker: Default::default(),
            config_blizzard_strayer: ConfigBlizzardStrayer {
                critical_bonus: 0.3
            },
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
            config_retracing_bolide: ConfigRate {
                rate: team_config.shield_coverage
            },
            config_shimenawas_reminiscence: Default::default(),
            config_tenacity_of_the_millelith: Default::default(),
            config_thundersoother: Default::default()
        }
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &Vec<&Artifact>, enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        let s_config: CharacterSkillConfig = CharacterSkillConfig::KamisatoAyaka { after_dash: true };
        type S = <KamisatoAyaka as CharacterTrait>::DamageEnumType;
        let dmg_q = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::Q1, &s_config).normal.expectation;
        let dmg_normal = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::Normal1, &s_config).normal.expectation;
        let dmg_charged = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::ChargedTimes3, &s_config).normal.expectation;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let r = recharge.min(1.6);

        dmg_q * r + dmg_normal + dmg_charged * 0.3
    }
}
