use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Nilou;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::transformative_damage::transformative_damage;
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct NilouDefaultTargetFunction {
    pub e_ratio: f64,
    pub q_ratio: f64,
    pub bloom_ratio: f64,
}

impl TargetFunction for NilouDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context = DamageContext {
            character_common_data: &character.common_data,
            enemy, attribute
        };

        type S = <Nilou as CharacterTrait>::DamageEnumType;
        let dmg_e1 = Nilou::damage::<SimpleDamageBuilder>(
            &context, S::E1, &CharacterSkillConfig::NoConfig, None
        ).normal.expectation;

        let dmg_q1 = Nilou::damage::<SimpleDamageBuilder>(
            &context, S::Q1, &CharacterSkillConfig::NoConfig, None
        ).normal.expectation;

        let bloom = transformative_damage(character.common_data.level, attribute, enemy).bloom;

        dmg_e1 * self.e_ratio + dmg_q1 * self.q_ratio + bloom * self.bloom_ratio
    }
}

impl TargetFunctionMetaTrait for NilouDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::NilouDefault,
        chs: "",
        description: "",
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Nilou),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_ratio",
            title: "t18",
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 5.0 },
        },
        ItemConfig {
            name: "q_ratio",
            title: "t19",
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 1.0 },
        },
        ItemConfig {
            name: "bloom_ratio",
            title: "t20",
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 3.0 },
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (e_ratio, q_ratio, bloom_ratio) = match *config {
            TargetFunctionConfig::NilouDefault { e_ratio, q_ratio, bloom_ratio } => (e_ratio, q_ratio, bloom_ratio),
            _ => (0.0, 0.0, 0.0)
        };
        Box::new(NilouDefaultTargetFunction {
            e_ratio, q_ratio, bloom_ratio
        })
    }
}