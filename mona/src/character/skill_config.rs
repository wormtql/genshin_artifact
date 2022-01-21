use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum CharacterSkillConfig {
    Albedo { fatal_count: usize },
    AratakiItto { after_q: bool },
    HuTao { after_e: bool },
    NoConfig
}