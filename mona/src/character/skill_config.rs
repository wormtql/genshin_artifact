use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum CharacterSkillConfig {
    Albedo { fatal_count: usize },
    AratakiItto { after_q: bool },
    Diluc { pyro: bool },
    Eula { lightfall_stack: usize },
    HuTao { after_e: bool },
    KamisatoAyaka { after_dash: bool },
    NoConfig
}