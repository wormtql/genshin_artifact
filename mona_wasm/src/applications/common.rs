use serde::{Serialize, Deserialize};
use mona::artifacts::{Artifact, ArtifactSlotName};
use mona::attribute::{Attribute, SimpleAttributeGraph2};
use mona::buffs::buff_name::BuffName;
use mona::buffs::{Buff, BuffConfig};
use mona::character::{Character, CharacterConfig, CharacterName};
use mona::character::skill_config::CharacterSkillConfig;
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::potential_function::potential_function::PotentialFunction;
use mona::potential_function::potential_function_config::PotentialFunctionConfig;
use mona::potential_function::potential_function_name::PotentialFunctionName;
use mona::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName, TargetFunctionUtils};
use mona::weapon::{Weapon, WeaponConfig, WeaponName};

#[derive(Serialize, Deserialize)]
pub struct SkillInterface {
    pub index: usize,
    pub config: CharacterSkillConfig
}

#[derive(Serialize, Deserialize)]
pub struct CharacterInterface {
    pub name: CharacterName,
    pub level: usize,
    pub ascend: bool,
    pub constellation: i32,
    pub skill1: usize,
    pub skill2: usize,
    pub skill3: usize,
    pub params: CharacterConfig,
}

fn default_false() -> bool {
    false
}

impl CharacterInterface {
    pub fn to_character<T: Attribute>(&self) -> Character<T> {
        Character::new(
            self.name,
            self.level,
            self.ascend,
            self.constellation,
            self.skill1,
            self.skill2,
            self.skill3,
            &self.params
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct WeaponInterface {
    pub name: WeaponName,
    pub level: i32,
    pub ascend: bool,
    pub refine: i32,
    pub params: WeaponConfig,
}

impl WeaponInterface {
    pub fn to_weapon<T: Attribute>(&self, character: &Character<T>) -> Weapon<T> {
        Weapon::new(
            self.name,
            self.level,
            self.ascend,
            self.refine,
            &self.params,
            character
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct TargetFunctionInterface {
    pub name: TargetFunctionName,
    pub params: TargetFunctionConfig,
    #[serde(default = "default_false")]
    pub use_dsl: bool,
    pub dsl_source: Option<String>,
}

impl TargetFunctionInterface {
    pub fn to_target_function(&self, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>) -> Box<dyn TargetFunction> {
        TargetFunctionUtils::new_target_function(
            self.name,
            character,
            weapon,
            &self.params
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct BuffInterface {
    pub name: BuffName,
    pub config: BuffConfig,
}

impl BuffInterface {
    pub fn to_buff<A: Attribute>(&self) -> Box<dyn Buff<A>> {
        self.name.create(&self.config)
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnemyInterface {
    pub level: usize,
    pub electro_res: f64,
    pub pyro_res: f64,
    pub hydro_res: f64,
    pub cryo_res: f64,
    pub geo_res: f64,
    pub anemo_res: f64,
    pub dendro_res: f64,
    pub physical_res: f64
}

impl EnemyInterface {
    pub fn to_enemy(&self) -> Enemy {
        Enemy {
            level: self.level as i32,
            electro_res: self.electro_res,
            pyro_res: self.pyro_res,
            hydro_res: self.hydro_res,
            cryo_res: self.cryo_res,
            anemo_res: self.anemo_res,
            geo_res: self.geo_res,
            dendro_res: self.dendro_res,
            physical_res: self.physical_res
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArtifactFilterConfig {
    pub sand_main_stat: Option<Vec<StatName>>,
    pub goblet_main_stat: Option<Vec<StatName>>,
    pub head_main_stat: Option<Vec<StatName>>,
}

impl ArtifactFilterConfig {
    pub fn filter_artifact<'a>(&self, artifacts: &[&'a Artifact]) -> Vec<&'a Artifact> {
        let mut results: Vec<&Artifact> = Vec::new();

        use ArtifactSlotName::*;
        for artifact in artifacts.iter() {
            match artifact.slot {
                Flower | Feather => results.push(artifact),
                Sand => {
                    match self.sand_main_stat {
                        None => results.push(artifact),
                        Some(ref li) => {
                            if li.contains(&artifact.main_stat.0) || li.len() == 0 {
                                results.push(artifact);
                            }
                        }
                    }
                },
                Goblet => {
                    match self.goblet_main_stat {
                        None => results.push(artifact),
                        Some(ref li) => {
                            if li.contains(&artifact.main_stat.0) || li.len() == 0 {
                                results.push(artifact);
                            }
                        }
                    }
                },
                Head => {
                    match self.head_main_stat {
                        None => results.push(artifact),
                        Some(ref li) => {
                            if li.contains(&artifact.main_stat.0) || li.len() == 0 {
                                results.push(artifact);
                            }
                        }
                    }
                }
            }
        }

        results
    }
}

#[derive(Serialize, Deserialize)]
pub struct PotentialFunctionInterface {
    pub name: PotentialFunctionName,
    pub config: Option<PotentialFunctionConfig>,
}

impl PotentialFunctionInterface {
    pub fn to_pf(&self) -> Box<dyn PotentialFunction> {
        let no_config = PotentialFunctionConfig::NoConfig;
        let config = self.config.as_ref().unwrap_or(&no_config);
        self.name.create(config)
    }
}
