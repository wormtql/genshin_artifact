use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum WeaponConfig {
    NoConfig,
    MistsplitterReforgedConfig(i32),
}

impl Default for WeaponConfig {
    fn default() -> Self {
        WeaponConfig::NoConfig
    }
}