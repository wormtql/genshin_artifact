use std::collections::HashMap;
use mona::common::i18n::I18nLocale;
use mona::common::item_config_type::{ItemConfig, ItemConfigType};
use serde_json::json;
use crate::gen_meta::gen_locale::get_index_mapping;

pub mod icon_hashmap;

pub fn config_to_json(config: &ItemConfig) -> String {
    let index_map = get_index_mapping();
    let name = config.name;
    let title_index = *index_map.get(&config.title).unwrap();

    let j = match config.config {
        ItemConfigType::Skill4 { default } => {
            json!({
                    "type": "skill4",
                    "title": title_index,
                    "name": name,
                    "default": default
                })
        },
        ItemConfigType::IntInput { min, max, default } => {
            json!({
                    "type": "intInput",
                    "title": title_index,
                    "name": name,
                    "min": min,
                    "max": max,
                    "default": default
                })
        },
        ItemConfigType::Element4 { default } => {
            json!({
                    "type": "element4",
                    "title": title_index,
                    "name": name,
                    "default": default
                })
        },
        ItemConfigType::Element8 { default } => {
            json!({
                    "type": "element8",
                    "title": title_index,
                    "name": name,
                    "default": default
                })
        },
        ItemConfigType::FloatPercentageInput { default } => {
            json!({
                    "type": "floatPercentageInput",
                    "title": title_index,
                    "name": name,
                    "default": default
                })
        },
        ItemConfigType::FloatInput { default } => {
            json!({
                    "type": "floatInput",
                    "title": title_index,
                    "name": name,
                    "default": default
                })
        }
        ItemConfigType::Float { min, max, default } => {
            json!({
                    "type": "float",
                    "title": title_index,
                    "name": name,
                    "min": min,
                    "max": max,
                    "default": default
                })
        },
        ItemConfigType::Int { min, max, default } => {
            json!({
                    "type": "int",
                    "title": title_index,
                    "name": name,
                    "min": min,
                    "max": max,
                    "default": default
                })
        },
        ItemConfigType::Bool { default } => {
            json!({
                    "type": "bool",
                    "title": title_index,
                    "name": name,
                    "default": default
                })
        },
        ItemConfigType::Option { options, default } => {
            let temp: Vec<&str> = options.split(",").collect();
            json!({
                    "type": "option",
                    "title": title_index,
                    "name": name,
                    "default": default,
                    "options": temp
                })
        }
    };

    j.to_string()
}