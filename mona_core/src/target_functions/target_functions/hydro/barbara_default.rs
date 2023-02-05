use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Barbara;
use crate::character::prelude::CharacterTrait;
use crate::character::skill_config::CharacterSkillConfig;
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

pub struct BarbaraDefaultTargetFunction;

impl TargetFunctionMetaTrait for BarbaraDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::BarbaraDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "芭芭拉-闪耀偶像",
            en: "Barbara-Shining Idol"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "使得芭芭拉Q技能治疗效果最好",
            en: "Miximize Barbara's Q regeneration"
        ),
        tags: "治疗,辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Barbara),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(BarbaraDefaultTargetFunction)
    }
}

impl TargetFunction for BarbaraDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 0.0,
        //     hp_fixed: 0.1,
        //     hp_percentage: 1.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.8,
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
        //         StatName::HealingBonus,
        //         StatName::HPPercentage
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::MaidenBeloved,
        //         ArtifactSetName::OceanHuedClam,
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
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        type S = <Barbara as CharacterTrait>::DamageEnumType;
        let heal_e1 = Barbara::damage::<SimpleDamageBuilder>(&context, S::EHeal1, &CharacterSkillConfig::NoConfig, None);
        const ENV_CHARGE: f64 = 2.3;
        const E1_COUNT: f64 = 10.0;
        const E2_COUNT: f64 = 3.0;

        let heal_e = E1_COUNT * heal_e1.normal.expectation + E2_COUNT * heal_e1.normal.expectation * 5.33;
        let heal_q = heal_e1.normal.expectation * 23.467;

        let recharge = attribute.get_value(AttributeName::Recharge);

        heal_q / 20.0_f64.max(80.0 / (ENV_CHARGE * recharge)) + heal_e / 32.0
    }
}
