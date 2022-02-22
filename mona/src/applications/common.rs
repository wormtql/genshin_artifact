use serde::{Serialize, Deserialize};
use crate::attribute::{Attribute, SimpleAttributeGraph2};
use crate::buffs::buff_name::BuffName;
use crate::buffs::{Buff, BuffConfig};
use crate::character::{Character, CharacterConfig, CharacterName};
use crate::character::skill_config::CharacterSkillConfig;
use crate::enemies::Enemy;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName, TargetFunctionUtils};
use crate::weapon::{Weapon, WeaponConfig, WeaponName};

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
