use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{Character, CharacterName};
use crate::character::characters::KaedeharaKazuha;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
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

pub struct KaedeharaKazuhaDamageTargetFunction {
    pub recharge_demand: f64,
    pub swirl_rate: f64,
    pub other_dmg_ratio: f64,
}

impl TargetFunctionMetaTrait for KaedeharaKazuhaDamageTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::KaedeharaKazuhaDamage,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "枫原万叶-输出",
            en: "Kazuha-DPS"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出枫原万叶（兼辅助）",
            en: "DPS Kazuha(also support)"
        ),
        tags: "输出,辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::KaedeharaKazuha),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement",
            ),
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.8 }
        },
        ItemConfig {
            name: "swirl_rate",
            title: locale!(
                zh_cn: "扩散相对频率",
                en: "Swirl Frequency",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 }
        },
        ItemConfig {
            name: "other_dmg_ratio",
            title: locale!(
                zh_cn: "他人伤害比例",
                en: "Other's DMG Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.9 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (a, b, c) = match *config {
            TargetFunctionConfig::KaedeharaKazuhaDamage { recharge_demand, other_dmg_ratio, swirl_rate } => (recharge_demand, other_dmg_ratio, swirl_rate),
            _ => (1.0, 0.0, 0.0)
        };

        Box::new(KaedeharaKazuhaDamageTargetFunction {
            recharge_demand: a,
            swirl_rate: c,
            other_dmg_ratio: b
        })
    }
}

impl TargetFunction for KaedeharaKazuhaDamageTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.7,
        //     elemental_mastery: 1.0,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 1.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::ElementalMastery,
        //         StatName::Recharge,
        //         StatName::ATKPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::ElementalMastery,
        //         StatName::AnemoBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::ElementalMastery,
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::ViridescentVenerer,
        //         ArtifactSetName::WanderersTroupe,
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence,
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

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand) / self.recharge_demand;
        // let em = attribute.get_value(AttributeName::ElementalMastery);
        let em = attribute.get_em_all();

        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        let swirl = context.swirl_without_element();

        type S = <KaedeharaKazuha as CharacterTrait>::DamageEnumType;

        let skill_config = if character.common_data.constellation >= 6 {
            CharacterSkillConfig::KaedeharaKazuha { after_e_or_q: true }
        } else {
            CharacterSkillConfig::KaedeharaKazuha { after_e_or_q: false }
        };
        let dmg_q = KaedeharaKazuha::damage::<SimpleDamageBuilder>(&context, S::Q2, &CharacterSkillConfig::NoConfig, None).normal.expectation;
        let dmg_plunging_e = KaedeharaKazuha::damage::<SimpleDamageBuilder>(&context, S::PlungingE2, &skill_config, None).normal.expectation;

        let bonus_for_other = if character.common_data.has_talent2 {
            em * 0.0004
        } else {
            0.0
        };

        const BONUS_VIRTUAL: f64 = 0.8;
        let ratio = (1.0 + BONUS_VIRTUAL + bonus_for_other) / (1.0 + BONUS_VIRTUAL);

        let dmg_self = dmg_q * 5.0 + dmg_plunging_e + swirl * self.swirl_rate * 6.0;
        let dmg_other = dmg_self * self.other_dmg_ratio * ratio;

        dmg_self * r + dmg_other
        // dmg_self * r * (1.0 - self.other_dmg_ratio) + 50000.0 * ratio * self.other_dmg_ratio
    }
}
