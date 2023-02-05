use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Gorou;
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

pub struct GorouDefaultTargetFunction {
    pub recharge_demand: f64,
}

impl GorouDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> GorouDefaultTargetFunction {
        match *config {
            TargetFunctionConfig::GorouDefault { recharge_demand } => GorouDefaultTargetFunction {
                recharge_demand
            },
            _ => GorouDefaultTargetFunction {
                recharge_demand: 1.7
            }
        }
    }
}

impl TargetFunctionMetaTrait for GorouDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::GorouDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "五郎-戎犬锵锵",
            en: "Gorou-Canine Warrior"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通岩辅五郎",
            en: "Support Gorou"
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Gorou),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.7 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(GorouDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for GorouDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 0.0,
        //     hp_fixed: 0.0,
        //     hp_percentage: 0.0,
        //     def_fixed: 0.1,
        //     def_percentage: 1.0,
        //     recharge: 1.0,
        //     elemental_mastery: 0.3,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 2.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::Recharge,
        //         StatName::DEFPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::GeoBonus,
        //         StatName::DEFPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::DEFPercentage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::HuskOfOpulentDreams,
        //         ArtifactSetName::NoblesseOblige,
        //         ArtifactSetName::EmblemOfSeveredFate,
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
            .husk_of_opulent_dreams(2.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Gorou as CharacterTrait>::DamageEnumType;
        let dmg_q = Gorou::damage::<SimpleDamageBuilder>(
            &context, S::Q2, &CharacterSkillConfig::NoConfig, None
        ).normal.expectation;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(self.recharge_demand);

        dmg_q * recharge_ratio
    }
}
