use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::electro::keqing::Keqing;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
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

pub struct KeqingDefaultTargetFunction {
    pub aggravate_rate: f64,
}

impl TargetFunctionMetaTrait for KeqingDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::KeqingDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "刻晴-霆霓快雨",
            en: "Keqing-Driving Thunder"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通雷伤刻晴",
            en: "Electro DPS Keqing"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Keqing),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "aggravate_rate",
            title: locale!(
                zh_cn: "超激化比例",
                en: "Aggravate Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let rate = match *config {
            TargetFunctionConfig::KeqingDefault { aggravate_rate } => aggravate_rate,
            _ => 0.0
        };
        Box::new(KeqingDefaultTargetFunction {
            aggravate_rate: rate.clamp(0.0, 1.0)
        })
    }
}

impl TargetFunction for KeqingDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
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
        //     bonus_electro: 2.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::ElectroBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::EmblemOfSeveredFate,
        //         ArtifactSetName::NoblesseOblige,
        //         ArtifactSetName::Thundersoother,
        //         ArtifactSetName::ThunderingFury,
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
            .thundersoother(1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        let config = CharacterSkillConfig::Keqing { after_e: true };
        type S = <Keqing as CharacterTrait>::DamageEnumType;
        let dmg_q = Keqing::damage::<SimpleDamageBuilder>(&context, S::Q2, &config, None);
        let dmg_charged = Keqing::damage::<SimpleDamageBuilder>(&context, S::Charged11, &config, None);

        // let recharge = attribute.get_value(AttributeName::Recharge);
        // let r = recharge.min(1.4);

        // r * dmg_q * 19.53 * 0.2 + dmg_charged * 2.094 * 0.8
        let normal = dmg_q.normal.expectation * 19.53 * 0.2 + dmg_charged.normal.expectation * 2.094 * 0.8;
        let aggravate = dmg_q.aggravate.unwrap().expectation * 19.53 * 0.2 + dmg_charged.aggravate.unwrap().expectation * 2.094 * 0.8;

        normal * (1.0 - self.aggravate_rate) + aggravate * self.aggravate_rate
    }
}
