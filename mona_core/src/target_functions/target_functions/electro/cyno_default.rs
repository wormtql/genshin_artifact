use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::cyno::Cyno;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;
extern crate web_sys;

// A target function for Cyno by Cor
// if you have any question or suggestions about this file, feel free to email 736674365@qq.com or corinthusyu@gmail.com

pub struct CynoDefaultTargetFunction {
    pub recharge_requirement:f64,
    pub hit_within_qte:f64,
    pub reaction_times:f64,
    pub extra_bolts:f64,
    pub aggravate_rate: f64,
    pub elecharged_rate:f64,
    pub overload_rate:f64
}

impl CynoDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (
                recharge_requirement,
                hit_within_qte,
                reaction_times,
                extra_bolts,
                aggravate_rate,
                elecharged_rate,
                overload_rate
            ) = match *config {
                    TargetFunctionConfig::CynoDefault {
                                                            recharge_requirement,
                                                            hit_within_qte,
                                                            reaction_times,
                                                            extra_bolts,
                                                            aggravate_rate,
                                                            elecharged_rate,
                                                            overload_rate  
                                                        } => 
                                                        (
                                                            recharge_requirement,
                                                            hit_within_qte,
                                                            reaction_times,
                                                            extra_bolts,
                                                            aggravate_rate,
                                                            elecharged_rate,
                                                            overload_rate
                                                        ),
            _ => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        };

        Self {
            recharge_requirement,
            hit_within_qte,
            reaction_times,
            extra_bolts,
            aggravate_rate,
            elecharged_rate,
            overload_rate
        }
    }
}

impl TargetFunctionMetaTrait for CynoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::CynoDefault,
        chs: "赛诺-激化",
        description: "打QTE并释放渡荒之雷,普攻命中次数、反应触发次数和6命参考数据：零命 7.0 5.0 0.0，，一命 9.0 5.0 0.0，六命 9.0 5.0 4.0 ",
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Cyno),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_requirement",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min:1.0, max: 3.0 , default:1.3 }
        },
        ItemConfig {
            name: "hit_within_qte",
            title: "t18", //一轮QTE内普通攻击命中次数
            config: ItemConfigType::Float { min: 0.0, max: 15.0, default: 7.0 }
        },
        ItemConfig {
            name: "reaction_times",
            title: "t19", //一轮QTE内触发反应次数
            config: ItemConfigType::Float { min: 0.0, max: 10.0 , default: 5.0 }
        },
        ItemConfig {
            name: "extra_bolts",
            title: "t22", //6命额外渡荒之雷
            config: ItemConfigType::Float { min: 0.0, max: 4.0 , default: 0.0 }
        },
        ItemConfig {
            name: "aggravate_rate",
            title: "t17", //超激化比例
            config: ItemConfigType::Float { min:0.0, max: 1.0 , default:1.0}
        },
        ItemConfig {
            name: "elecharged_rate",
            title: "t20", //感电比例
            config: ItemConfigType::Float { min: 0.0, max: 1.0 , default: 0.0}
        },
        ItemConfig {
            name: "overload_rate",
            title: "t21", //超载比例
            config: ItemConfigType::Float { min: 0.0, max: 1.0 , default:0.0}
        },
    ]);
    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(CynoDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for CynoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        let normal_rate = (1.0 - self.aggravate_rate).max(0.0);
        let em_weight = if normal_rate > 0.8 { 0.0 } else { 1.0 };
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.0,
            elemental_mastery: em_weight,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_electro: 0.0,
            bonus_pyro: 2.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::ElementalMastery,
            ],
            goblet_main_stats: vec![
                StatName::ElectroBonus,
                StatName::ATKPercentage,
                StatName::ElementalMastery,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::GildedDreams,
                ArtifactSetName::ThunderingFury,
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::Thundersoother,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::EchoesOfAnOffering,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }


    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
        .shimenawas_reminiscence(0.35)
        .thundersoother(1.0)
        .echoes_of_an_offering_avg()
        .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };
        //Cyno::SKILL.normal_dmg1[]
        type S = <Cyno as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::Cyno { under_judication: true };
        let dmg_normal1 = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal1, &config, None);
        let dmg_normal2 = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal2, &config, None);
        let dmg_normal3 = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal3, &config, None);
        let dmg_normal4 = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal4, &config, None);
        let dmg_normal5 = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal5, &config, None);
        let dmg_e2 = Cyno::damage::<SimpleDamageBuilder>(&context, S::E2, &config, None);
        let dmg_e3=Cyno::damage::<SimpleDamageBuilder>(&context, S::E3, &config, None);

        
        let normal1_normal = dmg_normal1.normal.expectation;
        let normal2_normal = dmg_normal2.normal.expectation;
        let normal3_normal = dmg_normal3.normal.expectation;
        let normal4_normal = dmg_normal4.normal.expectation;
        let normal5_normal = dmg_normal5.normal.expectation;

        let e2_normal = dmg_e2.normal.expectation;
        let e3_normal = dmg_e3.normal.expectation;

        let mut e2_agg=0.0;
        let mut normal1_agg = 0.0;
        let mut agg_bonus=0.0;
        let mut agg_bonus_e2 = 0.0;

        if (self.aggravate_rate == 0.0) == false {
            e2_agg=dmg_e2.aggravate.unwrap().expectation;
            normal1_agg = dmg_normal1.aggravate.unwrap().expectation;
            agg_bonus=normal1_agg-normal1_normal;
            agg_bonus_e2=e2_agg-e2_normal;
        }
        //let s = format!("{}",agg_bonus);
        //web_sys::console::log_1(&s.into());
        let mut dmg_electro_charged = 0.0;
        let mut dmg_overload = 0.0;

        if (self.elecharged_rate == 0.0 && self.overload_rate == 0.0) == false {
            let transformative = context.transformative();
            dmg_electro_charged = transformative.electro_charged;
            dmg_overload = transformative.overload;
        }

        let hits = [normal1_normal,normal2_normal,normal3_normal,normal4_normal,normal4_normal,normal5_normal,normal1_normal,normal2_normal,normal3_normal,normal4_normal,normal4_normal,normal5_normal];
        let hit_index=self.hit_within_qte.floor() as usize;
        let hits_actual = &hits[0..hit_index];

        let mut normal_dmg:f64 = (self.hit_within_qte - hit_index as f64)*hits[hit_index];
        for d in hits_actual.iter(){
            normal_dmg += d;
        }

        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_requirement);

        r*(
            normal_dmg + e2_normal*1.25 + e3_normal*(3.0 + self.extra_bolts) +
                        (
                            ((self.reaction_times-1.0).min(0.0) * agg_bonus + 1.25*agg_bonus_e2) * self.aggravate_rate +
                            (self.reaction_times).min(4.0) * dmg_electro_charged * self.elecharged_rate +
                            (self.reaction_times) * dmg_overload * self.overload_rate
                        ) 
            
        )
    }
}
