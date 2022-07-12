use std::collections::HashMap;
use askama::Template;
use mona::character::{CharacterName, CharacterStaticData};
use mona::character::traits::{CharacterSkillMap, CharacterSkillMapItem};
use mona::common::item_config_type::ItemConfig;
use lazy_static::lazy_static;
use crate::utils::character::{get_character_data, get_character_data_by_name, get_character_dmg_names_chs};
use crate::utils::get_internal_character_name;

struct CharacterMeta {
    name: String,
    // name_for_image: String,
    icon_name: String,
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
    characters: Vec<CharacterMeta>,
    dmg_name_map: HashMap<String, usize>,
}

fn get_dmg_name_map() -> HashMap<String, usize> {
    let v = get_character_dmg_names_chs();
    let mut result = HashMap::new();
    for (index, item) in v.into_iter().enumerate() {
        result.insert(item, index);
    }
    result
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

        let avatar_excel_config_data = get_character_data_by_name(name_enum);

        data.push(CharacterMeta {
            name: meta.name.to_string(),
            // name_for_image: get_internal_character_name(name_enum),
            icon_name: avatar_excel_config_data.iconName.clone(),
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
        characters: data,
        dmg_name_map: get_dmg_name_map(),
    };

    t.render().unwrap()
}
