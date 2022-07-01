use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use edit_distance::edit_distance;
use std::path::Path;
use std::fs;
use mona::weapon::WeaponName;
use crate::utils::common::merge_affix_string;
use crate::utils::text_map::get_text_map;
use super::CONFIG;
use crate::utils::equip_affix::{EQUIP_AFFIX_DATA, get_equip_affix_excel_config_data};

#[derive(Deserialize)]
pub struct WeaponExcelConfigDataItem {
    pub nameTextMapHash: u64,
    pub id: u64,
    pub icon: String,
    pub skillAffix: Vec<u64>,
}

impl WeaponExcelConfigDataItem {
    pub fn get_effect_string(&self, lang: &str) -> String {
        let mut items = Vec::new();
        let text_map = get_text_map(lang);

        for i in 0..5 {
            let affix_id = self.skillAffix[0] * 10 + i;
            let affix_item = match get_equip_affix_excel_config_data(affix_id) {
                Some(x) => x,
                None => {
                    // println!("{}", self.id);
                    continue;
                }
            };
            let text = text_map.get(affix_item.descTextMapHash.to_string().as_str()).unwrap();
            items.push(text.as_str());
        }

        merge_affix_string(&items)
    }
}

lazy_static! {
    pub static ref WEAPON_DATA: Vec<WeaponExcelConfigDataItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/WeaponExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();
        serde_json::from_str(&s).unwrap()
    };

    pub static ref WEAPON_DATA_MAP: HashMap<WeaponName, &'static WeaponExcelConfigDataItem> = {
        load_weapon_data_map()
    };
}

fn load_weapon_data_map() -> HashMap<WeaponName, &'static WeaponExcelConfigDataItem> {
    let mut result = HashMap::new();
    let eng_text_map = get_text_map("en");

    for i in 0..WeaponName::LEN {
        let name: WeaponName = num::FromPrimitive::from_usize(i).unwrap();

        let mut min_dis = 1000;
        let mut min_item = None;
        for item in WEAPON_DATA.iter() {
            let eng_name = eng_text_map.get(item.nameTextMapHash.to_string().as_str()).unwrap();
            let dis = edit_distance(eng_name.as_str(), name.to_string().as_str());

            if dis < min_dis {
                min_dis = dis;
                min_item = Some(item);
            }
        }

        result.insert(name, min_item.unwrap());
    }

    result
}


pub fn get_weapon_excel_config_data_item(name: WeaponName) -> &'static WeaponExcelConfigDataItem {
    *WEAPON_DATA_MAP.get(&name).unwrap()
}