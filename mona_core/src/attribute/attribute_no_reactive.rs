use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use super::complicated_attribute_graph::ComplicatedAttributeGraph;

use super::attribute_name::AttributeName;


type AttributeEntryType = HashMap<String, f64>;

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug)]
pub struct AttributeNoReactive {
    pub atk: AttributeEntryType,
    pub def: AttributeEntryType,
    pub hp: AttributeEntryType,

    pub healing_bonus: AttributeEntryType,
    pub elemental_mastery: AttributeEntryType,
    pub recharge: AttributeEntryType,
    pub shield_strength: AttributeEntryType,

    pub critical: AttributeEntryType,
    pub critical_damage: AttributeEntryType,

    pub bonus_electro: AttributeEntryType,
    pub bonus_pyro: AttributeEntryType,
    pub bonus_anemo: AttributeEntryType,
    pub bonus_cryo: AttributeEntryType,
    pub bonus_geo: AttributeEntryType,
    pub bonus_hydro: AttributeEntryType,
    pub bonus_dendro: AttributeEntryType,
    pub bonus_physical: AttributeEntryType,
}

// fn merge(x: &mut AttributeEntryType, y: &AttributeEntryType) {
//     for (key, value) in y.iter() {
//         *x.entry(key.clone()).or_insert(0.0) += value;
//     }
// }

impl AttributeNoReactive {
    pub fn new() -> AttributeNoReactive {
        Default::default()
    }
}

impl From<&ComplicatedAttributeGraph> for AttributeNoReactive {
    fn from(graph: &ComplicatedAttributeGraph) -> Self {
        let mut attribute = AttributeNoReactive::new();

        attribute.atk = graph.get_composition_merge(&vec![
            AttributeName::ATKBase,
            AttributeName::ATKPercentage,
            AttributeName::ATKFixed
        ]).0;

        attribute.def = graph.get_composition_merge(&vec![
            AttributeName::DEFBase,
            AttributeName::DEFPercentage,
            AttributeName::DEFFixed
        ]).0;

        attribute.hp = graph.get_composition_merge(&vec![
            AttributeName::HPBase,
            AttributeName::HPPercentage,
            AttributeName::HPFixed
        ]).0;

        attribute.healing_bonus = graph.get_attribute_composition(AttributeName::HealingBonus).0;
        attribute.elemental_mastery = graph.get_composition_merge(&vec![
            AttributeName::ElementalMastery,
            AttributeName::ElementalMasteryExtra
        ]).0;
        attribute.recharge = graph.get_attribute_composition(AttributeName::Recharge).0;
        attribute.shield_strength = graph.get_attribute_composition(AttributeName::ShieldStrength).0;
        attribute.critical = graph.get_attribute_composition(AttributeName::CriticalBase).0;
        attribute.critical_damage = graph.get_attribute_composition(AttributeName::CriticalDamageBase).0;

        attribute.bonus_electro = graph.get_attribute_composition(AttributeName::BonusElectro).0;
        attribute.bonus_pyro = graph.get_attribute_composition(AttributeName::BonusPyro).0;
        attribute.bonus_anemo = graph.get_attribute_composition(AttributeName::BonusAnemo).0;
        attribute.bonus_cryo = graph.get_attribute_composition(AttributeName::BonusCryo).0;
        attribute.bonus_hydro = graph.get_attribute_composition(AttributeName::BonusHydro).0;
        attribute.bonus_geo = graph.get_attribute_composition(AttributeName::BonusGeo).0;
        attribute.bonus_dendro = graph.get_attribute_composition(AttributeName::BonusDendro).0;
        attribute.bonus_physical = graph.get_attribute_composition(AttributeName::BonusPhysical).0;

        // todo other attributes

        attribute
    }
}