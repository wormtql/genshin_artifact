use lazy_static::lazy_static;
use super::CONFIG;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

lazy_static! {
    pub static ref CHS_TEXT_MAP: HashMap<String, String> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("TextMap/TextMapCHS.json");
        let s = fs::read_to_string(path).unwrap();

        serde_json::from_str::<HashMap<String, String>>(&s).unwrap()
    };

    pub static ref ENG_TEXT_MAP: HashMap<String, String> = {
        let path = Path::new(&CONFIG.genshin_data_path).join("TextMap/TextMapEN.json");
        // let path = Path::new("./sub/genshindata/TextMap/TextMapEN.json");
        let s = fs::read_to_string(path).unwrap();

        serde_json::from_str::<HashMap<String, String>>(&s).unwrap()
    };
}

pub fn get_text_map(lang: &str) -> &HashMap<String, String> {
    if lang == "chs" || lang == "zh-cn" {
        &CHS_TEXT_MAP
    } else if lang == "eng" || lang == "en" {
        &ENG_TEXT_MAP
    } else {
        unimplemented!()
    }
}
