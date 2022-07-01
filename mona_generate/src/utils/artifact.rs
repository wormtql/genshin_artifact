use std::collections::HashMap;
use lazy_static::lazy_static;
use super::CONFIG;
use std::fs;
use mona::artifacts::ArtifactSetName;
use edit_distance::edit_distance;
use crate::utils::text_map::get_text_map;
use std::path::Path;
use serde::{Serialize, Deserialize};
use crate::utils::equip_affix::EQUIP_AFFIX_DATA;

#[derive(Deserialize)]
pub struct ReliquarySetDataItem {
    pub setId: u64,
    pub containsList: Vec<u64>,
    pub EquipAffixId: Option<u64>,
    pub setNeedNum: Vec<usize>,
}

impl ReliquarySetDataItem {
    pub fn get_name_key(&self) -> u64 {
        let id = self.EquipAffixId.unwrap() * 10;
        let equip_item = EQUIP_AFFIX_DATA.get(&id).unwrap();
        equip_item.nameTextMapHash
    }

    pub fn get_items_name_key(&self) -> Vec<u64> {
        let mut result = Vec::new();
        for &id in self.containsList.iter() {
            let item = RELIQUARY_DATA.get(&id).unwrap();
            result.push(item.nameTextMapHash);
        }
        result
    }

    pub fn get_effects_name_key(&self) -> HashMap<usize, u64> {
        let mut result = HashMap::new();
        for (index, &num) in self.setNeedNum.iter().enumerate() {
            let affix_id = self.EquipAffixId.unwrap() * 10 + index as u64;
            let affix_item = EQUIP_AFFIX_DATA.get(&affix_id).unwrap();

            result.insert(num, affix_item.descTextMapHash);
        }
        result
    }
}

#[derive(Deserialize)]
pub struct ReliquaryDataItem {
    id: u64,
    nameTextMapHash: u64,
    icon: String,
}

#[derive(Debug)]
pub struct ParsedArtifactData {
    pub effect: HashMap<usize, u64>,
    pub set_name: u64,
    pub item_names: Vec<u64>,
}

lazy_static! {
    pub static ref RELIQUARY_SET_DATA: HashMap<u64, ReliquarySetDataItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/ReliquarySetExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();
        let temp = serde_json::from_str::<Vec<ReliquarySetDataItem>>(&s).unwrap()
            .into_iter().filter(|x| x.EquipAffixId.is_some()).collect::<Vec<_>>();

        let mut result = HashMap::new();
        for item in temp {
            result.insert(item.setId, item);
        }
        result
    };

    pub static ref RELIQUARY_DATA: HashMap<u64, ReliquaryDataItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/ReliquaryExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();
        let temp = serde_json::from_str::<Vec<ReliquaryDataItem>>(&s).unwrap();

        let mut result = HashMap::new();
        for item in temp {
            result.insert(item.id, item);
        }
        result
    };

    pub static ref ARTIFACT_ITEM_MAP: HashMap<ArtifactSetName, ParsedArtifactData> = {
        get_artifact_item_map()
    };

    pub static ref ARTIFACT_SET_MAP: HashMap<ArtifactSetName, &'static ReliquarySetDataItem> = {
        get_reliquary_set_data_map()
    };
}

pub fn get_reliquary_set_data_map() -> HashMap<ArtifactSetName, &'static ReliquarySetDataItem> {
    let mut result = HashMap::new();
    let eng_text_map = get_text_map("eng");

    for i in 0..ArtifactSetName::LEN {
        let name: ArtifactSetName = num::FromPrimitive::from_usize(i).unwrap();

        let mut min_dis = 10000;
        let mut min_item = None;
        for (set_id, item) in RELIQUARY_SET_DATA.iter() {
            let name_key = item.get_name_key();
            let eng_name = eng_text_map.get(&name_key.to_string()).unwrap();
            let dis = edit_distance(&name.to_string(), eng_name.as_str());

            if dis < min_dis {
                min_dis = dis;
                min_item = Some(item);
            }
        }

        result.insert(name, min_item.unwrap());
    }

    result
}

pub fn get_artifact_item_map() -> HashMap<ArtifactSetName, ParsedArtifactData> {
    let mut result = HashMap::new();

    for i in 0..ArtifactSetName::LEN {
        let name: ArtifactSetName = num::FromPrimitive::from_usize(i).unwrap();

        let reliquary_set_item = ARTIFACT_SET_MAP.get(&name).unwrap();

        let item = ParsedArtifactData {
            effect: reliquary_set_item.get_effects_name_key(),
            set_name: reliquary_set_item.get_name_key(),
            item_names:reliquary_set_item.get_items_name_key()
        };

        result.insert(name, item);
    }

    result
}

pub fn get_parsed_artifact_data(name: ArtifactSetName) -> &'static ParsedArtifactData {
    ARTIFACT_ITEM_MAP.get(&name).unwrap()
}
