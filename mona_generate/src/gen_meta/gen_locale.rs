use std::collections::{HashMap, HashSet};
use mona::character::{CharacterName, CharacterStaticData};
use mona::common::i18n::I18nLocale;
use strum::*;
use lazy_static::lazy_static;

pub fn collect_character_names() -> Vec<I18nLocale> {
    let mut set = HashSet::new();
    for c in CharacterName::iter() {
        let meta_data: CharacterStaticData = c.get_static_data();
        set.insert(meta_data.name_locale.clone());
    }
    set.into_iter().collect()
}

pub fn collect_character_skills() -> Vec<I18nLocale> {
    // it's impossible to duplicate
    let mut result = Vec::new();

    for c in CharacterName::iter() {
        let meta: CharacterStaticData = c.get_static_data();
        result.push(meta.skill_name1.clone());
        result.push(meta.skill_name2.clone());
        result.push(meta.skill_name3.clone());
    }
    result
}

pub fn collect_locale() -> Vec<I18nLocale> {
    let mut set = HashSet::new();

    // collect character names
    set.extend(collect_character_names());
    set.extend(collect_character_skills());

    set.into_iter().collect()
}

lazy_static! {
    static ref LOCALES: Vec<I18nLocale> = {
        let mut locs = collect_locale();
        locs.sort_by(|a, b| {
            a.zh_cn.cmp(&b.zh_cn)
        });
        locs
    };

    static ref LOCALE_INDEX_MAPPING: HashMap<I18nLocale, usize> = {
        let mut result = HashMap::new();
        for (index, item) in LOCALES.iter().enumerate() {
            result.insert(item.clone(), index);
        }
        result
    };
}

pub fn get_index_mapping() -> &'static HashMap<I18nLocale, usize> {
    &LOCALE_INDEX_MAPPING
}

pub fn generate_locale_vec(loc: &str) -> Vec<String> {
    let mut locs = LOCALES.clone();

    let mut result: Vec<String> = Vec::new();
    for item in locs.iter() {
        if loc == "zh_cn" || loc == "chs" || loc == "zh-cn" {
            result.push(String::from(item.zh_cn));
        } else if loc == "en" {
            result.push(String::from(item.en));
        }
    }


    result
}
