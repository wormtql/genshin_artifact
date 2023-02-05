use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigArchaicPetra, ConfigBlizzardStrayer, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Chongyun;
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

pub struct ChongyunDefaultTargetFunction;

impl TargetFunctionMetaTrait for ChongyunDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::ChongyunDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "重云-雪融有踪",
            en: "Chongyun-Frozen Ardor"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通副C重云",
            en: "Sub DPS Chongyun"
        ),
        tags: "副C,输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Chongyun),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(ChongyunDefaultTargetFunction)
    }
}

impl TargetFunction for ChongyunDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.5,
        //     elemental_mastery: 0.0,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 2.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //         StatName::Recharge
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::CryoBonus,
        //         StatName::ATKPercentage
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::NoblesseOblige,
        //         ArtifactSetName::BlizzardStrayer,
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .blizzard_strayer(0.2)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let mut noblesse_count = 0;
        for artifact in artifacts.iter() {
            if artifact.set_name == ArtifactSetName::NoblesseOblige {
                noblesse_count += 1;
            }
        }

        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Chongyun as CharacterTrait>::DamageEnumType;
        let dmg_q = Chongyun::damage::<SimpleDamageBuilder>(
            &context,
            S::Q1,
            &CharacterSkillConfig::NoConfig,
            None
        ).normal.expectation;
        let dmg_e = Chongyun::damage::<SimpleDamageBuilder>(
            &context, S::E1, &CharacterSkillConfig::NoConfig, None
        ).normal.expectation;

        let recharge = attribute.get_value(AttributeName::Recharge);

        const E_PARTICLE: f64 = 4.0;
        const ENV_PARTICLE: f64 = 0.2;

        let q_cd = (40.0 / ((E_PARTICLE / 15.0 + ENV_PARTICLE) * recharge)).max(12.0);
        let dmg = (1.0 / 15.0) * dmg_e + (1.0 / q_cd) * dmg_q;

        const DMG_OTHER: f64 = 6.0;
        const OTHER_ATK: f64 = 2500.0;
        const OTHER_ATK_BASE: f64 = 800.0;
        if noblesse_count < 4 {
            (DMG_OTHER + 1.0) * dmg
        } else {
            let other_ratio = (0.2 * OTHER_ATK_BASE + OTHER_ATK) / OTHER_ATK;
            (DMG_OTHER * other_ratio + 1.0) * dmg
        }
    }
}
