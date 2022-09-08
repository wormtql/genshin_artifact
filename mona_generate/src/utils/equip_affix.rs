use lazy_static::lazy_static;
use std::path::Path;
use std::fs;
use super::CONFIG;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct EquipAffixDataItem {
    pub id: u64,
    pub affixId: u64,
    pub nameTextMapHash: u64,
    pub descTextMapHash: u64,
}

lazy_static! {
    pub static ref EQUIP_AFFIX_DATA: HashMap<u64, EquipAffixDataItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/EquipAffixExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();
        let temp = serde_json::from_str::<Vec<EquipAffixDataItem>>(&s).unwrap();

        let mut result = HashMap::new();
        for item in temp {
            result.insert(item.affixId, item);
        }
        result
    };
}

pub fn get_equip_affix_excel_config_data(id: u64) -> Option<&'static EquipAffixDataItem> {
    EQUIP_AFFIX_DATA.get(&id)
}