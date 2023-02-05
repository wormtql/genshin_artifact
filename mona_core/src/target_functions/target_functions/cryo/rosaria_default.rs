use serde::{Serialize, Deserialize};
use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigBlizzardStrayer, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::cryo::rosaria::Rosaria;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::ItemConfig;
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct RosariaDefaultTargetFunction {
    pub other_atk_bonus_percentage: f64,
    pub other_critical: f64,
    pub other_critical_damage: f64,
}

#[derive(Serialize, Deserialize)]
pub struct RosariaDefaultTFConfig {
    pub other_atk_bonus_percentage: f64,
    pub other_critical: f64,
    pub other_critical_damage: f64,
}

impl Default for RosariaDefaultTargetFunction {
    fn default() -> Self {
        RosariaDefaultTargetFunction {
            other_critical: 0.7,
            other_atk_bonus_percentage: 2.5,
            other_critical_damage: 1.8
        }
    }
}

impl RosariaDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        match *config {
            TargetFunctionConfig::RosariaDefault(ref cfg) => {
                RosariaDefaultTargetFunction {
                    other_critical: cfg.other_critical,
                    other_critical_damage: cfg.other_critical_damage,
                    other_atk_bonus_percentage: cfg.other_atk_bonus_percentage
                }
            },
            _ => {
                Default::default()
            }
        }
    }
}

impl TargetFunctionMetaTrait for RosariaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::RosariaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "罗莎莉亚-棘冠恩典",
            en: "Rosaria-Thorny Benevolence"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通辅助罗莎莉亚兼一定的输出",
            en: "Support Rosaria with a certain amount of DPS"
        ),
        tags: "辅助,输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Rosaria),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(RosariaDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for RosariaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.2,
        //     elemental_mastery: 0.3,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 2.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::CryoBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::BlizzardStrayer
        //     ]),
        //     very_critical_set_names: Some(vec![
        //         ArtifactSetName::NoblesseOblige
        //     ]),
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: 0.01
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .blizzard_strayer(0.3)
            .noblesse_oblige(1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };
        type S = <Rosaria as CharacterTrait>::DamageEnumType;

        let atk_for_other = attribute.get_value(AttributeName::ATKBonusForOther);
        let bonus_crit = attribute.get_value(AttributeName::ATKBase).min(1.0) * 0.15;

        let dmg_q = Rosaria::damage::<SimpleDamageBuilder>(&context, S::Q11, &CharacterSkillConfig::NoConfig, None).normal.expectation;

        let other_damage_times = (1.0 + atk_for_other / (1.0 + self.other_atk_bonus_percentage)) * (1.0 + bonus_crit * self.other_critical_damage / (1.0 + self.other_critical * self.other_critical_damage));
        other_damage_times * dmg_q.ln()
    }
}
