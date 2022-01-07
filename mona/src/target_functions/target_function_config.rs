use serde::{Serialize, Deserialize};

use crate::common::{Element, SkillType};

#[derive(Serialize, Deserialize)]
pub enum TargetFunctionConfig {
    ExpectationConfig(Element, SkillType),
    // ExpectationConfig { element: Element, skill_type: SkillType }
}