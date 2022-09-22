use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CharacterConfig {
    Ganyu { talent2_rate: f64 },
    HuTao { le_50: bool },
    KamisatoAyaka { talent1_rate: f64, talent2_rate: f64 },
    Keqing { talent2_rate: f64 },
    KukiShinobu { hp_le_50: bool, use_c6: bool },
    Ningguang { talent2_rate: f64 },
    Rosaria { e_from_behind: bool },
    Razor { e_stack: f64, talent2_ratio: f64 },
    Yelan { team_element_count: usize },
    Yoimiya { talent1_level: f64 },
    Collei { background: bool },
    Tighnari { talent1_ratio: f64, c2_ratio: f64 },
    Cyno { c2_stack: f64, after_q: bool },
    Nilou { golden_rate: f64 },
    Candace { c2_rate: f64 },
    NoConfig,
}