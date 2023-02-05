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
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct XinyanDamageTargetFunction;

impl TargetFunctionMetaTrait for XinyanDamageTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::XinyanDamage,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "辛焱-输出",
            en: "Xinyan-DPS"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通物伤输出辛焱",
            en: "Physical DPS Xinyan"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Xinyan),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(XinyanDamageTargetFunction)
    }
}

impl TargetFunction for XinyanDamageTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 1.0,
        //     recharge: 0.0,
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
        //         ArtifactSetName::RetracingBolide,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::BloodstainedChivalry,
        //         ArtifactSetName::Lavawalker,
        //         ArtifactSetName::PaleFlame,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD,
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .retracing_bolide(0.66)
            .shimenawas_reminiscence(0.5)
            .bloodstained_chivalry(0.8)
            .lavawalker(1.0)
            .pale_flame(0.38, 0.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };

        type S = <Xinyan as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::Xinyan { shield_rate: 0.7 };
        let dmg_c = Xinyan::damage::<SimpleDamageBuilder>(&context, S::Charged1, &config, None).normal.expectation;

        dmg_c
    }
}
