use askama::Template;
use std::collections::HashMap;
use mona::artifacts::artifact_trait::ArtifactMetaData;
use mona::artifacts::ArtifactSetName;
use mona::common::item_config_type::ItemConfig;
use crate::gen_meta::gen_locale::get_index_mapping;
use crate::utils::config_to_json;
use crate::utils::icon_hashmap::ICON_HASHMAP;

struct ArtifactMeta {
    name_locale: usize,
    name: String,
    name_mona: String,
    min_star: usize,
    max_star: usize,
    effect1: Option<usize>,
    effect2: Option<usize>,
    effect3: Option<usize>,
    effect4: Option<usize>,
    effect5: Option<usize>,
    config4: Vec<String>,

    flower: Option<usize>,
    feather: Option<usize>,
    sand: Option<usize>,
    goblet: Option<usize>,
    head: Option<usize>,
    
    flower_icon: String,
    feather_icon: String,
    sand_icon: String,
    goblet_icon: String,
    head_icon: String,

    flower_hash: String,
    feather_hash: String,
    sand_hash: String,
    goblet_hash: String,
    head_hash: String,
}

#[derive(Template)]
#[template(path = "artifact_meta_template.js")]
struct ArtifactMetaTemplate {
    artifacts: Vec<ArtifactMeta>
}

fn get_icon_hash(icon_hashmap: &HashMap<&str, &str>, icon: &str) -> String {
    icon_hashmap.get(icon).map_or(String::new(), |&hash| hash.to_string())
}

pub fn gen_artifact_meta_as_js_file() -> String {
    let mut data = Vec::new();
    let index_map = get_index_mapping();
    let icon_hashmap = &ICON_HASHMAP;

    for i in 1_usize..ArtifactSetName::LEN {
        let e: ArtifactSetName = num::FromPrimitive::from_usize(i).unwrap();
        let meta: ArtifactMetaData = e.get_meta();
        let config4: Option<&'static [ItemConfig]> = e.get_config4();

        let flower_icon: String = if let Some(_) = meta.flower { format!("UI_RelicIcon_{}_4", meta.internal_id) } else { String::new() };
        let feather_icon: String = if let Some(_) = meta.feather { format!("UI_RelicIcon_{}_2", meta.internal_id) } else { String::new() };
        let sand_icon: String = if let Some(_) = meta.sand { format!("UI_RelicIcon_{}_5", meta.internal_id) } else { String::new() };
        let goblet_icon: String = if let Some(_) = meta.goblet { format!("UI_RelicIcon_{}_1", meta.internal_id) } else { String::new() };
        let head_icon: String = if let Some(_) = meta.head { format!("UI_RelicIcon_{}_3", meta.internal_id) } else { String::new() };

        data.push(ArtifactMeta {
            name_locale: *index_map.get(&meta.name_locale).unwrap(),
            name: meta.name.to_string(),
            name_mona: String::from(meta.name_mona),
            min_star: meta.star.0,
            max_star: meta.star.1,
            effect1: if let Some(ref x) = meta.effect1 { Some(*index_map.get(x).unwrap()) } else { None },
            effect2: if let Some(ref x) = meta.effect2 { Some(*index_map.get(x).unwrap()) } else { None },
            effect3: if let Some(ref x) = meta.effect3 { Some(*index_map.get(x).unwrap()) } else { None },
            effect4: if let Some(ref x) = meta.effect4 { Some(*index_map.get(x).unwrap()) } else { None },
            effect5: if let Some(ref x) = meta.effect5 { Some(*index_map.get(x).unwrap()) } else { None },
            config4: config4.unwrap_or(&[]).iter().map(|x| config_to_json(x)).collect(),
            flower: if let Some(ref x) = meta.flower { Some(*index_map.get(x).unwrap()) } else { None },
            feather: if let Some(ref x) = meta.feather { Some(*index_map.get(x).unwrap()) } else { None },
            sand: if let Some(ref x) = meta.sand { Some(*index_map.get(x).unwrap()) } else { None },
            goblet: if let Some(ref x) = meta.goblet { Some(*index_map.get(x).unwrap()) } else { None },
            head: if let Some(ref x) = meta.head { Some(*index_map.get(x).unwrap()) } else { None },
            flower_icon: flower_icon.clone(),
            feather_icon: feather_icon.clone(),
            sand_icon: sand_icon.clone(),
            goblet_icon: goblet_icon.clone(),
            head_icon: head_icon.clone(),
            flower_hash: get_icon_hash(icon_hashmap, flower_icon.as_str()),
            feather_hash: get_icon_hash(icon_hashmap, feather_icon.as_str()),
            sand_hash: get_icon_hash(icon_hashmap, sand_icon.as_str()),
            goblet_hash: get_icon_hash(icon_hashmap, goblet_icon.as_str()),
            head_hash: get_icon_hash(icon_hashmap, head_icon.as_str()),
        })
    }

    let t = ArtifactMetaTemplate {
        artifacts: data
    };

    t.render().unwrap()
}
