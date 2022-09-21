use lazy_static::lazy_static;
use super::CONFIG;
use std::fs;
use std::path::Path;
use mona::character::{CharacterName, CharacterStaticData};
use edit_distance::edit_distance;
use serde::{Deserialize};
use std::collections::{HashMap, HashSet};
use mona::character::traits::CharacterSkillMap;
use crate::utils::skill::{get_avatar_skill_data_item, get_avatar_skill_depot_data_item};
use crate::utils::text_map::get_text_map;

#[derive(Deserialize, Debug)]
pub struct AvatarExcelDataItem {
    pub nameTextMapHash: u64,
    pub iconName: String,
    pub skillDepotId: u64,
}

impl AvatarExcelDataItem {
    pub fn get_skill_names(&self) -> Vec<u64> {
        let mut result = Vec::new();

        let skill_depot = get_avatar_skill_depot_data_item(self.skillDepotId);
        for i in 0..2 {
            let skill_item = get_avatar_skill_data_item(skill_depot.skills[i]);
            result.push(skill_item.nameTextMapHash);
        }
        if let Some(x) = skill_depot.energySkill {
            let skill_item = get_avatar_skill_data_item(x);
            result.push(skill_item.nameTextMapHash);
        }

        result
    }
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
                let name_eng = match eng_text_map.get(&item.nameTextMapHash.to_string()) {
                    Some(x) => x.clone(),
                    None => continue,
                };

                let dis = edit_distance(&name_eng, &name_enum.to_string());
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

pub fn get_character_dmg_names_chs() -> Vec<String> {
    let mut set = HashSet::new();
    let mut result = Vec::new();

    use CharacterName::*;
    let names_order = vec![
        AetherAnemo,
        Albedo,
        Aloy,
        Amber,
        AratakiItto,
        Barbara,
        Beidou,
        Bennett,
        Chongyun,
        Diluc,
        Diona,
        Eula,
        Fischl,
        Ganyu,
        Gorou,
        HuTao,
        Jean,
        KaedeharaKazuha,
        Kaeya,
        KamisatoAyaka,
        KamisatoAyato,
        Keqing,
        Klee,
        KujouSara,
        KukiShinobu,
        Lisa,
        Mona,
        Ningguang,
        Noelle,
        Qiqi,
        RaidenShogun,
        Razor,
        Rosaria,
        SangonomiyaKokomi,
        Sayu,
        Shenhe,
        ShikanoinHeizou,
        Sucrose,
        Tartaglia,
        Thoma,
        // Traveler,
        Venti,
        Xiangling,
        Xiao,
        Xingqiu,
        Xinyan,
        YaeMiko,
        Yanfei,
        Yelan,
        Yoimiya,
        Yunjin,
        Zhongli,
        Collei,
        Tighnari,
        Dori,
        Nilou,
        Candace,
        Cyno,
    ];

    for &name in names_order.iter() {
        let skill_map: CharacterSkillMap = name.get_skill_map();

        let mut f = |s: &str| {
            if set.contains(s) {
                return;
            }
            set.insert(s.to_string());
            result.push(s.to_string());
        };

        if let Some(x) = skill_map.skill1 {
            for item in x.iter() {
                // set.insert(item.chs.to_string());
                f(item.chs);
            }
        }
        if let Some(x) = skill_map.skill2 {
            for item in x.iter() {
                // set.insert(item.chs.to_string());
                f(item.chs);
            }
        }
        if let Some(x) = skill_map.skill3 {
            for item in x.iter() {
                // set.insert(item.chs.to_string());
                f(item.chs);
            }
        }
    }

    // let mut temp: Vec<String> = set.into_iter().collect();
    // temp.sort();
    // temp
    result
}