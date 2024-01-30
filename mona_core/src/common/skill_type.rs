use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum SkillType {
    NormalAttack,
    ChargedAttack,
    PlungingAttackLow,
    PlungingAttackHigh,
    PlungingAttackGround,
    ElementalSkill,
    ElementalBurst,
}

impl SkillType {
    pub fn is_plunging(&self) -> bool {
        match *self {
            SkillType::PlungingAttackGround | SkillType::PlungingAttackLow | SkillType::PlungingAttackHigh => true,
            _ => false
        }
    }
}
