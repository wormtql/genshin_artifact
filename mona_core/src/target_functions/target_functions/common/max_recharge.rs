use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct MaxRechargeTargetFunction;

impl TargetFunctionMetaTrait for MaxRechargeTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::MaxRecharge,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "最大充能效率",
            en: "Max Recharge"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "最大化元素充能效率",
            en: "Maximize Energy Recharge"
        ),
        tags: "",
        four: TargetFunctionFor::Common,
        image: TargetFunctionMetaImage::Custom("misc/sword")
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(MaxRechargeTargetFunction)
    }
}

impl TargetFunction for MaxRechargeTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let recharge = attribute.get_value(AttributeName::Recharge);
        recharge
    }
}