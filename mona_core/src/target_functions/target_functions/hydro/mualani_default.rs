use crate::artifacts::Artifact;
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Mualani;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::ItemConfig;
use crate::common::i18n::locale;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct MualaniDefaultTargetFunction {
    pub vaporize_rate: f64,
}

impl MualaniDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> MualaniDefaultTargetFunction {
        MualaniDefaultTargetFunction {
            vaporize_rate: match *config {
                TargetFunctionConfig::MualaniDefault { vaporize_rate } => vaporize_rate,
                _ => 0.0
            }
        }
    }
}

impl TargetFunction for MualaniDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .obsidian_codex(1.0, 1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context = DamageContext {
            character_common_data: &character.common_data,
            enemy, attribute
        };

        type S = <Mualani as CharacterTrait>::DamageEnumType;
        let constellation = character.common_data.constellation;
        let dmg = if constellation >= 1 {
            Mualani::damage::<SimpleDamageBuilder>(&context, S::A_C1, &CharacterSkillConfig::NoConfig, None)
        } else {
            Mualani::damage::<SimpleDamageBuilder>(&context, S::A_Stack4, &CharacterSkillConfig::NoConfig, None)
        };

        dmg.normal.expectation * (1.0 - self.vaporize_rate) + dmg.vaporize.unwrap().expectation * self.vaporize_rate
    }
}

impl TargetFunctionMetaTrait for MualaniDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::MualaniDefault,
        name_locale: locale!(
            zh_cn: "玛拉妮-哗啦啦逐浪客",
            en: "Mualani: Splish-Splash Wavechaser"
        ),
        description: locale!(
            zh_cn: "最大化巨浪鲨鲨撕咬伤害",
            en: "Maximize Sharky's Surging Bite DMG"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Mualani),
        image: TargetFunctionMetaImage::Avatar,
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
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(MualaniDefaultTargetFunction::new(config))
    }
}
