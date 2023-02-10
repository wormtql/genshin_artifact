use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{Character, CharacterName};
use crate::character::characters::Bennett;
use crate::character::prelude::CharacterTrait;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct BennettDamageTargetFunction {
    pub recharge_demand: f64,
    pub other_dmg_ratio: f64,
}

impl TargetFunctionMetaTrait for BennettDamageTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::BennettDamage,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "班尼特-副C",
            en: "Bennett-Sub DPS"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通副C班尼特",
            en: "Sub DPS Bennett"
        ),
        tags: "辅助,输出,副C",
        four: TargetFunctionFor::SomeWho(CharacterName::Bennett),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: crate::common::i18n::locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement",
            ),
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.8 },
        },
        ItemConfig {
            name: "other_dmg_ratio",
            title: crate::common::i18n::locale!(
                zh_cn: "他人伤害比例",
                en: "Other's DMG Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.9 },
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (recharge_demand, other_dmg_ratio) = match *config {
            TargetFunctionConfig::BennettDamage { recharge_demand, other_dmg_ratio } => (recharge_demand, other_dmg_ratio),
            _ => (1.0, 0.0)
        };

        Box::new(BennettDamageTargetFunction {
            recharge_demand,
            other_dmg_ratio
        })
    }
}

impl TargetFunction for BennettDamageTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.8,
        //     elemental_mastery: 0.0,
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
        //         StatName::Recharge,
        //         StatName::ATKPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::PyroBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalDamage,
        //         StatName::CriticalRate,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::CrimsonWitchOfFlames,
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence,
        //     ]),
        //     very_critical_set_names: Some(vec![
        //         ArtifactSetName::NoblesseOblige
        //     ]),
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: 0.0,
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .crimson_witch_of_flames(1.0)
            .noblesse_oblige(1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let bonus_for_other = attribute.get_value(AttributeName::ATKBonusForOther);
        // if bonus_for_other > 1e-6 {
        //     crate::utils::log!("666 {}", bonus_for_other);
        // }
        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand) / self.recharge_demand;

        const VIRTUAL_BASE_ATK: f64 = 800.0;
        let mut atk_bonus = Bennett::atk_bonus(&character.common_data, attribute) * r + bonus_for_other * VIRTUAL_BASE_ATK;

        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        type S = <Bennett as CharacterTrait>::DamageEnumType;
        let dmg_q = Bennett::damage::<SimpleDamageBuilder>(&context, S::Q1, &CharacterSkillConfig::NoConfig, None).normal.expectation;

        let d = r * dmg_q * (1.0 - self.other_dmg_ratio) + (atk_bonus + VIRTUAL_BASE_ATK) / VIRTUAL_BASE_ATK * 20000.0 * self.other_dmg_ratio;

        d
    }
}
