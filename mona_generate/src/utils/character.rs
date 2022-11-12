use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use mona::character::CharacterName;
use mona::character::traits::CharacterSkillMap;

pub fn get_character_dmg_names_chs() -> Vec<String> {
    let mut set = HashSet::new();
    let mut result = Vec::new();

    use CharacterName::*;
    let names_order = vec![
        AetherAnemo,
        Albedo,
        Aloy,
        Amber,
        AratakiItto,
        Barbara,
        Beidou,
        Bennett,
        Chongyun,
        Diluc,
        Diona,
        Eula,
        Fischl,
        Ganyu,
        Gorou,
        HuTao,
        Jean,
        KaedeharaKazuha,
        Kaeya,
        KamisatoAyaka,
        KamisatoAyato,
        Keqing,
        Klee,
        KujouSara,
        KukiShinobu,
        Lisa,
        Mona,
        Ningguang,
        Noelle,
        Qiqi,
        RaidenShogun,
        Razor,
        Rosaria,
        SangonomiyaKokomi,
        Sayu,
        Shenhe,
        ShikanoinHeizou,
        Sucrose,
        Tartaglia,
        Thoma,
        // Traveler,
        Venti,
        Xiangling,
        Xiao,
        Xingqiu,
        Xinyan,
        YaeMiko,
        Yanfei,
        Yelan,
        Yoimiya,
        Yunjin,
        Zhongli,
        Collei,
        Tighnari,
        Dori,
        Nilou,
        Candace,
        Cyno,
        Nahida,
        Layla,
    ];

    for &name in names_order.iter() {
        let skill_map: CharacterSkillMap = name.get_skill_map();

        let mut f = |s: &str| {
            if set.contains(s) {
                return;
            }
            set.insert(s.to_string());
            result.push(s.to_string());
        };

        if let Some(x) = skill_map.skill1 {
            for item in x.iter() {
                // set.insert(item.chs.to_string());
                f(item.chs);
            }
        }
        if let Some(x) = skill_map.skill2 {
            for item in x.iter() {
                // set.insert(item.chs.to_string());
                f(item.chs);
            }
        }
        if let Some(x) = skill_map.skill3 {
            for item in x.iter() {
                // set.insert(item.chs.to_string());
                f(item.chs);
            }
        }
    }

    // let mut temp: Vec<String> = set.into_iter().collect();
    // temp.sort();
    // temp
    result
}