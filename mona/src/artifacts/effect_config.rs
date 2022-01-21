use serde::{Serialize, Deserialize};

use crate::common::Element;
use crate::common::max_trait::MaxValue;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ConfigArchaicPetra {
    pub element: Element,
    pub rate: f64,
}

impl Default for ConfigArchaicPetra {
    fn default() -> Self {
        ConfigArchaicPetra {
            element: Element::NoElement,
            rate: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ConfigBlizzardStrayer {
    pub critical_bonus: f64,
}

impl Default for ConfigBlizzardStrayer {
    fn default() -> Self {
        ConfigBlizzardStrayer {
            critical_bonus: 0.0
        }
    }
}

impl MaxValue for ConfigBlizzardStrayer {
    fn max_value() -> Self {
        ConfigBlizzardStrayer {
            critical_bonus: 0.4
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ConfigPaleFlame {
    pub avg_level: f64,
    pub full_rate: f64,
}

impl Default for ConfigPaleFlame {
    fn default() -> Self {
        ConfigPaleFlame {
            avg_level: 0.0,
            full_rate: 0.0,
        }
    }
}

impl MaxValue for ConfigPaleFlame {
    fn max_value() -> Self {
        ConfigPaleFlame {
            avg_level: 2.0,
            full_rate: 1.0
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ConfigRate {
    pub rate: f64,
}

impl MaxValue for ConfigRate {
    fn max_value() -> Self {
        ConfigRate {
            rate: 1.0
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ConfigLevel {
    pub level: f64,
}

impl Default for ConfigRate {
    fn default() -> Self {
        ConfigRate {
            rate: 0.0,
        }
    }
}

impl Default for ConfigLevel {
    fn default() -> Self {
        ConfigLevel {
            level: 0.0,
        }
    }
}

#[derive(Default, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ArtifactEffectConfig {
    pub config_archaic_petra: ConfigArchaicPetra,
    pub config_berserker: ConfigRate,
    pub config_blizzard_strayer: ConfigBlizzardStrayer,
    pub config_bloodstained_chivalry: ConfigRate,
    pub config_brave_heart: ConfigRate,
    pub config_crimson_witch_of_flames: ConfigLevel,
    pub config_heart_of_depth: ConfigRate,
    pub config_husk_of_opulent_dreams: ConfigLevel,
    pub config_instructor: ConfigRate,
    pub config_lavawalker: ConfigRate,
    pub config_martial_artist: ConfigRate,
    pub config_noblesse_oblige: ConfigRate,
    pub config_pale_flame: ConfigPaleFlame,
    pub config_retracing_bolide: ConfigRate,
    pub config_shimenawas_reminiscence: ConfigRate,
    pub config_tenacity_of_the_millelith: ConfigRate,
    pub config_thundersoother: ConfigRate,
}
