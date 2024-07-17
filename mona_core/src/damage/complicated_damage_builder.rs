use std::collections::HashMap;
use crate::attribute::{Attribute, AttributeName, ComplicatedAttributeGraph, AttributeCommon};
use crate::common::{DamageResult, Element, SkillType};
use crate::damage::damage_analysis::DamageAnalysis;
use crate::enemies::Enemy;
use crate::common::EntryType;
use crate::damage::damage_builder::{DamageBuilder};
use crate::damage::level_coefficient::LEVEL_MULTIPLIER;
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
    pub extra_em: EntryType,

    pub extra_def_minus: EntryType,
    pub extra_def_penetration: EntryType,
    pub extra_res_minus: EntryType,

    pub ratio_atk: EntryType,
    pub ratio_def: EntryType,
    pub ratio_hp: EntryType,
    pub ratio_em: EntryType,
}

impl DamageBuilder for ComplicatedDamageBuilder {
    type Result = DamageAnalysis;
    type AttributeType = ComplicatedAttributeGraph;

    fn new() -> Self {
        ComplicatedDamageBuilder::default()
    }

    fn add_em_ratio(&mut self, key: &str, value: f64) {
        *self.ratio_em.0.entry(String::from(key)).or_insert(0.0) += value;
    }
    
    fn add_atk_ratio(&mut self, key: &str, value: f64) {
        *self.ratio_atk.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_def_ratio(&mut self, key: &str, value: f64) {
        *self.ratio_def.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_hp_ratio(&mut self, key: &str, value: f64) {
        *self.ratio_hp.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_em(&mut self, key: &str, value: f64) {
        *self.extra_em.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_atk(&mut self, key: &str, value: f64) {
        *self.extra_atk.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_def(&mut self, key: &str, value: f64) {
        *self.extra_def.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_hp(&mut self, key: &str, value: f64) {
        *self.extra_hp.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_damage(&mut self, key: &str, value: f64) {
        *self.extra_damage.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_critical(&mut self, key: &str, value: f64) {
        *self.extra_critical_rate.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_critical_damage(&mut self, key: &str, value: f64) {
        *self.extra_critical_damage.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_bonus(&mut self, key: &str, value: f64) {
        *self.extra_bonus.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_enhance_melt(&mut self, key: &str, value: f64) {
        *self.extra_enhance_melt.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_enhance_vaporize(&mut self, key: &str, value: f64) {
        *self.extra_enhance_vaporize.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_def_minus(&mut self, key: &str, value: f64) {
        *self.extra_def_minus.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_def_penetration(&mut self, key: &str, value: f64) {
        *self.extra_def_penetration.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn add_extra_res_minus(&mut self, key: &str, value: f64) {
        *self.extra_res_minus.0.entry(String::from(key)).or_insert(0.0) += value;
    }

    fn damage(
        &self,
        attribute:
        &Self::AttributeType,
        enemy: &Enemy,
        element: Element,
        skill: SkillType,
        character_level: usize,
        fumo: Option<Element>
    ) -> Self::Result {
        let element = if skill == SkillType::NormalAttack || skill == SkillType::ChargedAttack || skill.is_plunging() {
            if let Some(x) = fumo {
                x
            } else {
                element
            }
        } else {
            element
        };

        let atk_comp = self.get_atk_composition(attribute);
        let atk = atk_comp.sum();
        let atk_ratio_comp = self.get_atk_ratio_composition(attribute, element, skill);
        let atk_ratio = atk_ratio_comp.sum();

        let def_comp = self.get_def_composition(attribute);
        let def = def_comp.sum();
        let def_ratio_comp = self.get_def_ratio_composition(attribute, element, skill);
        let def_ratio = def_ratio_comp.sum();

        let hp_comp = self.get_hp_composition(attribute);
        let hp = hp_comp.sum();
        let hp_ratio_comp = self.get_hp_ratio_composition(attribute, element, skill);
        let hp_ratio = hp_ratio_comp.sum();

        let em_comp = self.get_em_composition(attribute);
        let em = em_comp.sum();
        let em_ratio_comp = self.get_em_ratio_composition(attribute, element, skill);
        let em_ratio = em_ratio_comp.sum();

        let mut extra_damage_comp = self.get_extra_damage_composition(attribute, element, skill);
        let plunging_extra_damage = match skill {
            SkillType::PlungingAttackOnGround => attribute.get_attribute_composition(AttributeName::ExtraDmgPlungingAttackLowHigh),
            _ => Default::default()
        };
        extra_damage_comp.merge(&plunging_extra_damage);
        let extra_damage = extra_damage_comp.sum();

        let base_damage = atk * atk_ratio + def * def_ratio + hp * hp_ratio + em * em_ratio + extra_damage;

        let bonus_comp = self.get_bonus_composition(attribute, element, skill);
        let bonus = bonus_comp.sum();

        let critical_comp = self.get_critical_composition(attribute, element, skill);
        let critical = critical_comp.sum().clamp(0.0, 1.0);

        let critical_damage_comp = self.get_critical_damage_composition(attribute, element, skill);
        let critical_damage = critical_damage_comp.sum();

        let def_minus_comp = self.get_def_minus_composition(attribute);
        let def_minus = def_minus_comp.sum();
        let def_penetration_comp = self.get_def_penetration_composition(attribute);
        let def_penetration = def_penetration_comp.sum();
        let res_minus_comp = self.get_res_minus_composition(attribute, element);
        let res_minus = res_minus_comp.sum();
        let defensive_ratio = enemy.get_defensive_ratio(character_level, def_minus, def_penetration);
        let resistance_ratio = enemy.get_resistance_ratio(element, res_minus);

        let melt_enhance_comp = self.get_enhance_melt_composition(attribute);
        let melt_enhance = melt_enhance_comp.sum();
        let vaporize_enhance_comp = self.get_enhance_vaporize_composition(attribute);
        let vaporize_enhance = vaporize_enhance_comp.sum();
        let spread_enhance_comp = self.get_enhance_spread_composition(attribute);
        let spread_enhance = spread_enhance_comp.sum();
        let aggravate_enhance_comp = self.get_enhance_aggravate_composition(attribute);
        let aggravate_enhance = aggravate_enhance_comp.sum();

        let melt_ratio = if element == Element::Pyro { 2.0 } else { 1.5 };
        let vaporize_ratio = if element == Element::Hydro { 2.0 } else { 1.5 };

        let damage_normal = DamageResult {
            expectation: base_damage * (1.0 + bonus) * (1.0 + critical * critical_damage),
            critical: base_damage * (1.0 + bonus) * (1.0 + critical_damage),
            non_critical: base_damage * (1.0 + bonus),
            is_heal: false,
            is_shield: false
        } * (defensive_ratio * resistance_ratio);

        let damage_melt = if element == Element::Pyro || element == Element::Cryo {
            Some(damage_normal * melt_ratio * (1.0 + melt_enhance))
        } else {
            None
        };
        let damage_vaporize = if element == Element::Pyro || element == Element::Hydro {
            Some(damage_normal * vaporize_ratio * (1.0 + vaporize_enhance))
        } else {
            None
        };

        let damage_spread = if element != Element::Dendro {
            None
        } else {
            let spread_base_damage = {
                let reaction_ratio = 1.25;
                base_damage + LEVEL_MULTIPLIER[character_level - 1] * reaction_ratio * (1.0 + spread_enhance)
            };

            let dmg = DamageResult {
                critical: spread_base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                non_critical: spread_base_damage * (1.0 + bonus),
                expectation: spread_base_damage * (1.0 + bonus) * (1.0 + critical_damage * critical),
                is_heal: false,
                is_shield: false
            } * (defensive_ratio * resistance_ratio);
            Some(dmg)
        };

        let damage_aggravate = if element != Element::Electro {
            None
        } else {
            let aggravate_base_damage = {
                let reaction_ratio = 1.15;
                base_damage + LEVEL_MULTIPLIER[character_level - 1] * reaction_ratio * (1.0 + aggravate_enhance)
            };

            let dmg = DamageResult {
                critical: aggravate_base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                non_critical: aggravate_base_damage * (1.0 + bonus),
                expectation: aggravate_base_damage * (1.0 + bonus) * (1.0 + critical_damage * critical),
                is_heal: false,
                is_shield: false
            } * (defensive_ratio * resistance_ratio);
            Some(dmg)
        };

        DamageAnalysis {
            atk: atk_comp.0,
            atk_ratio: atk_ratio_comp.0,
            hp: hp_comp.0,
            hp_ratio: hp_ratio_comp.0,
            def: def_comp.0,
            def_ratio: def_ratio_comp.0,
            em: em_comp.0,
            em_ratio: em_ratio_comp.0,
            extra_damage: extra_damage_comp.0,
            bonus: bonus_comp.0,
            critical: critical_comp.0,
            critical_damage: critical_damage_comp.0,
            spread_compose: spread_enhance_comp.0,
            aggravate_compose: aggravate_enhance_comp.0,

            melt_enhance: melt_enhance_comp.0,
            vaporize_enhance: vaporize_enhance_comp.0,

            healing_bonus: HashMap::new(),
            shield_strength: HashMap::new(),
            def_minus: def_minus_comp.0,
            def_penetration: def_penetration_comp.0,
            res_minus: res_minus_comp.0,

            element,
            is_heal: false,
            is_shield: false,

            normal: damage_normal,
            melt: damage_melt,
            vaporize: damage_vaporize,
            spread: damage_spread,
            aggravate: damage_aggravate,
        }
    }

    fn heal(&self, attribute: &Self::AttributeType) -> Self::Result {
        let atk_comp = self.get_atk_composition(attribute);
        let atk = atk_comp.sum();
        let def_comp = self.get_def_composition(attribute);
        let def = def_comp.sum();
        let hp_comp = self.get_hp_composition(attribute);
        let hp = hp_comp.sum();
        let em_comp = self.get_em_composition(attribute);
        let em = em_comp.sum();

        let healing_bonus_comp = self.get_healing_bonus_composition(attribute);
        let healing_bonus = healing_bonus_comp.sum();

        let base = atk * self.ratio_atk.sum() + hp * self.ratio_hp.sum() + def * self.ratio_def.sum() + em * self.ratio_em.sum() + self.extra_damage.sum();

        let heal_value = base * (1.0 + healing_bonus);
        let damage_normal = DamageResult {
            expectation: heal_value,
            critical: heal_value,
            non_critical: heal_value,
            is_heal: true,
            is_shield: false
        };

        return DamageAnalysis {
            atk: atk_comp.0,
            atk_ratio: self.ratio_atk.0.clone(),
            hp: hp_comp.0,
            hp_ratio: self.ratio_hp.0.clone(),
            def: def_comp.0,
            def_ratio: self.ratio_def.0.clone(),
            em: em_comp.0,
            em_ratio: self.ratio_em.0.clone(),
            extra_damage: self.extra_damage.0.clone(),
            spread_compose: HashMap::new(),
            aggravate_compose: HashMap::new(),

            bonus: HashMap::new(),
            critical: HashMap::new(),
            critical_damage: HashMap::new(),

            melt_enhance: HashMap::new(),
            vaporize_enhance: HashMap::new(),

            healing_bonus: healing_bonus_comp.0,
            shield_strength: HashMap::new(),
            def_minus: HashMap::new(),
            def_penetration: HashMap::new(),
            res_minus: HashMap::new(),

            element: Element::Pyro,
            is_heal: true,
            is_shield: false,

            normal: damage_normal,
            melt: None,
            vaporize: None,
            spread: None,
            aggravate: None,
        }
    }

    fn shield(&self, attribute: &Self::AttributeType, element: Element) -> Self::Result {
        let atk_comp = self.get_atk_composition(attribute);
        let atk = atk_comp.sum();
        let def_comp = self.get_def_composition(attribute);
        let def = def_comp.sum();
        let hp_comp = self.get_hp_composition(attribute);
        let hp = hp_comp.sum();
        let em_comp = self.get_em_composition(attribute);
        let em = em_comp.sum();

        let shield_strength_comp = self.get_shield_strength_composition(attribute);
        let shield_strength = shield_strength_comp.sum();

        let base = atk * self.ratio_atk.sum() + hp * self.ratio_hp.sum() + def * self.ratio_def.sum() + em * self.ratio_em.sum() + self.extra_damage.sum();

        let shield_value = base * (1.0 + shield_strength);
        let damage_normal = DamageResult {
            expectation: shield_value,
            critical: 0.0,
            non_critical: 0.0,
            is_heal: false,
            is_shield: true
        };

        return DamageAnalysis {
            atk: atk_comp.0,
            atk_ratio: self.ratio_atk.0.clone(),
            hp: hp_comp.0,
            hp_ratio: self.ratio_hp.0.clone(),
            def: def_comp.0,
            def_ratio: self.ratio_def.0.clone(),
            em: em_comp.0,
            em_ratio: self.ratio_em.0.clone(),
            extra_damage: self.extra_damage.0.clone(),
            spread_compose: HashMap::new(),
            aggravate_compose: HashMap::new(),

            bonus: HashMap::new(),
            critical: HashMap::new(),
            critical_damage: HashMap::new(),

            melt_enhance: HashMap::new(),
            vaporize_enhance: HashMap::new(),

            healing_bonus: HashMap::new(),
            shield_strength: shield_strength_comp.0,
            def_minus: HashMap::new(),
            def_penetration: HashMap::new(),
            res_minus: HashMap::new(),

            element,
            is_heal: true,
            is_shield: false,

            normal: damage_normal,
            melt: None,
            vaporize: None,
            spread: None,
            aggravate: None,
        }
    }
}

impl ComplicatedDamageBuilder {
    fn get_def_minus_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::DefMinus);
        comp.merge(&self.extra_def_minus);
        comp
    }

    fn get_def_penetration_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::DefPenetration);
        comp.merge(&self.extra_def_penetration);
        comp
    }

    fn get_res_minus_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element) -> EntryType {
        let mut comp = attribute.get_composition_merge(&vec![
            AttributeName::ResMinusBase,
            AttributeName::res_minus_name_by_element(element)
        ]);
        comp.merge(&self.extra_res_minus);
        comp
    }

    fn get_extra_damage_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::ExtraDmgBase,
            AttributeName::extra_dmg_name_by_element(element),
        ];
        if let Some(name) = AttributeName::extra_dmg_name_by_skill_type(skill) {
            names.push(name);
        }
        let mut comp = attribute.get_composition_merge(&names);
        comp.merge(&self.extra_damage);
        comp
    }

    fn get_healing_bonus_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::HealingBonus);
        comp.merge(&self.extra_healing_bonus);
        comp
    }

    fn get_shield_strength_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::ShieldStrength);
        // todo, for now there is no extra shield strength
        comp
    }

    fn get_enhance_melt_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::EnhanceMelt);
        comp.merge(&self.extra_enhance_melt);
        let em = self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::amp(em));
        }
        comp
    }

    fn get_enhance_vaporize_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::EnhanceVaporize);
        comp.merge(&self.extra_enhance_vaporize);
        let em = self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::amp(em));
        }
        comp
    }

    fn get_enhance_spread_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::EnhanceSpread);
        let em = &self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::catalyze(em));
        }
        comp
    }

    fn get_enhance_aggravate_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut comp = attribute.get_attribute_composition(AttributeName::EnhanceAggravate);
        let em = &self.extra_em.sum() + attribute.get_em_all();
        if em > 0.0 {
            comp.add_value("精通", Reaction::catalyze(em));
        }
        comp
    }

    fn get_critical_damage_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::CriticalDamageBase,
            AttributeName::critical_damage_name_by_element(element),
        ];
        if let Some(name) = AttributeName::critical_damage_name_by_skill_name(skill) {
            names.push(name);
        }
        let mut comp = attribute.get_composition_merge(&names);
        comp.merge(&self.extra_critical_damage);
        comp
    }

    fn get_critical_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut comp = attribute.get_critical_composition(element, skill);
        comp.merge(&self.extra_critical_rate);
        comp
    }

    fn get_bonus_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::BonusBase,
            AttributeName::bonus_name_by_element(element),
        ];
        if let Some(name) = AttributeName::bonus_name_by_skill_type(skill) {
            names.push(name);
        }
        let mut comp = attribute.get_composition_merge(&names);
        if element != Element::Physical && skill == SkillType::NormalAttack {
            // todo refactor
            comp.merge(&attribute.get_attribute_composition(AttributeName::BonusNormalAndElemental));
        }
        comp.merge(&self.extra_bonus);
        comp
    }

    fn get_atk_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut atk_comp =
            attribute.get_composition_merge(&vec![AttributeName::ATKBase, AttributeName::ATKPercentage, AttributeName::ATKFixed]);
        atk_comp.merge(&self.extra_atk);

        atk_comp
    }

    fn get_atk_ratio_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::ATKRatioBase,
            AttributeName::atk_ratio_name_by_element(element),
        ];
        if let Some(name) = AttributeName::atk_ratio_name_by_skill_type(skill) {
            names.push(name)
        }
        let mut atk_ratio_comp = attribute.get_composition_merge(&names);
        atk_ratio_comp.merge(&self.ratio_atk);

        atk_ratio_comp
    }

    fn get_def_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut def_comp = attribute.get_composition_merge(&vec![
            AttributeName::DEFBase,
            AttributeName::DEFPercentage,
            AttributeName::DEFFixed
        ]);
        def_comp.merge(&self.extra_def);

        def_comp
    }

    fn get_def_ratio_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::DEFRatioBase,
            AttributeName::def_ratio_name_by_element(element),
        ];
        if let Some(name) = AttributeName::def_ratio_name_by_skill_type(skill) {
            names.push(name);
        }
        let mut def_ratio_comp = attribute.get_composition_merge(&names);
        def_ratio_comp.merge(&self.ratio_def);

        def_ratio_comp
    }

    fn get_hp_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut hp_comp = attribute.get_composition_merge(&vec![
            AttributeName::HPBase,
            AttributeName::HPPercentage,
            AttributeName::HPFixed
        ]);
        hp_comp.merge(&self.extra_hp);

        hp_comp
    }

    fn get_em_composition(&self, attribute: &ComplicatedAttributeGraph) -> EntryType {
        let mut em_comp = attribute.get_composition_merge(&vec![
            AttributeName::ElementalMastery,
            AttributeName::ElementalMasteryExtra,
        ]);
        em_comp.merge(&self.extra_em);
        em_comp
    }

    fn get_hp_ratio_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut names = vec![
            AttributeName::HPRatioBase,
            AttributeName::hp_ratio_name_by_element(element),
        ];
        if let Some(name) = AttributeName::hp_ratio_name_by_skill_type(skill) {
            names.push(name)
        }
        let mut hp_ratio_comp = attribute.get_composition_merge(&names);
        hp_ratio_comp.merge(&self.ratio_hp);

        hp_ratio_comp
    }

    fn get_em_ratio_composition(&self, attribute: &ComplicatedAttributeGraph, element: Element, skill: SkillType) -> EntryType {
        let mut em_ratio_comp = attribute.get_composition_merge(&vec![
            // todo
        ]);
        em_ratio_comp.merge(&self.ratio_em);
        em_ratio_comp
    }
}