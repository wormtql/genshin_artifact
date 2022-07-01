use std::collections::HashMap;
use lazy_static::lazy_static;
use edit_distance::edit_distance;
use mona::weapon::WeaponName;
use std::fs;
use std::path::Path;
use mona::artifacts::artifact_trait::ArtifactMetaData;
use mona::artifacts::ArtifactSetName;
use mona::character::CharacterName;
use serde::{Deserialize};

pub mod character;
pub mod text_map;
pub mod artifact;
pub mod weapon;
pub mod equip_affix;
pub mod common;


#[derive(Deserialize)]
struct GenshinDataWeaponItem {
    nameTextMapHash: u64,
    id: u64,
    icon: String,
}

#[derive(Deserialize)]
struct GenshinDataArtifactSetItem {
    setId: u64,
    containsList: Vec<u64>,
}

#[derive(Deserialize, Clone)]
struct GenshinDataArtifactItem {
    id: u64,
    nameTextMapHash: u64,
    icon: String,
}

#[derive(Deserialize)]
struct Config {
    genshin_data_path: String
}

lazy_static! {
    static ref CONFIG: Config = {
        let s = include_str!("../../mona_generate_config.yaml");
        serde_yaml::from_str(&s).unwrap()
    };
}

lazy_static! {
    static ref WEAPON_DATA: Vec<GenshinDataWeaponItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/WeaponExcelConfigData.json");
        // let path = Path::new("./sub/genshindata/ExcelBinOutput/WeaponExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();

        serde_json::from_str::<Vec<GenshinDataWeaponItem>>(&s).unwrap()
    };

    static ref ENG_TEXT_MAP: HashMap<String, String> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("TextMap/TextMapEN.json");
        // let path = Path::new("./sub/genshindata/TextMap/TextMapEN.json");
        let s = fs::read_to_string(path).unwrap();

        serde_json::from_str::<HashMap<String, String>>(&s).unwrap()
    };

    static ref CHS_TEXT_MAP: HashMap<String, String> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("TextMap/TextMapCHS.json");
        // let path = Path::new("./sub/genshindata/TextMap/TextMapCHS.json");
        let s = fs::read_to_string(path).unwrap();

        serde_json::from_str::<HashMap<String, String>>(&s).unwrap()
    };
}

// artifacts data
lazy_static! {
    static ref ARTIFACT_SET_DATA: Vec<GenshinDataArtifactSetItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/ReliquarySetExcelConfigData.json");
        // let path = Path::new("./sub/genshindata/ExcelBinOutput/ReliquarySetExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();

        serde_json::from_str::<Vec<GenshinDataArtifactSetItem>>(&s).unwrap()
    };

    static ref ARTIFACT_DATA: HashMap<u64, GenshinDataArtifactItem> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("ExcelBinOutput/ReliquaryExcelConfigData.json");
        // let path = Path::new("./sub/genshindata/ExcelBinOutput/ReliquaryExcelConfigData.json");
        let s = fs::read_to_string(path).unwrap();

        let temp = serde_json::from_str::<Vec<GenshinDataArtifactItem>>(&s).unwrap();
        let mut result = HashMap::new();

        for item in temp {
            result.entry(item.id).or_insert(item);
        }

        result
    };
}

// character name to internal_name map
lazy_static! {
    static ref IMAGE_NAME_MAP: HashMap<CharacterName, &'static str> = {
        let mut m = HashMap::new();

        m.insert(CharacterName::Amber, "Ambor");
        m.insert(CharacterName::KamisatoAyaka, "Ayaka");
        m.insert(CharacterName::KamisatoAyato, "Ayato");
        m.insert(CharacterName::Yanfei, "Feiyan");
        m.insert(CharacterName::HuTao, "Hutao");
        m.insert(CharacterName::AratakiItto, "Itto");
        m.insert(CharacterName::KaedeharaKazuha, "Kazuha");
        m.insert(CharacterName::SangonomiyaKokomi, "Kokomi");
        m.insert(CharacterName::Noelle, "Noel");
        m.insert(CharacterName::KujouSara, "Sara");
        m.insert(CharacterName::RaidenShogun, "Shougun");
        m.insert(CharacterName::Thoma, "Tohma");
        m.insert(CharacterName::YaeMiko, "Yae");
        m.insert(CharacterName::AetherAnemo, "PlayerBoy");
        m.insert(CharacterName::Jean, "Qin");
        m.insert(CharacterName::KukiShinobu, "Shinobu");

        m
    };
}

pub fn get_internal_character_name(name: CharacterName) -> String {
    if IMAGE_NAME_MAP.contains_key(&name) {
        String::from(*IMAGE_NAME_MAP.get(&name).unwrap())
    } else {
        name.to_string()
    }
}

pub fn get_artifact_icon_name(chs: &str) -> String {
    let mut min = 100000;
    let mut result_id = 0;

    for (_, v) in ARTIFACT_DATA.iter() {
        let name = CHS_TEXT_MAP.get(&v.nameTextMapHash.to_string()).unwrap();
        let dis = edit_distance(&name, chs);
        if dis < min {
            min = dis;
            result_id = v.id;
        }
    }

    ARTIFACT_DATA.get(&result_id).unwrap().icon.clone()
}

pub fn get_artifact_icon_names(name: ArtifactSetName) -> Vec<Option<String>> {
    let meta: ArtifactMetaData = name.get_meta();

    let mut result = Vec::new();

    let mut temp = vec![&meta.flower, &meta.feather, &meta.sand, &meta.goblet, &meta.head];

    for position in temp.iter() {
        if let Some(x) = position {
            result.push(Some(get_artifact_icon_name(x)));
        } else {
            result.push(None);
        }
    }

    result
}

pub fn get_internal_weapon_name(name: WeaponName) -> String {
    let name = name.to_string();

    let mut min = 1000000;
    let mut result_index = 0;
    for (index, item) in WEAPON_DATA.iter().enumerate() {
        let id_s = item.nameTextMapHash.to_string();
        let name_eng = ENG_TEXT_MAP.get(&id_s).unwrap();

        let dis = edit_distance(&name, &name_eng);
        if dis < min {
            // println!("{}, {}", dis, name_eng);
            min = dis;
            result_index = index;
        }
    }

    let temp: Vec<_> = WEAPON_DATA[result_index].icon.split("_").skip(2).collect();

    temp.join("_")
}