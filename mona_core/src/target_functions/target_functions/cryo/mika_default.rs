use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Mika;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct MikaDefaultTargetFunction {
    pub recharge_demand: f64,
    pub crit_demand: f64,
}

impl TargetFunctionMetaTrait for MikaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::MikaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "米卡-晴霜的标绘",
            en: "Mika-Coordinate of Clear Frost"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通治疗辅助",
            en: "Healing"
        ),
        tags: "治疗,护盾",
        four: TargetFunctionFor::SomeWho(CharacterName::Mika),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement",
            ),
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 2.0 },
        },
        ItemConfig {
            name: "crit_demand",
            title: locale!(
                zh_cn: "暴击需求",
                en: "CRIT Rate Requirement",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.6 },
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (recharge_demand, crit_demand) = match *config {
            TargetFunctionConfig::MikaDefault { recharge_demand, crit_demand } => (recharge_demand, crit_demand),
            _ => (1.0, 1.0)
        };
        Box::new(MikaDefaultTargetFunction {
            recharge_demand,
            crit_demand,
        })
    }
}

impl TargetFunction for MikaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };

        type S = <Mika as CharacterTrait>::DamageEnumType;
        let q_heal = Mika::damage::<SimpleDamageBuilder>(
            &context, S::QHeal1, &CharacterSkillConfig::NoConfig, None,
        ).normal.expectation;

        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand);
        let c = attribute.get_value(AttributeName::CriticalBase).min(self.crit_demand).max(0.01);

        r * q_heal * c
    }
}
