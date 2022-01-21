use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{Element, StatName, WeaponType};

pub struct RazorSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_a_speedup: [f64; 15],
}

pub const RAZOR_SKILL: RazorSkillType = RazorSkillType {
    normal_dmg1: [0.9592, 1.0246, 1.09, 1.1772, 1.2426, 1.3189, 1.417, 1.5151, 1.6132, 1.7113, 1.8094, 1.9075, 2.0056, 2.1037, 2.2018],
    normal_dmg2: [0.8263, 0.8827, 0.939, 1.0141, 1.0705, 1.1362, 1.2207, 1.3052, 1.3897, 1.4742, 1.5587, 1.6433, 1.7278, 1.8123, 1.8968],
    normal_dmg3: [1.0331, 1.1036, 1.174, 1.2679, 1.3384, 1.4205, 1.5262, 1.6319, 1.7375, 1.8432, 1.9488, 2.0545, 2.1602, 2.2658, 2.3715],
    normal_dmg4: [1.3605, 1.4532, 1.546, 1.6697, 1.7624, 1.8707, 2.0098, 2.1489, 2.2881, 2.4272, 2.5664, 2.7055, 2.8446, 2.9838, 3.1229],
    charged_dmg1: [0.6254, 0.6763, 0.7272, 0.7999, 0.8508, 0.909, 0.989, 1.069, 1.149, 1.2362, 1.3235, 1.4108, 1.498, 1.5853, 1.6726],
    charged_dmg2: [1.1309, 1.223, 1.315, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933, 2.5511, 2.7089, 2.8667, 3.0245],
    plunging_dmg1: [0.8205, 0.8872, 0.954, 1.0494, 1.1162, 1.1925, 1.2975, 1.4024, 1.5074, 1.6219, 1.7363, 1.8508, 1.9653, 2.0798, 2.1943],
    plunging_dmg2: [1.6406, 1.7741, 1.9077, 2.0984, 2.232, 2.3846, 2.5944, 2.8043, 3.0141, 3.243, 3.4719, 3.7009, 3.9298, 4.1587, 4.3876],
    plunging_dmg3: [2.0492, 2.216, 2.3828, 2.621, 2.7878, 2.9785, 3.2406, 3.5027, 3.7648, 4.0507, 4.3366, 4.6226, 4.9085, 5.1944, 5.4804],
    elemental_skill_dmg1: [1.992, 2.1414, 2.2908, 2.49, 2.6394, 2.7888, 2.988, 3.1872, 3.3864, 3.5856, 3.7848, 3.984, 4.233, 4.482, 4.731],
    elemental_skill_dmg2: [2.952, 3.1734, 3.3948, 3.69, 3.9114, 4.1328, 4.428, 4.7232, 5.0184, 5.3136, 5.6088, 5.904, 6.273, 6.642, 7.011],
    elemental_burst_dmg1: [1.6, 1.72, 1.84, 2., 2.12, 2.24, 2.4, 2.56, 2.72, 2.88, 3.04, 3.2, 3.4, 3.6, 3.8],
    elemental_burst_dmg2: [0.24, 0.258, 0.276, 0.3, 0.318, 0.336, 0.36, 0.384, 0.408, 0.432, 0.456, 0.48, 0.51, 0.54, 0.57],
    elemental_burst_a_speedup: [0.26, 0.28, 0.3, 0.32, 0.34, 0.36, 0.37, 0.38, 0.39, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4]
};

pub const RAZOR_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Electro,
    hp: [1003, 2577, 3326, 4982, 5514, 6343, 7052, 7881, 8413, 9241, 9773, 10602, 11134, 11962],
    atk: [20, 50, 65, 97, 108, 124, 138, 154, 164, 180, 191, 207, 217, 234],
    def: [63, 162, 209, 313, 346, 398, 443, 495, 528, 580, 613, 665, 699, 751],
    sub_stat: CharacterSubStatFamily::Bonus300(StatName::PhysicalBonus),
    weapon_type: WeaponType::Claymore,
    star: 4
};