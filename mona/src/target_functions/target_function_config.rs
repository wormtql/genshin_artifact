use serde::{Serialize, Deserialize};

use crate::common::{Element, SkillType};

#[derive(Serialize, Deserialize)]
pub enum TargetFunctionConfig {
    // ExpectationConfig(Element, SkillType),
    // MaxConfig { element: Element, skill_type: SkillType },
    GanyuDefault { melt_rate: f64 },
    GorouDefault { recharge_demand: f64 },
    // ExpectationConfig { element: Element, skill_type: SkillType }
    HuTaoDefault { vaporize_rate: f64 },
    JeanDefault { recharge_demand: f64, damage_weight: f64 },
    NoConfig
}