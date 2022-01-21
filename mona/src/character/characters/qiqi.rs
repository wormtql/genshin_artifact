use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{Element, WeaponType};

pub struct QiqiSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg41: [f64; 15],
    pub normal_dmg42: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_heal1: [f64; 15],
    pub elemental_skill_heal1_fixed: [f64; 15],
    pub elemental_skill_heal2: [f64; 15],
    pub elemental_skill_heal2_fixed: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_heal1: [f64; 15],
    pub elemental_burst_heal1_fixed: [f64; 15],
}

pub const QIQI_SKILL: QiqiSkillType = QiqiSkillType {
    normal_dmg1: [0.3775, 0.4083, 0.439, 0.4829, 0.5136, 0.5488, 0.597, 0.6453, 0.6936, 0.7463, 0.799, 0.8517, 0.9043, 0.957, 1.0097],
    normal_dmg2: [0.3887, 0.4204, 0.452, 0.4972, 0.5288, 0.565, 0.6147, 0.6644, 0.7142, 0.7684, 0.8226, 0.8769, 0.9311, 0.9854, 1.0396],
    normal_dmg31: [0.2417, 0.2613, 0.281, 0.3091, 0.3288, 0.3513, 0.3822, 0.4131, 0.444, 0.4777, 0.5114, 0.5451, 0.5789, 0.6126, 0.6463],
    normal_dmg32: [0.2417, 0.2613, 0.281, 0.3091, 0.3288, 0.3513, 0.3822, 0.4131, 0.444, 0.4777, 0.5114, 0.5451, 0.5789, 0.6126, 0.6463],
    normal_dmg41: [0.2468, 0.2669, 0.287, 0.3157, 0.3358, 0.3588, 0.3903, 0.4219, 0.4535, 0.4879, 0.5223, 0.5568, 0.5912, 0.6257, 0.6601],
    normal_dmg42: [0.2468, 0.2669, 0.287, 0.3157, 0.3358, 0.3588, 0.3903, 0.4219, 0.4535, 0.4879, 0.5223, 0.5568, 0.5912, 0.6257, 0.6601],
    normal_dmg5: [0.6304, 0.6817, 0.733, 0.8063, 0.8576, 0.9163, 0.9969, 1.0775, 1.1581, 1.2461, 1.3341, 1.422, 1.51, 1.5979, 1.6859],
    charged_dmg11: [0.6433, 0.6956, 0.748, 0.8228, 0.8752, 0.935, 1.0173, 1.0996, 1.1818, 1.2716, 1.3614, 1.4511, 1.5409, 1.6306, 1.7204],
    charged_dmg12: [0.6433, 0.6956, 0.748, 0.8228, 0.8752, 0.935, 1.0173, 1.0996, 1.1818, 1.2716, 1.3614, 1.4511, 1.5409, 1.6306, 1.7204],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    elemental_skill_heal1: [0.1056, 0.1135, 0.1214, 0.132, 0.1399, 0.1478, 0.1584, 0.169, 0.1795, 0.1901, 0.2006, 0.2112, 0.2244, 0.2376, 0.2508],
    elemental_skill_heal1_fixed: [67.0, 74.0, 81.0, 89.0, 98.0, 107.0, 116.0, 126.0, 137.0, 148.0, 160.0, 172.0, 185.0, 199.0, 213.0],
    elemental_skill_heal2: [0.696, 0.7482, 0.8004, 0.87, 0.9222, 0.9744, 1.044, 1.1136, 1.1832, 1.2528, 1.3224, 1.392, 1.479, 1.566, 1.653],
    elemental_skill_heal2_fixed: [451.0, 496.0, 544.0, 597.0, 653.0, 713.0, 777.0, 845.0, 916.0, 991.0, 1070.0, 1153.0, 1239.0, 1329.0, 1423.0],
    elemental_skill_dmg2: [0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765, 0.81, 0.855],
    elemental_burst_dmg1: [2.848, 3.0616, 3.2752, 3.56, 3.7736, 3.9872, 4.272, 4.5568, 4.8416, 5.1264, 5.4112, 5.696, 6.052, 6.408, 6.764],
    elemental_burst_heal1: [0.9, 0.9675, 1.035, 1.125, 1.1925, 1.26, 1.35, 1.44, 1.53, 1.62, 1.71, 1.8, 1.9125, 2.025, 2.1375],
    elemental_burst_heal1_fixed: [577.0, 635.0, 698.0, 765.0, 837.0, 914.0, 996.0, 1083.0, 1174.0, 1270.0, 1371.0, 1477.0, 1588.0, 1703.0, 1824.0],
};

pub const QIQI_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Cryo,
    hp: [963, 2498, 3323, 4973, 5559, 6396, 7178, 8023, 8610, 9463, 10050, 10912, 11499, 12368],
    atk: [22, 58, 77, 115, 129, 148, 167, 186, 200, 220, 233, 253, 267, 287],
    def: [72, 186, 248, 371, 415, 477, 535, 598, 642, 706, 749, 814, 857, 922],
    sub_stat: CharacterSubStatFamily::HealingBonus222,
    weapon_type: WeaponType::Sword,
    star: 5
};