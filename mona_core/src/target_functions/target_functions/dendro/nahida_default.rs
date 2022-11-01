use crate::artifacts::Artifact;
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder};
use crate::attribute::{Attribute, SimpleAttributeGraph2};
use crate::attribute::attribute_name::AttributeName;
use crate::character::{Character, character_common_data, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Nahida;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct NahidaDefaultTargetFunction {
    pub em_requirement: usize,
    pub spread_rate: f64,
    pub bloom_count: f64,
    pub burn_duration: f64,
    pub pryo_teammate_count: usize,
}

impl NahidaDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (em_requirement,
            spread_rate,
            bloom_count,
            burn_duration,
            pryo_teammate_count
        ) = match *config {
            TargetFunctionConfig::NahidaDefault {
                em_requirement,
                spread_rate,
                bloom_count,
                burn_duration,
                pryo_teammate_count
            } =>
                (em_requirement,
                 spread_rate,
                 bloom_count,
                 burn_duration,
                 pryo_teammate_count),
            _ => (0, 0.0, 0.0, 0.0, 0),
        };

        Self {
            em_requirement,
            spread_rate,
            bloom_count,
            burn_duration,
            pryo_teammate_count,
        }
    }
}

impl TargetFunctionMetaTrait for NahidaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::NahidaDefault,
        chs: "纳西妲-白草净华",
        description: "",
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Nahida),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em_requirement",
            title: "t28", //精通需求
            config: ItemConfigType::Int { min: 0, max: 1500, default: 0 },
        },
        ItemConfig {
            name: "spread_rate",
            title: "t16", //蔓激化比例
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "bloom_count",
            title: "t20", //绽放倍数
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 },
        },
        ItemConfig {
            name: "burn_duration",
            title: "t29", //燃烧持续秒数
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 },
        },
        ItemConfig {
            name: "pryo_teammate_count",
            title: "t30", // 火系队友数量
            config: ItemConfigType::Int { min: 0, max: 2, default: 0 },
        },
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(NahidaDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for NahidaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .gilded_dreams(1, 2, 0.8)
            .deepwood_memories(1.0)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute: &attribute,
            enemy,
        };
        let em_req = if self.em_requirement == 0 { self.em_requirement + 1 } else { self.em_requirement } as f64;
        type S = <Nahida as CharacterTrait>::DamageEnumType;

        let pryo_count =
            if character.common_data.constellation >= 1 {
                (self.pryo_teammate_count + 1).min(2)
            } else {
                self.pryo_teammate_count.min(2)
            };

        let skill_config = CharacterSkillConfig::Nahida {
            q_bonus: if self.pryo_teammate_count > 0 { true } else { false },
            q_bonus_count: self.pryo_teammate_count,
        };
        let dmg_e3 = Nahida::damage::<SimpleDamageBuilder>(&context, S::E3, &skill_config, None);
        let trans = context.transformative();

        (dmg_e3.spread.unwrap().expectation * self.spread_rate + dmg_e3.normal.expectation * (1.0 - self.spread_rate) +
            self.bloom_count * trans.bloom
            //+self.burn_duration*trans.burn
        ) * (em_req.min(attribute.get_value(AttributeName::ElementalMastery)))
    }
}