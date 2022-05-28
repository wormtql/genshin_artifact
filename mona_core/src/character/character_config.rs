use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum CharacterConfig {
    Ganyu { talent2_rate: f64 },
    HuTao { le_50: bool },
    KamisatoAyaka { talent1_rate: f64, talent2_rate: f64 },
    KamisatoAyato { use_c2: bool },
    Keqing { talent2_rate: f64 },
    KukiShinobu { hp_le_50: bool },
    Ningguang { talent2_rate: f64 },
    Razor { e_stack: f64, talent2_ratio: f64 },
    Yelan { team_element_count: usize },
    Yoimiya { talent1_level: f64 },
    NoConfig,
}