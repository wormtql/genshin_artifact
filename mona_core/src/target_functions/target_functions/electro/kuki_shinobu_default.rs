use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{Character, CharacterName};
use crate::character::characters::KukiShinobu;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
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

pub struct KukiShinobuDefaultTargetFunction {
    pub e_ratio: f64
}

impl TargetFunctionMetaTrait for KukiShinobuDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::KukiShinobuDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "久岐忍-烦恼刈除",
            en: "Shinobu-Mender of Tribulations"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "输出型久岐忍。使其大招和越祓雷草之轮伤害按一定比例之和最大",
            en: "DPS Shinobu, maximizing the sum of E and Q damage by a certain ratio."
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::KukiShinobu),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_ratio",
            title: crate::common::i18n::locale!(
                zh_cn: "E技能伤害占比",
                en: "E-skill DMG Rate",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.6 }
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let e_ratio = match *config {
            TargetFunctionConfig::KukiShinobuDefault { e_ratio } => e_ratio,
            _ => 0.0
        };
        Box::new(KukiShinobuDefaultTargetFunction {
            e_ratio
        })
    }
}

impl TargetFunction for KukiShinobuDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // TargetFunctionOptConfig {
        //     atk_fixed: 0.0,
        //     atk_percentage: 0.0,
        //     hp_fixed: 0.1,
        //     hp_percentage: 1.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 1.0,
        //     elemental_mastery: 1.0,
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
        //         StatName::HPPercentage,
        //         StatName::Recharge,
        //         StatName::ATKPercentage,
        //     ],
        //     goblet_main_stats: vec![
        //         StatName::ElectroBonus,
        //         StatName::HPPercentage,
        //     ],
        //     head_main_stats: vec![
        //         StatName::HPPercentage,
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::EmblemOfSeveredFate,
        //         ArtifactSetName::ThunderingFury,
        //         ArtifactSetName::TenacityOfTheMillelith,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <KukiShinobu as CharacterTrait>::DamageEnumType;
        let dmg_q = KukiShinobu::damage::<SimpleDamageBuilder>(&context, S::Q1, &CharacterSkillConfig::NoConfig, None);
        let dmg_e = KukiShinobu::damage::<SimpleDamageBuilder>(&context, S::E2, &CharacterSkillConfig::NoConfig, None);

        let result = dmg_q.normal.expectation * (1.0 - self.e_ratio) + dmg_e.normal.expectation * self.e_ratio;

        return result
    }
}