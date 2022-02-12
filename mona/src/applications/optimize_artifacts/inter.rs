use serde::{Serialize, Deserialize};
use crate::applications::common::{BuffInterface, CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::buffs::BuffConfig;

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
    pub artifacts: Vec<Artifact>,
    pub artifact_config: Option<ArtifactEffectConfig>,
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub target_function: TargetFunctionInterface,
    pub constraint: Option<ConstraintConfig>,
    pub buffs: Vec<BuffInterface>,
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
    pub flower: Option<usize>,
    pub feather: Option<usize>,
    pub sand: Option<usize>,
    pub goblet: Option<usize>,
    pub head: Option<usize>,
    pub value: f64,
    pub ratio: f64
}
