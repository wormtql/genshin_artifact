use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{Element, StatName, WeaponType};

pub struct SucroseSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const SUCROSE_SKILL: SucroseSkillType = SucroseSkillType {
    normal_dmg1: [0.3346, 0.3597, 0.3848, 0.4183, 0.4434, 0.4685, 0.502, 0.5354, 0.5689, 0.6024, 0.6358, 0.6693, 0.7111, 0.7529, 0.7948],
    normal_dmg2: [0.3062, 0.3291, 0.3521, 0.3827, 0.4057, 0.4286, 0.4592, 0.4899, 0.5205, 0.5511, 0.5817, 0.6123, 0.6506, 0.6889, 0.7271],
    normal_dmg3: [0.3845, 0.4133, 0.4422, 0.4806, 0.5094, 0.5383, 0.5767, 0.6152, 0.6536, 0.6921, 0.7305, 0.769, 0.817, 0.8651, 0.9131],
    normal_dmg4: [0.4792, 0.5151, 0.5511, 0.599, 0.6349, 0.6708, 0.7188, 0.7667, 0.8146, 0.8625, 0.9104, 0.9584, 1.0182, 1.0781, 1.138],
    charged_dmg1: [1.2016, 1.2917, 1.3818, 1.502, 1.5921, 1.6822, 1.8024, 1.9226, 2.0427, 2.1629, 2.283, 2.4032, 2.5534, 2.7036, 2.8538],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [2.112, 2.2704, 2.4288, 2.64, 2.7984, 2.9568, 3.168, 3.3792, 3.5904, 3.8016, 4.0128, 4.224, 4.488, 4.752, 5.016],
    elemental_burst_dmg1: [1.48, 1.591, 1.702, 1.85, 1.961, 2.072, 2.22, 2.368, 2.516, 2.664, 2.812, 2.96, 3.145, 3.33, 3.515],
    elemental_burst_dmg2: [0.44, 0.473, 0.506, 0.55, 0.583, 0.616, 0.66, 0.704, 0.748, 0.792, 0.836, 0.88, 0.935, 0.99, 1.045]
};

pub const SUCROSE_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Anemo,
    hp: [775, 1991, 2570, 3850, 4261, 4901, 5450, 6090, 6501, 7141, 7552, 8192, 8604, 9244],
    atk: [14, 37, 47, 71, 78, 90, 100, 112, 120, 131, 139, 151, 158, 170],
    def: [59, 151, 195, 293, 324, 373, 414, 463, 494, 543, 574, 623, 654, 703],
    sub_stat: CharacterSubStatFamily::Bonus240(StatName::AnemoBonus),
    weapon_type: WeaponType::Catalyst,
    star: 4
};