use askama::Template;
use mona::character::CharacterStaticData;
use mona::common::item_config_type::ItemConfig;
use mona::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use mona::target_functions::TargetFunctionName;
use crate::gen_meta::gen_locale::get_index_mapping;
use crate::utils::config_to_json;
use crate::utils::icon_hashmap::ICON_HASHMAP;

struct TFMeta {
    name: String,
    name_locale: usize,
    description: usize,
    tags: Vec<String>,
    four: String,
    badge_path: String,
    // character | misc
    badge_type: String,
    // if badge is character avatar, use mihoyo image url
    character_icon_name: String,
    icon_hash: String,
    config: Vec<String>,
}

#[derive(Template)]
#[template(path = "tf_meta_template.js")]
struct TFMetaTemplate {
    tfs: Vec<TFMeta>
}

fn convert_for(f: &TargetFunctionFor) -> String {
    match *f {
        TargetFunctionFor::Common => String::from("common"),
        TargetFunctionFor::SomeWho(c) => c.to_string()
    }
}

fn convert_badge_path(p: &TargetFunctionMetaImage, f: &TargetFunctionFor) -> String {
    match *p {
        TargetFunctionMetaImage::Avatar => {
            let name = match *f {
                TargetFunctionFor::SomeWho(c) => c.to_string(),
                _ => panic!(),
            };

            format!("characters/{}_avatar", name)
        },
        TargetFunctionMetaImage::Custom(s) => String::from(s)
    }
}

pub fn gen_tf_meta_as_js_file() -> String {
    let mut data: Vec<TFMeta> = Vec::new();
    let index_map = get_index_mapping();
    let icon_hashmap = &ICON_HASHMAP;

    for i in 0_usize..TargetFunctionName::LEN {
        let e: TargetFunctionName = num::FromPrimitive::from_usize(i).unwrap();

        let meta: TargetFunctionMeta = e.get_meta_data();
        let config = if let Some(x) = e.get_config() {
            x.iter().map(|c| config_to_json(&c)).collect()
        } else {
            Vec::new()
        };

        let icon_name: String = if let TargetFunctionMetaImage::Avatar = meta.image {
            if let TargetFunctionFor::SomeWho(c) = meta.four {
                let c_meta: CharacterStaticData = c.get_static_data();
                c_meta.internal_name.to_string()
            } else { String::new() }
        } else { String::new() };
        let icon_hash: String = icon_hashmap.get(icon_name.as_str())
            .map_or(String::new(), |&hash| hash.to_string());

        data.push(TFMeta {
            name: meta.name.to_string(),
            name_locale: *index_map.get(&meta.name_locale).unwrap(),
            description: *index_map.get(&meta.description).unwrap(),
            tags: meta.tags.split(",").map(|x| String::from(x)).collect(),
            four: convert_for(&meta.four),
            badge_path: convert_badge_path(&meta.image, &meta.four),
            badge_type: if let TargetFunctionMetaImage::Avatar = meta.image { String::from("character") } else { String::from("misc") },
            character_icon_name: format!("UI_AvatarIcon_{}", icon_name),
            icon_hash,
            config
        })
    }

    let t = TFMetaTemplate {
        tfs: data
    };

    t.render().unwrap()
}
