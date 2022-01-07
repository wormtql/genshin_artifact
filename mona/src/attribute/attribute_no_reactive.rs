use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::attribute::{AttributeGraph, AttributeName};


type AttributeEntryType = HashMap<String, f64>;

#[derive(Serialize, Deserialize)]
pub struct AttributeNoReactive {
    pub atk: AttributeEntryType,
    pub def: AttributeEntryType,
    pub hp: AttributeEntryType,

    pub healing_bonus: AttributeEntryType,
    pub elemental_mastery: AttributeEntryType,
    pub recharge: AttributeEntryType,

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

impl AttributeNoReactive {
    pub fn new() -> AttributeNoReactive {
        AttributeNoReactive {
            atk: Default::default(),
            def: Default::default(),
            hp: Default::default(),
            healing_bonus: Default::default(),
            elemental_mastery: Default::default(),
            recharge: Default::default(),
            critical: Default::default(),
            critical_damage: Default::default(),
            bonus_electro: Default::default(),
            bonus_pyro: Default::default(),
            bonus_anemo: Default::default(),
            bonus_cryo: Default::default(),
            bonus_geo: Default::default(),
            bonus_hydro: Default::default(),
            bonus_dendro: Default::default(),
            bonus_physical: Default::default()
        }
    }
}

impl From<&AttributeGraph> for AttributeNoReactive {
    fn from(graph: &AttributeGraph) -> Self {
        let mut attribute = AttributeNoReactive::new();

        attribute.atk.extend(graph.get_attribute_composition(AttributeName::ATKBase));
        attribute.atk.extend(graph.get_attribute_composition(AttributeName::ATKFixed));
        attribute.atk.extend(graph.get_attribute_composition(AttributeName::ATKPercentage));

        attribute.def.extend(graph.get_attribute_composition(AttributeName::DEFBase));
        attribute.def.extend(graph.get_attribute_composition(AttributeName::DEFFixed));
        attribute.def.extend(graph.get_attribute_composition(AttributeName::DEFPercentage));

        attribute.hp.extend(graph.get_attribute_composition(AttributeName::HPBase));
        attribute.hp.extend(graph.get_attribute_composition(AttributeName::HPFixed));
        attribute.hp.extend(graph.get_attribute_composition(AttributeName::HPPercentage));

        attribute.healing_bonus.extend(graph.get_attribute_composition(AttributeName::HealingBonus));
        attribute.elemental_mastery.extend(graph.get_attribute_composition(AttributeName::ElementalMastery));
        attribute.recharge.extend(graph.get_attribute_composition(AttributeName::Recharge));

        // todo other attributes

        attribute
    }
}