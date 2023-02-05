use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigPaleFlame};
use crate::attribute::{Attribute, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Eula;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::ItemConfig;
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

pub struct EulaDefaultTargetFunction {
    pub is_c2: bool
}

impl EulaDefaultTargetFunction {
    pub fn new(character: &CharacterCommonData) -> Self {
        Self {
            is_c2: character.constellation >= 2
        }
    }
}

impl TargetFunctionMetaTrait for EulaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::EulaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "优菈-浪花骑士",
            en: "Eula-Spindrift Knight"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通优菈输出",
            en: "DPS Eula"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Eula),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(EulaDefaultTargetFunction::new(character))
    }
}

impl TargetFunction for EulaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.2,
        //     elemental_mastery: 0.0,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 2.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::PhysicalBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::PaleFlame,
        //         ArtifactSetName::BloodstainedChivalry,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::GladiatorsFinale,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        let (stack, full_rate) = if self.is_c2 {
            const CD: f64 = 5.0;
            (7.0 / CD, (7.0 - CD) / CD)
        } else {
            const CD: f64 = 11.0;
            (7.0 / CD, 0.0)
        };

        ArtifactEffectConfigBuilder::new()
            .pale_flame(stack, full_rate)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        let skill_config = if character.common_data.constellation < 6 {
            CharacterSkillConfig::Eula { lightfall_stack: 10 }
        } else {
            CharacterSkillConfig::Eula { lightfall_stack: 20 }
        };

        type S = <Eula as CharacterTrait>::DamageEnumType;
        let dmg_q = Eula::damage::<SimpleDamageBuilder>(
            &context, S::QLightfall, &skill_config, None
        ).normal.expectation;
        let dmg_a = Eula::damage::<SimpleDamageBuilder>(
            &context, S::Normal1, &skill_config, None
        ).normal.expectation;

        dmg_q + dmg_a * 25.0
    }
}
