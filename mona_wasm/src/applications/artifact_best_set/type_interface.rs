use mona::artifacts::effect_config::ArtifactConfigInterface;
use crate::applications::common::{BuffInterface, CharacterInterface, EnemyInterface, TargetFunctionInterface, WeaponInterface};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CalcArtifactBestSetInterface {
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub artifact_config: Option<ArtifactConfigInterface>,
    pub target_function: TargetFunctionInterface,
    pub buffs: Option<Vec<BuffInterface>>,
    pub enemy: Option<EnemyInterface>,
}
