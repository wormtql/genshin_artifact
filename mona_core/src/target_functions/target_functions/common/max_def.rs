use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{SimpleAttributeGraph2, AttributeCommon};
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::StatName;
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct MaxDEFTargetFunction;

impl TargetFunctionMetaTrait for MaxDEFTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::MaxDEF,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "最大防御力",
            en: "Max DEF"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "最大化防御力",
            en: "Maximize DEF"
        ),
        tags: "防御",
        four: TargetFunctionFor::Common,
        image: TargetFunctionMetaImage::Custom("misc/sword")
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(MaxDEFTargetFunction)
    }
}

impl TargetFunction for MaxDEFTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 0.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.5,
        //     def_percentage: 1.0,
        //     recharge: 0.0,
        //     elemental_mastery: 0.0,
        //     critical: 0.0,
        //     critical_damage: 0.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::DEFPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::DEFPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::DEFPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::HuskOfOpulentDreams,
        //         ArtifactSetName::DefendersWill,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, _character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        attribute.get_def()
    }
}
