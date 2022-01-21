use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::{ArtifactEffectConfig, ConfigArchaicPetra, ConfigLevel, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::Character;
use crate::character::characters::{HU_TAO_SKILL, HuTaoDamage, HuTaoDamageEnum};
use crate::common::{Element, SkillType, StatName};
use crate::damage::reaction::Reaction;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::{DefaultArtifactConfig, GetTargetFunctionOptConfig};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct HuTaoDefaultTargetFunction {
    skill1: usize,
    skill2: usize,
    skill3: usize,
    vaporize_rate: f64,
}

impl HuTaoDefaultTargetFunction {
    pub fn new<T: Attribute>(character: &Character<T>, config: &TargetFunctionConfig) -> HuTaoDefaultTargetFunction {
        HuTaoDefaultTargetFunction {
            skill1: character.common_data.skill1,
            skill2: character.common_data.skill2,
            skill3: character.common_data.skill3,
            vaporize_rate: match *config {
                TargetFunctionConfig::HuTaoDefault { vaporize_rate } => vaporize_rate,
                _ => 0.0
            },
        }
    }
}

impl GetTargetFunctionOptConfig for HuTaoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.05,
            atk_percentage: 0.5,
            hp_fixed: 0.1,
            hp_percentage: 1.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.1,
            elemental_mastery: self.vaporize_rate,
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
                StatName::ElementalMastery,
                StatName::HPPercentage
            ],
            goblet_main_stats: vec![
                StatName::PyroBonus,
                StatName::ATKPercentage,
                StatName::ElementalMastery,
                StatName::HPPercentage
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ElementalMastery,
                StatName::HPPercentage
            ],
            set_names: Some(vec![
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::RetracingBolide,
                ArtifactSetName::NoblesseOblige,
                ArtifactSetName::CrimsonWitchOfFlames
            ]),
        }
    }
}

impl DefaultArtifactConfig for HuTaoDefaultTargetFunction {
    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfig {
            config_archaic_petra: ConfigArchaicPetra {
                rate: team_config.shield_coverage,
                element: Element::Pyro
            },
            config_berserker: Default::default(),
            config_blizzard_strayer: Default::default(),
            config_bloodstained_chivalry: Default::default(),
            config_brave_heart: ConfigRate {
                rate: 0.5
            },
            config_crimson_witch_of_flames: ConfigLevel {
                level: 1.0
            },
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
            config_retracing_bolide: ConfigRate {
                rate: team_config.shield_coverage
            },
            config_shimenawas_reminiscence: Default::default(),
            config_tenacity_of_the_millelith: Default::default(),
            config_thundersoother: Default::default()
        }
    }
}

impl TargetFunction for HuTaoDefaultTargetFunction {
    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, enemy: &Enemy) -> f64 {
        let context = DamageContext {
            character_common_data: &character.common_data,
            enemy,
            attribute
        };

        let damage_charged = HuTaoDamage::damage::<SimpleDamageBuilder>(&context, HuTaoDamageEnum::Charged, true);
        damage_charged.normal.expectation * (1.0 - self.vaporize_rate) + self.vaporize_rate * damage_charged.vaporize.unwrap().expectation

        // let hp = attribute.get_value(AttributeName::HP);
        // let base_atk = attribute.get_value(AttributeName::ATKBase);
        // let atk_bonus = (HU_TAO_SKILL.elemental_skill_atk_bonus[self.skill2] * hp).min(4.0 * base_atk);
        // let em = attribute.get_value(AttributeName::ElementalMastery);
        //
        // // let ratio_normal = HU_TAO_SKILL.normal_dmg1[self.skill1];
        // let ratio_charged = HU_TAO_SKILL.charged_dmg1[self.skill1];
        // // let ratio_q = HU_TAO_SKILL.elemental_burst_dmg1[self.skill3];
        //
        // let mut builder_charged = SimpleDamageBuilder::new(ratio_charged, 0.0, 0.0);
        // builder_charged.add_extra_atk(atk_bonus);
        // let damage_charged = builder_charged.damage(
        //     attribute, enemy, Element::Pyro, SkillType::ChargedAttack, 90
        // ).expectation;
        // let damage_charged_vaporize = builder_charged.vaporize(
        //     attribute, enemy, Element::Pyro, SkillType::ChargedAttack, 90
        // ).expectation;
        //
        // damage_charged * (1.0 - self.vaporize_rate) + damage_charged_vaporize * self.vaporize_rate
    }
}