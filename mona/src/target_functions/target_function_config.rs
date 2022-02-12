use serde::{Serialize, Deserialize};

use crate::common::{Element};
use crate::target_functions::target_functions::rosaria_default::RosariaDefaultTFConfig;

#[derive(Serialize, Deserialize)]
pub enum TargetFunctionConfig {
    // ExpectationConfig(Element, SkillType),
    // MaxConfig { element: Element, skill_type: SkillType },
    GanyuDefault { melt_rate: f64 },
    GorouDefault { recharge_demand: f64 },
    // ExpectationConfig { element: Element, skill_type: SkillType }
    HuTaoDefault { vaporize_rate: f64, melt_rate: f64 },
    JeanDefault { recharge_demand: f64, damage_weight: f64 },
    LisaDefault { recharge_demand: f64 },
    MonaDefault { recharge_demand: f64 },
    QiqiDefault { recharge_demand: f64 },
    RosariaDefault(RosariaDefaultTFConfig),
    SayuDefault { recharge_demand: f64 },
    ShenheDefault { recharge_demand: f64 },
    SucroseDefault { recharge_demand: f64 },
    ThomaDefault { recharge_demand: f64 },
    VentiDefault { swirl_rate: f64 },
    XianglingDefault { recharge_demand: f64 },
    XingqiuDefault { recharge_demand: f64 },
    XinyanDefault { recharge_demand: f64, damage_demand: f64 },
    YoimiyaDefault { vaporize_rate: f64, melt_rate: f64 },
    YunjinDefault { recharge_demand: f64 },
    ZhongliDefault { recharge_demand: f64 },
    NoConfig
}
