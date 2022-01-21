use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::{ArtifactEffectConfig, ConfigArchaicPetra, ConfigBlizzardStrayer, ConfigRate};
use crate::attribute::{Attribute, SimpleAttributeGraph2};
use crate::character::{Character, GANYU_SKILL};
use crate::character::characters::{GanyuDamage, GanyuDamageEnum};
use crate::common::{Element, SkillType, StatName};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::{TargetFunction, TargetFunctionConfig};
use crate::target_functions::target_function::{DefaultArtifactConfig, GetTargetFunctionOptConfig};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct GanyuDefaultTargetFunction {
    pub skill1: usize,
    pub skill2: usize,
    pub skill3: usize,
    pub melt_rate: f64
}

impl GanyuDefaultTargetFunction {
    pub fn new<T>(character: &Character<T>, config: &TargetFunctionConfig) -> GanyuDefaultTargetFunction {
        GanyuDefaultTargetFunction {
            skill1: character.common_data.skill1 as usize,
            skill2: character.common_data.skill2 as usize,
            skill3: character.common_data.skill3 as usize,
            melt_rate: match *config {
                TargetFunctionConfig::GanyuDefault { melt_rate } => melt_rate,
                _ => 0.0
            }
        }
    }
}

impl GetTargetFunctionOptConfig for GanyuDefaultTargetFunction {
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
                StatName::ElementalMastery,
            ],
            goblet_main_stats: vec![
                StatName::CryoBonus,
                StatName::ATKPercentage,
                StatName::ElementalMastery,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ElementalMastery,
            ],
            set_names: Some(vec![
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::EmblemOfSeveredFate,
                ArtifactSetName::RetracingBolide,
                ArtifactSetName::BlizzardStrayer,
                ArtifactSetName::NoblesseOblige
            ]),
        }
    }
}

impl DefaultArtifactConfig for GanyuDefaultTargetFunction {
    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfig {
            config_archaic_petra: if team_config.shield_coverage > 0.5 {
                ConfigArchaicPetra {
                    rate: 0.5,
                    element: Element::Cryo
                }
            } else {
                Default::default()
            },
            config_berserker: Default::default(),
            config_blizzard_strayer: ConfigBlizzardStrayer {
                critical_bonus: 0.3
            },
            config_bloodstained_chivalry: Default::default(),
            config_brave_heart: ConfigRate {
                rate: 0.5
            },
            config_crimson_witch_of_flames: Default::default(),
            config_heart_of_depth: Default::default(),
            config_husk_of_opulent_dreams: Default::default(),
            config_instructor: Default::default(),
            config_lavawalker: Default::default(),
            config_martial_artist: ConfigRate {
                rate: 0.5
            },
            config_noblesse_oblige: ConfigRate {
                rate: 0.5
            },
            config_pale_flame: Default::default(),
            config_retracing_bolide: if team_config.shield_coverage > 0.5 {
                ConfigRate {
                    rate: 0.5
                }
            } else {
                Default::default()
            },
            config_shimenawas_reminiscence: Default::default(),
            config_tenacity_of_the_millelith: Default::default(),
            config_thundersoother: Default::default()
        }
    }
}

impl TargetFunction for GanyuDefaultTargetFunction {
    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, enemy: &Enemy) -> f64 {
        // let charged_ratio1 = GANYU_SKILL.charged_dmg3[self.skill1 - 1];
        // let charged_ratio2 = GANYU_SKILL.charged_dmg4[self.skill1 - 1];
        // let q_ratio = GANYU_SKILL.elemental_burst_dmg1[self.skill3 - 1];
        //
        // let charged_damage = SimpleDamageBuilder::new(
        //     charged_ratio1 * 0.8 + charged_ratio2 * 1.2,
        //     0.0,
        //     0.0
        // ).damage(attribute, enemy, Element::Cryo, SkillType::ChargedAttack, 90);
        //
        // let elemental_burst_damage = SimpleDamageBuilder::new(
        //     q_ratio, 0.0, 0.0
        // ).damage(
        //     attribute,
        //     enemy,
        //     Element::Cryo,
        //     SkillType::ElementalBurst,
        //     90
        // );
        //
        // charged_damage.expectation * 0.7 + elemental_burst_damage.expectation * 0.3

        let damage_context = DamageContext {
            enemy,
            character_common_data: &character.common_data,
            attribute
        };

        let charged3_damage = GanyuDamage::damage::<SimpleDamageBuilder>(
            &damage_context,
            GanyuDamageEnum::Charged3
        );
        let charged4_damage = GanyuDamage::damage::<SimpleDamageBuilder>(
            &damage_context,
            GanyuDamageEnum::Charged4
        );
        let charged_mean = (1.0 - self.melt_rate) * (charged3_damage.normal.expectation * 0.8 + charged4_damage.normal.expectation * 1.2)
            + self.melt_rate * (charged3_damage.melt.unwrap().expectation * 0.8 + charged4_damage.melt.unwrap().expectation * 1.2);

        let q_damage = GanyuDamage::damage::<SimpleDamageBuilder>(
            &damage_context,
            GanyuDamageEnum::Q1
        );
        let q_mean = (1.0 - self.melt_rate) * q_damage.normal.expectation + self.melt_rate * q_damage.melt.unwrap().expectation;

        charged_mean + q_mean
    }
}