use lazy_static::lazy_static;
use super::CONFIG;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use edit_distance::edit_distance;
use regex::Regex;

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

fn remove_param(s: &str) -> String {
    let re = Regex::new("\\|\\{.*}").unwrap();
    let x = re.replace_all(s, "");
    x.to_string()
}

pub fn translate(txt: &str, src: &str, dst: &str) -> String {
    let map1 = get_text_map(src);
    let map2 = get_text_map(dst);

    // get key
    let mut min_dis = 100000;
    let mut min_key = String::from("0");
    for (k, v) in map1.iter() {
        let v = remove_param(&v);
        let dis = edit_distance(v.as_str(), txt);
        if dis < min_dis {
            min_dis = dis;
            min_key = k.clone();
        }
        if dis == 0 {
            break;
        }
    }
    println!("{}", min_key);

    let temp = map2.get(&min_key).unwrap().clone();
    remove_param(&temp)
}