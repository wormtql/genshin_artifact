use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{Element, WeaponType};

pub struct XingqiuSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dm51: [f64; 15],
    pub normal_dm52: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg11: [f64; 15],
    pub elemental_skill_dmg12: [f64; 15],
    pub elemental_skill_dmg_reduction: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
}

pub const XINGQIU_SKILL: XingqiuSkillType = XingqiuSkillType {
    normal_dmg1: [0.4661, 0.5041, 0.542, 0.5962, 0.6341, 0.6775, 0.7371, 0.7967, 0.8564, 0.9214, 0.9959, 1.0836, 1.1712, 1.2588, 1.3545],
    normal_dmg2: [0.4764, 0.5152, 0.554, 0.6094, 0.6482, 0.6925, 0.7534, 0.8144, 0.8753, 0.9418, 1.018, 1.1076, 1.1971, 1.2867, 1.3844],
    normal_dmg31: [0.2855, 0.3088, 0.332, 0.3652, 0.3884, 0.415, 0.4515, 0.488, 0.5246, 0.5644, 0.6101, 0.6637, 0.7174, 0.7711, 0.8297],
    normal_dmg32: [0.2855, 0.3088, 0.332, 0.3652, 0.3884, 0.415, 0.4515, 0.488, 0.5246, 0.5644, 0.6101, 0.6637, 0.7174, 0.7711, 0.8297],
    normal_dmg4: [0.5599, 0.6054, 0.651, 0.7161, 0.7617, 0.8138, 0.8854, 0.957, 1.0286, 1.1067, 1.1962, 1.3015, 1.4067, 1.512, 1.6268],
    normal_dm51: [0.3586, 0.3878, 0.417, 0.4587, 0.4879, 0.5213, 0.5671, 0.613, 0.6589, 0.7089, 0.7662, 0.8337, 0.9011, 0.9685, 1.0421],
    normal_dm52: [0.3586, 0.3878, 0.417, 0.4587, 0.4879, 0.5213, 0.5671, 0.613, 0.6589, 0.7089, 0.7662, 0.8337, 0.9011, 0.9685, 1.0421],
    charged_dmg11: [0.473, 0.5115, 0.55, 0.605, 0.6435, 0.6875, 0.748, 0.8085, 0.869, 0.935, 1.0106, 1.0996, 1.1885, 1.2774, 1.3745],
    charged_dmg12: [0.5616, 0.6073, 0.653, 0.7183, 0.764, 0.8163, 0.8881, 0.9599, 1.0317, 1.1101, 1.1999, 1.3055, 1.4111, 1.5167, 1.6318],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg11: [1.68, 1.806, 1.932, 2.1, 2.226, 2.352, 2.52, 2.688, 2.856, 3.024, 3.192, 3.36, 3.57, 3.78, 3.99],
    elemental_skill_dmg12: [1.912, 2.0554, 2.1988, 2.39, 2.5334, 2.6768, 2.868, 3.0592, 3.2504, 3.4416, 3.6328, 3.824, 4.063, 4.302, 4.541],
    elemental_skill_dmg_reduction: [0.2, 0.21, 0.22, 0.23, 0.24, 0.25, 0.26, 0.27, 0.28, 0.29, 0.29, 0.29, 0.29, 0.29, 0.29],
    elemental_burst_dmg1: [0.5427, 0.5834, 0.6241, 0.6784, 0.7191, 0.7598, 0.8141, 0.8684, 0.9226, 0.9769, 1.0312, 1.0854, 1.1533, 1.2211, 1.289],
};

pub const XINGQIU_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Hydro,
    hp: [857, 2202, 2842, 4257, 4712, 5420, 6027, 6735, 7190, 7897, 8352, 9060, 9514, 10222],
    atk: [17, 43, 56, 84, 93, 107, 119, 133, 142, 156, 165, 179, 188, 202],
    def: [64, 163, 211, 316, 349, 402, 447, 499, 533, 585, 619, 671, 705, 758],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Sword,
    star: 4
};