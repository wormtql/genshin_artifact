use crate::attribute::{AttributeGraph, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use super::super::character::Character;
use super::super::character_name::CharacterName;
use crate::common::{ChangeAttribute, Element, WeaponType};
use crate::character::CharacterStaticData;
use crate::character::character_sub_stat::CharacterSubStatFamily;

pub struct MonaSkillType {
    normal_dmg1: [f64; 15],
    normal_dmg2: [f64; 15],
    normal_dmg3: [f64; 15],
    normal_dmg4: [f64; 15],
    charged_dmg1: [f64; 15],
    plunging_dmg1: [f64; 15],
    plunging_dmg2: [f64; 15],
    plunging_dmg3: [f64; 15],

    elemental_skill_dmg1: [f64; 15],
    elemental_skill_dmg2: [f64; 15],

    elemental_burst_dmg1: [f64; 15],
    elemental_burst_bonus: [f64; 15],
}

pub const MONA_SKILL: MonaSkillType = MonaSkillType {
    normal_dmg1: [0.376, 0.4042, 0.4324, 0.47, 0.4982, 0.5264, 0.564, 0.6016, 0.6392, 0.6768, 0.7144, 0.752, 0.799, 0.846, 0.893],
    normal_dmg2: [0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765, 0.81, 0.855],
    normal_dmg3: [0.448, 0.4816, 0.5152, 0.56, 0.5936, 0.6272, 0.672, 0.7168, 0.7616, 0.8064, 0.8512, 0.896, 0.952, 1.008, 1.064],
    normal_dmg4: [0.5616, 0.6037, 0.6458, 0.702, 0.7441, 0.7862, 0.8424, 0.8986, 0.9547, 1.0109, 1.067, 1.1232, 1.1934, 1.2636, 1.3338],
    charged_dmg1: [1.4972, 1.6095, 1.7218, 1.8715, 1.9838, 2.0961, 2.2458, 2.3955, 2.5452, 2.695, 2.8507, 3.0543, 3.2579, 3.4615, 3.6651],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.32, 0.344, 0.368, 0.4, 0.424, 0.448, 0.48, 0.512, 0.544, 0.576, 0.608, 0.64, 0.68, 0.72, 0.76],
    elemental_skill_dmg2: [1.328, 1.4276, 1.5272, 1.66, 1.7596, 1.8592, 1.992, 2.1248, 2.2576, 2.3904, 2.5232, 2.656, 2.822, 2.988, 3.154],
    elemental_burst_dmg1: [4.424, 4.7558, 5.0876, 5.53, 5.8618, 6.1936, 6.636, 7.0784, 7.5208, 7.9632, 8.4056, 8.848, 9.401, 9.954, 10.507],
    elemental_burst_bonus: [0.42, 0.44, 0.46, 0.48, 0.5, 0.52, 0.54, 0.56, 0.58, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6],
};

pub const MONA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Hydro,
    hp: [810, 2102, 2797, 4185, 4678, 5383, 6041, 6752, 7246, 7964, 8458, 9184, 9677, 10409],
    atk: [22, 58, 77, 115, 129, 148, 167, 186, 200, 220, 233, 253, 267, 287],
    def: [51, 132, 176, 263, 294, 338, 379, 424, 455, 500, 531, 576, 607, 653],
    sub_stat: CharacterSubStatFamily::Recharge320,
    weapon_type: WeaponType::Catalyst,
    star: 5,
};

pub struct MonaEffect {
    has_talent2: bool
}

impl MonaEffect {
    pub fn new(common_data: &CharacterCommonData) -> MonaEffect {
        MonaEffect {
            has_talent2: common_data.has_talent2,
        }
    }
}

impl ChangeAttribute for MonaEffect {
    fn change_attribute(&self, attribute: &mut AttributeGraph) {
        if self.has_talent2 {
            attribute.add_edge(
                AttributeName::Recharge,
                AttributeName::BonusHydro,
                Box::new(|n| (String::from("莫娜天赋2"), n.value() * 0.2))
            );
        }
    }
}
