use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::pyro::xiangling::Xiangling;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
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

pub struct XianglingDefaultTargetFunction {
    pub recharge_demand: f64,
    pub melt_rate: f64,
    pub vaporize_rate: f64,
    pub overload_rate: f64,
}

impl XianglingDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        match *config {
            TargetFunctionConfig::XianglingDefault { recharge_demand, mut melt_rate, mut vaporize_rate, overload_rate } => {
                let temp = melt_rate + vaporize_rate;
                if temp > 1.0 {
                    melt_rate /= temp;
                    vaporize_rate /= temp;
                }

                XianglingDefaultTargetFunction {
                    recharge_demand,
                    melt_rate,
                    vaporize_rate,
                    overload_rate
                }
            },
            _ => XianglingDefaultTargetFunction {
                recharge_demand: 1.0,
                melt_rate: 0.0,
                vaporize_rate: 0.0,
                overload_rate: 0.0
            }
        }
    }
}

impl TargetFunctionMetaTrait for XianglingDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::XianglingDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "香菱-万民百味",
            en: "Xiangling-Exquisite Delicacy"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出火伤香菱",
            en: "Cryo DPS Xiangling"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Xiangling),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.8 }
        },
        ItemConfig {
            name: "melt_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "融化占比",
                en: "Melt Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "vaporize_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "蒸发占比",
                en: "Vaporize Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "overload_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "超载频率",
                en: "Overload Frequency",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(XianglingDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for XianglingDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // let em = if self.melt_rate + self.vaporize_rate > 0.5 || self.overload_rate > 0.5 {
        //     1.0
        // } else {
        //     0.3
        // };
        //
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 1.0,
        //     elemental_mastery: em,
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
        //         StatName::Recharge,
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
        //         ArtifactSetName::EmblemOfSeveredFate,
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::CrimsonWitchOfFlames,
        //         ArtifactSetName::NoblesseOblige,
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
            .lavawalker(0.7)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Xiangling as CharacterTrait>::DamageEnumType;
        let dmg_q = Xiangling::damage::<SimpleDamageBuilder>(&context, S::Q4, &CharacterSkillConfig::NoConfig, None);

        let normal = dmg_q.normal.expectation;
        let melt = dmg_q.melt.unwrap().expectation;
        let vaporize = dmg_q.vaporize.unwrap().expectation;

        let normal_rate = (1.0 - self.melt_rate - self.vaporize_rate).clamp(0.0, 1.0);

        let transformative = context.transformative();
        let overload = transformative.overload;

        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand);

        r * (normal * normal_rate + melt * self.melt_rate + vaporize * self.vaporize_rate + overload * self.overload_rate)
    }
}
