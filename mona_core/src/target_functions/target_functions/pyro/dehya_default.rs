use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigArchaicPetra, ConfigLevel, ConfigRate};
use crate::attribute::{AttributeCommon, SimpleAttributeGraph2};
use crate::attribute::attribute_name::AttributeName;
use crate::character::{Character, character_common_data, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Dehya;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::skill_type::SkillType;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct DehyaDefaultTargetFunction {
    pub melt_rate: f64,
    pub vaporize_rate: f64,
    pub e_count: usize,
}

impl TargetFunctionMetaTrait for DehyaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::DehyaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "迪希雅-炽鬃之狮",
            en: "Dehya-Default"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出迪希雅",
            en: "DPS Dehya"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Dehya),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "melt_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "融化占比",
                en: "Melt Ratio",
            ),
            config: ItemConfig::RATE01_TYPE,
        },
        ItemConfig {
            name: "vaporize_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "蒸发占比",
                en: "Vaporize Ratio",
            ),
            config: ItemConfig::RATE01_TYPE,
        },
        ItemConfig {
            name: "e_count",
            title: crate::common::i18n::locale!(
                zh_cn: "两次E中间触发的Q次数",
                en: "E Triggered Count Within a Cycle",
            ),
            config: ItemConfigType::Int { min: 0, max: 20, default: 6 },
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (mut melt_rate, mut vaporize_rate, e_count) = match *config {
            TargetFunctionConfig::DehyaDefault { melt_rate, vaporize_rate, e_count } => (melt_rate, vaporize_rate, e_count),
            _ => (0.0, 0.0, 0)
        };

        let sum = melt_rate + vaporize_rate;
        if sum > 1.0 {
            melt_rate /= sum;
            vaporize_rate /= sum;
        }

        Box::new(DehyaDefaultTargetFunction {
            melt_rate,
            vaporize_rate,
            e_count,
        })
    }
}

impl TargetFunction for DehyaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .crimson_witch_of_flames(1.0)
            .echoes_of_an_offering_avg()
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };

        let crit = context.attribute.get_critical_rate(Element::Pyro, SkillType::ElementalBurst).min(1.0);
        let mut stack: f64 = 0.0;
        for i in 1..17 {
            stack += (crit * i as f64).max(4.0);
        }
        let avg_stack = 17.0 * 4.0 / stack;
        let config: CharacterSkillConfig = CharacterSkillConfig::Dehya { c2_rate: 1.0, c6_stack: avg_stack };
        type S = <Dehya as CharacterTrait>::DamageEnumType;
        let dmgs = [
            (Dehya::damage::<SimpleDamageBuilder>(
                &context, S::E1, &config, None,
            ), 1.0),
            (Dehya::damage::<SimpleDamageBuilder>(
                &context, S::E2, &config, None,
            ), 1.0),
            (Dehya::damage::<SimpleDamageBuilder>(
                &context, S::E3, &config, None,
            ), self.e_count as f64),
            (Dehya::damage::<SimpleDamageBuilder>(
                &context, S::Q1, &config, None,
            ), if context.character_common_data.constellation == 6 { 15.0 } else { 11.0 }),
            (Dehya::damage::<SimpleDamageBuilder>(
                &context, S::Q2, &config, None,
            ), 1.0),
        ];

        let normal_rate = (1.0 - self.vaporize_rate - self.melt_rate).clamp(0.0, 1.0);

        let mut sum: f64 = 0.0;
        for d in dmgs {
            sum += (d.0.normal.expectation * normal_rate +
                d.0.vaporize.unwrap().expectation * self.vaporize_rate +
                d.0.melt.unwrap().expectation * self.melt_rate) * d.1;
        }

        sum
    }
}
