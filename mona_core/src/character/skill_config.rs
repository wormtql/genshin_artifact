use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CharacterSkillConfig {
    Albedo { fatal_count: usize },
    Aloy { coil_count: usize },
    AratakiItto { after_q: bool },

    Diluc { pyro: bool },
    Eula { lightfall_stack: usize },
    Ganyu { talent1_rate: f64 },
    HuTao { after_e: bool },
    KaedeharaKazuha { after_e_or_q: bool },
    KamisatoAyaka { #[serde(default = "default_true")] after_dash: bool, #[serde(default = "default_false")] use_c6: bool },
    KamisatoAyato { e_stack: usize, in_q: bool },
    Keqing { after_e: bool },
    Noelle { after_q: bool },
    RaidenShogun { under_e: bool, resolve_stack: usize },
    SangonomiyaKokomi { after_q: bool },
    Xiao { after_q: bool, talent1_stack: f64, talent2_stack: f64 },
    Xingqiu { c4: bool },
    Xinyan { shield_rate: f64 },
    Yanfei { after_q: bool },
    Yoimiya { after_e: bool },
    Dori { c6: bool },
    Candace { crown: bool },
    Cyno { under_judication: bool },
    Nahida { q_bonus: bool, q_bonus_count: usize },
    Wanderer { e_enabled: bool, e_hydro: bool, sdpoints: f64 },
    Faruzan { talent2_ratio: f64 },
    Alhaitham { under_e: bool },
    Dehya { c2_rate: f64, c6_stack: f64 },
    Kaveh { after_q: bool },
    Freminet { talent2_rate: f64 },
    Lyney { prop_stack: f64, under_pyro: bool, pyro_count: usize, },
    Neuvillette { talent1_stack: usize },
    Wriothesley { under_chilling_penalty: bool },
    Furina { hp_above50_count: usize, #[serde(default = "default_false")] c6_after_e: bool, #[serde(default = "default_false")] c6_pneuma: bool },
    Navia { shard_count: usize, strike11: bool, after_e: bool },
    Gaming { pyro: bool },
    Arlecchino { bond_of_life: f64 },
    Clorinde { bond_of_life: f64 },
    Emilie { enemy_burn: bool, use_c6: bool },
    NoConfig,
}
