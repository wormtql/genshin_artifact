use std::collections::HashMap;
use crate::attribute::{Attribute, AttributeName, ComplicatedAttributeGraph};
use crate::common::{DamageResult, Element, SkillType};
use crate::damage::damage_analysis::DamageAnalysis;
use crate::enemies::Enemy;
use crate::common::EntryType;
use crate::damage::damage_builder::{DamageBuilder};
use crate::damage::reaction::Reaction;
use crate::damage::SimpleDamageBuilder;

#[derive(Default)]
pub struct ComplicatedDamageBuilder {
    pub extra_critical_damage: EntryType,
    pub extra_critical_rate: EntryType,
    pub extra_bonus: EntryType,
    pub extra_damage: EntryType,
    pub extra_atk: EntryType,
    pub extra_def: EntryType,
    pub extra_hp: EntryType,
    pub extra_healing_bonus: EntryType,

    pub extra_enhance_melt: EntryType,
    pub extra_enhance_vaporize: EntryType,

    pub extra_def_minus: EntryType,
    pub extra_res_minus: EntryType,

    pub ratio_atk: EntryType,
    pub ratio_def: EntryType,
    pub ratio_hp: EntryType
}

impl DamageBuilder for ComplicatedDamageBuilder {
    type Result = DamageAnalysis;
    type AttributeType = ComplicatedAttributeGraph;

    fn new() -> Self {
        ComplicatedDamageBuilder::default()
    }

    fn add_atk_ratio(&mut self, key: &str, value: f64) {
        *self.ratio_atk.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_def_ratio(&mut self, key: &str, value: f64) {
        *self.ratio_def.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_hp_ratio(&mut self, key: &str, value: f64) {
        *self.ratio_hp.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_atk(&mut self, key: &str, value: f64) {
        *self.extra_atk.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_def(&mut self, key: &str, value: f64) {
        *self.extra_def.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_hp(&mut self, key: &str, value: f64) {
        *self.extra_hp.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_damage(&mut self, key: &str, value: f64) {
        *self.extra_damage.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_critical(&mut self, key: &str, value: f64) {
        *self.extra_critical_rate.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_critical_damage(&mut self, key: &str, value: f64) {
        *self.extra_critical_damage.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_bonus(&mut self, key: &str, value: f64) {
        *self.extra_bonus.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_enhance_melt(&mut self, key: &str, value: f64) {
        *self.extra_enhance_melt.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_enhance_vaporize(&mut self, key: &str, value: f64) {
        *self.extra_enhance_vaporize.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_def_minus(&mut self, key: &str, value: f64) {
        *self.extra_def_minus.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn add_extra_res_minus(&mut self, key: &str, value: f64) {
        *self.extra_res_minus.0.entry(String::from(key)).or_insert(0.0) = value;
    }

    fn build(
        &self,
        attribute:
        &Self::AttributeType,
        enemy: &Enemy,
        element: Element,
        skill: SkillType,
        is_heal: bool,
        character_level: usize
    ) -> Self::Result {
        let (atk_comp, atk_ratio_comp) = self.get_atk_composition(attribute, element, skill);
        let (atk, atk_ratio) = (atk_comp.sum(), atk_ratio_comp.sum());
        let (def_comp, def_ratio_comp) = self.get_def_composition(attribute, element, skill);
        let (def, def_ratio) = (def_comp.sum(), def_ratio_comp.sum());
        let (hp_comp, hp_ratio_comp) = self.get_hp_composition(attribute, element, skill);
        let (hp, hp_ratio) = (hp_comp.sum(), hp_ratio_comp.sum());
        let extra_damage_comp = self.get_extra_damage_composition(attribute, element, skill);
        let extra_damage = extra_damage_comp.sum();

        let base_damage = atk * atk_ratio + def * def_ratio + hp * hp_ratio + extra_damage;

        if is_heal {
            let healing_bonus_comp = self.get_healing_bonus_composition(attribute);
            let healing_bonus = healing_bonus_comp.sum();

            let heal_value = base_damage * (1.0 + healing_bonus);
            let damage_normal = DamageResult {
                expectation: heal_value,
                critical: heal_value,
                non_critical: heal_value
            };

            return DamageAnalysis {
                atk: atk_comp.0,
                atk_ratio: atk_ratio_comp.0,
                hp: hp_comp.0,
                hp_ratio: hp_ratio_comp.0,
                def: def_comp.0,
                def_ratio: def_ratio_comp.0,
                extra_damage: extra_damage_comp.0,

                bonus: HashMap::new(),
                critical: HashMap::new(),
                critical_damage: HashMap::new(),

                melt_enhance: HashMap::new(),
                vaporize_enhance: HashMap::new(),

                healing_bonus: healing_bonus_comp.0,
                def_minus: HashMap::new(),
                res_minus: HashMap::new(),

                element,
                is_heal: true,

                normal: damage_normal,
                melt: damage_normal,
                vaporize: damage_normal
            }
        }

        let bonus_comp = self.get_bonus_composition(attribute, element, skill);
        let bonus = bonus_comp.sum();

        let critical_comp = self.get_critical_composition(attribute, element, skill);
        let critical = critical_comp.sum();

        let critical_damage_comp = self.get_critical_damage_composition(attribute, element, skill);
        let critical_damage = critical_damage_comp.sum();

        let def_minus_comp = self.get_def_minus_composition(attribute);
        let def_minus = def_minus_comp.sum();
        let res_minus_comp = self.get_res_minus_composition(attribute);
        let res_minus = res_minus_comp.sum();
        let defensive_ratio = enemy.get_defensive_ratio(character_level, def_minus);
        let resistance_ratio = enemy.get_resistance_ratio(element, res_minus);

        let melt_enhance_comp = self.get_enhance_melt_composition(attribute);
        let melt_enhance = melt_enhance_comp.sum();
        let vaporize_enhance_comp = self.get_enhance_vaporize_composition(attribute);
        let vaporize_enhance = vaporize_enhance_comp.sum();

        let melt_ratio = if element == Element::Pyro { 2.0 } else { 1.5 };
        let vaporize_ratio = if element == Element::Hydro { 2.0 } else { 1.5 };

        let damage_normal = DamageResult {
            expectation: base_damage * (1.0 + bonus) * (1.0 + critical * critical_damage),
            critical: base_damage * (1.0 + bonus) * (1.0 + critical_damage),
            non_critical: base_damage * (1.0 + bonus)
        } * (defensive_ratio * resistance_ratio);

        let damage_melt = damage_normal * melt_ratio * (1.0 + melt_enhance);
        let damage_vaporize = damage_normal * vaporize_ratio * (1.0 + vaporize_enhance);

        DamageAnalysis {
            atk: atk_comp.0,
            atk_ratio: atk_ratio_comp.0,
            hp: hp_comp.0,
            hp_ratio: hp_ratio_comp.0,
            def: def_comp.0,
            def_ratio: def_ratio_comp.0,
            extra_damage: extra_damage_comp.0,
            bonus: bonus_comp.0,
            critical: critical_comp.0,
            critical_damage: critical_damage_comp.0,

            melt_enhance: melt_enhance_comp.0,
            vaporize_enhance: vaporize_enhance_comp.0,

            healing_bonus: HashMap::new(),
            def_minus: self.extra_def_minus.0.clone(),
            res_minus: self.extra_res_minus.0.clone(),

            element,
            is_heal: false,

            normal: damage_normal,
            melt: damage_melt,
            vaporize: damage_vaporize
        }
    }
}

impl ComplicatedDamageBuilder {
    fn get_def_minus_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::DefMinus);
        comp.merge(&self.extra_def_minus);
        comp
    }

    fn get_res_minus_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::ResMinus);
        comp.merge(&self.extra_res_minus);
        comp
    }

    fn get_extra_damage_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut comp = attribute.get_composition_merge(&vec![
            AttributeName::ExtraDmgBase,
            AttributeName::extra_dmg_name_by_element(element),
            AttributeName::extra_dmg_name_by_skill_type(skill)
        ]);
        comp.merge(&self.extra_damage);
        comp
    }

    fn get_healing_bonus_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::HealingBonus);
        comp.merge(&self.extra_healing_bonus);
        comp
    }

    fn get_enhance_melt_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::EnhanceMelt);
        comp.merge(&self.extra_enhance_melt);
        let em = attribute.get_value(AttributeName::ElementalMastery);
        if em > 0.0 {
            comp.add_value("精通", Reaction::amp(em));
        }
        comp
    }

    fn get_enhance_vaporize_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::EnhanceVaporize);
        comp.merge(&self.extra_enhance_vaporize);
        let em = attribute.get_value(AttributeName::ElementalMastery);
        if em > 0.0 {
            comp.add_value("精通", Reaction::amp(em));
        }
        comp
    }

    fn get_critical_damage_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut comp = attribute.get_composition_merge(&vec![
            AttributeName::CriticalDamageBase,
            AttributeName::critical_damage_name_by_element(element),
            AttributeName::critical_damage_name_by_skill_name(skill)
        ]);
        comp.merge(&self.extra_critical_damage);
        comp
    }

    fn get_critical_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut comp = attribute.get_composition_merge(&vec![
            AttributeName::CriticalBase,
            AttributeName::critical_rate_name_by_element(element),
            AttributeName::critical_rate_name_by_skill_type(skill)
        ]);
        comp.merge(&self.extra_critical_rate);
        comp
    }

    fn get_bonus_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut comp = attribute.get_composition_merge(&vec![
            AttributeName::BonusBase,
            AttributeName::bonus_name_by_element(element),
            AttributeName::bonus_name_by_skill_type(skill)
        ]);
        comp.merge(&self.extra_bonus);
        comp
    }

    fn get_atk_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> (EntryType, EntryType) {
        let mut atk_comp =
            attribute.get_composition_merge(&vec![AttributeName::ATKBase, AttributeName::ATKPercentage, AttributeName::ATKFixed]);
        atk_comp.merge(&self.extra_atk);

        let mut atk_ratio_comp = attribute.get_composition_merge(&vec![
            AttributeName::ATKRatioBase,
            AttributeName::atk_ratio_name_by_element(element),
            AttributeName::atk_ratio_name_by_skill_type(skill)
        ]);
        atk_ratio_comp.merge(&self.ratio_atk);

        (atk_comp, atk_ratio_comp)
    }

    fn get_def_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> (EntryType, EntryType) {
        let mut def_comp = attribute.get_composition_merge(&vec![
            AttributeName::DEFBase,
            AttributeName::DEFPercentage,
            AttributeName::DEFFixed
        ]);
        def_comp.merge(&self.extra_def);

        let mut def_ratio_comp = attribute.get_composition_merge(&vec![
            AttributeName::DEFRatioBase,
            AttributeName::def_ratio_name_by_element(element),
            AttributeName::def_ratio_name_by_skill_type(skill)
        ]);
        def_ratio_comp.merge(&self.ratio_def);

        (def_comp, def_ratio_comp)
    }

    fn get_hp_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> (EntryType, EntryType) {
        let mut hp_comp = attribute.get_composition_merge(&vec![
            AttributeName::HPBase,
            AttributeName::HPPercentage,
            AttributeName::HPFixed
        ]);
        hp_comp.merge(&self.extra_hp);

        let mut hp_ratio_comp = attribute.get_composition_merge(&vec![
            AttributeName::HPRatioBase,
            AttributeName::hp_ratio_name_by_element(element),
            AttributeName::hp_ratio_name_by_skill_type(skill)
        ]);
        hp_ratio_comp.merge(&self.ratio_hp);

        (hp_comp, hp_ratio_comp)
    }
}