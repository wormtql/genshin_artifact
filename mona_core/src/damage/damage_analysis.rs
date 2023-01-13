use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::common::{DamageResult, Element};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct DamageAnalysis {
    pub atk: HashMap<String, f64>,
    pub atk_ratio: HashMap<String, f64>,
    pub hp: HashMap<String, f64>,
    pub hp_ratio: HashMap<String, f64>,
    pub def: HashMap<String, f64>,
    pub def_ratio: HashMap<String, f64>,
    pub em: HashMap<String, f64>,
    pub em_ratio: HashMap<String, f64>,
    pub extra_damage: HashMap<String, f64>,
    pub bonus: HashMap<String, f64>,
    pub critical: HashMap<String, f64>,
    pub critical_damage: HashMap<String, f64>,
    pub melt_enhance: HashMap<String, f64>,
    pub vaporize_enhance: HashMap<String, f64>,
    pub healing_bonus: HashMap<String, f64>,
    pub shield_strength: HashMap<String, f64>,
    pub spread_compose: HashMap<String, f64>,
    pub aggravate_compose: HashMap<String, f64>,

    pub def_minus: HashMap<String, f64>,
    pub def_penetration: HashMap<String, f64>,
    pub res_minus: HashMap<String, f64>,

    pub element: Element,
    pub is_heal: bool,
    pub is_shield: bool,

    pub normal: DamageResult,
    pub melt: Option<DamageResult>,
    pub vaporize: Option<DamageResult>,
    pub spread: Option<DamageResult>,
    pub aggravate: Option<DamageResult>,
}

pub struct HealAnalysis {
    pub atk: HashMap<String, f64>,
    pub atk_ratio: HashMap<String, f64>,
    pub hp: HashMap<String, f64>,
    pub hp_ratio: HashMap<String, f64>,
    pub def: HashMap<String, f64>,
    pub def_ratio: HashMap<String, f64>,
    pub extra_damage: HashMap<String, f64>,
}

// #[derive(Debug)]
// #[derive(Serialize, Deserialize)]
// pub struct DamageAnalysis {
//     pub atk: HashMap<String, f64>,
//     pub atk_ratio: HashMap<String, f64>,
//     pub hp: HashMap<String, f64>,
//     pub hp_ratio: HashMap<String, f64>,
//     pub def: HashMap<String, f64>,
//     pub def_ratio: HashMap<String, f64>,
//     pub extra_damage: HashMap<String, f64>,
//     pub bonus: HashMap<String, f64>,
//     pub critical: HashMap<String, f64>,
//     pub critical_damage: HashMap<String, f64>,
//
//     pub def_minus: HashMap<String, f64>,
//     pub res_minus: HashMap<String, f64>,
//
//     pub result: DamageResult,
// }