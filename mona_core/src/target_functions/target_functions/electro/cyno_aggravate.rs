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

pub struct CynoAggravateTargetFunction {
    pub recharge_requirement:f64,
    pub aggravate_rate: f64,
}

impl CynoAggravateTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (recharge_requirement, aggravate_rate) = match *config {
            TargetFunctionConfig::CynoAggravate { recharge_requirement,aggravate_rate  } => (recharge_requirement, aggravate_rate),
            _ => (0.0, 0.0)
        };

        Self {
            recharge_requirement,
            aggravate_rate
        }
    }
}

impl TargetFunctionMetaTrait for CynoAggravateTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::CynoAggravate,
        chs: "赛诺-激化",
        description: "激化输出赛诺,手法7ae循环，打qte并释放渡荒之雷，共5次激化，比例留1表示所有循环都按标准激化次数计算，激化覆盖率不足比例应小于1，攻速变高时比例应大于1",
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Cyno),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_requirement",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min:1.0, max: 3.0 , default:1.3}
        },
        ItemConfig {
            name: "aggravate_rate",
            title: "t17",
            config: ItemConfigType::Float { min:0.0, max: 2.0 , default:1.0}
        },
    ]);
    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(CynoAggravateTargetFunction::new(config))
    }
}

impl TargetFunction for CynoAggravateTargetFunction {
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

        type S = <Cyno as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::Cyno { under_judication: true };
        let dmg_normal = Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal1, &config, None);
        let dmg_e2 = Cyno::damage::<SimpleDamageBuilder>(&context, S::E2, &config, None);
        let dmg_e3=Cyno::damage::<SimpleDamageBuilder>(&context, S::QNormal1, &config, None);

        let normal_agg = dmg_normal.aggravate.unwrap().expectation;
        let e2_agg = dmg_e2.aggravate.unwrap().expectation;
        let e3_agg = dmg_e3.aggravate.unwrap().expectation;
        let normal_normal = dmg_normal.normal.expectation;
        let e2_normal = dmg_e2.normal.expectation;
        let e3_normal = dmg_e3.normal.expectation;
        
        let normal_agg_bonus=normal_agg-normal_normal;
        let e2_agg_bonus=e2_agg-e2_normal;
        let e3_agg_bonus=e3_agg-e3_normal;

        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_requirement);

        r*(1.25*e2_normal + 12.18*normal_normal + 3.0*e3_normal + self.aggravate_rate*(1.25*e2_agg_bonus+3.0*normal_agg_bonus+1.0*e3_agg_bonus))

    }
}
