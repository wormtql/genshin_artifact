use lazy_static::lazy_static;
use super::CONFIG;
use std::fs;
use std::path::Path;
use mona::character::{CharacterName, CharacterStaticData};
use edit_distance::edit_distance;
use serde::{Deserialize};
use std::collections::HashMap;
use crate::utils::text_map::get_text_map;

#[derive(Deserialize, Debug)]
pub struct AvatarExcelDataItem {
    pub nameTextMapHash: u64,
    pub iconName: String,
}

lazy_static! {
    pub static ref CHARACTER_DATA: Vec<AvatarExcelDataItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/AvatarExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();
        serde_json::from_str(&s).unwrap()
    };

    pub static ref INTERNAL_CHARACTER_ITEM_MAP: HashMap<CharacterName, &'static AvatarExcelDataItem> = {
        let mut result = HashMap::new();
        let eng_text_map = get_text_map("eng");
        for i in 0..CharacterName::LEN {
            let name_enum: CharacterName = num::FromPrimitive::from_usize(i).unwrap();
            let meta: CharacterStaticData = name_enum.get_static_data();

            // get internal name
            let mut min_dis = 1000;
            let mut min_item = &CHARACTER_DATA[0];
            for item in CHARACTER_DATA.iter() {
                let dis = edit_distance(&eng_text_map.get(&item.nameTextMapHash.to_string()).unwrap(), &name_enum.to_string());
                if dis < min_dis {
                    min_dis = dis;
                    min_item = item;
                }
            }

            result.insert(name_enum, min_item);
        }

        result
    };
}

pub fn get_character_data() -> &'static [AvatarExcelDataItem] {
    &CHARACTER_DATA[..]
}

pub fn get_character_data_by_name(name: CharacterName) -> &'static AvatarExcelDataItem {
    *INTERNAL_CHARACTER_ITEM_MAP.get(&name).unwrap()
}