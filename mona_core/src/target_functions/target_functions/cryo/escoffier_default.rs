use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Escoffier;
use crate::character::prelude::CharacterTrait;
use crate::character::{CharacterConfig, CharacterName};
use crate::character::skill_config::CharacterSkillConfig;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::artifacts::Artifact;
use crate::common::StatName;

pub struct EscoffierDefaultTargetFunction;

impl TargetFunctionMetaTrait for EscoffierDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::EscoffierDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "爱可菲-标准循环",
            en: "Escoffier-Standard Rotation"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "21次冻霜芭菲伤害+1次大招伤害的标准循环",
            en: "Standard rotation: 21x Frosty Parfait DMG + 1x Burst DMG"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Escoffier),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(EscoffierDefaultTargetFunction)
    }
}

impl TargetFunction for EscoffierDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.3,
            elemental_mastery: 0.0,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_electro: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 2.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::ElementalMastery,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::CryoBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                crate::artifacts::ArtifactSetName::BlizzardStrayer,
                crate::artifacts::ArtifactSetName::GladiatorsFinale,
                crate::artifacts::ArtifactSetName::ShimenawasReminiscence,
                crate::artifacts::ArtifactSetName::EchoesOfAnOffering,
            ]),
            very_critical_set_names: Some(vec![
                crate::artifacts::ArtifactSetName::BlizzardStrayer,
            ]),
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD,
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };

        type S = <Escoffier as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::NoConfig;
        
        // 爱可菲的标准战斗循环：21次冻霜芭菲伤害 + 1次大招伤害
        // 冻霜芭菲是元素战技，可以吃到战技伤害加成
        let dmg_frosty = Escoffier::damage::<SimpleDamageBuilder>(&context, S::EFrostyParfait, &config, None);
        let dmg_q1 = Escoffier::damage::<SimpleDamageBuilder>(&context, S::Q1, &config, None);

        // 一轮伤害期望：21次冻霜芭菲 + 1次大招
        dmg_frosty.normal.expectation * 21.0 + dmg_q1.normal.expectation * 1.0
    }
}