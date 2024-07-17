use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum SkillType {
    NoneType,
    NormalAttack,
    ChargedAttack,
    PlungingAttackInAction,
    PlungingAttackOnGround,
    ElementalSkill,
    ElementalBurst,
}

impl SkillType {
    pub fn is_plunging(&self) -> bool {
        match *self {
            SkillType::PlungingAttackInAction | SkillType::PlungingAttackOnGround => true,
            _ => false
        }
    }
}
