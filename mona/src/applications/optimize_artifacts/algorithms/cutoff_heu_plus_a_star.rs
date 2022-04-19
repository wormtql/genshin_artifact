use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::algorithms::cutoff_a_star::AStarCutoff;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};
use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::buffs::Buff;
use crate::character::Character;
use crate::enemies::Enemy;
use crate::target_functions::TargetFunction;
use crate::weapon::Weapon;

pub struct CutoffHeuristicPlusAStar;


impl SingleOptimizeAlgorithm for CutoffHeuristicPlusAStar {
    fn optimize(&self, artifacts: &[&Artifact], artifact_config: Option<ArtifactEffectConfig>, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, target_function: &Box<dyn TargetFunction>, enemy: &Enemy, buffs: &[Box<dyn Buff<SimpleAttributeGraph2>>], constraint: &ConstraintConfig, count: usize) -> Vec<OptimizationResult> {
        let target_function_opt_config = target_function.get_target_function_opt_config();

        let artifacts_vec: Vec<&Artifact> = artifacts.iter().cloned().collect();
        let filtered_artifacts = target_function_opt_config.filter(artifacts_vec);

        let a_star_algo = AStarCutoff;

        a_star_algo.optimize(
            &filtered_artifacts,
            artifact_config,
            character,
            weapon,
            target_function,
            enemy,
            buffs,
            constraint,
            count
        )
    }
}