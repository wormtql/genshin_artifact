use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::yae_miko::YaeMiko;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct YaeMikoDefaultTargetFunction {
    pub recharge_demand: f64,
    pub electro_charged_times: f64,
    pub overload_times: f64,
    // pub electro_rate: f64,
}

impl TargetFunction for YaeMikoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.5,
            elemental_mastery: 1.0,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_electro: 2.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::ElementalMastery,
            ],
            goblet_main_stats: vec![
                StatName::ElectroBonus,
                StatName::ATKPercentage,
                StatName::ElementalMastery,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ElementalMastery,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::ThunderingFury,
                ArtifactSetName::Thundersoother,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .thundersoother(1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <YaeMiko as CharacterTrait>::DamageEnumType;
        let dmg_e = YaeMiko::damage::<SimpleDamageBuilder>(&context, S::E3, &CharacterSkillConfig::NoConfig).normal.expectation;

        let transformative = context.transformative();
        let dmg_electro_charged = transformative.electro_charged;
        let dmg_overload = transformative.overload;

        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand);

        r * (dmg_electro_charged * self.electro_charged_times + dmg_overload * self.overload_times + dmg_e)
    }
}

impl TargetFunctionMetaTrait for YaeMikoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::YaeMikoDefault,
        chs: "八重神子-浮世笑百姿",
        description: "普通输出八重神子",
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::YaeMiko),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "electro_charged_times",
            title: "感电相对频率",
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        },
        ItemConfig {
            name: "overload_times",
            title: "超载相对频率",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "recharge_demand",
            title: "充能需求",
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.0 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (a, b, c) = match *config {
            TargetFunctionConfig::YaeMikoDefault { recharge_demand, electro_charged_times, overload_times } => (recharge_demand, electro_charged_times, overload_times),
            _ => (1.0, 0.0, 0.0)
        };

        Box::new(YaeMikoDefaultTargetFunction {
            recharge_demand: a,
            electro_charged_times: b,
            overload_times: c
        })
    }
}
