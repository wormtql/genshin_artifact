use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Xinyan;
use crate::character::prelude::CharacterTrait;
use crate::character::skill_config::CharacterSkillConfig;
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

pub struct XinyanDefaultTargetFunction {
    pub recharge_demand: f64,
    pub damage_demand: f64,
}

impl XinyanDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (recharge_demand, damage_demand) = match *config {
            TargetFunctionConfig::XinyanDefault { recharge_demand, damage_demand } => (recharge_demand, damage_demand),
            _ => (1.0, 0.5)
        };
        Self {
            recharge_demand,
            damage_demand
        }
    }
}

impl TargetFunctionMetaTrait for XinyanDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::XinyanDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "辛焱-燥热旋律",
            en: "Xinyan-Blazing Riff"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通辅助辛焱",
            en: "Support Xinyan"
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Xinyan),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.4 }
        },
        ItemConfig {
            name: "damage_demand",
            title: crate::common::i18n::locale!(
                zh_cn: "伤害需求",
                en: "Damage Requirement",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(XinyanDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for XinyanDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.1,
        //     def_percentage: 1.0,
        //     recharge: 1.0,
        //     elemental_mastery: 0.0,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 1.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //         StatName::DEFPercentage,
        //         StatName::Recharge,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::PhysicalBonus,
        //         StatName::ATKPercentage,
        //         StatName::DEFPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::DEFPercentage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::HuskOfOpulentDreams,
        //         ArtifactSetName::DefendersWill,
        //         ArtifactSetName::Gambler,
        //         ArtifactSetName::ArchaicPetra,
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
            .noblesse_oblige(0.5)
            .tenacity_of_the_millelith(0.5)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        let atk_for_other = attribute.get_value(AttributeName::ATKBonusForOther);
        let def = attribute.get_value(AttributeName::DEF);
        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand) / self.recharge_demand;

        type S = <Xinyan as CharacterTrait>::DamageEnumType;
        let dmg_q = Xinyan::damage::<SimpleDamageBuilder>(&context, S::Q1, &CharacterSkillConfig::NoConfig, None).normal.expectation;

        ((1.0 + atk_for_other) * r) * (r * dmg_q * self.damage_demand + def * 10.0 * (1.0 - self.damage_demand))
    }
}
