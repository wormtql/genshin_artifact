use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigArchaicPetra, ConfigBlizzardStrayer, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{Character, CharacterName};
use crate::character::characters::KamisatoAyaka;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::{Weapon, WeaponName};

pub struct KamisatoAyakaDpsTargetFunction {
    pub is_kageuchi: bool,
    pub kageuchi_refine: usize,
}

impl TargetFunctionMetaTrait for KamisatoAyakaDpsTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::KamisatoAyakaDps,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "神里绫华-DPS",
            en: "Ayaka-DPS with Recharge"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "期望DPS输出，输出手法模拟如下循环：4s 左右辅助铺场，平a4段接重击，有e放e，有大开大",
            en: "DPS Ayaka, with recharge into consideration, Simulation: 4s 左右辅助铺场，平a4段接重击，有e放e，有大开大"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::KamisatoAyaka),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(KamisatoAyakaDpsTargetFunction {
            is_kageuchi: weapon.name == WeaponName::AmenomaKageuchi,
            kageuchi_refine: weapon.refine as usize,
        })
    }
}

impl TargetFunction for KamisatoAyakaDpsTargetFunction {
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
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::CryoBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::EmblemOfSeveredFate,
        //         ArtifactSetName::GladiatorsFinale,
        //     ]),
        //     very_critical_set_names: Some(vec![
        //         ArtifactSetName::BlizzardStrayer,
        //     ]),
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .blizzard_strayer(0.35)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <KamisatoAyaka as CharacterTrait>::DamageEnumType;

        let config = CharacterSkillConfig::KamisatoAyaka { after_dash: true, use_c6: false };
        let dmg_a1 = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::Normal1, &config, None).normal.expectation;
        let dmg_a = dmg_a1 * (1.0 + 1.0656 + 1.3698 + 0.4955 * 3.0);
        let time_a = 2.66;

        let dmg_b = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::ChargedTimes3, &config, None).normal.expectation;

        let dmg_e = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::E1, &config, None).normal.expectation;
        let time_e = 2.53;

        let dmg_q1 = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::Q1, &config, None).normal.expectation;
        let dmg_q = dmg_q1 * (19.0 + 1.5);
        let time_q = 2.16;

        let time_dodge = 0.5;

        let weapon_element = if self.is_kageuchi {
            (self.kageuchi_refine as f64) * 1.5 + 4.5
        } else {
            0.0
        };

        let helper_element = 2.0 / 15.0;
        let ayaka_element = 4.5 / 10.0;
        let additional_element = 4.0 / 20.0;

        let total_element_per_sec = (helper_element + ayaka_element + additional_element) * 2.7;
        let recharge = attribute.get_value(AttributeName::Recharge);
        let energy_per_sec = total_element_per_sec * recharge + weapon_element;
        let q_interval = 20.0_f64.max(80.0 / energy_per_sec);

        let mut time_elapse = 0.0;
        let mut e_timer = 0.0;
        let mut q_timer = 0.0;

        let mut e_count = 0;
        let mut a_count = 0;
        let mut q_count = 0;
        while q_count < 3 {
            let used_time = 4.0 + time_a;
            time_elapse += used_time;
            e_timer += used_time;
            q_timer += used_time;
            a_count += 1;

            if e_timer > 10.0 {
                e_count += 1;
                e_timer = 0.0;
                time_elapse += time_e;
            }

            if q_timer > q_interval {
                q_count += 1;
                q_timer = 0.0;
                time_elapse += time_q + time_dodge;
            }
        }

        let total_damage = (dmg_a + dmg_b) * (a_count as f64) + dmg_e * (e_count as f64) + dmg_q * (q_count as f64);
        total_damage / time_elapse
    }
}
