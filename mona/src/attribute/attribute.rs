use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;

use super::entry::{NodeHandle, EdgeFunction};
use std::collections::HashMap;
use crate::attribute::entry::Node;
use crate::common::{Element, SkillType, DamageResult};
use crate::enemies::Enemy;


#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
#[wasm_bindgen]
pub enum AttributeName {
    HealingBonus,
    ElementalMastery,
    Recharge,
    ShieldStrength,

    HPBase,
    HPFixed,
    HPPercentage,

    ATKBase,
    ATKFixed,
    ATKPercentage,

    DEFBase,
    DEFFixed,
    DEFPercentage,

    CriticalBase,
    CriticalNormalAttack,
    CriticalChargedAttack,
    CriticalPlungingAttack,
    CriticalElementalSkill,
    CriticalElementalBurst,
    CriticalElectro,
    CriticalPyro,
    CriticalHydro,
    CriticalCryo,
    CriticalAnemo,
    CriticalGeo,
    CriticalDendro,
    CriticalPhysical,

    CriticalDamageBase,
    CriticalDamageNormalAttack,
    CriticalDamageChargedAttack,
    CriticalDamagePlungingAttack,
    CriticalDamageElementalSkill,
    CriticalDamageElementalBurst,
    CriticalDamageElectro,
    CriticalDamagePyro,
    CriticalDamageHydro,
    CriticalDamageCryo,
    CriticalDamageAnemo,
    CriticalDamageGeo,
    CriticalDamageDendro,
    CriticalDamagePhysical,

    BonusBase,
    BonusNormalAttack,
    BonusChargedAttack,
    BonusPlungingAttack,
    BonusElementalSkill,
    BonusElementalBurst,
    BonusElectro,
    BonusPyro,
    BonusHydro,
    BonusCryo,
    BonusAnemo,
    BonusGeo,
    BonusDendro,
    BonusPhysical,

    EnhanceOverload,
    EnhanceBurning,
    EnhanceElectroCharged,
    EnhanceSuperconduct,
    EnhanceSwirlElectro,
    EnhanceSwirlPyro,
    EnhanceSwirlHydro,
    EnhanceSwirlCryo,
    EnhanceSwirlBase,
    EnhanceVaporize,
    EnhanceMelt,

    HPRatioBase,
    HPRatioNormalAttack,
    HPRatioChargedAttack,
    HPRatioPlungingAttack,
    HPRatioElementalSkill,
    HPRatioElementalBurst,
    HPRatioElectro,
    HPRatioPyro,
    HPRatioHydro,
    HPRatioCryo,
    HPRatioAnemo,
    HPRatioGeo,
    HPRatioDendro,
    HPRatioPhysical,

    DEFRatioBase,
    DEFRatioNormalAttack,
    DEFRatioChargedAttack,
    DEFRatioPlungingAttack,
    DEFRatioElementalSkill,
    DEFRatioElementalBurst,
    DEFRatioElectro,
    DEFRatioPyro,
    DEFRatioHydro,
    DEFRatioCryo,
    DEFRatioAnemo,
    DEFRatioGeo,
    DEFRatioDendro,
    DEFRatioPhysical,

    ATKRatioBase,
    ATKRatioNormalAttack,
    ATKRatioChargedAttack,
    ATKRatioPlungingAttack,
    ATKRatioElementalSkill,
    ATKRatioElementalBurst,
    ATKRatioElectro,
    ATKRatioPyro,
    ATKRatioHydro,
    ATKRatioCryo,
    ATKRatioAnemo,
    ATKRatioGeo,
    ATKRatioDendro,
    ATKRatioPhysical,

    ExtraDmgBase,
    ExtraDmgNormalAttack,
    ExtraDmgChargedAttack,
    ExtraDmgPlungingAttack,
    ExtraDmgElementalSkill,
    ExtraDmgElementalBurst,
    ExtraDmgElectro,
    ExtraDmgPyro,
    ExtraDmgHydro,
    ExtraDmgCryo,
    ExtraDmgAnemo,
    ExtraDmgGeo,
    ExtraDmgDendro,
    ExtraDmgPhysical,
}

impl AttributeName {
    pub fn bonus_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::BonusElectro,
            Element::Hydro => AttributeName::BonusHydro,
            Element::Anemo => AttributeName::BonusAnemo,
            Element::Pyro => AttributeName::BonusPyro,
            Element::Cryo => AttributeName::BonusCryo,
            Element::Dendro => AttributeName::BonusDendro,
            Element::Geo => AttributeName::BonusGeo,
            Element::Physical => AttributeName::BonusPhysical,
            _ => unreachable!(),
        }
    }

    pub fn bonus_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::BonusNormalAttack,
            SkillType::ChargedAttack => AttributeName::BonusChargedAttack,
            SkillType::PlungingAttack => AttributeName::BonusPlungingAttack,
            SkillType::ElementalSkill => AttributeName::BonusElementalSkill,
            SkillType::ElementalBurst => AttributeName::BonusElementalBurst,
        }
    }

    pub fn critical_rate_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::CriticalElectro,
            Element::Hydro => AttributeName::CriticalHydro,
            Element::Anemo => AttributeName::CriticalAnemo,
            Element::Pyro => AttributeName::CriticalPyro,
            Element::Cryo => AttributeName::CriticalCryo,
            Element::Dendro => AttributeName::CriticalDendro,
            Element::Geo => AttributeName::CriticalGeo,
            Element::Physical => AttributeName::CriticalPhysical,
            _ => unreachable!(),
        }
    }

    pub fn critical_rate_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::CriticalNormalAttack,
            SkillType::ChargedAttack => AttributeName::CriticalChargedAttack,
            SkillType::PlungingAttack => AttributeName::CriticalPlungingAttack,
            SkillType::ElementalSkill => AttributeName::CriticalElementalSkill,
            SkillType::ElementalBurst => AttributeName::CriticalElementalBurst,
        }
    }

    pub fn critical_damage_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::CriticalDamageElectro,
            Element::Hydro => AttributeName::CriticalDamageHydro,
            Element::Anemo => AttributeName::CriticalDamageAnemo,
            Element::Pyro => AttributeName::CriticalDamagePyro,
            Element::Cryo => AttributeName::CriticalDamageCryo,
            Element::Dendro => AttributeName::CriticalDamageDendro,
            Element::Geo => AttributeName::CriticalDamageGeo,
            Element::Physical => AttributeName::CriticalDamagePhysical,
            _ => unreachable!(),
        }
    }

    pub fn critical_damage_name_by_skill_name(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::CriticalDamageNormalAttack,
            SkillType::ChargedAttack => AttributeName::CriticalDamageChargedAttack,
            SkillType::PlungingAttack => AttributeName::CriticalDamagePlungingAttack,
            SkillType::ElementalSkill => AttributeName::CriticalDamageElementalSkill,
            SkillType::ElementalBurst => AttributeName::CriticalDamageElementalBurst,
        }
    }

    pub fn hp_ratio_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::HPRatioElectro,
            Element::Hydro => AttributeName::HPRatioHydro,
            Element::Anemo => AttributeName::HPRatioAnemo,
            Element::Pyro => AttributeName::HPRatioPyro,
            Element::Cryo => AttributeName::HPRatioCryo,
            Element::Dendro => AttributeName::HPRatioDendro,
            Element::Geo => AttributeName::HPRatioGeo,
            Element::Physical => AttributeName::HPRatioPhysical,
            _ => unreachable!(),
        }
    }

    pub fn hp_ratio_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::HPRatioNormalAttack,
            SkillType::ChargedAttack => AttributeName::HPRatioChargedAttack,
            SkillType::PlungingAttack => AttributeName::HPRatioPlungingAttack,
            SkillType::ElementalSkill => AttributeName::HPRatioElementalSkill,
            SkillType::ElementalBurst => AttributeName::HPRatioElementalBurst,
        }
    }

    pub fn def_ratio_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::DEFRatioElectro,
            Element::Hydro => AttributeName::DEFRatioHydro,
            Element::Anemo => AttributeName::DEFRatioAnemo,
            Element::Pyro => AttributeName::DEFRatioPyro,
            Element::Cryo => AttributeName::DEFRatioCryo,
            Element::Dendro => AttributeName::DEFRatioDendro,
            Element::Geo => AttributeName::DEFRatioGeo,
            Element::Physical => AttributeName::DEFRatioPhysical,
            _ => unreachable!(),
        }
    }

    pub fn def_ratio_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::DEFRatioNormalAttack,
            SkillType::ChargedAttack => AttributeName::DEFRatioChargedAttack,
            SkillType::PlungingAttack => AttributeName::DEFRatioPlungingAttack,
            SkillType::ElementalSkill => AttributeName::DEFRatioElementalSkill,
            SkillType::ElementalBurst => AttributeName::DEFRatioElementalBurst,
        }
    }

    pub fn atk_ratio_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::ATKRatioElectro,
            Element::Hydro => AttributeName::ATKRatioHydro,
            Element::Anemo => AttributeName::ATKRatioAnemo,
            Element::Pyro => AttributeName::ATKRatioPyro,
            Element::Cryo => AttributeName::ATKRatioCryo,
            Element::Dendro => AttributeName::ATKRatioDendro,
            Element::Geo => AttributeName::ATKRatioGeo,
            Element::Physical => AttributeName::ATKRatioPhysical,
            _ => unreachable!(),
        }
    }

    pub fn atk_ratio_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::ATKRatioNormalAttack,
            SkillType::ChargedAttack => AttributeName::ATKRatioChargedAttack,
            SkillType::PlungingAttack => AttributeName::ATKRatioPlungingAttack,
            SkillType::ElementalSkill => AttributeName::ATKRatioElementalSkill,
            SkillType::ElementalBurst => AttributeName::ATKRatioElementalBurst,
        }
    }

    pub fn extra_dmg_name_by_element(element: Element) -> AttributeName {
        match element {
            Element:: Electro => AttributeName::ExtraDmgElectro,
            Element::Hydro => AttributeName::ExtraDmgHydro,
            Element::Anemo => AttributeName::ExtraDmgAnemo,
            Element::Pyro => AttributeName::ExtraDmgPyro,
            Element::Cryo => AttributeName::ExtraDmgCryo,
            Element::Dendro => AttributeName::ExtraDmgDendro,
            Element::Geo => AttributeName::ExtraDmgGeo,
            Element::Physical => AttributeName::ExtraDmgPhysical,
            _ => unreachable!(),
        }
    }

    pub fn extra_dmg_name_by_skill_type(skill_type: SkillType) -> AttributeName {
        match skill_type {
            SkillType::NormalAttack => AttributeName::ExtraDmgNormalAttack,
            SkillType::ChargedAttack => AttributeName::ExtraDmgChargedAttack,
            SkillType::PlungingAttack => AttributeName::ExtraDmgPlungingAttack,
            SkillType::ElementalSkill => AttributeName::ExtraDmgElementalSkill,
            SkillType::ElementalBurst => AttributeName::ExtraDmgElementalBurst,
        }
    }
}

// #[wasm_bindgen]
pub struct AttributeGraph {
    pub attributes: HashMap<AttributeName, RefCell<NodeHandle>>
}

impl AttributeGraph {
    pub fn new() -> AttributeGraph {
        let mut ret = AttributeGraph {
            attributes: HashMap::new(),
        };

        ret.add_entry(AttributeName::HealingBonus, 0.0);
        ret.add_entry(AttributeName::ElementalMastery, 0.0);
        ret.add_entry(AttributeName::Recharge, 1.0);
        ret.add_entry(AttributeName::ShieldStrength, 0.0);

        ret.add_entry(AttributeName::HPBase, 0.0);
        ret.add_entry(AttributeName::HPFixed, 0.0);
        ret.add_entry(AttributeName::HPPercentage, 0.0);

        ret.add_entry(AttributeName::ATKBase, 0.0);
        ret.add_entry(AttributeName::ATKFixed, 0.0);
        ret.add_entry(AttributeName::ATKPercentage, 0.0);

        ret.add_entry(AttributeName::DEFBase, 0.0);
        ret.add_entry(AttributeName::DEFFixed, 0.0);
        ret.add_entry(AttributeName::DEFPercentage, 0.0);

        ret.add_entry(AttributeName::CriticalBase, 0.05);
        ret.add_entry(AttributeName::CriticalNormalAttack, 0.0);
        ret.add_entry(AttributeName::CriticalChargedAttack, 0.0);
        ret.add_entry(AttributeName::CriticalPlungingAttack, 0.0);
        ret.add_entry(AttributeName::CriticalElementalSkill, 0.0);
        ret.add_entry(AttributeName::CriticalElementalBurst, 0.0);
        ret.add_entry(AttributeName::CriticalElectro, 0.0);
        ret.add_entry(AttributeName::CriticalPyro, 0.0);
        ret.add_entry(AttributeName::CriticalHydro, 0.0);
        ret.add_entry(AttributeName::CriticalCryo, 0.0);
        ret.add_entry(AttributeName::CriticalAnemo, 0.0);
        ret.add_entry(AttributeName::CriticalGeo, 0.0);
        ret.add_entry(AttributeName::CriticalDendro, 0.0);
        ret.add_entry(AttributeName::CriticalPhysical, 0.0);

        ret.add_entry(AttributeName::CriticalDamageBase, 0.5);
        ret.add_entry(AttributeName::CriticalDamageNormalAttack, 0.0);
        ret.add_entry(AttributeName::CriticalDamageChargedAttack, 0.0);
        ret.add_entry(AttributeName::CriticalDamagePlungingAttack, 0.0);
        ret.add_entry(AttributeName::CriticalDamageElementalSkill, 0.0);
        ret.add_entry(AttributeName::CriticalDamageElementalBurst, 0.0);
        ret.add_entry(AttributeName::CriticalDamageElectro, 0.0);
        ret.add_entry(AttributeName::CriticalDamagePyro, 0.0);
        ret.add_entry(AttributeName::CriticalDamageHydro, 0.0);
        ret.add_entry(AttributeName::CriticalDamageCryo, 0.0);
        ret.add_entry(AttributeName::CriticalDamageAnemo, 0.0);
        ret.add_entry(AttributeName::CriticalDamageGeo, 0.0);
        ret.add_entry(AttributeName::CriticalDamageDendro, 0.0);
        ret.add_entry(AttributeName::CriticalDamagePhysical, 0.0);

        ret.add_entry(AttributeName::BonusBase, 0.0);
        ret.add_entry(AttributeName::BonusNormalAttack, 0.0);
        ret.add_entry(AttributeName::BonusChargedAttack, 0.0);
        ret.add_entry(AttributeName::BonusPlungingAttack, 0.0);
        ret.add_entry(AttributeName::BonusElementalSkill, 0.0);
        ret.add_entry(AttributeName::BonusElementalBurst, 0.0);
        ret.add_entry(AttributeName::BonusElectro, 0.0);
        ret.add_entry(AttributeName::BonusPyro, 0.0);
        ret.add_entry(AttributeName::BonusHydro, 0.0);
        ret.add_entry(AttributeName::BonusCryo, 0.0);
        ret.add_entry(AttributeName::BonusAnemo, 0.0);
        ret.add_entry(AttributeName::BonusGeo, 0.0);
        ret.add_entry(AttributeName::BonusDendro, 0.0);
        ret.add_entry(AttributeName::BonusPhysical, 0.0);

        ret.add_entry(AttributeName::EnhanceOverload, 0.0);
        ret.add_entry(AttributeName::EnhanceBurning, 0.0);
        ret.add_entry(AttributeName::EnhanceElectroCharged, 0.0);
        ret.add_entry(AttributeName::EnhanceSuperconduct, 0.0);
        ret.add_entry(AttributeName::EnhanceSwirlElectro, 0.0);
        ret.add_entry(AttributeName::EnhanceSwirlPyro, 0.0);
        ret.add_entry(AttributeName::EnhanceSwirlHydro, 0.0);
        ret.add_entry(AttributeName::EnhanceSwirlCryo, 0.0);
        ret.add_entry(AttributeName::EnhanceSwirlBase, 0.0);
        ret.add_entry(AttributeName::EnhanceVaporize, 0.0);
        ret.add_entry(AttributeName::EnhanceMelt, 0.0);

        ret.add_entry(AttributeName::HPRatioBase, 0.0);
        ret.add_entry(AttributeName::HPRatioNormalAttack, 0.0);
        ret.add_entry(AttributeName::HPRatioChargedAttack, 0.0);
        ret.add_entry(AttributeName::HPRatioPlungingAttack, 0.0);
        ret.add_entry(AttributeName::HPRatioElementalSkill, 0.0);
        ret.add_entry(AttributeName::HPRatioElementalBurst, 0.0);
        ret.add_entry(AttributeName::HPRatioElectro, 0.0);
        ret.add_entry(AttributeName::HPRatioPyro, 0.0);
        ret.add_entry(AttributeName::HPRatioHydro, 0.0);
        ret.add_entry(AttributeName::HPRatioCryo, 0.0);
        ret.add_entry(AttributeName::HPRatioAnemo, 0.0);
        ret.add_entry(AttributeName::HPRatioGeo, 0.0);
        ret.add_entry(AttributeName::HPRatioDendro, 0.0);
        ret.add_entry(AttributeName::HPRatioPhysical, 0.0);

        ret.add_entry(AttributeName::DEFRatioBase, 0.0);
        ret.add_entry(AttributeName::DEFRatioNormalAttack, 0.0);
        ret.add_entry(AttributeName::DEFRatioChargedAttack, 0.0);
        ret.add_entry(AttributeName::DEFRatioPlungingAttack, 0.0);
        ret.add_entry(AttributeName::DEFRatioElementalSkill, 0.0);
        ret.add_entry(AttributeName::DEFRatioElementalBurst, 0.0);
        ret.add_entry(AttributeName::DEFRatioElectro, 0.0);
        ret.add_entry(AttributeName::DEFRatioPyro, 0.0);
        ret.add_entry(AttributeName::DEFRatioHydro, 0.0);
        ret.add_entry(AttributeName::DEFRatioCryo, 0.0);
        ret.add_entry(AttributeName::DEFRatioAnemo, 0.0);
        ret.add_entry(AttributeName::DEFRatioGeo, 0.0);
        ret.add_entry(AttributeName::DEFRatioDendro, 0.0);
        ret.add_entry(AttributeName::DEFRatioPhysical, 0.0);

        ret.add_entry(AttributeName::ATKRatioBase, 0.0);
        ret.add_entry(AttributeName::ATKRatioNormalAttack, 0.0);
        ret.add_entry(AttributeName::ATKRatioChargedAttack, 0.0);
        ret.add_entry(AttributeName::ATKRatioPlungingAttack, 0.0);
        ret.add_entry(AttributeName::ATKRatioElementalSkill, 0.0);
        ret.add_entry(AttributeName::ATKRatioElementalBurst, 0.0);
        ret.add_entry(AttributeName::ATKRatioElectro, 0.0);
        ret.add_entry(AttributeName::ATKRatioPyro, 0.0);
        ret.add_entry(AttributeName::ATKRatioHydro, 0.0);
        ret.add_entry(AttributeName::ATKRatioCryo, 0.0);
        ret.add_entry(AttributeName::ATKRatioAnemo, 0.0);
        ret.add_entry(AttributeName::ATKRatioGeo, 0.0);
        ret.add_entry(AttributeName::ATKRatioDendro, 0.0);
        ret.add_entry(AttributeName::ATKRatioPhysical, 0.0);

        ret.add_entry(AttributeName::ExtraDmgBase, 0.0);
        ret.add_entry(AttributeName::ExtraDmgNormalAttack, 0.0);
        ret.add_entry(AttributeName::ExtraDmgChargedAttack, 0.0);
        ret.add_entry(AttributeName::ExtraDmgPlungingAttack, 0.0);
        ret.add_entry(AttributeName::ExtraDmgElementalSkill, 0.0);
        ret.add_entry(AttributeName::ExtraDmgElementalBurst, 0.0);
        ret.add_entry(AttributeName::ExtraDmgElectro, 0.0);
        ret.add_entry(AttributeName::ExtraDmgPyro, 0.0);
        ret.add_entry(AttributeName::ExtraDmgHydro, 0.0);
        ret.add_entry(AttributeName::ExtraDmgCryo, 0.0);
        ret.add_entry(AttributeName::ExtraDmgAnemo, 0.0);
        ret.add_entry(AttributeName::ExtraDmgGeo, 0.0);
        ret.add_entry(AttributeName::ExtraDmgDendro, 0.0);
        ret.add_entry(AttributeName::ExtraDmgPhysical, 0.0);

        ret
    }

    pub fn get_atk(&self) -> f64 {
        self.get_value(AttributeName::ATKBase)
        + self.get_value(AttributeName::ATKFixed)
        + self.get_value(AttributeName::ATKPercentage)
    }

    pub fn get_def(&self) -> f64 {
        self.get_value(AttributeName::DEFBase)
        + self.get_value(AttributeName::DEFFixed)
        + self.get_value(AttributeName::DEFPercentage)
    }

    pub fn get_hp(&self) -> f64 {
        self.get_value(AttributeName::HPBase)
        + self.get_value(AttributeName::HPFixed)
        + self.get_value(AttributeName::HPPercentage)
    }

    pub fn add_entry(&mut self, name: AttributeName, initial_value: f64) {
        let node_handle = NodeHandle::new(initial_value);
        self.attributes.insert(name, RefCell::new(node_handle));
    }

    pub fn get_value(&self, name: AttributeName) -> f64 {
        let node = self.attributes.get(&name).unwrap();
        node.borrow().value()
    }

    pub fn add_edge(&mut self, from: AttributeName, to: AttributeName, f: EdgeFunction) {
        let mut from_node_handle = self.attributes.get(&from).unwrap().borrow_mut();
        let to_node_handle = self.attributes.get(&to).unwrap().borrow();
        from_node_handle.add_edge(&to_node_handle, f);
    }

    pub fn remove_edge(&mut self, name: AttributeName, id: i32) {
        let mut node = self.attributes.get(&name).unwrap();
        node.borrow_mut().remove_edge(id);
    }

    pub fn add_value(&mut self, name: AttributeName, key: &str, value: f64) {
        let mut node = self.attributes.get(&name).unwrap().borrow_mut();
        node.set_value(key, value);
    }

    pub fn add_value_delta(&mut self, name: AttributeName, key: &str, delta: f64) {
        let mut node = self.attributes.get(&name).unwrap().borrow_mut();
        let new_value = delta + node.get_value_by_key(key);
        node.set_value(key, new_value);
    }

    pub fn get_attribute_composition(&self, name: AttributeName) -> HashMap<String, f64> {
        // to refresh dirty
        self.get_value(name);

        let node = self.attributes.get(&name).unwrap();
        node.borrow().get_composition()
    }
}

// damage related methods
impl AttributeGraph {
    pub fn get_atk_ratio(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::atk_ratio_name_by_element(element);
        let key2 = AttributeName::atk_ratio_name_by_skill_type(skill);

        self.get_value(AttributeName::ATKRatioBase)
            + self.get_value(key1) + self.get_value(key2)
    }

    pub fn get_def_ratio(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::def_ratio_name_by_element(element);
        let key2 = AttributeName::def_ratio_name_by_skill_type(skill);

        self.get_value(AttributeName::DEFRatioBase)
            + self.get_value(key1) + self.get_value(key2)
    }

    pub fn get_hp_ratio(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::hp_ratio_name_by_element(element);
        let key2 = AttributeName::hp_ratio_name_by_skill_type(skill);

        self.get_value(AttributeName::HPRatioBase)
            + self.get_value(key1) + self.get_value(key2)
    }

    pub fn get_extra_dmg(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::extra_dmg_name_by_element(element);
        let key2 = AttributeName::extra_dmg_name_by_skill_type(skill);

        self.get_value(AttributeName::ExtraDmgBase)
            + self.get_value(key1) + self.get_value(key2)
    }

    pub fn get_bonus(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::bonus_name_by_element(element);
        let key2 = AttributeName::bonus_name_by_skill_type(skill);

        self.get_value(AttributeName::BonusBase)
            + self.get_value(key1) + self.get_value(key2)
    }

    pub fn get_critical_rate(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::critical_rate_name_by_element(element);
        let key2 = AttributeName::critical_rate_name_by_skill_type(skill);

        self.get_value(AttributeName::CriticalBase)
            + self.get_value(key1) + self.get_value(key2)
    }

    pub fn get_critical_damage(&self, element: Element, skill: SkillType) -> f64 {
        let key1 = AttributeName::critical_damage_name_by_element(element);
        let key2 = AttributeName::critical_damage_name_by_skill_name(skill);

        self.get_value(AttributeName::CriticalDamageBase)
            + self.get_value(key1) + self.get_value(key2)
    }

    pub fn damage_without_enemy(&self, base: f64, element: Element, skill: SkillType) -> DamageResult {
        let atk = self.get_atk();
        let def = self.get_def();
        let hp = self.get_hp();

        let base2 = base
            + self.get_def_ratio(element, skill) * def
            + self.get_hp_ratio(element, skill) * hp
            + self.get_atk_ratio(element, skill) * atk
            + self.get_extra_dmg(element, skill)
            ;

        let bonus = self.get_bonus(element, skill);
        let critical_rate = self.get_critical_rate(element, skill);
        let critical_damage = self.get_critical_damage(element, skill);

        DamageResult {
            critical: base2 * (1.0 + bonus) * (1.0 + critical_damage),
            non_critical: base2 * (1.0 + bonus),
            expectation: base2 * (1.0 + bonus) * (1.0 + critical_damage * critical_rate),
        }
    }

    pub fn damage(&self, base: f64, element: Element, skill: SkillType, enemy: &Enemy, character_level: i32) -> DamageResult {
        let dmg_without_enemy = self.damage_without_enemy(base, element, skill);

        let defensive_ratio = enemy.get_defensive_ratio(character_level);
        let resistance_ratio = enemy.get_resistance_ratio(element);

        dmg_without_enemy * (defensive_ratio * resistance_ratio)
    }
}

// #[wasm_bindgen]
// impl Attribute {
//
// }