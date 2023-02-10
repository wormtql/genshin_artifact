use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigArchaicPetra, ConfigBlizzardStrayer, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::cryo::kamisato_ayaka::KamisatoAyaka;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct KamisatoAyakaDefaultTargetFunction {
    pub recharge_demand: f64
}

impl TargetFunctionMetaTrait for KamisatoAyakaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::KamisatoAyakaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "神里绫华-白鹭霜华",
            en: "Ayaka-Frostflake Heron"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通主C绫华",
            en: "Main DPS Ayaka"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::KamisatoAyaka),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement"
            ),
            config: ItemConfigType::Float { default: 1.0, min: 1.0, max: 3.0 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let recharge_demand = match *config {
            TargetFunctionConfig::KamisatoAyakaDefault { recharge_demand } => recharge_demand,
            _ => 1.0
        };
        Box::new(KamisatoAyakaDefaultTargetFunction {
            recharge_demand
        })
    }
}

impl TargetFunction for KamisatoAyakaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.3,
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
        //         ArtifactSetName::BlizzardStrayer,
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::EmblemOfSeveredFate,
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
        ArtifactEffectConfigBuilder::new()
            .blizzard_strayer(0.3)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        let s_config: CharacterSkillConfig = CharacterSkillConfig::KamisatoAyaka { after_dash: true, use_c6: false };
        type S = <KamisatoAyaka as CharacterTrait>::DamageEnumType;
        let dmg_q = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::Q1, &s_config, None).normal.expectation;
        let dmg_normal = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::Normal1, &s_config, None).normal.expectation;
        let dmg_charged = KamisatoAyaka::damage::<SimpleDamageBuilder>(&context, S::ChargedTimes3, &s_config, None).normal.expectation;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let r = recharge.min(self.recharge_demand);

        dmg_q * r + dmg_normal + dmg_charged * 0.3
    }
}
