use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::electro::kujou_sara::KujouSara;
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

pub struct KujouSaraDefaultTargetFunction;

impl TargetFunctionMetaTrait for KujouSaraDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::KujouSaraDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "九条裟罗-黑羽鸣镝",
            en: "Kujou Sara-Crowfeather Kaburaya"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通雷系辅助九条",
            en: "Support Sara"
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::KujouSara),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(KujouSaraDefaultTargetFunction)
    }
}

impl TargetFunction for KujouSaraDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 1.1,
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
        //         StatName::Recharge,
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
        //         ArtifactSetName::EmblemOfSeveredFate,
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence,
        //     ]),
        //     very_critical_set_names: Some(vec![ArtifactSetName::NoblesseOblige]),
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
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
        type S = <KujouSara as CharacterTrait>::DamageEnumType;
        let dmg_q = KujouSara::damage::<SimpleDamageBuilder>(&context, S::Q1, &CharacterSkillConfig::NoConfig, None).normal.expectation;

        let atk_for_other_using_other_base = attribute.get_value(AttributeName::ATKBonusForOther);
        let self_base_atk = attribute.get_value(AttributeName::ATKBase);
        let atk_for_other_using_self_base = KujouSara::SKILL.elemental_skill_atk_bonus[character.common_data.skill2];
        let other_base_atk = 800.0;

        let atk_bonus_for_other = atk_for_other_using_other_base * other_base_atk + self_base_atk * atk_for_other_using_self_base;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_bonus = recharge.floor() * 1.2;

        recharge_bonus * atk_bonus_for_other * 1000.0 + dmg_q
    }
}
