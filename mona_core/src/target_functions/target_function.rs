use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::TargetFunctionMeta;
use crate::target_functions::TargetFunctionConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;
use super::target_function_opt_config::TargetFunctionOptConfig;

pub trait TargetFunctionMetaTrait: TargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta;

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = None;

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction>;
}

pub trait TargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig;

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig;

    fn target(
        &self,
        attribute: &SimpleAttributeGraph2,
        character: &Character<SimpleAttributeGraph2>,
        weapon: &Weapon<SimpleAttributeGraph2>,
        artifacts: &[&Artifact],
        enemy: &Enemy
    ) -> f64;
}