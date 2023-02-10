use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigArchaicPetra, ConfigLevel, ConfigRate};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Diluc;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::common::item_config_type::ItemConfig;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct DilucDefaultTargetFunction {
    pub melt_rate: f64,
    pub vaporize_rate: f64,
}

impl TargetFunctionMetaTrait for DilucDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::DilucDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "迪卢克-晨曦酒庄的贵公子",
            en: "Diluc-Default"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出迪卢克",
            en: "DPS Diluc"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Diluc),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "melt_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "融化占比",
                en: "Melt Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "vaporize_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "蒸发占比",
                en: "Vaporize Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (mut melt_rate, mut vaporize_rate) = match *config {
            TargetFunctionConfig::DilucDefault { melt_rate, vaporize_rate } => (melt_rate, vaporize_rate),
            _ => (0.0, 0.0)
        };

        let sum = melt_rate + vaporize_rate;
        if sum > 1.0 {
            melt_rate /= sum;
            vaporize_rate /= sum;
        }

        Box::new(DilucDefaultTargetFunction {
            melt_rate, vaporize_rate
        })
    }
}

impl TargetFunction for DilucDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // let reaction_rate = (self.vaporize_rate + self.melt_rate).clamp(0.2, 1.0);
        //
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.0,
        //     elemental_mastery: reaction_rate,
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
        //         StatName::ElementalMastery
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::PyroBonus,
        //         StatName::ATKPercentage
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::CrimsonWitchOfFlames,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .crimson_witch_of_flames(3.0)
            .echoes_of_an_offering_avg()
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        const CONFIG: CharacterSkillConfig = CharacterSkillConfig::Diluc { pyro: true };
        type S = <Diluc as CharacterTrait>::DamageEnumType;
        let dmg_e = Diluc::damage::<SimpleDamageBuilder>(
            &context, S::E1, &CONFIG, None
        );

        let e_normal = dmg_e.normal.expectation;
        let e_melt = dmg_e.melt.unwrap().expectation;
        let e_vaporize = dmg_e.vaporize.unwrap().expectation;

        let dmg_q = Diluc::damage::<SimpleDamageBuilder>(
            &context, S::Q1, &CONFIG, None
        );

        let q_normal = dmg_q.normal.expectation;
        let q_melt = dmg_q.melt.unwrap().expectation;
        let q_vaporize = dmg_q.vaporize.unwrap().expectation;

        let normal_rate = (1.0 - self.vaporize_rate - self.melt_rate).clamp(0.0, 1.0);
        let e = self.melt_rate * e_melt + self.vaporize_rate * e_vaporize + normal_rate * e_normal;
        let q = self.melt_rate * q_melt + self.vaporize_rate * q_vaporize + normal_rate * q_normal;

        e * 3.4 + q * 2.5
    }
}
