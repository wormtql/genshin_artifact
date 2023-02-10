use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Bennett;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
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

pub struct BennettDefaultTargetFunction {
    pub recharge_demand: f64,
}

impl TargetFunctionMetaTrait for BennettDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::BennettDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "班尼特-命运试金石",
            en: "Bennett-Trial by Fire"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通辅助班尼特",
            en: "Support Bennett"
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Bennett),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: crate::common::i18n::locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement",
            ),
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.6 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let recharge_demand = match *config {
            TargetFunctionConfig::BennettDefault { recharge_demand } => recharge_demand,
            _ => 1.0
        };
        Box::new(BennettDefaultTargetFunction {
            recharge_demand
        })
    }
}

impl TargetFunction for BennettDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 0.0,
        //     hp_fixed: 0.1,
        //     hp_percentage: 1.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 1.0,
        //     elemental_mastery: 0.0,
        //     critical: 0.0,
        //     critical_damage: 0.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: 0.0,
        //     bonus_hydro: 0.0,
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::HPPercentage,
        //         StatName::Recharge
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::HPPercentage
        //     ],
        //     head_main_stats: vec![
        //         StatName::HPPercentage,
        //         StatName::HealingBonus
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::NoblesseOblige
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
        let mut noblesse_count = 0;
        for artifact in artifacts.iter() {
            if artifact.set_name == ArtifactSetName::NoblesseOblige {
                noblesse_count += 1;
            }
        }

        let mut atk_bonus = Bennett::atk_bonus(&character.common_data, attribute);
        const VIRTUAL_BASE_ATK: f64 = 700.0;
        if noblesse_count >= 4 {
            atk_bonus += VIRTUAL_BASE_ATK * 0.2;
        }

        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };
        let heal = Bennett::damage::<SimpleDamageBuilder>(
            &context,
            <Bennett as CharacterTrait>::DamageEnumType::QHeal,
            &CharacterSkillConfig::NoConfig,
            None
        ).normal.expectation;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(self.recharge_demand);

        recharge_ratio * (atk_bonus * 1000.0 + heal)
    }
}