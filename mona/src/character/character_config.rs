use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum CharacterConfig {
    Ganyu { talent1_rate: f64, talent2_rate: f64 },
    HuTao { le_50: bool },
    KamisatoAyaka { talent1_rate: f64, talent2_rate: f64 },
    Keqing { talent2_rate: f64 },
    Ningguang { talent2_rate: f64 },
    Razor { e_stack: f64, talent2_ratio: f64 },
    Yoimiya { talent1_level: f64 },
    NoConfig,
}