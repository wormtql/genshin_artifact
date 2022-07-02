use std::collections::HashMap;
use lazy_static::lazy_static;
use super::CONFIG;
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct AvatarSkillDepotExcelConfigDataItem {
    pub id: u64,
    pub energySkill: Option<u64>,
    pub skills: Vec<u64>,
}

#[derive(Deserialize)]
pub struct AvatarSkillExcelConfigDataItem {
    pub id: u64,
    pub nameTextMapHash: u64,
    pub descTextMapHash: u64,
}

lazy_static! {
    pub static ref SKILL_DEPOT_DATA: HashMap<u64, AvatarSkillDepotExcelConfigDataItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/AvatarSkillDepotExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();
        let items: Vec<AvatarSkillDepotExcelConfigDataItem> = serde_json::from_str(&s).unwrap();

        let mut result = HashMap::new();
        for item in items {
            result.insert(item.id, item);
        }
        result
    };

    pub static ref SKILL_DATA: HashMap<u64, AvatarSkillExcelConfigDataItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/AvatarSkillExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();
        let items: Vec<AvatarSkillExcelConfigDataItem> = serde_json::from_str(&s).unwrap();

        let mut result = HashMap::new();
        for item in items {
            result.insert(item.id, item);
        }
        result
    };
}

pub fn get_avatar_skill_depot_data_item(id: u64) -> &'static AvatarSkillDepotExcelConfigDataItem {
    SKILL_DEPOT_DATA.get(&id).unwrap()
}

pub fn get_avatar_skill_data_item(id: u64) -> &'static AvatarSkillExcelConfigDataItem {
    SKILL_DATA.get(&id).unwrap()
}