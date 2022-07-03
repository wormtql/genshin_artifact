use serde::{Serialize, Deserialize};
use crate::artifacts::Artifact;
use crate::artifacts::eff::{ARTIFACT_EFF4, ARTIFACT_EFF5};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::potential_function::potential_function::{PotentialFunction, PotentialFunctionMeta, PotentialFunctionMetaData};
use crate::potential_function::potential_function_config::PotentialFunctionConfig;
use crate::potential_function::potential_function_name::PotentialFunctionName;

pub type PotentialFunctionArtifactEffConfig = PotentialFunctionArtifactEff;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PotentialFunctionArtifactEff {
    pub atk_use: bool,
    pub atk_weight: f64,

    pub atk_p_use: bool,
    pub atk_p_weight: f64,

    pub hp_use: bool,
    pub hp_weight: f64,

    pub hp_p_use: bool,
    pub hp_p_weight: f64,

    pub def_use: bool,
    pub def_weight: f64,

    pub def_p_use: bool,
    pub def_p_weight: f64,

    pub critical_use: bool,
    pub critical_weight: f64,

    pub critical_damage_use: bool,
    pub critical_damage_weight: f64,

    pub elemental_mastery_use: bool,
    pub elemental_mastery_weight: f64,

    pub recharge_use: bool,
    pub recharge_weight: f64
}

impl PotentialFunction for PotentialFunctionArtifactEff {
    fn potential(&self, artifact: &Artifact) -> f64 {
        let mut result = 0.0;

        let mut temp = [0.0; 20];
        // temp[artifact.main_stat.0 as usize] += artifact.main_stat.1;
        for stat in artifact.sub_stats.iter() {
            let denominator = if artifact.star == 5 {
                ARTIFACT_EFF5.get_value(stat.0, 3)
            } else {
                ARTIFACT_EFF4.get_value(stat.0, 3)
            };
            temp[stat.0 as usize] += stat.1 / denominator;
        }

        if self.atk_use {
            result += temp[StatName::ATKFixed as usize] * self.atk_weight;
        }
        if self.atk_p_use {
            result += temp[StatName::ATKPercentage as usize] * self.atk_p_weight;
        }
        if self.hp_use {
            result += temp[StatName::HPFixed as usize] * self.hp_weight;
        }
        if self.hp_p_use {
            result += temp[StatName::HPPercentage as usize] * self.hp_p_weight;
        }
        if self.def_use {
            result += temp[StatName::DEFFixed as usize] * self.def_weight;
        }
        if self.def_p_use {
            result += temp[StatName::DEFPercentage as usize] * self.def_p_weight;
        }
        if self.critical_use {
            result += temp[StatName::CriticalRate as usize] * self.critical_weight;
        }
        if self.critical_damage_use {
            result += temp[StatName::CriticalDamage as usize] * self.critical_damage_weight;
        }
        if self.elemental_mastery_use {
            result += temp[StatName::ElementalMastery as usize] * self.elemental_mastery_weight;
        }
        if self.recharge_use {
            result += temp[StatName::Recharge as usize] * self.recharge_weight;
        }

        result
    }

    fn get_effective_stats(&self) -> Vec<StatName> {
        let mut result = Vec::new();
        if self.atk_use {
            result.push(StatName::ATKFixed);
        }
        if self.atk_p_use {
            result.push(StatName::ATKPercentage);
        }
        if self.hp_use {
            result.push(StatName::HPFixed);
        }
        if self.hp_p_use {
            result.push(StatName::HPPercentage);
        }
        if self.def_use {
            result.push(StatName::DEFFixed);
        }
        if self.def_p_use {
            result.push(StatName::DEFPercentage);
        }
        if self.critical_use {
            result.push(StatName::CriticalRate);
        }
        if self.critical_damage_use {
            result.push(StatName::CriticalDamage);
        }
        if self.elemental_mastery_use {
            result.push(StatName::ElementalMastery);
        }
        if self.recharge_use {
            result.push(StatName::Recharge);
        }

        result
    }
}

impl PotentialFunctionMeta for PotentialFunctionArtifactEff {
    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk_use",
            title: "p1",
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "atk_weight",
            title: "p2",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "atk_p_use",
            title: "p3",
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "atk_p_weight",
            title: "p4",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "hp_use",
            title: "p5",
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "hp_weight",
            title: "p6",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "hp_p_use",
            title: "p7",
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "hp_p_weight",
            title: "p8",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "def_use",
            title: "p9",
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "def_weight",
            title: "p10",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "def_p_use",
            title: "p11",
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "def_p_weight",
            title: "p12",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "critical_use",
            title: "p13",
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "critical_weight",
            title: "p14",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "critical_damage_use",
            title: "p15",
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "critical_damage_weight",
            title: "p16",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "elemental_mastery_use",
            title: "p17",
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "elemental_mastery_weight",
            title: "p18",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "recharge_use",
            title: "p19",
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "recharge_weight",
            title: "p20",
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const META: PotentialFunctionMetaData = PotentialFunctionMetaData {
        name: PotentialFunctionName::ArtifactEff,
        chs: "有效词条",
        description: "以单次强化最大档位为1分",
        image: "misc/sword"
    };

    fn create(config: &PotentialFunctionConfig) -> Box<dyn PotentialFunction> {
        let pf = if let PotentialFunctionConfig::ArtifactEff(x) = config {
            x.clone()
        } else {
            Default::default()
        };

        Box::new(pf)
    }
}
