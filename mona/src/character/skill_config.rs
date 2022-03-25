use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum CharacterSkillConfig {
    Albedo { fatal_count: usize },
    Aloy { coil_count: usize },
    AratakiItto { after_q: bool },
    Diluc { pyro: bool },
    Eula { lightfall_stack: usize },
    HuTao { after_e: bool },
    KamisatoAyaka { after_dash: bool, use_c6: bool },
    Keqing { after_e: bool },
    Noelle { after_q: bool },
    RaidenShogun { under_e: bool, resolve_stack: usize },
    Rosaria { e_from_behind: bool },
    SangonomiyaKokomi { after_q: bool },
    Xiao { after_q: bool, talent1_stack: f64, talent2_stack: f64 },
    Xingqiu { c4: bool },
    Yanfei { after_q: bool },
    Yoimiya { after_e: bool },
    NoConfig
}
