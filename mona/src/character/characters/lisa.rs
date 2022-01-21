use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{Element, WeaponType};

pub struct LisaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_dmg4: [f64; 15],
    pub elemental_skill_dmg5: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
}

pub const LISA_SKILL: LisaSkillType = LisaSkillType {
    normal_dmg1: [0.396, 0.4257, 0.4554, 0.495, 0.5247, 0.5544, 0.594, 0.6336, 0.6732, 0.7128, 0.754, 0.8078, 0.8617, 0.9156, 0.9694],
    normal_dmg2: [0.3592, 0.3861, 0.4131, 0.449, 0.4759, 0.5029, 0.5388, 0.5747, 0.6106, 0.6466, 0.6839, 0.7328, 0.7816, 0.8305, 0.8793],
    normal_dmg3: [0.428, 0.4601, 0.4922, 0.535, 0.5671, 0.5992, 0.642, 0.6848, 0.7276, 0.7704, 0.8149, 0.8731, 0.9313, 0.9895, 1.0477],
    normal_dmg4: [0.5496, 0.5908, 0.632, 0.687, 0.7282, 0.7694, 0.8244, 0.8794, 0.9343, 0.9893, 1.0464, 1.1212, 1.1959, 1.2707, 1.3454],
    charged_dmg1: [1.7712, 1.904, 2.0369, 2.214, 2.3468, 2.4797, 2.6568, 2.8339, 3.011, 3.1882, 3.3724, 3.6132, 3.8541, 4.095, 4.3359],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.8, 0.86, 0.92, 1., 1.06, 1.12, 1.2, 1.28, 1.36, 1.44, 1.52, 1.6, 1.7, 1.8, 1.9],
    elemental_skill_dmg2: [3.2, 3.44, 3.68, 4., 4.24, 4.48, 4.8, 5.12, 5.44, 5.76, 6.08, 6.4, 6.8, 7.2, 7.6],
    elemental_skill_dmg3: [3.68, 3.956, 4.232, 4.6, 4.876, 5.152, 5.52, 5.888, 6.256, 6.624, 6.992, 7.36, 7.82, 8.28, 8.74],
    elemental_skill_dmg4: [4.24, 4.558, 4.876, 5.3, 5.618, 5.936, 6.36, 6.784, 7.208, 7.632, 8.056, 8.48, 9.01, 9.54, 10.07],
    elemental_skill_dmg5: [4.872, 5.2374, 5.6028, 6.09, 6.4554, 6.8208, 7.308, 7.7952, 8.2824, 8.7696, 9.2568, 9.744, 10.353, 10.962, 11.571],
    elemental_burst_dmg1: [0.3656, 0.393, 0.4204, 0.457, 0.4844, 0.5118, 0.5484, 0.585, 0.6215, 0.6581, 0.6946, 0.7312, 0.7769, 0.8226, 0.8683],
};

pub const LISA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Electro,
    hp: [802, 2061, 2661, 3985, 4411, 5074, 5642, 6305, 6731, 7393, 7818, 8481, 8907, 9570],
    atk: [19, 50, 64, 96, 107, 123, 136, 153, 163, 179, 189, 205, 215, 232],
    def: [48, 123, 159, 239, 264, 304, 338, 378, 403, 443, 468, 508, 534, 573],
    sub_stat: CharacterSubStatFamily::ElementalMastery96,
    weapon_type: WeaponType::Catalyst,
    star: 4
};