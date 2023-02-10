use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{Character, CharacterName};
use crate::character::characters::Yelan;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
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

pub struct YelanDefaultTargetFunction {
    recharge_demand: f64,
    vaporize_rate: f64,
}

impl TargetFunctionMetaTrait for YelanDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::YelanDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "夜兰-兰生幽谷",
            en: "Yelan-Valley Orchid"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出夜兰。使得Q伤害最大",
            en: "DPS Yelan, maximizing Q damage"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Yelan),
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
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.0 }
        },
        ItemConfig {
            name: "vaporize_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "蒸发占比",
                en: "Vaporize Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (recharge_demand, v) = match *config {
            TargetFunctionConfig::YelanDefault { recharge_demand, vaporize_rate } => (recharge_demand, vaporize_rate),
            _ => (1.0, 0.0)
        };

        Box::new(YelanDefaultTargetFunction {
            recharge_demand,
            vaporize_rate: v.clamp(0.0, 1.0)
        })
    }
}

impl TargetFunction for YelanDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 0.0,
        //     hp_fixed: 0.1,
        //     hp_percentage: 1.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 1.0,
        //     elemental_mastery: 0.5,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 2.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::HPPercentage,
        //         StatName::Recharge,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::HPPercentage,
        //         StatName::HydroBonus,
        //     ],
        //     head_main_stats: vec![
        //         StatName::HPPercentage,
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::EmblemOfSeveredFate,
        //         ArtifactSetName::TenacityOfTheMillelith,
        //         ArtifactSetName::HeartOfDepth,
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

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Yelan as CharacterTrait>::DamageEnumType;
        let dmg_q = Yelan::damage::<SimpleDamageBuilder>(&context, S::Q2, &CharacterSkillConfig::NoConfig, None);

        let normal = dmg_q.normal.expectation;
        let vaporize = dmg_q.vaporize.unwrap().expectation;

        let dmg = normal * (1.0 - self.vaporize_rate) + self.vaporize_rate * vaporize;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let r = recharge.min(self.recharge_demand);

        r * dmg
    }
}
