use crate::attribute::{Attribute, AttributeCommon, AttributeName, SimpleAttributeGraph2};
use crate::common::{DamageResult, Element, SkillType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::damage_result::SimpleDamageResult;
use crate::damage::level_coefficient::LEVEL_MULTIPLIER;
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
    pub extra_def_penetration: f64,

    pub ratio_atk: f64,
    pub ratio_def: f64,
    pub ratio_hp: f64,
    pub ratio_em: f64,

    pub extra_enhance_melt: f64,
    pub extra_enhance_vaporize: f64,
    pub enhance_melt: f64,
    pub enhance_vaporize: f64,
    pub extra_em: f64,
}

impl DamageBuilder for SimpleDamageBuilder {
    type Result = SimpleDamageResult;
    type AttributeType = SimpleAttributeGraph2;

    fn new() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn add_em_ratio(&mut self, _key: &str, value: f64) {
        self.ratio_em += value;
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

    fn add_extra_em(&mut self, _key: &str, value: f64) {
        self.extra_em += value;
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

    fn add_extra_def_penetration(&mut self, _key: &str, value: f64) {
        self.extra_def_penetration += value;
    }

    fn add_extra_res_minus(&mut self, _key: &str, value: f64) {
        self.extra_res_minus += value
    }

    fn damage(&self, attribute: &Self::AttributeType, enemy: &Enemy, element: Element, skill: SkillType, character_level: usize, fumo: Option<Element>) -> Self::Result {
        let atk = attribute.get_atk() + self.extra_atk;
        let def = attribute.get_def() + self.extra_def;
        let hp = attribute.get_hp() + self.extra_hp;
        let em = self.extra_em + attribute.get_em_all();

        let element = if skill == SkillType::NormalAttack || skill == SkillType::ChargedAttack || skill.is_plunging() {
            if let Some(x) = fumo {
                x
            } else {
                element
            }
        } else {
            element
        };

        let element = if element == Element::Physical {
            if let Some(x) = fumo {
                x
            } else {
                element
            }
        } else {
            element
        };

        let base_plunging = match skill {
            SkillType::PlungingAttackOnGround => attribute.get_value(AttributeName::ExtraDmgPlungingAttackLowHigh),
            _ => 0.0
        };
        let base
            = (attribute.get_def_ratio(element, skill) + self.ratio_def) * def
            + (attribute.get_hp_ratio(element, skill) + self.ratio_hp) * hp
            + (attribute.get_atk_ratio(element, skill) + self.ratio_atk) * atk
            + em * self.ratio_em
            + attribute.get_extra_damage(element, skill)
            + self.extra_damage
            + base_plunging;

        let bonus
            = attribute.get_bonus(element, skill)
            + self.extra_bonus;

        let critical_rate
            = attribute.get_critical_rate(element, skill)
            + self.extra_critical_rate;
        let critical_rate = critical_rate.clamp(0.0, 1.0);

        let critical_damage
            = attribute.get_critical_damage(element, skill)
            + self.extra_critical_damage;


        let defensive_ratio = {
            let def_minus = self.extra_def_minus + attribute.get_enemy_def_minus(element, skill);
            let def_penetration = self.extra_def_penetration + attribute.get_value(AttributeName::DefPenetration);
            enemy.get_defensive_ratio(character_level, def_minus, def_penetration)
        };
        let resistance_ratio = {
            let res_minus = self.extra_res_minus + attribute.get_enemy_res_minus(element, skill);
            enemy.get_resistance_ratio(element, res_minus)
        };

        let normal_damage = DamageResult {
            critical: base * (1.0 + bonus) * (1.0 + critical_damage),
            non_critical: base * (1.0 + bonus),
            expectation: base * (1.0 + bonus) * (1.0 + critical_damage * critical_rate),
            is_heal: false,
            is_shield: false
        } * (defensive_ratio * resistance_ratio);

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

        let spread_damage = if element != Element::Dendro {
            None
        } else {
            let spread_base_damage = {
                let reaction_ratio = 1.25;
                let bonus = Reaction::catalyze(em);
                let enhance_spread = attribute.get_value(AttributeName::EnhanceSpread);
                base + LEVEL_MULTIPLIER[character_level - 1] * reaction_ratio * (1.0 + bonus + enhance_spread)
            };

            let dmg = DamageResult {
                critical: spread_base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                non_critical: spread_base_damage * (1.0 + bonus),
                expectation: spread_base_damage * (1.0 + bonus) * (1.0 + critical_damage * critical_rate),
                is_heal: false,
                is_shield: false
            } * (defensive_ratio * resistance_ratio);
            Some(dmg)
        };

        let aggravate_damage = if element != Element::Electro {
            None
        } else {
            let aggravate_base_damage = {
                let reaction_ratio = 1.15;
                let bonus = Reaction::catalyze(em);
                let enhance_aggravate = attribute.get_value(AttributeName::EnhanceAggravate);
                base + LEVEL_MULTIPLIER[character_level - 1] * reaction_ratio * (1.0 + bonus + enhance_aggravate)
            };

            let dmg = DamageResult {
                critical: aggravate_base_damage * (1.0 + bonus) * (1.0 + critical_damage),
                non_critical: aggravate_base_damage * (1.0 + bonus),
                expectation: aggravate_base_damage * (1.0 + bonus) * (1.0 + critical_damage * critical_rate),
                is_heal: false,
                is_shield: false
            } * (defensive_ratio * resistance_ratio);
            Some(dmg)
        };

        SimpleDamageResult {
            normal: normal_damage,
            melt: melt_damage,
            vaporize: vaporize_damage,
            spread: spread_damage,
            aggravate: aggravate_damage,
            is_shield: false,
            is_heal: false,
        }
    }

    fn heal(&self, attribute: &Self::AttributeType) -> Self::Result {
        let atk = attribute.get_atk() + self.extra_atk;
        let def = attribute.get_def() + self.extra_def;
        let hp = attribute.get_hp() + self.extra_hp;

        let base = self.ratio_def * def + self.ratio_hp * hp + self.ratio_atk * atk + self.extra_damage;

        let healing_bonus = attribute.get_value(AttributeName::HealingBonus);
        let heal_value = base * (1.0 + healing_bonus);
        let result = {
            DamageResult {
                critical: heal_value,
                non_critical: heal_value,
                expectation: heal_value,
                is_heal: true,
                is_shield: false
            }
        };
        return SimpleDamageResult {
            normal: result,
            melt: None,
            vaporize: None,
            spread: None,
            aggravate: None,
            is_heal: true,
            is_shield: false,
        };
    }

    fn shield(&self, attribute: &Self::AttributeType, _element: Element) -> Self::Result {
        let atk = attribute.get_atk() + self.extra_atk;
        let def = attribute.get_def() + self.extra_def;
        let hp = attribute.get_hp() + self.extra_hp;

        let base = self.ratio_def * def + self.ratio_hp * hp + self.ratio_atk * atk + self.extra_damage;

        let shield_strength = attribute.get_value(AttributeName::ShieldStrength);
        let shield_value = base * (1.0 + shield_strength);
        let result = {
            DamageResult {
                critical: shield_value,
                non_critical: shield_value,
                expectation: shield_value,
                is_heal: false,
                is_shield: true
            }
        };
        return SimpleDamageResult {
            normal: result,
            melt: None,
            vaporize: None,
            spread: None,
            aggravate: None,
            is_shield: true,
            is_heal: false,
        };
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
            ratio_em: 0.0,
            extra_atk: 0.0,

            extra_def: 0.0,
            extra_hp: 0.0,
            extra_def_minus: 0.0,
            extra_def_penetration: 0.0,
            extra_res_minus: 0.0,

            enhance_melt: 0.0,
            enhance_vaporize: 0.0,
            extra_enhance_vaporize: 0.0,
            extra_em: 0.0
        }
    }

    // pub fn damage_without_attribute(&self, enemy: &Enemy) -> <SimpleDamageBuilder as DamageBuilder>::Result {
    //     let atk = self.extra_atk;
    //     let def = self.extra_def;
    //     let hp = self.extra_hp;
    //
    //     let base
    //         = self.ratio_def * def
    //         + self.ratio_hp * hp
    //         + self.ratio_atk * atk
    //         + self.extra_damage;
    //
    //     let bonus = self.extra_bonus;
    //
    //     let critical_rate = self.extra_critical_rate;
    //     let critical_rate = critical_rate.clamp(0.0, 1.0);
    //
    //     let critical_damage = self.extra_critical_damage;
    //
    //     let def_minus = self.extra_def_minus;
    //     let def_penetration = self.extra_def_penetration;
    //     let defensive_ratio = enemy.get_defensive_ratio(character_level, def_minus, def_penetration);
    //     let res_minus = self.extra_res_minus;
    //     let resistance_ratio = enemy.get_resistance_ratio(element, res_minus);
    //
    //     let normal_damage = DamageResult {
    //         critical: base * (1.0 + bonus) * (1.0 + critical_damage),
    //         non_critical: base * (1.0 + bonus),
    //         expectation: base * (1.0 + bonus) * (1.0 + critical_damage * critical_rate),
    //         is_heal: false,
    //         is_shield: false
    //     } * (defensive_ratio * resistance_ratio);
    //
    //     let em = self.extra_em;
    //     let em_amp = Reaction::amp(em);
    //
    //     let melt_damage = if element != Element::Pyro && element != Element::Cryo {
    //         None
    //     } else {
    //         let reaction_ratio = if element == Element::Pyro { 2.0 } else { 1.5 };
    //         let enhance = em_amp + self.extra_enhance_melt;
    //         Some(normal_damage * (reaction_ratio * (1.0 + enhance)))
    //     };
    //
    //     let vaporize_damage = if element != Element::Pyro && element != Element::Hydro {
    //         None
    //     } else {
    //         let reaction_ratio = if element == Element::Pyro { 1.5 } else { 2.0 };
    //         let enhance = em_amp + self.extra_enhance_vaporize;
    //         Some(normal_damage * (reaction_ratio * (1.0 + enhance)))
    //     };
    //
    //     SimpleDamageResult {
    //         normal: normal_damage,
    //         melt: melt_damage,
    //         vaporize: vaporize_damage,
    //         is_shield: false,
    //         is_heal: false,
    //     }
    // }
}
