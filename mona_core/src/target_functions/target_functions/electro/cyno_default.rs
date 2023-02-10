extern crate web_sys;

use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, character_common_data, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::electro::cyno::Cyno;
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


// A target function for Cyno by Cor
// if you have any question or suggestions about this file, feel free to email 736674365@qq.com or corinthusyu@gmail.com

pub struct CynoDefaultTargetFunction {
    pub recharge_requirement: f64,
    pub combo: usize,
    pub until_expire: bool,
    pub aggravate_rate: f64,
    pub elecharged_rate: f64,
    pub overload_rate: f64,
    pub hyperbloom_rate: f64,
}

impl CynoDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (
            recharge_requirement,
            combo,
            until_expire,
            aggravate_rate,
            elecharged_rate,
            overload_rate,
            hyperbloom_rate
        ) = match *config {
            TargetFunctionConfig::CynoDefault {
                recharge_requirement,
                combo,
                until_expire,
                aggravate_rate,
                elecharged_rate,
                overload_rate,
                hyperbloom_rate
            } =>
                (
                    recharge_requirement,
                    combo,
                    until_expire,
                    aggravate_rate,
                    elecharged_rate,
                    overload_rate,
                    hyperbloom_rate
                ),
            _ => (0.0, 0, false, 0.0, 0.0, 0.0, 0.0)
        };

        Self {
            recharge_requirement,
            combo,
            until_expire,
            aggravate_rate,
            elecharged_rate,
            overload_rate,
            hyperbloom_rate,
        }
    }
}

impl TargetFunctionMetaTrait for CynoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::CynoDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "赛诺-缄秘的裁谴",
            en: "Cyno-Judicator of Secrets"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "打QTE并释放渡荒之雷，会根据套装自动选择手法，如激化草全覆盖则激化比例留1.0即可",
            en: "Perform QTE and unleash Duststalker Bolts, best combo would be applied automatically basing on artifact set"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Cyno),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_requirement",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.3 },
        },
        ItemConfig {
            name: "combo",
            title: locale!(
                zh_cn: "连招选择",
                en: "Combo",
            ), //连招选择
            config: ItemConfigType::Option { options: "乱a不取消,取消第五段", default: 0 },
        },
        ItemConfig {
            name: "until_expire",
            title: locale!(
                zh_cn: "a到大招完全结束",
                en: "Attack Till Burst Expires",
            ), //a到大招完全结束
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "aggravate_rate",
            title: locale!(
                zh_cn: "超激化比例",
                en: "Aggravate Ratio",
            ), //超激化比例
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "elecharged_rate",
            title: locale!(
                zh_cn: "感电比例",
                en: "Electro-charge Ratio"
            ), //感电比例
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "overload_rate",
            title: locale!(
                zh_cn: "超载比例",
                en: "Overload Ratio",
            ), //超载比例
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "hyperbloom_rate",
            title: locale!(
                zh_cn: "超绽放比例",
                en: "Hyperbloom Ratio",
            ), //超绽放比例
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
    ]);
    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(CynoDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for CynoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
        // let normal_rate = (1.0 - self.aggravate_rate).max(0.0);
        // let em_weight = if normal_rate > 0.8 { 0.0 } else { 1.0 };
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.0,
        //     elemental_mastery: em_weight,
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
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::ElectroBonus,
        //         StatName::ATKPercentage,
        //         StatName::ElementalMastery,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::GildedDreams,
        //         ArtifactSetName::ThunderingFury,
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::Thundersoother,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::EchoesOfAnOffering,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .shimenawas_reminiscence(0.35)
            .thundersoother(1.0)
            .echoes_of_an_offering_avg()
            .gilded_dreams(1, 2, 1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };
        type S = <Cyno as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::Cyno { under_judication: true };
        let dmg_normal1: SimpleDamageResult = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal1, &config, None);
        let dmg_normal2 = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal2, &config, None);
        let dmg_normal3 = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal3, &config, None);
        let dmg_normal4 = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal4, &config, None);
        let dmg_normal5 = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal5, &config, None);
        let dmg_e2 = Cyno::damage::<SimpleDamageBuilder>(&context, S::E2, &config, None);
        let dmg_e3 = Cyno::damage::<SimpleDamageBuilder>(&context, S::E3, &config, None);
        let config_tf = CharacterSkillConfig::Cyno { under_judication: false };
        let dmg_e2_noqte = Cyno::damage::<SimpleDamageBuilder>(&context, S::E2, &config_tf, None);

        let mut tf_count = 0;
        for a in artifacts.iter() {
            if a.set_name == ArtifactSetName::ThunderingFury {
                tf_count += 1;
            }
        }
        let is_thunderingfury = tf_count >= 4;

        // auto combo selecton
        // | non-tf |  C0   |  C1   |
        // | combo0 | 5a+1a | 5a+3a | 
        // | combo1 | 3ad4a | 4ad5a |
        // 
        // |   tf   |   C0   |   C1   |
        // | combo0 | 3ae3aE | 3ae4aE | 
        // | combo1 | 3ae3aE | 4ae4aE |
        // 

        let normal1_normal = dmg_normal1.normal.expectation;
        let normal2_normal = dmg_normal2.normal.expectation;
        let normal3_normal = dmg_normal3.normal.expectation;
        let normal4_normal = dmg_normal4.normal.expectation;
        let normal5_normal = dmg_normal5.normal.expectation;
        let e2_normal = dmg_e2.normal.expectation;
        let e3_normal = dmg_e3.normal.expectation;
        let e2_noqte_normal = dmg_e2_noqte.normal.expectation;

        let normal1_agg = dmg_normal1.aggravate.unwrap().expectation;
        let e2_agg = dmg_e2.aggravate.unwrap().expectation;
        let e2_noqte_agg = dmg_e2_noqte.aggravate.unwrap().expectation;
        let e3_agg = dmg_e3.aggravate.unwrap().expectation;

        let agg_bonus_normal = normal1_agg - normal1_normal;
        let agg_bonus_e2 = e2_agg - e2_normal;
        let agg_bonus_e3 = e3_agg - e3_normal;
        let agg_bonus_e2_noqte = e2_noqte_agg - e2_noqte_normal;

        let rounds_count:f64 = if self.until_expire { 4.0 } else { 3.0 };

        //let s = format!("{}",agg_bonus);
        //web_sys::console::log_1(&s.into());
        let mut dmg_electro_charged = 0.0;
        let mut dmg_overload = 0.0;
        let mut dmg_hyperbloom = 0.0;
        let ec_count = 2.5 * (rounds_count + 1.0); // triggers one more time electro-charged after q expires
        let ol_count = 4.0 * rounds_count.max(3.0); // can't trigger more than 3 rounds of overload cuz even xiangling's q couldnt last that long
        let hb_count = 5.0 * rounds_count; // seeds produced 5 at maximum every round

        //if transformative > 0: calc transformative dmg
        if self.elecharged_rate > 0.0 && self.overload_rate > 0.0 && self.hyperbloom_rate > 0.0 {
            let transformative = context.transformative();
            dmg_electro_charged = transformative.electro_charged;
            dmg_overload = transformative.overload;
            dmg_hyperbloom = transformative.hyperbloom;
        }

        let mut dmgsum_normal = e2_noqte_normal + (e2_normal + e3_normal * 3.0 + (if is_thunderingfury { e2_noqte_normal } else { 0.0 })) * rounds_count;
        let mut dmgsum_agg = agg_bonus_e2_noqte + (agg_bonus_e2 + agg_bonus_e3 + (if is_thunderingfury { agg_bonus_e2_noqte } else { 0.0 })) * rounds_count;

        if character.common_data.constellation == 0 {
            dmgsum_normal += match (is_thunderingfury, self.combo, self.until_expire) {
                (false, 0, true) => (normal1_normal + normal2_normal + normal3_normal + normal4_normal * 2.0) * 5.0 + normal5_normal * 4.0 + normal1_normal, //qe 5a+1aE 5a+1aE 5a+1aE 5a+1aE 4a
                (false, 0, false) => (normal1_normal * 2.0 + normal2_normal + normal3_normal + normal4_normal * 2.0 + normal5_normal) * 3.0 + normal1_normal, //qe 5a+1aE 5a+1aE 5a+1aE
                (false, 1, true) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0 * 1.0) * 4.0 + normal1_normal + normal2_normal, //qe 3ad4aE 3ad4aE 3ad4aE 3ad4aE 2a
                (false, 1, false) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0 * 1.0) * 3.0, //qe 3ad4aE 3ad4aE 3ad4aE
                (true, 0, true) | (true, 1, true) => e2_noqte_normal * 4.0 + (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0) * 4.0 + normal1_normal + normal2_normal, //qe 3ae3aE 3ae3aE 3ae3aE 3ae3aE 2a
                (true, 0, false) | (true, 1, false) => e2_noqte_normal * 3.0 + (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0) * 3.0, //qe 3ae3aE 3ae3aE 3ae3aE
                _ => 0.0,
            };
            if self.aggravate_rate > 0.0 {
                dmgsum_agg += agg_bonus_normal * match (is_thunderingfury, self.combo, self.until_expire) {
                    (false, 0, true) => 3.0 * 4.0 + 1.0, //qe 5a+1aE 5a+1aE 5a+1aE 5a+1aE 4a
                    (false, 0, false) => 3.0 * 3.0, //qe 5a+1aE 5a+1aE 5a+1aE
                    (false, 1, true) => 3.0 * 4.0, //qe 3ad4aE 3ad4aE 3ad4aE 3ad4aE 2a
                    (false, 1, false) => 3.0 * 3.0, //qe 3ad4aE 3ad4aE 3ad4aE
                    (true, 0, true) | (true, 1, true) => 2.25 * 4.0, //qe 3ae3aE 3ae3aE 3ae3aE 3ae3aE 2a
                    (true, 0, false) | (true, 1, false) => 2.33 * 3.0, //qe 3ae3aE 3ae3aE 3ae3aE
                    _ => 0.0,
                };
            }
        } else {
            dmgsum_normal += match (is_thunderingfury, self.combo, self.until_expire) {
                (true, 0, true) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0) * 4.0 + normal5_normal * 2.0, // qe 3ae4aE 3ae4aE 3ae4aE 3ae4aE plus a5*2
                (true, 0, false) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0) * 3.0 + normal5_normal * 1.33, // qe 3ae4aE 3ae4aE 3ae4aE plus a5*1.33
                (true, 1, true) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0 * 2.0) * 4.0 + normal1_normal + normal2_normal, // qe 4ae4aE 4ae4aE 4ae4aE 4ae4aE 2a
                (true, 1, false) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0 * 2.0) * 3.0, // qe 4ae4aE 4ae4aE 4ae4aE
                (false, 0, true) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0 * 1.0 + normal5_normal * 1.0) * 4.0 + normal1_normal + normal2_normal, // qe 5a+3aE 5a+3aE 5a+3aE 5a+3aE 2a
                (false, 0, false) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0 * 1.0 + normal5_normal * 1.0) * 3.0, // qe 5a+3aE 5a+3aE 5a+3aE
                (false, 1, true) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0 * 2.0 + normal5_normal * 1.0) * 4.0,// qe 4ad5aE 4ad5aE 4ad5aE 4ad5aE
                (false, 1, false) => (normal1_normal * 2.0 + normal2_normal * 2.0 + normal3_normal * 2.0 + normal4_normal * 2.0 * 2.0 + normal5_normal * 1.0) * 3.0, // qe 4ad5aE 4ad5aE 4ad5aE
                _ => 0.0
            };
            if self.aggravate_rate > 0.0 {
                dmgsum_agg += agg_bonus_normal * match (is_thunderingfury, self.combo, self.until_expire) {
                    (true, 0, true) => 3.0 * 4.0 + 1.0, // qe 3ae4aE 3ae4aE 3ae4aE 3ae4aE plus a5*2
                    (true, 0, false) => 3.0 * 3.0 + 1.33, // qe 3ae4aE 3ae4aE 3ae4aE plus a5*1.33
                    (true, 1, true) => 4.0 * 4.0, // qe 4ae4aE 4ae4aE 4ae4aE 4ae4aE 2a
                    (true, 1, false) => 4.0 * 3.0, // qe 4ae4aE 4ae4aE 4ae4aE
                    (false, 0, true) => 3.0 * 4.0 + 1.0, // qe 5a+3aE 5a+3aE 5a+3aE 5a+3aE 2a
                    (false, 0, false) => 3.0 * 3.0 + 0.33, // qe 5a+3aE 5a+3aE 5a+3aE
                    (false, 1, true) => 4.0 * 4.0,// qe 4ad5aE 4ad5aE 4ad5aE 4ad5aE
                    (false, 1, false) => 4.0 * 3.0, // qe 4ad5aE 4ad5aE 4ad5aE
                    _ => 0.0,
                };
            }
        }

        if character.common_data.constellation == 6 {
            dmgsum_normal += e3_normal * 4.0 * rounds_count;
            if self.aggravate_rate > 0.0 {
                dmgsum_agg += agg_bonus_e3 * 2.0 * rounds_count;
            }
        }
        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_requirement);
        r * (
              dmgsum_normal + dmgsum_agg * self.aggravate_rate 
            + dmg_electro_charged * ec_count * self.elecharged_rate 
            + dmg_overload * ol_count * self.overload_rate 
            + dmg_hyperbloom * hb_count
        )
    }
}
