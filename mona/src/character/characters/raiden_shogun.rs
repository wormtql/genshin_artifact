use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{ChangeAttribute, Element, WeaponType};

pub struct RaidenShogunSkill {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg41: [f64; 15],
    pub normal_dmg42: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_q_bonus: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_bonus1: [f64; 15],
    pub elemental_burst_bonus2: [f64; 15],
    pub elemental_burst_normal_dmg1: [f64; 15],
    pub elemental_burst_normal_dmg2: [f64; 15],
    pub elemental_burst_normal_dmg3: [f64; 15],
    pub elemental_burst_normal_dmg41: [f64; 15],
    pub elemental_burst_normal_dmg42: [f64; 15],
    pub elemental_burst_normal_dmg5: [f64; 15],
    pub elemental_burst_charged_dmg11: [f64; 15],
    pub elemental_burst_charged_dmg12: [f64; 15],
    pub elemental_burst_plunging_dmg1: [f64; 15],
    pub elemental_burst_plunging_dmg2: [f64; 15],
    pub elemental_burst_plunging_dmg3: [f64; 15],
}

pub const RAIDEN_SHOGUN_SKILL: RaidenShogunSkill = RaidenShogunSkill {
    normal_dmg1: [0.3965, 0.4287, 0.461, 0.5071, 0.5394, 0.5763, 0.627, 0.6777, 0.7284, 0.7837, 0.8471, 0.9216, 0.9962, 1.0707, 1.152],
    normal_dmg2: [0.3973, 0.4297, 0.462, 0.5082, 0.5405, 0.5775, 0.6283, 0.6791, 0.73, 0.7854, 0.8489, 0.9236, 0.9983, 1.073, 1.1545],
    normal_dmg3: [0.4988, 0.5394, 0.58, 0.638, 0.6786, 0.725, 0.7888, 0.8526, 0.9164, 0.986, 1.0658, 1.1595, 1.2533, 1.3471, 1.4494],
    normal_dmg41: [0.2898, 0.3134, 0.337, 0.3707, 0.3943, 0.4213, 0.4583, 0.4954, 0.5325, 0.5729, 0.6192, 0.6737, 0.7282, 0.7827, 0.8422],
    normal_dmg42: [0.2898, 0.3134, 0.337, 0.3707, 0.3943, 0.4213, 0.4583, 0.4954, 0.5325, 0.5729, 0.6192, 0.6737, 0.7282, 0.7827, 0.8422],
    normal_dmg5: [0.6545, 0.7077, 0.761, 0.8371, 0.8904, 0.9513, 1.035, 1.1187, 1.2024, 1.2937, 1.3983, 1.5214, 1.6444, 1.7675, 1.9017],
    charged_dmg1: [0.9959, 1.0769, 1.158, 1.2738, 1.3549, 1.4475, 1.5749, 1.7023, 1.8296, 1.9686, 2.1278, 2.3151, 2.5023, 2.6896, 2.8938],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.172, 1.2599, 1.3478, 1.465, 1.5529, 1.6408, 1.758, 1.8752, 1.9924, 2.1096, 2.2268, 2.344, 2.4905, 2.637, 2.7835],
    elemental_skill_dmg2: [0.42, 0.4515, 0.483, 0.525, 0.5565, 0.588, 0.63, 0.672, 0.714, 0.756, 0.798, 0.84, 0.8925, 0.945, 0.9975],
    elemental_skill_q_bonus: [0.0022, 0.0023, 0.0024, 0.0025, 0.0026, 0.0027, 0.0028, 0.0029, 0.003, 0.003, 0.003, 0.003, 0.003, 0.003, 0.003],
    elemental_burst_dmg1: [4.008, 4.3086, 4.6092, 5.01, 5.3106, 5.6112, 6.012, 6.4128, 6.8136, 7.2144, 7.6152, 8.016, 8.517, 9.018, 9.519],
    elemental_burst_bonus1: [0.0389, 0.0418, 0.0447, 0.0486, 0.0515, 0.0544, 0.0583, 0.0622, 0.0661, 0.07, 0.0739, 0.0778, 0.0826, 0.0875, 0.0923],
    elemental_burst_bonus2: [0.0073, 0.0078, 0.0084, 0.0091, 0.0096, 0.0102, 0.0109, 0.0116, 0.0123, 0.0131, 0.0138, 0.0145, 0.0154, 0.0163, 0.0172],
    elemental_burst_normal_dmg1: [0.4474, 0.4779, 0.5084, 0.5491, 0.5796, 0.6151, 0.6609, 0.7066, 0.7524, 0.7982, 0.8439, 0.8897, 0.9354, 0.9812, 1.0269],
    elemental_burst_normal_dmg2: [0.4396, 0.4695, 0.4995, 0.5395, 0.5694, 0.6044, 0.6494, 0.6943, 0.7393, 0.7842, 0.8292, 0.8741, 0.9191, 0.964, 1.009],
    elemental_burst_normal_dmg3: [0.5382, 0.5749, 0.6116, 0.6605, 0.6972, 0.74, 0.7951, 0.8501, 0.9052, 0.9602, 1.0153, 1.0703, 1.1254, 1.1804, 1.2355],
    elemental_burst_normal_dmg41: [0.3089, 0.3299, 0.351, 0.3791, 0.4001, 0.4247, 0.4563, 0.4879, 0.5195, 0.5511, 0.5827, 0.6143, 0.6458, 0.6774, 0.709],
    elemental_burst_normal_dmg42: [0.3098, 0.3309, 0.352, 0.3802, 0.4013, 0.4259, 0.4576, 0.4893, 0.521, 0.5526, 0.5843, 0.616, 0.6477, 0.6794, 0.711],
    elemental_burst_normal_dmg5: [0.7394, 0.7899, 0.8403, 0.9075, 0.9579, 1.0167, 1.0924, 1.168, 1.2436, 1.3192, 1.3948, 1.4705, 1.5461, 1.6217, 1.6973],
    elemental_burst_charged_dmg11: [0.616, 0.658, 0.7, 0.756, 0.798, 0.847, 0.91, 0.973, 1.036, 1.099, 1.162, 1.225, 1.288, 1.351, 1.414],
    elemental_burst_charged_dmg12: [0.7436, 0.7943, 0.845, 0.9126, 0.9633, 1.0225, 1.0985, 1.1746, 1.2506, 1.3267, 1.4027, 1.4788, 1.5548, 1.6309, 1.7069],
    elemental_burst_plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    elemental_burst_plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    elemental_burst_plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
};

pub const RAIDEN_SHOGUN_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Electro,
    hp: [1005, 2606, 3468, 5189, 5801, 6675, 7491, 8373, 8985, 9875, 10487, 11388, 12000, 12907],
    atk: [26, 68, 91, 136, 152, 174, 196, 219, 235, 258, 274, 298, 314, 337],
    def: [61, 159, 212, 317, 355, 408, 458, 512, 549, 604, 641, 696, 734, 789],
    sub_stat: CharacterSubStatFamily::Recharge320,
    weapon_type: WeaponType::Polearm,
    star: 5
};

pub struct RaidenShogunEffect {
    pub has_talent2: bool,
}

impl RaidenShogunEffect {
    pub fn new(common_data: &CharacterCommonData) -> RaidenShogunEffect {
        RaidenShogunEffect {
            has_talent2: common_data.has_talent2
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for RaidenShogunEffect {
    fn change_attribute(&self, attribute: &mut T) {
        if self.has_talent2 {
            attribute.add_edge1(
                AttributeName::Recharge,
                AttributeName::BonusElectro,
                Box::new(|recharge, _| (recharge - 1.0) * 0.4),
                Box::new(|grad, _x1, _x2| (grad * 0.4, 0.0)),
                "雷电将军天赋：殊胜之御体"
            );
        }
    }
}