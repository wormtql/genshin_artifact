use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigRate};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::hydro::sangonomiya_kokomi::SangonomiyaKokomi;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::ItemConfig;
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct SangonomiyaKokomiDefaultTargetFunction;

impl TargetFunctionMetaTrait for SangonomiyaKokomiDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::SangonomiyaKokomiDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "珊瑚宫心海-真珠之智",
            en: "Kokomi-Pearl of Wisdom"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出心海",
            en: "DPS Kokomi"
        ),
        tags: "输出,治疗",
        four: TargetFunctionFor::SomeWho(CharacterName::SangonomiyaKokomi),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(SangonomiyaKokomiDefaultTargetFunction)
    }
}

impl TargetFunction for SangonomiyaKokomiDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.1,
        //     hp_percentage: 1.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 1.0,
        //     elemental_mastery: 0.0,
        //     critical: 0.0,
        //     critical_damage: 0.0,
        //     healing_bonus: 2.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 2.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::HPPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::HydroBonus,
        //         StatName::HPPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::HPPercentage,
        //         StatName::HealingBonus,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::TenacityOfTheMillelith,
        //         ArtifactSetName::HeartOfDepth,
        //     ]),
        //     very_critical_set_names: Some(vec![
        //         ArtifactSetName::OceanHuedClam
        //     ]),
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .heart_of_depth(1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        let mut ohc_count = 0;
        for art in artifacts.iter() {
            if art.set_name == ArtifactSetName::OceanHuedClam {
                ohc_count += 1;
            }
        }

        type S = <SangonomiyaKokomi as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::SangonomiyaKokomi { after_q: true };
        let dmg_a = SangonomiyaKokomi::damage::<SimpleDamageBuilder>(&context, S::Normal1, &config, None).normal.expectation;

        let dmg_other = if ohc_count >= 4 {
            10000.0
        } else {
            0.0
        };

        dmg_a + dmg_other / 3.5
    }
}