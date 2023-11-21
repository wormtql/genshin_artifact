use std::collections::HashMap;
use askama::Template;
use mona::character::{CharacterName, CharacterStaticData};
use mona::character::traits::{CharacterSkillMap, CharacterSkillMapItem};
use mona::common::item_config_type::ItemConfig;
use lazy_static::lazy_static;
use crate::gen_meta::gen_locale::get_index_mapping;
use crate::utils::config_to_json;
use crate::utils::icon_hashmap::ICON_HASHMAP;

struct CharacterMeta {
    name: String,
    // name_for_image: String,
    internal_name: String,
    name_locale: usize,
    // icon_name: String,
    icon_hash: String,
    star: usize,
    skill1_name_index: usize,
    skill2_name_index: usize,
    skill3_name_index: usize,
    element: String,
    weapon: String,
    skill_map1: Vec<SkillMapItem>,
    skill_map2: Vec<SkillMapItem>,
    skill_map3: Vec<SkillMapItem>,
    config: Vec<String>,
    config_skill: Vec<String>,
}

struct SkillMapItem {
    index: usize,
    locale_index: usize,
}

#[derive(Template)]
#[template(path = "character_meta_template.js")]
struct CharacterMetaTemplate {
    characters: Vec<CharacterMeta>,
}

pub fn gen_character_meta_as_js_file() -> String {
    let mut data: Vec<CharacterMeta> = Vec::new();
    let index_mapping = get_index_mapping();
    let icon_hashmap = &ICON_HASHMAP;

    for i in 0_usize..CharacterName::LEN {
        let name_enum: CharacterName = num::FromPrimitive::from_usize(i).unwrap();
        let meta: CharacterStaticData = name_enum.get_static_data();

        let skill_map: CharacterSkillMap = name_enum.get_skill_map();
        let s1: Vec<SkillMapItem> = if let Some(x) = skill_map.skill1 {
            x.iter().cloned().map(|a| {
                SkillMapItem {
                    index: a.index,
                    locale_index: *index_mapping.get(&a.text).unwrap()
                }
            }).collect()
        } else {
            Vec::new()
        };

        let s2: Vec<SkillMapItem> = if let Some(x) = skill_map.skill2 {
            x.iter().cloned().map(|a| {
                SkillMapItem {
                    index: a.index,
                    locale_index: *index_mapping.get(&a.text).unwrap()
                }
            }).collect()
        } else {
            Vec::new()
        };

        let s3: Vec<SkillMapItem> = if let Some(x) = skill_map.skill3 {
            x.iter().cloned().map(|a| {
                SkillMapItem {
                    index: a.index,
                    locale_index: *index_mapping.get(&a.text).unwrap()
                }
            }).collect()
        } else {
            Vec::new()
        };

        let config_data: Vec<String> = if let Some(x) = name_enum.get_config_data() {
            x.iter().map(|c| config_to_json(&c)).collect()
        } else {
            Vec::new()
        };

        let config_skill: Vec<String> = if let Some(x) = name_enum.get_config_skill() {
            x.iter().map(|c| config_to_json(&c)).collect()
        } else {
            Vec::new()
        };

        let name_locale = *index_mapping.get(&meta.name_locale).unwrap();

        let icon_hash: String = icon_hashmap.get(meta.internal_name)
            .map_or(String::new(), |&hash| hash.to_string());

        data.push(CharacterMeta {
            name: meta.name.to_string(),
            internal_name: String::from(meta.internal_name),
            icon_hash,
            name_locale,
            star: meta.star as usize,
            skill1_name_index: *index_mapping.get(&meta.skill_name1).unwrap(),
            skill2_name_index: *index_mapping.get(&meta.skill_name2).unwrap(),
            skill3_name_index: *index_mapping.get(&meta.skill_name3).unwrap(),
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
    };

    t.render().unwrap()
}
