use std::collections::HashMap;
use mona::artifacts::artifact_trait::ArtifactMetaData;
use mona::artifacts::ArtifactSetName;
use crate::utils::artifact::get_parsed_artifact_data;
use crate::utils::text_map::get_text_map;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct ArtifactLocaleItem {
    setName: String,
    items: Vec<String>,
    effects: HashMap<String, String>,
}

pub fn gen_artifact_locale(lang: &str) -> String {
    let mut result: HashMap<String, ArtifactLocaleItem> = HashMap::new();
    let text_map = get_text_map(lang);

    for i in 0..ArtifactSetName::LEN {
        let name: ArtifactSetName = num::FromPrimitive::from_usize(i).unwrap();
        let meta: ArtifactMetaData = name.get_meta();
        let mona_name = meta.name_mona;

        let data = get_parsed_artifact_data(name);

        let set_name = text_map.get(data.set_name.to_string().as_str()).unwrap();
        let items: Vec<String> = data.item_names.iter().map(|x| {
            text_map.get(x.to_string().as_str()).unwrap().clone()
        }).collect();
        let mut effects = HashMap::new();
        for (&num, &text_key) in data.effect.iter() {
            let effect = text_map.get(text_key.to_string().as_str()).unwrap();
            effects.insert(num.to_string(), effect.clone());
        }

        let locale_item = ArtifactLocaleItem {
            setName: set_name.clone(),
            items,
            effects
        };

        result.insert(mona_name.to_string(), locale_item);
    }

    serde_json::to_string_pretty(&result).unwrap()
}