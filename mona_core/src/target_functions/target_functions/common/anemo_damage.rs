use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{SimpleAttributeGraph2, AttributeCommon, Attribute, AttributeName};
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct AnemoDamageTargetFunction {
    pub t: usize,
}

impl TargetFunctionMetaTrait for AnemoDamageTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::AnemoDamage,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "风伤",
            en: "Anemo DMG"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "风元素伤害最大化或最大化期望",
            en: "Maximize Crit or Avg Anemo Damage"
        ),
        tags: "输出",
        four: TargetFunctionFor::Common,
        image: TargetFunctionMetaImage::Custom("misc/wind_slime")
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "t",
            title: locale!(
                zh_cn: "类型",
                en: "Type"
            ),
            config: ItemConfigType::Option {
                options: "期望,最大值",
                default: 0
            }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let t = match *config {
            TargetFunctionConfig::AnemoDamage { t } => t,
            _ => 0
        };

        Box::new(AnemoDamageTargetFunction {
            t
        })
    }
}

impl TargetFunction for AnemoDamageTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.0,
        //     elemental_mastery: 0.0,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 1.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::AnemoBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::ViridescentVenerer,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::GladiatorsFinale,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, _character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let atk = attribute.get_atk();
        let crit = attribute.get_value(AttributeName::CriticalBase) + attribute.get_value(AttributeName::CriticalAnemo)
            + attribute.get_value(AttributeName::CriticalAttacking);
        let crit = crit.clamp(0.0, 1.0);

        let critical_damage = attribute.get_value(AttributeName::CriticalDamageBase) + attribute.get_value(AttributeName::CriticalDamageAnemo);

        let bonus = attribute.get_value(AttributeName::BonusBase) + attribute.get_value(AttributeName::BonusAnemo);

        if self.t == 0 {
            atk * (1.0 + crit * critical_damage) * (1.0 + bonus)
        } else {
            atk * (1.0 + critical_damage) * (1.0 + bonus)
        }
    }
}
