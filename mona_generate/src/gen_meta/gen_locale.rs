use std::collections::{HashMap, HashSet};
use mona::character::{CharacterName, CharacterStaticData};
use mona::common::i18n::I18nLocale;
use strum::*;
use lazy_static::lazy_static;
use mona::artifacts::artifact_trait::ArtifactMetaData;
use mona::artifacts::ArtifactSetName;
use mona::buffs::buff_meta::BuffMetaData;
use mona::buffs::buff_name::BuffName;
use mona::character::traits::CharacterSkillMap;
use mona::potential_function::potential_function_name::PotentialFunctionName;
use mona::target_functions::target_function_meta::TargetFunctionMeta;
use mona::target_functions::TargetFunctionName;
use mona::weapon::weapon_static_data::WeaponStaticData;
use mona::weapon::WeaponName;

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

pub fn collect_character_skill_names() -> Vec<I18nLocale> {
    let mut set = HashSet::new();
    for c in CharacterName::iter() {
        let skill_map: CharacterSkillMap = c.get_skill_map();
        if let Some(x) = skill_map.skill1 {
            for item in x.iter() {
                set.insert(item.text.clone());
            }
        }
        if let Some(x) = skill_map.skill2 {
            for item in x.iter() {
                set.insert(item.text.clone());
            }
        }
        if let Some(x) = skill_map.skill3 {
            for item in x.iter() {
                set.insert(item.text.clone());
            }
        }
    }
    set.into_iter().collect()
}

pub fn collect_weapon_names() -> Vec<I18nLocale> {
    let mut set = HashSet::new();
    for w in WeaponName::iter() {
        let meta: WeaponStaticData = w.get_static_data();
        set.insert(meta.name_locale.clone());
    }
    set.into_iter().collect()
}

pub fn collect_weapon_effect() -> Vec<I18nLocale> {
    let mut set = HashSet::new();
    for w in WeaponName::iter() {
        let meta: WeaponStaticData = w.get_static_data();
        if let Some(ref x) = meta.effect {
            set.insert(x.clone());
        }
    }
    set.into_iter().collect()
}

pub fn collect_tf_locale() -> Vec<I18nLocale> {
    let mut set = HashSet::new();
    for tf in TargetFunctionName::iter() {
        let meta: TargetFunctionMeta = tf.get_meta_data();
        set.insert(meta.name_locale.clone());
        set.insert(meta.description.clone());
    }
    set.into_iter().collect()
}

pub fn collect_buff_locale() -> Vec<I18nLocale> {
    let mut set = HashSet::new();
    for buff in BuffName::iter() {
        let meta: BuffMetaData = buff.get_meta();
        set.insert(meta.name_locale.clone());
        if let Some(ref x) = meta.description {
            set.insert(x.clone());
        }
    }
    set.into_iter().collect()
}

pub fn collect_artifact_locale() -> Vec<I18nLocale> {
    let mut set = HashSet::new();
    for artifact in ArtifactSetName::iter() {
        let meta: ArtifactMetaData = artifact.get_meta();
        set.insert(meta.name_locale.clone());
        if let Some(ref x) = meta.flower {
            set.insert(x.clone());
        }
        if let Some(ref x) = meta.feather {
            set.insert(x.clone());
        }
        if let Some(ref x) = meta.sand {
            set.insert(x.clone());
        }
        if let Some(ref x) = meta.goblet {
            set.insert(x.clone());
        }
        if let Some(ref x) = meta.head {
            set.insert(x.clone());
        }
        if let Some(ref x) = meta.effect1 {
            set.insert(x.clone());
        }
        if let Some(ref x) = meta.effect2 {
            set.insert(x.clone());
        }
        if let Some(ref x) = meta.effect3 {
            set.insert(x.clone());
        }
        if let Some(ref x) = meta.effect4 {
            set.insert(x.clone());
        }
        if let Some(ref x) = meta.effect5 {
            set.insert(x.clone());
        }
    }
    set.into_iter().collect()
}

pub fn collect_config_locale() -> Vec<I18nLocale> {
    let mut set = HashSet::new();

    for c in CharacterName::iter() {
        if let Some(x) = c.get_config_data() {
            for item in x.iter() {
                set.insert(item.title.clone());
            }
        }
        if let Some(x) = c.get_config_skill() {
            for item in x.iter() {
                set.insert(item.title.clone());
            }
        }
    }

    for w in WeaponName::iter() {
        if let Some(x) = w.get_config_data() {
            for item in x.iter() {
                set.insert(item.title.clone());
            }
        }
    }

    for tf in TargetFunctionName::iter() {
        if let Some(x) = tf.get_config() {
            for item in x.iter() {
                set.insert(item.title.clone());
            }
        }
    }

    for a in ArtifactSetName::iter() {
        if let Some(x) = a.get_config4() {
            for item in x.iter() {
                set.insert(item.title.clone());
            }
        }
        if let Some(x) = a.get_config2() {
            for item in x.iter() {
                set.insert(item.title.clone());
            }
        }
    }

    for pf in PotentialFunctionName::iter() {
        if let Some(x) = pf.get_config() {
            for item in x.iter() {
                set.insert(item.title.clone());
            }
        }
    }

    for b in BuffName::iter() {
        if let Some(x) = b.get_config() {
            for item in x.iter() {
                set.insert(item.title.clone());
            }
        }
    }

    set.into_iter().collect()
}

pub fn collect_locale() -> Vec<I18nLocale> {
    let mut set = HashSet::new();

    // collect character names
    set.extend(collect_character_names());
    set.extend(collect_character_skills());
    set.extend(collect_character_skill_names());

    set.extend(collect_weapon_names());
    set.extend(collect_weapon_effect());

    set.extend(collect_tf_locale());

    set.extend(collect_buff_locale());

    set.extend(collect_artifact_locale());

    set.extend(collect_config_locale());

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
