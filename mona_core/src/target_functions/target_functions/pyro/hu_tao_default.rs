use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigArchaicPetra, ConfigLevel, ConfigRate};
use crate::attribute::{SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::{HuTao};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct HuTaoDefaultTargetFunction {
    pub vaporize_rate: f64,
    pub melt_rate: f64,
}

impl HuTaoDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> HuTaoDefaultTargetFunction {
        let (vaporize_rate, melt_rate) = match *config {
            TargetFunctionConfig::HuTaoDefault { vaporize_rate, melt_rate } => (vaporize_rate, melt_rate),
            _ => (0.0, 0.0)
        };
        HuTaoDefaultTargetFunction {
            vaporize_rate, melt_rate
        }
    }
}

impl TargetFunctionMetaTrait for HuTaoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::HuTaoDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "胡桃-雪霁梅香",
            en: "Hutao-Fragrance in Thaw"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出主C胡桃",
            en: "Main DPS Hutao"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::HuTao),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "vaporize_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "蒸发占比",
                en: "Vaporize Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 }
        },
        ItemConfig {
            name: "melt_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "融化占比",
                en: "Melt Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(HuTaoDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for HuTaoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.05,
        //     atk_percentage: 0.5,
        //     hp_fixed: 0.1,
        //     hp_percentage: 1.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.1,
        //     elemental_mastery: self.vaporize_rate,
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
        //         StatName::ATKPercentage,
        //         StatName::ElementalMastery,
        //         StatName::HPPercentage
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::PyroBonus,
        //         StatName::ATKPercentage,
        //         StatName::ElementalMastery,
        //         StatName::HPPercentage
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ElementalMastery,
        //         StatName::HPPercentage
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::RetracingBolide,
        //         ArtifactSetName::CrimsonWitchOfFlames
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
            .crimson_witch_of_flames(1.0)
            .shimenawas_reminiscence(1.0)
            .echoes_of_an_offering_avg()
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context = DamageContext {
            character_common_data: &character.common_data,
            enemy,
            attribute
        };

        type S = <HuTao as CharacterTrait>::DamageEnumType;
        let damage_charged = HuTao::damage::<SimpleDamageBuilder>(
            &context, S::Charged, &CharacterSkillConfig::HuTao { after_e: true }, None,
        );

        let normal = 0.0_f64.max(1.0 - self.melt_rate - self.vaporize_rate);

        let normal_dmg = damage_charged.normal.expectation;
        let vaporize = damage_charged.vaporize.unwrap().expectation;
        let melt = damage_charged.melt.unwrap().expectation;

        normal * normal_dmg + self.vaporize_rate * vaporize + self.melt_rate * melt
    }
}
