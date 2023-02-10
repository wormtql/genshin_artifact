extern crate web_sys;

use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, character_common_data, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::anemo::wanderer::Wanderer;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::damage_result::SimpleDamageResult;
use crate::enemies::Enemy;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_name::WeaponName;

// A target function for Wanderer by Cor
// if you have any question or suggestions about this file, feel free to email 736674365@qq.com or corinthusyu@gmail.com

pub struct WandererDefaultTargetFunction {
    pub e_hydro: bool,
    pub e_pyro: bool,
    pub e_cryo: bool,
    pub spd_extra: f64,
    pub spd_comp: f64,
    pub dash_count: usize,
    pub q_count: usize,
    pub swirl_count: usize,
}

impl WandererDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (
            e_hydro,
            e_pyro,
            e_cryo,
            spd_extra,
            spd_comp,
            dash_count,
            q_count,
            swirl_count,
        ) = match *config {
            TargetFunctionConfig::WandererDefault {
                e_hydro,
                e_pyro,
                e_cryo,
                spd_extra,
                spd_comp,
                dash_count,
                q_count,
                swirl_count
            } =>
                (
                    e_hydro,
                    e_pyro,
                    e_cryo,
                    spd_extra,
                    spd_comp,
                    dash_count,
                    q_count,
                    swirl_count,
                ),
            _ => (false, false, false, 0.0, 0.0, 0, 0, 0)
        };

        Self {
            e_hydro,
            e_pyro,
            e_cryo,
            spd_extra,
            spd_comp,
            dash_count,
            q_count,
            swirl_count,
        }
    }
}

impl TargetFunctionMetaTrait for WandererDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::WandererDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "流浪者-久世浮倾",
            en: "Wanderer-Eons Adrift"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "计算一轮内流浪者造成伤害的总和",
            en: "Maximize Wanderer's total damage in one cycle"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Wanderer),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_hydro",
            title: locale!(
                zh_cn: "「拾玉得花」染水",
                en: "Jade-Claimed Flower Hydro"
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "e_pyro",
            title: locale!(
                zh_cn: "「拾玉得花」染火",
                en: "Jade-Claimed Flower Pyro",
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "e_cryo",
            title: locale!(
                zh_cn: "「拾玉得花」染冰",
                en: "Jade-Claimed Flower Cryo"
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "spd_extra",
            title: locale!(
                zh_cn: "额外攻速（不算自身和专武）",
                en: "Addtional Normal SPD (exclude self and weapon)",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "spd_comp",
            title: locale!(
                zh_cn: "攻速伤害修正",
                en: "SPD-DMG compensation"
            ),
            config: ItemConfigType::Float { min: 0.5, max: 1.5, default: 1.0 },
        },
        ItemConfig {
            name: "dash_count",
            title: locale!(
                zh_cn: "「梦迹一风」风矢命中数",
                en: "Gales of Reverie Wind Arrow Hits",
            ),
            config: ItemConfigType::Int { min: 0, max: 12, default: 3 },
        },
        ItemConfig {
            name: "q_count",
            title: locale!(
                zh_cn: "Q命中数",
                en: "Q Hits",
            ),
            config: ItemConfigType::Int { min: 0, max: 5, default: 5 },
        },
        ItemConfig {
            name: "swirl_count",
            title: locale!(
                zh_cn: "扩散命中数",
                en: "Swirl Hits",
            ),
            config: ItemConfigType::Int { min: 0, max: 24, default: 12 },
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(WandererDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for WandererDefaultTargetFunction {
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
        //     bonus_pyro: 1.0,
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
        //         StatName::AnemoBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::EchoesOfAnOffering,
        //         ArtifactSetName::DesertPavilionChronicle,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD,
        // }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .shimenawas_reminiscence(1.0)
            .echoes_of_an_offering_avg()
            .desert_pavilion_chronicle(1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };

        type S = <Wanderer as CharacterTrait>::DamageEnumType;

        let config = CharacterSkillConfig::Wanderer {
            e_enabled: true,
            e_hydro: self.e_hydro,
            sdpoints: 0.0,
        };

        // calc attack spd
        let mut spd_na = 1.0;
        let mut dc_count = 0;
        for a in artifacts
        {
            if a.set_name == ArtifactSetName::DesertPavilionChronicle {
                dc_count += 1;
            }
        }
        spd_na += if weapon.common_data.name == WeaponName::TulaytullahsRemembrance { (0.02 * weapon.common_data.refine as f64) + 0.08 } else { 0.0 }
            + if character.common_data.constellation >= 1 { 0.1 } else { 0.0 }
            + if dc_count >= 4 { 0.1 } else { 0.0 }
            + self.spd_extra;


        let dmg_normal1 = Wanderer::damage::<SimpleDamageBuilder>(&context, S::Normal1, &config, None);
        let dmg_normal2 = Wanderer::damage::<SimpleDamageBuilder>(&context, S::Normal2, &config, None);
        let dmg_normal3 = Wanderer::damage::<SimpleDamageBuilder>(&context, S::Normal3, &config, None);
        let dmg_charged = Wanderer::damage::<SimpleDamageBuilder>(&context, S::Charged1, &config, None);
        let dmg_e = Wanderer::damage::<SimpleDamageBuilder>(&context, S::E1, &config, None);
        let dmg_q = Wanderer::damage::<SimpleDamageBuilder>(&context, S::Q1, &config, None);
        let dmg_d = Wanderer::damage::<SimpleDamageBuilder>(&context, S::Dash1, &config, None);
        let dmg_swirl = context.transformative().swirl_pyro;

        let mut dmg_c6 = 0.0;
        if context.character_common_data.constellation == 6 {
            let dmg_n1_c6 = Wanderer::damage::<SimpleDamageBuilder>(&context, S::Normal1C6, &config, None);
            let dmg_n2_c6 = Wanderer::damage::<SimpleDamageBuilder>(&context, S::Normal2C6, &config, None);
            let dmg_n3_c6 = Wanderer::damage::<SimpleDamageBuilder>(&context, S::Normal3C6, &config, None);
            dmg_c6 = dmg_n1_c6.normal.expectation
                + dmg_n2_c6.normal.expectation
                + dmg_n3_c6.normal.expectation * 2.0;
        };

        (dmg_normal1.normal.expectation
            + dmg_normal2.normal.expectation
            + dmg_normal3.normal.expectation * 2.0
            + dmg_c6) * spd_na * self.spd_comp * (if dc_count >= 4 { 5.0 } else { 5.5 })
            + (if dc_count >= 4 { dmg_charged.normal.expectation } else { 0.0 })
            + dmg_e.normal.expectation * 1.0
            + dmg_q.normal.expectation * self.q_count as f64
            + dmg_d.normal.expectation * self.dash_count as f64
            + dmg_swirl * self.swirl_count as f64
    }
}
