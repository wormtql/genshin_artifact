use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct YunjinDefaultTargetFunction {
    pub recharge_demand: f64
}

impl YunjinDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let recharge_demand = match *config {
            TargetFunctionConfig::YunjinDefault { recharge_demand } => recharge_demand,
            _ => 1.0
        };
        Self {
            recharge_demand
        }
    }
}

impl TargetFunctionMetaTrait for YunjinDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::YunjinDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "云堇-红毹婵娟",
            en: "Yunjin-Stage Lucida"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通增伤辅助云堇",
            en: "Support Yunjin"
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Yunjin),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.4 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(YunjinDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for YunjinDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 0.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.1,
        //     def_percentage: 1.0,
        //     recharge: 1.0,
        //     elemental_mastery: 0.0,
        //     critical: 0.5,
        //     critical_damage: 0.5,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 1.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::Recharge,
        //         StatName::DEFPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::GeoBonus,
        //         StatName::DEFPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::DEFPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::HuskOfOpulentDreams,
        //         ArtifactSetName::NoblesseOblige,
        //         ArtifactSetName::DefendersWill,
        //         ArtifactSetName::EmblemOfSeveredFate,
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
            .husk_of_opulent_dreams(4.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, _character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let def = attribute.get_value(AttributeName::DEF);
        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand);

        r * def
    }
}
