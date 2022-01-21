use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{Element, StatName, WeaponType};

pub struct NingguangSkillType {
    pub normal_dmg1: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_hp: [f64; 15],
    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
}

pub const NINGGUANG_SKILL: NingguangSkillType = NingguangSkillType {
    normal_dmg1: [0.28, 0.301, 0.322, 0.35, 0.371, 0.392, 0.42, 0.448, 0.476, 0.504, 0.5331, 0.5712, 0.6093, 0.6474, 0.6854],
    charged_dmg1: [1.7408, 1.8714, 2.0019, 2.176, 2.3066, 2.4371, 2.6112, 2.7853, 2.9594, 3.1334, 3.3145, 3.5512, 3.788, 4.0247, 4.2615],
    charged_dmg2: [0.496, 0.5332, 0.5704, 0.62, 0.6572, 0.6944, 0.744, 0.7936, 0.8432, 0.8928, 0.9444, 1.0118, 1.0793, 1.1468, 1.2142],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_hp: [0.501, 0.531, 0.561, 0.6, 0.63, 0.66, 0.699, 0.738, 0.777, 0.816, 0.855, 0.894, 0.933, 0.972, 1.011],
    elemental_skill_dmg1: [2.304, 2.4768, 2.6496, 2.88, 3.0528, 3.2256, 3.456, 3.6864, 3.9168, 4.1472, 4.3776, 4.608, 4.896, 5.184, 5.472],
    elemental_burst_dmg1: [0.8696, 0.9348, 1., 1.087, 1.1522, 1.2174, 1.3044, 1.3914, 1.4783, 1.5653, 1.6522, 1.7392, 1.8479, 1.9566, 2.0653],
};

pub const NINGGUANG_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Geo,
    hp: [821, 2108, 2721, 4076, 4512, 5189, 5770, 6448, 6884, 7561, 7996, 8674, 9110, 9787],
    atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212],
    def: [48, 123, 159, 239, 264, 304, 338, 378, 403, 443, 468, 508, 534, 573],
    sub_stat: CharacterSubStatFamily::Bonus240(StatName::GeoBonus),
    weapon_type: WeaponType::Catalyst,
    star: 4
};