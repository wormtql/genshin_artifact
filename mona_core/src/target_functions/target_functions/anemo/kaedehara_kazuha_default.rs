use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2, AttributeCommon};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct KaedeharaKazuhaDefaultTargetFunction {
    pub recharge_demand: f64
}

impl TargetFunctionMetaTrait for KaedeharaKazuhaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::KaedeharaKazuhaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "枫原万叶-红叶逐荒波",
            en: "Kazuha-Scarlet Leaves Pursue Wild Waves"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通辅助万叶",
            en: "Support Kazuha"
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::KaedeharaKazuha),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement",
            ),
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.8 },
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let recharge_demand = match *config {
            TargetFunctionConfig::KaedeharaKazuhaDefault { recharge_demand } => recharge_demand,
            _ => 1.0
        };
        Box::new(KaedeharaKazuhaDefaultTargetFunction {
            recharge_demand
        })
    }
}

impl TargetFunction for KaedeharaKazuhaDefaultTargetFunction {
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
        //         StatName::ElementalMastery,
        //         StatName::Recharge,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::ElementalMastery,
        //     ],
        //     head_main_stats: vec![
        //         StatName::ElementalMastery,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::ViridescentVenerer,
        //         ArtifactSetName::WanderersTroupe,
        //         ArtifactSetName::Instructor
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

    fn target(&self, attribute: &SimpleAttributeGraph2, _character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let mut vv_count = 0;
        for artifact in artifacts.iter() {
            if artifact.set_name == ArtifactSetName::ViridescentVenerer {
                vv_count += 1;
            }
        }

        let vv_ratio = if vv_count >= 4 { 1.167 } else { 1.0 };

        // let em = attribute.get_value(AttributeName::ElementalMastery);
        let em = attribute.get_em_all();
        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand);
        r * (1.0 + (em * 0.0004) / 1.5) * vv_ratio
    }
}
