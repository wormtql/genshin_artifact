use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::anemo::venti::Venti;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
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

pub struct VentiDefaultTargetFunction {
    pub swirl_rate: f64,
}

impl VentiDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let swirl_rate = match *config {
            TargetFunctionConfig::VentiDefault { swirl_rate } => swirl_rate,
            _ => 0.0
        };
        VentiDefaultTargetFunction {
            swirl_rate
        }
    }
}

impl TargetFunctionMetaTrait for VentiDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::VentiDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "温迪-风色诗人",
            en: "Venti-Windborne Bard"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出温迪",
            en: "DPS Venti"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Venti),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "swirl_rate",
            title: locale!(
                zh_cn: "扩散占比",
                en: "Swirl Frequency"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.7 },
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(VentiDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for VentiDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.1,
        //     atk_percentage: 1.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.2,
        //     elemental_mastery: 1.0,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 2.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //         StatName::ElementalMastery
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::AnemoBonus,
        //         StatName::ATKPercentage,
        //         StatName::ElementalMastery
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ATKPercentage,
        //         StatName::ElementalMastery
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::ViridescentVenerer,
        //         ArtifactSetName::GladiatorsFinale,
        //         ArtifactSetName::ShimenawasReminiscence,
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
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        let mut vv_count = 0;
        for art in artifacts.iter() {
            if art.set_name == ArtifactSetName::ViridescentVenerer {
                vv_count += 1;
            }
        }

        type S = <Venti as CharacterTrait>::DamageEnumType;
        let dmg_q = Venti::damage::<SimpleDamageBuilder>(&context, S::Q1, &CharacterSkillConfig::NoConfig, None).normal.expectation;
        let dmg_swirl = context.swirl_without_element();

        let vv_ratio = if vv_count >= 4 {
            1.278
        } else {
            1.0
        };

        (dmg_swirl * self.swirl_rate * 2.0 + dmg_q) * vv_ratio
    }
}
