use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::enemies::Enemy;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use super::target_function_opt_config::TargetFunctionOptConfig;

pub trait GetTargetFunctionOptConfig {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig;
}

pub trait DefaultArtifactConfig {
    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig;
}

pub trait TargetFunction: GetTargetFunctionOptConfig + DefaultArtifactConfig {
    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, enemy: &Enemy) -> f64;
}