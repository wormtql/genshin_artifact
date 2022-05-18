use std::collections::HashMap;
use askama::Template;
use crate::character::{CharacterName, CharacterStaticData};
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem};
use crate::common::item_config_type::ItemConfig;
use lazy_static::lazy_static;

struct CharacterMeta {
    name: String,
    name_for_image: String,
    chs: String,
    star: usize,
    skill1_name: String,
    skill2_name: String,
    skill3_name: String,
    element: String,
    weapon: String,
    skill_map1: Vec<CharacterSkillMapItem>,
    skill_map2: Vec<CharacterSkillMapItem>,
    skill_map3: Vec<CharacterSkillMapItem>,
    config: Vec<String>,
    config_skill: Vec<String>,
}

#[derive(Template)]
#[template(path = "character_meta_template.js")]
struct CharacterMetaTemplate {
    characters: Vec<CharacterMeta>
}

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

        m
    };
}

pub fn convert_image_name(name: CharacterName) -> String {
    if IMAGE_NAME_MAP.contains_key(&name) {
        String::from(*IMAGE_NAME_MAP.get(&name).unwrap())
    } else {
        name.to_string()
    }
}

pub fn gen_character_meta_as_js_file() -> String {
    let mut data: Vec<CharacterMeta> = Vec::new();

    for i in 0_usize..CharacterName::LEN {
        let name_enum: CharacterName = num::FromPrimitive::from_usize(i).unwrap();
        let meta: CharacterStaticData = name_enum.get_static_data();

        let skill_map: CharacterSkillMap = name_enum.get_skill_map();
        let s1: Vec<CharacterSkillMapItem> = if let Some(x) = skill_map.skill1 {
            x.iter().cloned().collect()
        } else {
            Vec::new()
        };

        let s2: Vec<CharacterSkillMapItem> = if let Some(x) = skill_map.skill2 {
            x.iter().cloned().collect()
        } else {
            Vec::new()
        };

        let s3: Vec<CharacterSkillMapItem> = if let Some(x) = skill_map.skill3 {
            x.iter().cloned().collect()
        } else {
            Vec::new()
        };

        let config_data: Vec<String> = if let Some(x) = name_enum.get_config_data() {
            x.iter().map(|c| c.to_json()).collect()
        } else {
            Vec::new()
        };

        let config_skill: Vec<String> = if let Some(x) = name_enum.get_config_skill() {
            x.iter().map(|c| c.to_json()).collect()
        } else {
            Vec::new()
        };

        data.push(CharacterMeta {
            name: meta.name.to_string(),
            name_for_image: convert_image_name(name_enum),
            chs: String::from(meta.chs),
            star: meta.star as usize,
            skill1_name: String::from(meta.skill_name1),
            skill2_name: String::from(meta.skill_name2),
            skill3_name: String::from(meta.skill_name3),
            element: meta.element.to_string(),
            weapon: meta.weapon_type.to_string(),
            skill_map1: s1,
            skill_map2: s2,
            skill_map3: s3,
            config: config_data,
            config_skill
        })
    }

    let t = CharacterMetaTemplate {
        characters: data
    };

    t.render().unwrap()
}
