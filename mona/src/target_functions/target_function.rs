use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::enemies::Enemy;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use super::target_function_opt_config::TargetFunctionOptConfig;

pub trait TargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig;

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig;

    fn target(
        &self,
        attribute: &SimpleAttributeGraph2,
        character: &Character<SimpleAttributeGraph2>,
        weapon: &Weapon<SimpleAttributeGraph2>,
        artifacts: &Vec<&Artifact>,
        enemy: &Enemy
    ) -> f64;
}