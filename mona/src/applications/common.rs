use serde::{Serialize, Deserialize};
use crate::attribute::{Attribute, SimpleAttributeGraph2};
use crate::character::{Character, CharacterConfig, CharacterName};
use crate::character::skill_config::CharacterSkillConfig;
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