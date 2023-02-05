use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::anemo::sayu::Sayu;
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

pub struct SayuDefaultTargetFunction {
    pub c6: bool,
    pub recharge_demand: f64
}

impl SayuDefaultTargetFunction {
    pub fn new(c: &CharacterCommonData, config: &TargetFunctionConfig) -> Self {
        let recharge_demand = match *config {
            TargetFunctionConfig::SayuDefault { recharge_demand } => recharge_demand,
            _ => 1.0
        };

        SayuDefaultTargetFunction {
            c6: c.constellation >= 6,
            recharge_demand
        }
    }
}

impl TargetFunctionMetaTrait for SayuDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::SayuDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "早柚-忍里之貉",
            en: "Sayu-Mujina Ninja"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出型早柚",
            en: "DPS Sayu"
        ),
        tags: "输出,治疗",
        four: TargetFunctionFor::SomeWho(CharacterName::Sayu),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.4 }
        }
    ]);

    fn create(character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(SayuDefaultTargetFunction::new(character, config))
    }
}

impl TargetFunction for SayuDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // if self.c6 {
        //     TargetFunctionOptConfig {
        //         atk_fixed: 0.0,
        //         atk_percentage: 0.9,
        //         hp_fixed: 0.0,
        //         hp_percentage: 0.0,
        //         def_fixed: 0.0,
        //         def_percentage: 0.0,
        //         recharge: 1.0,
        //         elemental_mastery: 1.5,
        //         critical: 1.0,
        //         critical_damage: 1.0,
        //         healing_bonus: 0.0,
        //         bonus_electro: 0.0,
        //         bonus_pyro: 0.0,
        //         bonus_hydro: 0.0,
        //         bonus_anemo: 1.0,
        //         bonus_cryo: 0.0,
        //         bonus_geo: 0.0,
        //         bonus_dendro: 0.0,
        //         bonus_physical: 0.0,
        //         sand_main_stats: vec![
        //             StatName::ElementalMastery,
        //             StatName::Recharge
        //         ],
        //         goblet_main_stats: vec![
        //             StatName::ElementalMastery,
        //             StatName::AnemoBonus
        //         ],
        //         head_main_stats: vec![
        //             StatName::ElementalMastery,
        //             StatName::ATKPercentage,
        //         ],
        //         set_names: Some(vec![
        //             ArtifactSetName::ViridescentVenerer,
        //             ArtifactSetName::GladiatorsFinale,
        //         ]),
        //         very_critical_set_names: None,
        //         normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //         critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //         very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        //     }
        // } else {
        //     TargetFunctionOptConfig {
        //         atk_fixed: 0.0,
        //         atk_percentage: 1.0,
        //         hp_fixed: 0.0,
        //         hp_percentage: 0.0,
        //         def_fixed: 0.0,
        //         def_percentage: 0.0,
        //         recharge: 1.0,
        //         elemental_mastery: if self.c6 { 1.0 } else { 0.0 },
        //         critical: 0.0,
        //         critical_damage: 0.0,
        //         healing_bonus: 0.0,
        //         bonus_electro: 0.0,
        //         bonus_pyro: 0.0,
        //         bonus_hydro: 0.0,
        //         bonus_anemo: 0.0,
        //         bonus_cryo: 0.0,
        //         bonus_geo: 0.0,
        //         bonus_dendro: 0.0,
        //         bonus_physical: 0.0,
        //         sand_main_stats: vec![
        //             StatName::Recharge
        //         ],
        //         goblet_main_stats: vec![
        //             StatName::ATKPercentage
        //         ],
        //         head_main_stats: vec![
        //             StatName::HealingBonus
        //         ],
        //         set_names: Some(vec![
        //             ArtifactSetName::ViridescentVenerer,
        //             ArtifactSetName::GladiatorsFinale,
        //         ]),
        //         very_critical_set_names: None,
        //         normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //         critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //         very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        //     }
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Sayu as CharacterTrait>::DamageEnumType;
        let dmg_q = Sayu::damage::<SimpleDamageBuilder>(&context, S::Q2, &CharacterSkillConfig::NoConfig, None).normal.expectation;
        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand);

        r * dmg_q
    }
}
