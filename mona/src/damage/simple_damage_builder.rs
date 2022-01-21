use crate::attribute::{Attribute, AttributeCommon, AttributeName, SimpleAttributeGraph2};
use crate::common::{DamageResult, Element, SkillType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::damage_result::SimpleDamageResult;
use crate::damage::reaction::Reaction;
use crate::enemies::Enemy;

pub struct SimpleDamageBuilder {
    pub extra_critical_damage: f64,
    pub extra_critical_rate: f64,
    pub extra_bonus: f64,
    pub extra_damage: f64,
    pub extra_atk: f64,
    pub extra_def: f64,
    pub extra_hp: f64,

    pub extra_def_minus: f64,
    pub extra_res_minus: f64,

    pub ratio_atk: f64,
    pub ratio_def: f64,
    pub ratio_hp: f64,

    pub extra_enhance_melt: f64,
    pub extra_enhance_vaporize: f64,

    pub enhance_melt: f64,
    pub enhance_vaporize: f64,
}

impl DamageBuilder for SimpleDamageBuilder {
    type Result = SimpleDamageResult;
    type AttributeType = SimpleAttributeGraph2;

    fn new() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn add_atk_ratio(&mut self, _key: &str, value: f64) {
        self.ratio_atk += value
    }

    fn add_def_ratio(&mut self, _key: &str, value: f64) {
        self.ratio_def += value
    }

    fn add_hp_ratio(&mut self, _key: &str, value: f64) {
        self.ratio_hp += value
    }

    fn add_extra_atk(&mut self, _key: &str, value: f64) {
        self.extra_atk += value
    }

    fn add_extra_def(&mut self, _key: &str, value: f64) {
        self.extra_def += value
    }

    fn add_extra_hp(&mut self, _key: &str, value: f64) {
        self.extra_hp += value
    }

    fn add_extra_damage(&mut self, _key: &str, value: f64) {
        self.extra_damage += value
    }

    fn add_extra_critical(&mut self, _key: &str, value: f64) {
        self.extra_critical_rate += value
    }

    fn add_extra_critical_damage(&mut self, _key: &str, value: f64) {
        self.extra_critical_damage += value
    }

    fn add_extra_bonus(&mut self, _key: &str, value: f64) {
        self.extra_bonus += value
    }

    fn add_extra_enhance_melt(&mut self, _key: &str, value: f64) {
        self.extra_enhance_melt += value
    }

    fn add_extra_enhance_vaporize(&mut self, _key: &str, value: f64) {
        self.extra_enhance_vaporize += value
    }

    fn add_extra_def_minus(&mut self, _key: &str, value: f64) {
        self.extra_def_minus += value
    }

    fn add_extra_res_minus(&mut self, _key: &str, value: f64) {
        self.extra_res_minus += value
    }

    fn build(&self, attribute: &Self::AttributeType, enemy: &Enemy, element: Element, skill: SkillType, is_heal: bool, character_level: usize) -> Self::Result {
        let atk = attribute.get_atk() + self.extra_atk;
        let def = attribute.get_def() + self.extra_def;
        let hp = attribute.get_hp() + self.extra_hp;

        if is_heal {
            let base = self.ratio_def * def + self.ratio_hp * hp + self.ratio_atk * atk + self.extra_damage;

            let healing_bonus = attribute.get_value(AttributeName::HealingBonus);
            let heal_value = base * (1.0 + healing_bonus);
            let result = {
                DamageResult {
                    critical: heal_value,
                    non_critical: heal_value,
                    expectation: heal_value
                }
            };
            return SimpleDamageResult {
                normal: result,
                melt: None,
                vaporize: None
            };
        }

        let base
            = (attribute.get_def_ratio(element, skill) + self.ratio_def) * def
            + (attribute.get_hp_ratio(element, skill) + self.ratio_hp) * hp
            + (attribute.get_atk_ratio(element, skill) + self.ratio_atk) * atk
            + attribute.get_extra_damage(element, skill)
            + self.extra_damage;

        let bonus
            = attribute.get_bonus(element, skill)
            + self.extra_bonus;

        let critical_rate
            = attribute.get_critical_rate(element, skill)
            + self.extra_critical_rate;
        let critical_rate = critical_rate.min(1.0);

        let critical_damage
            = attribute.get_critical_damage(element, skill)
            + self.extra_critical_damage;

        let def_minus = self.extra_def_minus + attribute.get_value(AttributeName::DefMinus);
        let defensive_ratio = enemy.get_defensive_ratio(character_level, def_minus);
        let res_minus = self.extra_res_minus + attribute.get_value(AttributeName::ResMinus);
        let resistance_ratio = enemy.get_resistance_ratio(element, res_minus);

        let normal_damage = DamageResult {
            critical: base * (1.0 + bonus) * (1.0 + critical_damage),
            non_critical: base * (1.0 + bonus),
            expectation: base * (1.0 + bonus) * (1.0 + critical_damage * critical_rate),
        } * (defensive_ratio * resistance_ratio);

        let em = attribute.get_value(AttributeName::ElementalMastery);
        let em_amp = Reaction::amp(em);

        let melt_damage = if element != Element::Pyro && element != Element::Cryo {
            None
        } else {
            let reaction_ratio = if element == Element::Pyro { 2.0 } else { 1.5 };
            let enhance = em_amp + self.extra_enhance_melt + attribute.get_value(AttributeName::EnhanceMelt);
            Some(normal_damage * (reaction_ratio * (1.0 + enhance)))
        };

        let vaporize_damage = if element != Element::Pyro && element != Element::Hydro {
            None
        } else {
            let reaction_ratio = if element == Element::Pyro { 1.5 } else { 2.0 };
            let enhance = em_amp + self.extra_enhance_vaporize + attribute.get_value(AttributeName::EnhanceVaporize);
            Some(normal_damage * (reaction_ratio * (1.0 + enhance)))
        };

        SimpleDamageResult {
            normal: normal_damage,
            melt: melt_damage,
            vaporize: vaporize_damage
        }
    }
}

impl SimpleDamageBuilder {
    pub fn new(ratio_atk: f64, ratio_def: f64, ratio_hp: f64) -> SimpleDamageBuilder {
        SimpleDamageBuilder {
            extra_damage: 0.0,
            extra_critical_rate: 0.0,
            extra_critical_damage: 0.0,
            extra_bonus: 0.0,
            ratio_atk,
            ratio_hp,
            extra_enhance_melt: 0.0,
            ratio_def,
            extra_atk: 0.0,

            extra_def: 0.0,
            extra_hp: 0.0,
            extra_def_minus: 0.0,
            extra_res_minus: 0.0,

            enhance_melt: 0.0,
            enhance_vaporize: 0.0,
            extra_enhance_vaporize: 0.0
        }
    }
}