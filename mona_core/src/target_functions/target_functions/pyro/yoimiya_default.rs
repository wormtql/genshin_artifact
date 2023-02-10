use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel, ConfigRate};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::pyro::yoimiya::Yoimiya;
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

pub struct YoimiyaDefaultTargetFunction {
    pub vaporize_rate: f64,
    pub melt_rate: f64,
}

impl YoimiyaDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (vaporize_rate, melt_rate) = match *config {
            TargetFunctionConfig::YoimiyaDefault { vaporize_rate, melt_rate } => (vaporize_rate, melt_rate),
            _ => (0.0, 0.0)
        };

        Self {
            vaporize_rate,
            melt_rate
        }
    }
}

impl TargetFunctionMetaTrait for YoimiyaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::YoimiyaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "宵宫-琉焰华舞",
            en: "Yoimiya-Frolicking Flames"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出宵宫",
            en: "DPS Yoimiya"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Yoimiya),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "vaporize_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "蒸发占比",
                en: "Vaporize Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "melt_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "融化占比",
                en: "Melt Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(YoimiyaDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for YoimiyaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // let normal_rate = (1.0 - self.vaporize_rate - self.melt_rate).max(0.0);
        // let em_weight = if normal_rate > 0.8 { 0.0 } else { 1.0 };
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.0,
        //     elemental_mastery: em_weight,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 2.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //         StatName::ElementalMastery,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::PyroBonus,
        //         StatName::ATKPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::ShimenawasReminiscence,
        //         ArtifactSetName::CrimsonWitchOfFlames,
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::WanderersTroupe,
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
            .crimson_witch_of_flames(1.0)
            .shimenawas_reminiscence(1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Yoimiya as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::Yoimiya { after_e: true };
        let dmg = Yoimiya::damage::<SimpleDamageBuilder>(&context, S::Normal1, &config, None);

        let pyro = dmg.normal.expectation;
        let melt = dmg.melt.unwrap().expectation;
        let vaporize = dmg.vaporize.unwrap().expectation;

        let normal_rate = (1.0 - self.melt_rate - self.vaporize_rate).max(0.0);

        pyro * normal_rate + melt * self.melt_rate + vaporize * self.vaporize_rate
    }
}
