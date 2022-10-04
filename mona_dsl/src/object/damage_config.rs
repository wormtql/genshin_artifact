use mona::character::CharacterName;
use mona::character::skill_config::CharacterSkillConfig;
use mona::common::Element;
use mona::common::SkillType;

#[derive(Debug,Default)]
pub struct  MonaCustomSkill {
    pub skill_type: Option<SkillType>,
    pub atk_ratio: f64,
    pub hp_ratio: f64,
    pub def_ratio: f64,
    //pub em_ratio: f64,
    pub base_dmg: f64,
    pub heal: bool,
    pub shield: bool,
    pub element: Option<Element>,
}

#[derive(Debug)]
pub struct MonaObjectDamageConfig {
    pub character_name: CharacterName,
    pub skill_index: usize,
    pub skill_config: CharacterSkillConfig,
    pub var_name: String,
    pub is_transformative: bool,
    pub is_custom: bool,
    pub custom_skill: MonaCustomSkill,
    pub fumo: Option<Element>,
}
