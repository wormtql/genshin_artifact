use crate::attribute::typing::EdgeFunctionBwd;
use super::attribute_name::AttributeName;
use super::typing::EdgeFunctionFwd;
use crate::common::{Element, SkillType};


pub trait Attribute: Default {
    type EdgeHandle: Copy;

    fn get_value(&self, key: AttributeName) -> f64;

    fn set_value_to(&mut self, name: AttributeName, key: &str, value: f64);

    fn set_value_by(&mut self, name: AttributeName, key: &str, value: f64);

    // fn add_edge(&mut self, from: AttributeName, to: AttributeName, edge: EdgeFunction, priority: usize, key: &str) -> Self::EdgeHandle;

    fn add_edge(
        &mut self,
        from1: usize,
        from2: usize,
        to: usize,
        fwd: EdgeFunctionFwd,
        bwd: EdgeFunctionBwd,
        key: &str
    ) -> Self::EdgeHandle;

    fn remove_edge(&mut self, handle: Self::EdgeHandle);
}

pub trait AttributeCommon<T> {
    fn get_em_all(&self) -> f64;

    fn get_atk(&self) -> f64;

    fn get_hp(&self) -> f64;

    fn get_def(&self) -> f64;

    fn get_atk_ratio(&self, element: Element, skill: SkillType) -> f64;

    fn get_def_ratio(&self, element: Element, skill: SkillType) -> f64;

    fn get_hp_ratio(&self, element: Element, skill: SkillType) -> f64;

    fn get_extra_damage(&self, element: Element, skill: SkillType) -> f64;

    fn get_bonus(&self, element: Element, skill: SkillType) -> f64;

    fn get_critical_rate(&self, element: Element, skill: SkillType) -> f64;

    fn get_critical_damage(&self, element: Element, skill: SkillType) -> f64;

    fn get_enemy_res_minus(&self, element: Element, skill: SkillType) -> f64;

    fn get_enemy_def_minus(&self, element: Element, skill: SkillType) -> f64;

    fn new_with_base_edge() -> T;

    fn add_edge1(
        &mut self,
        from: AttributeName,
        to: AttributeName,
        fwd: EdgeFunctionFwd,
        bwd: EdgeFunctionBwd,
        key: &str
    );

    fn add_edge2(
        &mut self,
        from1: AttributeName,
        from2: AttributeName,
        to: AttributeName,
        fwd: EdgeFunctionFwd,
        bwd: EdgeFunctionBwd,
        key: &str
    );

    fn add_atk_percentage(&mut self, key: &str, value: f64);

    fn add_def_percentage(&mut self, key: &str, value: f64);

    fn add_hp_percentage(&mut self, key: &str, value: f64);

    fn add_elemental_bonus(&mut self, key: &str, value: f64);
}

impl<T: Attribute> AttributeCommon<T> for T {
    fn get_em_all(&self) -> f64 {
        self.get_value(AttributeName::ElementalMastery) + self.get_value(AttributeName::ElementalMasteryExtra)
    }

    fn get_atk(&self) -> f64 {
        self.get_value(AttributeName::ATKBase)
            + self.get_value(AttributeName::ATKPercentage)
            + self.get_value(AttributeName::ATKFixed)
    }

    fn get_hp(&self) -> f64 {
        self.get_value(AttributeName::HPBase)
            + self.get_value(AttributeName::HPPercentage)
            + self.get_value(AttributeName::HPFixed)
    }

    fn get_def(&self) -> f64 {
        self.get_value(AttributeName::DEFBase)
            + self.get_value(AttributeName::DEFPercentage)
            + self.get_value(AttributeName::DEFFixed)
    }

    fn get_atk_ratio(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::atk_ratio_name_by_element(element);
        let key2 = AttributeName::atk_ratio_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::ATKRatioBase)
            + self.get_value(key1) + value2
    }

    fn get_def_ratio(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::def_ratio_name_by_element(element);
        let key2 = AttributeName::def_ratio_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::DEFRatioBase)
            + self.get_value(key1) + value2
    }

    fn get_hp_ratio(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::hp_ratio_name_by_element(element);
        let key2 = AttributeName::hp_ratio_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::HPRatioBase)
            + self.get_value(key1) + value2
    }

    fn get_extra_damage(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::extra_dmg_name_by_element(element);
        let key2 = AttributeName::extra_dmg_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::ExtraDmgBase)
            + self.get_value(key1) + value2
    }

    fn get_bonus(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::bonus_name_by_element(element);
        let key2 = AttributeName::bonus_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        let mut temp = self.get_value(AttributeName::BonusBase)
            + self.get_value(key1) + value2;
        // todo refactor
        if element != Element::Physical && skill == SkillType::NormalAttack {
            temp += self.get_value(AttributeName::BonusNormalAndElemental);
        }
        temp
    }

    fn get_critical_rate(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::critical_rate_name_by_element(element);
        let key2 = AttributeName::critical_rate_name_by_skill_type(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::CriticalBase) + self.get_value(AttributeName::CriticalAttacking)
            + self.get_value(key1) + value2
    }

    fn get_critical_damage(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::critical_damage_name_by_element(element);
        let key2 = AttributeName::critical_damage_name_by_skill_name(skill);

        let value2 = if let Some(name) = key2 {
            self.get_value(name)
        } else {
            0.0
        };

        self.get_value(AttributeName::CriticalDamageBase)
            + self.get_value(key1) + value2
    }

    fn get_enemy_res_minus(&self, element: Element, _skill: SkillType) -> f64 {
        self.get_value(AttributeName::ResMinusBase)
            + self.get_value(AttributeName::res_minus_name_by_element(element))
    }

    fn get_enemy_def_minus(&self, _element: Element, _skill: SkillType) -> f64 {
        self.get_value(AttributeName::DefMinus)
    }

    fn new_with_base_edge() -> T {
        let mut temp: T = Default::default();

        temp.add_edge1(
            AttributeName::ATKBase, AttributeName::ATK,
            Box::new(|x1, _x2| x1),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "atk_base",
        );
        temp.add_edge1(
            AttributeName::ATKPercentage, AttributeName::ATK,
            Box::new(|x1, _x2| x1),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "atk_percentage"
        );
        temp.add_edge1(
            AttributeName::ATKFixed, AttributeName::ATK,
            Box::new(|x1, _x2| x1),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "atk_fixed"
        );

        temp.add_edge1(
            AttributeName::HPBase, AttributeName::HP,
            Box::new(|x1, _x2| x1),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "hp_base"
        );
        temp.add_edge1(
            AttributeName::HPPercentage, AttributeName::HP,
            Box::new(|x1, _x2| x1),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "hp_percentage"
        );
        temp.add_edge1(
            AttributeName::HPFixed, AttributeName::HP,
            Box::new(|x1, _x2| x1),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "hp_fixed"
        );

        temp.add_edge1(
            AttributeName::DEFBase, AttributeName::DEF,
            Box::new(|x1, _x2| x1),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "def_base"
        );
        temp.add_edge1(
            AttributeName::DEFPercentage, AttributeName::DEF,
            Box::new(|x1, _x2| x1),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "def_percentage"
        );
        temp.add_edge1(
            AttributeName::DEFFixed, AttributeName::DEF,
            Box::new(|x1, _x2| x1),
            Box::new(|grad, _x1, _x2| (grad, 0.0)),
            "def_fixed"
        );

        temp
    }

    fn add_edge1(&mut self, from: AttributeName, to: AttributeName, fwd: EdgeFunctionFwd, bwd: EdgeFunctionBwd, key: &str) {
        self.add_edge(
            from as usize,
            usize::MAX,
            to as usize,
            fwd,
            bwd,
            key
        );
    }

    fn add_edge2(&mut self, from1: AttributeName, from2: AttributeName, to: AttributeName, fwd: EdgeFunctionFwd, bwd: EdgeFunctionBwd, key: &str) {
        self.add_edge(
            from1 as usize,
            from2 as usize,
            to as usize,
            fwd,
            bwd,
            key
        );
    }

    fn add_atk_percentage(&mut self, key: &str, value: f64) {
        self.add_edge(
            AttributeName::ATKBase as usize,
            usize::MAX,
            AttributeName::ATKPercentage as usize,
            Box::new(move |x, _| x * value),
            Box::new(move |grad, _x1, _x2| (grad * value, 0.0)),
            key
        );
    }

    fn add_def_percentage(&mut self, key: &str, value: f64) {
        self.add_edge(
            AttributeName::DEFBase as usize,
            usize::MAX,
            AttributeName::DEFPercentage as usize,
            Box::new(move |x, _| x * value),
            Box::new(move |grad, _x1, _x2| (grad * value, 0.0)),
            key
        );
    }

    fn add_hp_percentage(&mut self, key: &str, value: f64) {
        self.add_edge(
            AttributeName::HPBase as usize,
            usize::MAX,
            AttributeName::HPPercentage as usize,
            Box::new(move |x, _| x * value),
            Box::new(move |grad, _x1, _x2| (grad * value, 0.0)),
            key
        );
    }

    fn add_elemental_bonus(&mut self, key: &str, value: f64) {
        self.set_value_by(AttributeName::BonusElectro, key, value);
        self.set_value_by(AttributeName::BonusPyro, key, value);
        self.set_value_by(AttributeName::BonusHydro, key, value);
        self.set_value_by(AttributeName::BonusAnemo, key, value);
        self.set_value_by(AttributeName::BonusCryo, key, value);
        self.set_value_by(AttributeName::BonusGeo, key, value);
        self.set_value_by(AttributeName::BonusDendro, key, value);
    }
}