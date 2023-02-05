use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2, AttributeCommon};
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

pub struct SucroseDefaultTargetFunction {
    pub recharge_demand: f64
}

impl SucroseDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let recharge_demand = match *config {
            TargetFunctionConfig::SucroseDefault { recharge_demand } => recharge_demand,
            _ => 1.0
        };

        SucroseDefaultTargetFunction {
            recharge_demand
        }
    }
}

impl TargetFunctionMetaTrait for SucroseDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::SucroseDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "砂糖-无害甜度",
            en: "Sucrose-Harmless Sweetie"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通辅助砂糖",
            en: "Support Sucrose"
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Sucrose),
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
        Box::new(SucroseDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for SucroseDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 0.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 1.0,
        //     elemental_mastery: 1.0,
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
        //         StatName::Recharge,
        //         StatName::ElementalMastery,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::ElementalMastery,
        //     ],
        //     head_main_stats: vec![
        //         StatName::ElementalMastery,
        //     ],
        //     set_names: Some(vec![
        //     ]),
        //     very_critical_set_names: Some(vec![
        //         ArtifactSetName::ViridescentVenerer
        //     ]),
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, _character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let mut vv_count = 0;
        for art in artifacts.iter() {
            if art.set_name == ArtifactSetName::ViridescentVenerer {
                vv_count += 1;
            }
        }

        let vv = if vv_count >= 4 {
            1.278
        } else {
            1.0
        };
        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand);

        // let em = attribute.get_value(AttributeName::ElementalMastery);
        let em = attribute.get_em_all();

        vv * r * em
    }
}
