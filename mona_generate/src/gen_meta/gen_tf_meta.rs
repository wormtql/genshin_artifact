use askama::Template;
use mona::common::item_config_type::ItemConfig;
use mona::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use mona::target_functions::TargetFunctionName;
use crate::utils::character::get_character_data_by_name;
use crate::utils::get_internal_character_name;

struct TFMeta {
    name: String,
    chs: String,
    description: String,
    tags: Vec<String>,
    four: String,
    badge_path: String,
    // character | misc
    badge_type: String,
    // if badge is character avatar, use mihoyo image url
    character_icon_name: String,
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

    for i in 0_usize..TargetFunctionName::LEN {
        let e: TargetFunctionName = num::FromPrimitive::from_usize(i).unwrap();

        let meta: TargetFunctionMeta = e.get_meta_data();
        let config = if let Some(x) = e.get_config() {
            x.iter().map(|c| c.to_json()).collect()
        } else {
            Vec::new()
        };



        data.push(TFMeta {
            name: meta.name.to_string(),
            chs: String::from(meta.chs),
            description: String::from(meta.description),
            tags: meta.tags.split(",").map(|x| String::from(x)).collect(),
            four: convert_for(&meta.four),
            badge_path: convert_badge_path(&meta.image, &meta.four),
            badge_type: if let TargetFunctionMetaImage::Avatar = meta.image { String::from("character") } else { String::from("misc") },
            character_icon_name: if let TargetFunctionMetaImage::Avatar = meta.image {
                if let TargetFunctionFor::SomeWho(c) = meta.four {
                    let avatar_excel_config_data = get_character_data_by_name(c);
                    avatar_excel_config_data.iconName.clone()
                } else {
                    String::new()
                }
            } else { String::new() },
            config
        })
    }

    let t = TFMetaTemplate {
        tfs: data
    };

    t.render().unwrap()
}
