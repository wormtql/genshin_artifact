use mona::buffs::buff_meta::{BuffImage, BuffMetaData};
use mona::buffs::buff_name::BuffName;
use askama::Template;
use mona::artifacts::artifact_trait::ArtifactMetaData;
use mona::character::CharacterStaticData;
use mona::weapon::weapon_static_data::WeaponStaticData;
use crate::gen_meta::gen_locale::get_index_mapping;
use crate::utils::config_to_json;

struct BuffMeta {
    name: String,
    name_locale: usize,
    image: String,
    // character | misc | weapon | artifact
    image_type: String,
    // if image_type is character, use internal name to get mihoyo image url, not using mine
    character_internal_name: String,
    weapon_internal_name: String,
    artifact_internal_name: String,
    genre: String,
    config: Vec<String>,
    description: Option<usize>,
}

#[derive(Template)]
#[template(path = "buff_meta_template.js")]
struct BuffMetaTemplate {
    buffs: Vec<BuffMeta>
}

fn convert_image(i: &BuffImage) -> String {
    match *i {
        BuffImage::Avatar(x) => format!("characters/{}_avatar", x.to_string()),
        BuffImage::Custom(x) => String::from(x),
        BuffImage::Misc(x) => format!("misc/{}", x),
        BuffImage::Weapon(x) => format!("weapons/{}_tn", x.to_string()),
        BuffImage::Artifact(x) => {
            let meta: ArtifactMetaData = x.get_meta();
            if meta.flower.is_some() {
                format!("artifacts/{}_flower", x.to_string())
            } else if meta.feather.is_some() {
                format!("artifacts/{}_feater", x.to_string())
            } else if meta.sand.is_some() {
                format!("artifacts/{}_sand", x.to_string())
            } else if meta.goblet.is_some() {
                format!("artifacts/{}_goblet", x.to_string())
            } else {
                format!("artifacts/{}_head", x.to_string())
            }
        }
    }
}

pub fn gen_buff_meta_as_js_file() -> String {
    let mut data: Vec<BuffMeta> = Vec::new();
    let index_map = get_index_mapping();

    for i in 0_usize..BuffName::LEN {
        let e: BuffName = num::FromPrimitive::from_usize(i).unwrap();

        let meta: BuffMetaData = e.get_meta();
        let config = if let Some(x) = e.get_config() {
            x.iter().map(|c| config_to_json(&c)).collect()
        } else {
            Vec::new()
        };

        let description = if let Some(ref x) = meta.description {
            Some(*index_map.get(x).unwrap())
        } else {
            None
        };

        let image_type = if let BuffImage::Avatar(_) = meta.image {
            String::from("character")
        } else if let BuffImage::Weapon(_) = meta.image {
            String::from("weapon")
        } else if let BuffImage::Artifact(_) = meta.image {
            String::from("artifact")
        } else {
            String::from("misc")
        };

        data.push(BuffMeta {
            name: meta.name.to_string(),
            name_locale: *index_map.get(&meta.name_locale).unwrap(),
            image: convert_image(&meta.image),
            image_type,
            character_internal_name: if let BuffImage::Avatar(c) = meta.image {
                let c_meta: CharacterStaticData = c.get_static_data();
                String::from(c_meta.internal_name)
            } else { String::new() },
            weapon_internal_name: if let BuffImage::Weapon(w) = meta.image {
                let w_meta: WeaponStaticData = w.get_static_data();
                String::from(w_meta.internal_name)
            } else {
                String::new()
            },
            artifact_internal_name: if let BuffImage::Artifact(a) = meta.image {
                let a_meta: ArtifactMetaData = a.get_meta();
                if let Some(_) = a_meta.flower {
                    format!("UI_RelicIcon_{}_4", a_meta.internal_id)
                } else {
                    String::new()
                }
            } else {
                String::new()
            },
            genre: meta.genre.to_string(),
            config,
            description,
        })
    }

    let t = BuffMetaTemplate {
        buffs: data
    };

    t.render().unwrap()
}
