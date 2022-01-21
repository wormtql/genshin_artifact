use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{Element, WeaponType};

pub struct ThomaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_shield1: [f64; 15],
    pub elemental_skill_shield1_fixed: [f64; 15],
    pub elemental_skill_shield2: [f64; 15],
    pub elemental_skill_shield2_fixed: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_shield1: [f64; 15],
    pub elemental_burst_shield1_fixed: [f64; 15],
}

pub const THOMA_SKILL: ThomaSkillType = ThomaSkillType {
    normal_dmg1: [0.4439, 0.4801, 0.5162, 0.5678, 0.604, 0.6453, 0.702, 0.7588, 0.8156, 0.8775, 0.9395, 1.0014, 1.0634, 1.1253, 1.1873],
    normal_dmg2: [0.4363, 0.4718, 0.5073, 0.558, 0.5935, 0.6341, 0.6899, 0.7457, 0.8015, 0.8624, 0.9233, 0.9842, 1.045, 1.1059, 1.1668],
    normal_dmg3: [0.2679, 0.2897, 0.3115, 0.3427, 0.3645, 0.3894, 0.4236, 0.4579, 0.4922, 0.5296, 0.5669, 0.6043, 0.6417, 0.6791, 0.7165],
    normal_dmg4: [0.6736, 0.7284, 0.7832, 0.8615, 0.9163, 0.979, 1.0652, 1.1513, 1.2375, 1.3314, 1.4254, 1.5194, 1.6134, 1.7074, 1.8014],
    charged_dmg1: [1.1275, 1.2192, 1.311, 1.4421, 1.5339, 1.6388, 1.783, 1.9272, 2.0714, 2.2287, 2.386, 2.5433, 2.7007, 2.858, 3.0153],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.464, 1.5738, 1.6836, 1.83, 1.9398, 2.0496, 2.196, 2.3424, 2.4888, 2.6352, 2.7816, 2.928, 3.111, 3.294, 3.477],
    elemental_skill_shield1: [0.072, 0.0774, 0.0828, 0.09, 0.0954, 0.1008, 0.108, 0.1152, 0.1224, 0.1296, 0.1368, 0.144, 0.153, 0.162, 0.171],
    elemental_skill_shield1_fixed: [693.0, 763.0, 838.0, 919.0, 1005.0, 1098.0, 1196.0, 1300.0, 1410.0, 1525.0, 1647.0, 1774.0, 1907.0, 2046.0, 2190.0],
    elemental_skill_shield2: [0.196, 0.2107, 0.2254, 0.245, 0.2597, 0.2744, 0.294, 0.3136, 0.3332, 0.3528, 0.3724, 0.392, 0.4165, 0.441, 0.4655],
    elemental_skill_shield2_fixed: [1887.0, 2076.0, 2281.0, 2501.0, 2737.0, 2989.0, 3256.0, 3539.0, 3838.0, 4153.0, 4483.0, 4829.0, 5191.0, 5568.0, 5962.0],
    elemental_burst_dmg1: [0.88, 0.946, 1.012, 1.1, 1.166, 1.232, 1.32, 1.408, 1.496, 1.584, 1.672, 1.76, 1.87, 1.98, 2.09],
    elemental_burst_dmg2: [0.58, 0.6235, 0.667, 0.725, 0.7685, 0.812, 0.87, 0.928, 0.986, 1.044, 1.102, 1.16, 1.2325, 1.305, 1.3775],
    elemental_burst_shield1: [0.0114, 0.0123, 0.0132, 0.0143, 0.0152, 0.016, 0.0172, 0.0183, 0.0194, 0.0206, 0.0217, 0.0229, 0.0243, 0.0257, 0.0272],
    elemental_burst_shield1_fixed: [110.0, 121.0, 133.0, 146.0, 160.0, 174.0, 190.0, 206.0, 224.0, 242.0, 261.0, 282.0, 303.0, 325.0, 348.0],
};

pub const THOMA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Pyro,
    hp: [866, 2225, 2872, 4302, 4762, 5478, 6091, 6806, 7266, 7981, 8440, 9156, 9616, 10331],
    atk: [17, 43, 56, 84, 93, 107, 119, 133, 142, 156, 165, 179, 188, 202],
    def: [63, 162, 209, 313, 346, 398, 443, 495, 528, 580, 613, 665, 699, 751],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Polearm,
    star: 4
};