use serde::{Serialize, Deserialize};
use crate::applications::common::{ArtifactFilterConfig, BuffInterface, CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithmName;
use mona::artifacts::{Artifact, ArtifactSetName};
use mona::artifacts::effect_config::{ArtifactConfigInterface, ArtifactEffectConfig};
use mona::buffs::BuffConfig;

#[derive(Serialize, Deserialize)]
pub enum ConstraintSetMode {
    Any,
    Set2(ArtifactSetName),
    Set4(ArtifactSetName),
    Set22(ArtifactSetName, ArtifactSetName),
}

#[derive(Serialize, Deserialize)]
pub struct ConstraintConfig {
    pub set_mode: Option<ConstraintSetMode>,

    pub hp_min: Option<f64>,
    pub atk_min: Option<f64>,
    pub def_min: Option<f64>,
    pub recharge_min: Option<f64>,
    pub em_min: Option<f64>,
    pub crit_min: Option<f64>,
    pub crit_dmg_min: Option<f64>,
}

impl ConstraintConfig {
    pub fn is_any(&self) -> bool {
        if let Some(ref x) = self.set_mode {
            match x {
                ConstraintSetMode::Any => true,
                _ => false
            }
        } else {
            false
        }
    }

    // pub fn need_optimization(&self) -> bool {
    //     if !self.is_any() {
    //         false
    //     } else {
    //         // let recharge_min = self.recharge_min.unwrap_or(1.0);
    //         // let em_min = self.em_min.unwrap_or(0.0);
    //         // let crit_min = self.crit_min.unwrap_or(0.0);
    //         // let crit_dmg_min = self.crit_dmg_min.unwrap_or(0.0);
    //         //
    //         // let has_min = recharge_min > 1.0 || em_min > 0.0 || crit_min > 0.0 || crit_dmg_min > 0.0;
    //         // !has_min
    //     }
    // }
}

impl Default for ConstraintConfig {
    fn default() -> Self {
        ConstraintConfig {
            set_mode: None,
            hp_min: None,
            atk_min: None,
            def_min: None,
            recharge_min: None,
            em_min: None,
            crit_min: None,
            crit_dmg_min: None
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OptimizeArtifactInterface {
    pub artifact_config: Option<ArtifactConfigInterface>,
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub target_function: TargetFunctionInterface,
    pub constraint: Option<ConstraintConfig>,
    pub filter: Option<ArtifactFilterConfig>,
    pub buffs: Vec<BuffInterface>,
    #[serde(default)]
    pub algorithm: SingleOptimizeAlgorithmName,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PerStatBonus {
    pub atk_fixed: f64,
    pub atk_percentage: f64,
    pub def_fixed: f64,
    pub def_percentage: f64,
    pub hp_fixed: f64,
    pub hp_percentage: f64,
    pub elemental_mastery: f64,
    pub recharge: f64,
    pub critical: f64,
    pub critical_damage: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptimizationResult {
    pub flower: Option<u64>,
    pub feather: Option<u64>,
    pub sand: Option<u64>,
    pub goblet: Option<u64>,
    pub head: Option<u64>,
    pub value: f64,
    pub ratio: f64
}
