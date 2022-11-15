use wasm_bindgen::prelude::*;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::{Element, SkillType};
use crate::common::reaction_type::TransformativeType;
use crate::enemies::Enemy;
use crate::damage::level_coefficient::{LEVEL_MULTIPLIER, CRYSTALLIZE_BASE};

#[wasm_bindgen]
pub struct TransformativeDamage {
    pub swirl_cryo: f64,
    pub swirl_hydro: f64,
    pub swirl_pyro: f64,
    pub swirl_electro: f64,
    pub overload: f64,
    pub electro_charged: f64,
    pub shatter: f64,
    pub superconduct: f64,
    pub bloom: f64,
    pub hyperbloom: f64,
    pub burgeon: f64,
    pub burning: f64,
    pub crystallize: f64,
}

#[inline]
fn get_reaction_coefficient(t: TransformativeType) -> f64 {
    use TransformativeType::*;
    match t {
        Burgeon | Hyperbloom => 3.0,
        Overload | Bloom => 2.0,
        Superconduct => 0.5,
        SwirlHydro | SwirlElectro | SwirlCryo | SwirlPyro => 0.6,
        ElectroCharged => 1.2,
        Shatter => 1.5,
        Burning => 0.25,
    }
}

#[inline]
pub fn get_transformative_base(level: usize, t: TransformativeType) -> f64 {
    LEVEL_MULTIPLIER[level - 1] * get_reaction_coefficient(t)
}

#[inline]
pub fn get_em_bonus(em: f64) -> f64 {
    16.0 * em / (em + 2000.0)
}

pub fn transformative_damage_simple(level: usize, em: f64, enemy: &Enemy) -> TransformativeDamage {
    let base_swirl = get_transformative_base(level, TransformativeType::SwirlPyro);
    let base_overload = get_transformative_base(level, TransformativeType::Overload);
    let base_superconduct = get_transformative_base(level, TransformativeType::Superconduct);
    let base_shatter = get_transformative_base(level, TransformativeType::Shatter);
    let base_electro_charged = get_transformative_base(level, TransformativeType::ElectroCharged);
    let base_bloom = get_transformative_base(level, TransformativeType::Bloom);
    let base_hyperbloom = get_transformative_base(level, TransformativeType::Hyperbloom);
    let base_burgeon = base_hyperbloom;
    let base_burning = get_transformative_base(level, TransformativeType::Burning);

    let em_bonus = get_em_bonus(em);

    let res_ratio_pyro = enemy.get_resistance_ratio(Element::Pyro, 0.0);
    let res_ratio_cryo = enemy.get_resistance_ratio(Element::Cryo, 0.0);
    let res_ratio_electro = enemy.get_resistance_ratio(Element::Electro, 0.0);
    let res_ratio_hydro = enemy.get_resistance_ratio(Element::Hydro, 0.0);
    let res_ratio_physical = enemy.get_resistance_ratio(Element::Physical, 0.0);
    let res_ratio_dendro = enemy.get_resistance_ratio(Element::Dendro, 0.0);

    let dmg_swirl_pyro = base_swirl * res_ratio_pyro * (1.0 + em_bonus);
    let dmg_swirl_cryo = base_swirl * res_ratio_cryo * (1.0 + em_bonus);
    let dmg_swirl_electro = base_swirl * res_ratio_electro * (1.0 + em_bonus);
    let dmg_swirl_hydro = base_swirl * res_ratio_hydro * (1.0 + em_bonus);
    let dmg_overload = base_overload * res_ratio_pyro * (1.0 + em_bonus);
    let dmg_superconduct = base_superconduct * res_ratio_cryo * (1.0 + em_bonus);
    let dmg_shatter = base_shatter * res_ratio_physical * (1.0 + em_bonus);
    let dmg_electro_charged = base_electro_charged * res_ratio_electro * (1.0 + em_bonus);
    let dmg_bloom = base_bloom * res_ratio_dendro * (1.0 + em_bonus);
    let dmg_hyperbloom = base_hyperbloom * res_ratio_dendro * (1.0 + em_bonus);
    let dmg_burgeon = base_burgeon * res_ratio_dendro * (1.0 + em_bonus);
    let dmg_burning = base_burning * res_ratio_pyro * (1.0 + em_bonus);
    let shield_crystallize = CRYSTALLIZE_BASE[level - 1] * (1.0 + 40.0 / 9.0 * em / (em + 1400.0));


    TransformativeDamage {
        swirl_cryo: dmg_swirl_cryo,
        swirl_hydro: dmg_swirl_hydro,
        swirl_pyro: dmg_swirl_pyro,
        swirl_electro: dmg_swirl_electro,
        overload: dmg_overload,
        electro_charged: dmg_electro_charged,
        shatter: dmg_shatter,
        superconduct: dmg_superconduct,
        bloom: dmg_bloom,
        hyperbloom: dmg_hyperbloom,
        burgeon: dmg_burgeon,
        burning: dmg_burning,
        crystallize: shield_crystallize,
    }
}

pub fn transformative_damage<A: Attribute>(level: usize, attribute: &A, enemy: &Enemy) -> TransformativeDamage {
    let enhance_base = attribute.get_value(AttributeName::EnhanceSwirlBase);
    let enhance_swirl_pyro = attribute.get_value(AttributeName::EnhanceSwirlPyro) + enhance_base;
    let enhance_swirl_cryo = attribute.get_value(AttributeName::EnhanceSwirlCryo) + enhance_base;
    let enhance_swirl_electro = attribute.get_value(AttributeName::EnhanceSwirlElectro) + enhance_base;
    let enhance_swirl_hydro = attribute.get_value(AttributeName::EnhanceSwirlHydro) + enhance_base;
    let enhance_overload = attribute.get_value(AttributeName::EnhanceOverload);
    let enhance_superconduct = attribute.get_value(AttributeName::EnhanceSuperconduct);
    let enhance_shatter = attribute.get_value(AttributeName::EnhanceShatter);
    let enhance_electro_charged = attribute.get_value(AttributeName::EnhanceElectroCharged);
    let enhance_bloom = attribute.get_value(AttributeName::EnhanceBloom);
    let enhance_hyperbloom = attribute.get_value(AttributeName::EnhanceHyperbloom);
    let enhance_burgeon = attribute.get_value(AttributeName::EnhanceBurgeon);
    let enhance_burning = attribute.get_value(AttributeName::EnhanceBurning);

    let base_swirl = get_transformative_base(level, TransformativeType::SwirlPyro);
    let base_overload = get_transformative_base(level, TransformativeType::Overload);
    let base_superconduct = get_transformative_base(level, TransformativeType::Superconduct);
    let base_shatter = get_transformative_base(level, TransformativeType::Shatter);
    let base_electro_charged = get_transformative_base(level, TransformativeType::ElectroCharged);
    let base_bloom = get_transformative_base(level, TransformativeType::Bloom);
    let base_hyperbloom = get_transformative_base(level, TransformativeType::Hyperbloom);
    let base_burgeon = base_hyperbloom;
    let base_burning = get_transformative_base(level, TransformativeType::Burning);

    let em = attribute.get_em_all();
    let em_bonus = get_em_bonus(em);

    // skill type is not used, thus passing normal attack
    let res_pyro = attribute.get_enemy_res_minus(Element::Pyro, SkillType::NormalAttack);
    let res_cryo = attribute.get_enemy_res_minus(Element::Cryo, SkillType::NormalAttack);
    let res_electro = attribute.get_enemy_res_minus(Element::Electro, SkillType::NormalAttack);
    let res_hydro = attribute.get_enemy_res_minus(Element::Hydro, SkillType::NormalAttack);
    let res_physical = attribute.get_enemy_res_minus(Element::Physical, SkillType::NormalAttack);
    let res_dendro = attribute.get_enemy_res_minus(Element::Dendro, SkillType::NormalAttack);

    let res_ratio_pyro = enemy.get_resistance_ratio(Element::Pyro, res_pyro);
    let res_ratio_cryo = enemy.get_resistance_ratio(Element::Cryo, res_cryo);
    let res_ratio_electro = enemy.get_resistance_ratio(Element::Electro, res_electro);
    let res_ratio_hydro = enemy.get_resistance_ratio(Element::Hydro, res_hydro);
    let res_ratio_physical = enemy.get_resistance_ratio(Element::Physical, res_physical);
    let res_ratio_dendro = enemy.get_resistance_ratio(Element::Dendro, res_dendro);

    let dmg_swirl_pyro = base_swirl * res_ratio_pyro * (1.0 + em_bonus + enhance_swirl_pyro);
    let dmg_swirl_cryo = base_swirl * res_ratio_cryo * (1.0 + em_bonus + enhance_swirl_cryo);
    let dmg_swirl_electro = base_swirl * res_ratio_electro * (1.0 + em_bonus + enhance_swirl_electro);
    let dmg_swirl_hydro = base_swirl * res_ratio_hydro * (1.0 + em_bonus + enhance_swirl_hydro);
    let dmg_overload = base_overload * res_ratio_pyro * (1.0 + em_bonus + enhance_overload);
    let dmg_superconduct = base_superconduct * res_ratio_cryo * (1.0 + em_bonus + enhance_superconduct);
    let dmg_shatter = base_shatter * res_ratio_physical * (1.0 + em_bonus + enhance_shatter);
    let dmg_electro_charged = base_electro_charged * res_ratio_electro * (1.0 + em_bonus + enhance_electro_charged);
    let dmg_bloom = base_bloom * res_ratio_dendro * (1.0 + em_bonus + enhance_bloom);
    let dmg_hyperbloom = base_hyperbloom * res_ratio_dendro * (1.0 + em_bonus + enhance_hyperbloom);
    let dmg_burgeon = base_burgeon * res_ratio_dendro * (1.0 + em_bonus + enhance_burgeon);
    let dmg_burning = base_burning * res_ratio_pyro * (1.0 + em_bonus + enhance_burning);
    let shield_crystallize = CRYSTALLIZE_BASE[level - 1] * (1.0 + 40.0 / 9.0 * em / (em + 1400.0));

    TransformativeDamage {
        swirl_cryo: dmg_swirl_cryo,
        swirl_hydro: dmg_swirl_hydro,
        swirl_pyro: dmg_swirl_pyro,
        swirl_electro: dmg_swirl_electro,
        overload: dmg_overload,
        electro_charged: dmg_electro_charged,
        shatter: dmg_shatter,
        superconduct: dmg_superconduct,
        bloom: dmg_bloom,
        hyperbloom: dmg_hyperbloom,
        burgeon: dmg_burgeon,
        burning: dmg_burning,
        crystallize: shield_crystallize,
    }
}

// because there is no element, so we don't know which res to use, so use res_ratio explicitly
pub fn swirl_without_element<A: Attribute>(level: usize, attribute: &A, res_ratio: f64) -> f64 {
    let enhance_swirl_base = attribute.get_value(AttributeName::EnhanceSwirlBase);
    let em = attribute.get_em_all();
    let em_bonus = get_em_bonus(em);

    let base = get_transformative_base(level, TransformativeType::SwirlPyro) as f64;

    base * res_ratio * (1.0 + em_bonus + enhance_swirl_base)
}
