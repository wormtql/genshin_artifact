use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{Element, WeaponType};

pub struct EulaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg51: [f64; 15],
    pub normal_dmg52: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_def_bonus: [f64; 15],
    pub elemental_skill_res_cryo_minus: [f64; 15],
    pub elemental_skill_res_physical_minus: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_dmg3: [f64; 15]
}

pub const EULA_SKILL: EulaSkillType = EulaSkillType {
    normal_dmg1: [0.8973, 0.9704, 1.0434, 1.1477, 1.2208, 1.3043, 1.419, 1.5338, 1.6486, 1.7738, 1.9172, 2.086, 2.2547, 2.4234, 2.6075],
    normal_dmg2: [0.9355, 1.0117, 1.0878, 1.1966, 1.2727, 1.3598, 1.4794, 1.5991, 1.7187, 1.8493, 1.9988, 2.1747, 2.3506, 2.5265, 2.7184],
    normal_dmg31: [0.568, 0.6142, 0.6605, 0.7265, 0.7727, 0.8256, 0.8982, 0.9709, 1.0435, 1.1228, 1.2136, 1.3204, 1.4272, 1.534, 1.6505],
    normal_dmg32: [0.568, 0.6142, 0.6605, 0.7265, 0.7727, 0.8256, 0.8982, 0.9709, 1.0435, 1.1228, 1.2136, 1.3204, 1.4272, 1.534, 1.6505],
    normal_dmg4: [1.1264, 1.2181, 1.3098, 1.4408, 1.5325, 1.6373, 1.7813, 1.9254, 2.0695, 2.2267, 2.4068, 2.6186, 2.8303, 3.0421, 3.2732],
    normal_dmg51: [0.7183, 0.7768, 0.8353, 0.9188, 0.9773, 1.0441, 1.136, 1.2279, 1.3197, 1.42, 1.5348, 1.6699, 1.8049, 1.94, 2.0874],
    normal_dmg52: [0.7183, 0.7768, 0.8353, 0.9188, 0.9773, 1.0441, 1.136, 1.2279, 1.3197, 1.42, 1.5348, 1.6699, 1.8049, 1.94, 2.0874],
    charged_dmg1: [0.688, 0.744, 0.8, 0.88, 0.936, 1., 1.088, 1.176, 1.264, 1.36, 1.47, 1.5994, 1.7287, 1.8581, 1.9992],
    charged_dmg2: [1.244, 1.3452, 1.4465, 1.5912, 1.6924, 1.8081, 1.9672, 2.1264, 2.2855, 2.4591, 2.6579, 2.8918, 3.1257, 3.3596, 3.6148],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5937, 1.7339, 1.8741, 2.0144, 2.1674],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1867, 3.4671, 3.7475, 4.0279, 4.3338],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9803, 4.3306, 4.6808, 5.0311, 5.4132],
    elemental_skill_dmg1: [1.464, 1.5738, 1.6836, 1.83, 1.9398, 2.0496, 2.196, 2.3424, 2.4888, 2.6352, 2.7816, 2.928, 3.111, 3.294, 3.477],
    elemental_skill_dmg2: [2.456, 2.6402, 2.8244, 3.07, 3.2542, 3.4384, 3.684, 3.9296, 4.1752, 4.4208, 4.6664, 4.912, 5.219, 5.526, 5.833],
    elemental_skill_dmg3: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    elemental_skill_def_bonus: [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
    elemental_skill_res_cryo_minus: [0.16, 0.17, 0.18, 0.19, 0.2, 0.21, 0.22, 0.23, 0.24, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25],
    elemental_skill_res_physical_minus: [0.16, 0.17, 0.18, 0.19, 0.2, 0.21, 0.22, 0.23, 0.24, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25],
    elemental_burst_dmg1: [2.456, 2.6402, 2.8244, 3.07, 3.2542, 3.4384, 3.684, 3.9296, 4.1752, 4.4208, 4.6664, 4.912, 5.219, 5.526, 5.833],
    elemental_burst_dmg2: [3.6705, 3.9692, 4.268, 4.6948, 4.9936, 5.335, 5.8045, 6.274, 6.7434, 7.2556, 7.8425, 8.5326, 9.2227, 9.9129, 10.6657],
    elemental_burst_dmg3: [0.7499, 0.811, 0.872, 0.9592, 1.0202, 1.09, 1.1859, 1.2818, 1.3778, 1.4824, 1.6023, 1.7433, 1.8843, 2.0253, 2.1791]
};

pub const EULA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Cryo,
    hp: [1030, 2671, 3554, 5317, 5944, 6839, 7675, 8579, 9207, 10119, 10746, 11699, 12296, 13226],
    atk: [27, 69, 92, 138, 154, 177, 198, 222, 238, 262, 278, 302, 318, 342],
    def: [58, 152, 202, 302, 337, 388, 436, 487, 523, 574, 610, 662, 698, 751],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Claymore,
    star: 5
};