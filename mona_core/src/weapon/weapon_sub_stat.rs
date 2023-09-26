use crate::common::{StatName, ChangeAttribute};
use crate::attribute::Attribute;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum WeaponSubStatFamily {
    ATK30,
    ATK36,
    ATK51,
    ATK60,
    ATK72,
    ATK77,
    ATK90,
    ATK108,
    ATK120,

    CriticalDamage68,
    CriticalDamage80,
    CriticalDamage96,
    CriticalDamage102,
    CriticalDamage120,
    CriticalDamage144,
    CriticalDamage192,

    CriticalRate34,
    CriticalRate40,
    CriticalRate48,
    CriticalRate51,
    CriticalRate60,
    CriticalRate68,
    CriticalRate72,
    CriticalRate80,
    CriticalRate96,

    DEF64,
    DEF96,
    DEF113,
    DEF150,

    EM12,
    EM20,
    EM24,
    EM31,
    EM36,
    EM41,
    EM43,
    EM48,
    EM58,

    HP60,
    HP77,
    HP90,
    HP102,
    HP108,
    HP120,
    HP144,

    PhysicalBonus45,
    PhysicalBonus75,
    PhysicalBonus90,
    PhysicalBonus96,
    PhysicalBonus113,
    PhysicalBonus150,

    Recharge67,
    Recharge80,
    Recharge85,
    Recharge100,
    Recharge113,
    Recharge120,
    Recharge133,
}

pub fn get_stat_name_from_family(family: WeaponSubStatFamily) -> StatName {
    match family {
        WeaponSubStatFamily::ATK30 => StatName::ATKPercentage,
        WeaponSubStatFamily::ATK36 => StatName::ATKPercentage,
        WeaponSubStatFamily::ATK51 => StatName::ATKPercentage,
        WeaponSubStatFamily::ATK60 => StatName::ATKPercentage,
        WeaponSubStatFamily::ATK72 => StatName::ATKPercentage,
        WeaponSubStatFamily::ATK77 => StatName::ATKPercentage,
        WeaponSubStatFamily::ATK90 => StatName::ATKPercentage,
        WeaponSubStatFamily::ATK108 => StatName::ATKPercentage,
        WeaponSubStatFamily::ATK120 => StatName::ATKPercentage,

        WeaponSubStatFamily::CriticalDamage68 => StatName::CriticalDamage,
        WeaponSubStatFamily::CriticalDamage80 => StatName::CriticalDamage,
        WeaponSubStatFamily::CriticalDamage96 => StatName::CriticalDamage,
        WeaponSubStatFamily::CriticalDamage102 => StatName::CriticalDamage,
        WeaponSubStatFamily::CriticalDamage120 => StatName::CriticalDamage,
        WeaponSubStatFamily::CriticalDamage144 => StatName::CriticalDamage,
        WeaponSubStatFamily::CriticalDamage192 => StatName::CriticalDamage,

        WeaponSubStatFamily::CriticalRate34 => StatName::CriticalRate,
        WeaponSubStatFamily::CriticalRate40 => StatName::CriticalRate,
        WeaponSubStatFamily::CriticalRate48 => StatName::CriticalRate,
        WeaponSubStatFamily::CriticalRate51 => StatName::CriticalRate,
        WeaponSubStatFamily::CriticalRate60 => StatName::CriticalRate,
        WeaponSubStatFamily::CriticalRate68 => StatName::CriticalRate,
        WeaponSubStatFamily::CriticalRate72 => StatName::CriticalRate,
        WeaponSubStatFamily::CriticalRate80 => StatName::CriticalRate,
        WeaponSubStatFamily::CriticalRate96 => StatName::CriticalRate,

        WeaponSubStatFamily::DEF64 => StatName::DEFPercentage,
        WeaponSubStatFamily::DEF96 => StatName::DEFPercentage,
        WeaponSubStatFamily::DEF113 => StatName::DEFPercentage,
        WeaponSubStatFamily::DEF150 => StatName::DEFPercentage,

        WeaponSubStatFamily::EM12 => StatName::ElementalMastery,
        WeaponSubStatFamily::EM20 => StatName::ElementalMastery,
        WeaponSubStatFamily::EM24 => StatName::ElementalMastery,
        WeaponSubStatFamily::EM31 => StatName::ElementalMastery,
        WeaponSubStatFamily::EM36 => StatName::ElementalMastery,
        WeaponSubStatFamily::EM41 => StatName::ElementalMastery,
        WeaponSubStatFamily::EM43 => StatName::ElementalMastery,
        WeaponSubStatFamily::EM48 => StatName::ElementalMastery,
        WeaponSubStatFamily::EM58 => StatName::ElementalMastery,

        WeaponSubStatFamily::HP60 => StatName::HPPercentage,
        WeaponSubStatFamily::HP77 => StatName::HPPercentage,
        WeaponSubStatFamily::HP90 => StatName::HPPercentage,
        WeaponSubStatFamily::HP102 => StatName::HPPercentage,
        WeaponSubStatFamily::HP108 => StatName::HPPercentage,
        WeaponSubStatFamily::HP120 => StatName::HPPercentage,
        WeaponSubStatFamily::HP144 => StatName::HPPercentage,

        WeaponSubStatFamily::PhysicalBonus45 => StatName::PhysicalBonus,
        WeaponSubStatFamily::PhysicalBonus75 => StatName::PhysicalBonus,
        WeaponSubStatFamily::PhysicalBonus90 => StatName::PhysicalBonus,
        WeaponSubStatFamily::PhysicalBonus96 => StatName::PhysicalBonus,
        WeaponSubStatFamily::PhysicalBonus113 => StatName::PhysicalBonus,
        WeaponSubStatFamily::PhysicalBonus150 => StatName::PhysicalBonus,

        WeaponSubStatFamily::Recharge67 => StatName::Recharge,
        WeaponSubStatFamily::Recharge80 => StatName::Recharge,
        WeaponSubStatFamily::Recharge85 => StatName::Recharge,
        WeaponSubStatFamily::Recharge100 => StatName::Recharge,
        WeaponSubStatFamily::Recharge113 => StatName::Recharge,
        WeaponSubStatFamily::Recharge120 => StatName::Recharge,
        WeaponSubStatFamily::Recharge133 => StatName::Recharge,
    }
}

pub fn get_value_array(family: WeaponSubStatFamily) -> [f64; 8] {
    match family {
        WeaponSubStatFamily::ATK30 => [0.03, 0.053, 0.077, 0.089, 0.101, 0.114, 0.126, 0.138],
        WeaponSubStatFamily::ATK36 => [0.036, 0.064, 0.093, 0.107, 0.122, 0.136, 0.151, 0.165],
        WeaponSubStatFamily::ATK51 => [0.051, 0.09, 0.131, 0.152, 0.173, 0.193, 0.201, 0.221],
        WeaponSubStatFamily::ATK60 => [0.06, 0.106, 0.155, 0.179, 0.203, 0.227, 0.251, 0.276],
        WeaponSubStatFamily::ATK72 => [0.072, 0.127, 0.185, 0.214, 0.244, 0.273, 0.302, 0.331],
        WeaponSubStatFamily::ATK77 => [0.077, 0.135, 0.197, 0.228, 0.259, 0.29, 0.321, 0.352],
        WeaponSubStatFamily::ATK90 => [0.09, 0.159, 0.232, 0.268, 0.304, 0.341, 0.377, 0.413],
        WeaponSubStatFamily::ATK108 => [0.108, 0.191, 0.278, 0.322, 0.365, 0.409, 0.453, 0.496],
        WeaponSubStatFamily::ATK120 => [0.12, 0.212, 0.309, 0.357, 0.406, 0.454, 0.503, 0.551],

        WeaponSubStatFamily::CriticalDamage68 => [0.068, 0.12, 0.175, 0.203, 0.23, 0.257, 0.285, 0.312],
        WeaponSubStatFamily::CriticalDamage80 => [0.08, 0.141, 0.206, 0.238, 0.271, 0.303, 0.335, 0.368],
        WeaponSubStatFamily::CriticalDamage96 => [0.096, 0.17, 0.247, 0.286, 0.325, 0.363, 0.402, 0.441],
        WeaponSubStatFamily::CriticalDamage102 => [0.102, 0.18, 0.263, 0.304, 0.345, 0.386, 0.427, 0.469],
        WeaponSubStatFamily::CriticalDamage120 => [0.12, 0.212, 0.31, 0.358, 0.406, 0.454, 0.502, 0.551],
        WeaponSubStatFamily::CriticalDamage144 => [0.144, 0.254, 0.371, 0.429, 0.487, 0.545, 0.603, 0.662],
        WeaponSubStatFamily::CriticalDamage192 => [0.192, 0.339, 0.494, 0.572, 0.65, 0.727, 0.804, 0.882],

        WeaponSubStatFamily::CriticalRate34 => [0.034, 0.06, 0.088, 0.101, 0.115, 0.129, 0.142, 0.156],
        WeaponSubStatFamily::CriticalRate40 => [0.04, 0.0707, 0.103, 0.1192, 0.1353, 0.1514, 0.1676, 0.1838],
        WeaponSubStatFamily::CriticalRate48 => [0.048, 0.085, 0.124, 0.143, 0.162, 0.182, 0.201, 0.221],
        WeaponSubStatFamily::CriticalRate51 => [0.051, 0.09, 0.131, 0.152, 0.173, 0.193, 0.201, 0.221],
        WeaponSubStatFamily::CriticalRate60 => [0.06, 0.106, 0.155, 0.179, 0.203, 0.227, 0.251, 0.276],
        WeaponSubStatFamily::CriticalRate68 => [0.068, 0.12, 0.175, 0.203, 0.23, 0.257, 0.285, 0.312],
        WeaponSubStatFamily::CriticalRate72 => [0.072, 0.12, 0.185, 0.214, 0.244, 0.273, 0.302, 0.331],
        WeaponSubStatFamily::CriticalRate80 => [0.08, 0.141, 0.206, 0.238, 0.271, 0.303, 0.335, 0.368],
        WeaponSubStatFamily::CriticalRate96 => [0.096, 0.17, 0.247, 0.286, 0.325, 0.363, 0.402, 0.441],

        WeaponSubStatFamily::DEF64 => [0.064, 0.113, 0.164, 0.19, 0.216, 0.241, 0.267, 0.293],
        WeaponSubStatFamily::DEF96 => [0.096, 0.169, 0.246, 0.285, 0.323, 0.362, 0.401, 0.439],
        WeaponSubStatFamily::DEF113 => [0.113, 0.199, 0.29, 0.335, 0.381, 0.426, 0.472, 0.517],
        WeaponSubStatFamily::DEF150 => [0.15, 0.265, 0.387, 0.447, 0.508, 0.568, 0.629, 0.69],

        WeaponSubStatFamily::EM12 => [12.0, 21.0, 31.0, 36.0, 41.0, 45.0, 50.0, 55.0],
        WeaponSubStatFamily::EM20 => [20.0, 36.0, 53.0, 61.0, 69.0, 77.0, 85.0, 94.0],
        WeaponSubStatFamily::EM24 => [24.0, 42.0, 62.0, 71.0, 81.0, 91.0, 101.0, 110.0],
        WeaponSubStatFamily::EM31 => [31.0, 54.0, 79.0, 91.0, 104.0, 116.0, 128.0, 141.0],
        WeaponSubStatFamily::EM36 => [36.0, 64.0, 93.0, 107.0, 122.0, 136.0, 151.0, 165.0],
        WeaponSubStatFamily::EM41 => [41.0, 72.0, 105.0, 122.0, 138.0, 154.0, 171.0, 187.0],
        WeaponSubStatFamily::EM43 => [43.0, 76.0, 111.0, 129.0, 146.0, 164.0, 181.0, 198.0],
        WeaponSubStatFamily::EM48 => [48.0, 85.0, 124.0, 143.0, 162.0, 182.0, 201.0, 221.0],
        WeaponSubStatFamily::EM58 => [57.6, 101.78, 148.32, 171.59, 194.86, 218.07, 241.34, 264.61],

        WeaponSubStatFamily::HP60 => [0.06, 0.106, 0.1545, 0.1787, 0.203, 0.2272, 0.2514, 0.2756],
        WeaponSubStatFamily::HP77 => [0.077, 0.135, 0.197, 0.228, 0.259, 0.29, 0.321, 0.413],
        WeaponSubStatFamily::HP90 => [0.09, 0.159, 0.232, 0.268, 0.304, 0.341, 0.377, 0.413],
        WeaponSubStatFamily::HP102 => [0.102, 0.18, 0.263, 0.304, 0.345, 0.386, 0.427, 0.469],
        WeaponSubStatFamily::HP108 => [0.108, 0.191, 0.278, 0.322, 0.365, 0.409, 0.453, 0.496],
        WeaponSubStatFamily::HP120 => [0.12, 0.212, 0.309, 0.3575, 0.406, 0.4543, 0.5028, 0.5513],
        WeaponSubStatFamily::HP144 => [0.144, 0.2544, 0.3708, 0.429, 0.4872, 0.5452, 0.6034, 0.6615],

        WeaponSubStatFamily::PhysicalBonus45 => [0.045, 0.08, 0.116, 0.134, 0.152, 0.17, 0.189, 0.207],
        WeaponSubStatFamily::PhysicalBonus75 => [0.075, 0.133, 0.193, 0.224, 0.254, 0.284, 0.315, 0.345],
        WeaponSubStatFamily::PhysicalBonus90 => [0.09, 0.159, 0.232, 0.268, 0.304, 0.341, 0.377, 0.413],
        WeaponSubStatFamily::PhysicalBonus96 => [0.096, 0.169, 0.246, 0.265, 0.323, 0.362, 0.401, 0.439],
        WeaponSubStatFamily::PhysicalBonus113 => [0.113, 0.199, 0.29, 0.335, 0.381, 0.426, 0.472, 0.517],
        WeaponSubStatFamily::PhysicalBonus150 => [0.15, 0.265, 0.387, 0.447, 0.508, 0.568, 0.629, 0.69],

        WeaponSubStatFamily::Recharge67 => [0.067, 0.118, 0.172, 0.199, 0.226, 0.252, 0.279, 0.306],
        WeaponSubStatFamily::Recharge80 => [0.08, 0.141, 0.206, 0.238, 0.271, 0.303, 0.335, 0.368],
        WeaponSubStatFamily::Recharge85 => [0.085, 0.15, 0.219, 0.253, 0.288, 0.322, 0.356, 0.39],
        WeaponSubStatFamily::Recharge100 => [0.1, 0.177, 0.258, 0.298, 0.338, 0.379, 0.419, 0.459],
        WeaponSubStatFamily::Recharge113 => [0.113, 0.2, 0.292, 0.338, 0.383, 0.429, 0.475, 0.521],
        WeaponSubStatFamily::Recharge120 => [0.12, 0.212, 0.309, 0.357, 0.406, 0.454, 0.503, 0.551],
        WeaponSubStatFamily::Recharge133 => [0.133, 0.236, 0.343, 0.397, 0.451, 0.505, 0.559, 0.613],
    }
}

pub struct WeaponSubStat {
    pub value: f64,
    pub stat_name: StatName,
    pub attribute_key: String,
}

impl WeaponSubStat {
    pub fn new(family: WeaponSubStatFamily, level: i32, _ascend: bool) -> WeaponSubStat {
        let array = get_value_array(family);

        let temp = [1, 20, 40, 50, 60, 70, 80, 90];
        let mut index = 0;
        while index < temp.len() && temp[index] < level {
            index += 1;
        }

        let value = if level == temp[index] {
            array[index]
        } else {
            let delta = (array[index] - array[index - 1]) / (temp[index] as f64 - temp[index - 1] as f64);
            let count = level / 5 * 5 - temp[index - 1];
            array[index - 1] + delta * count as f64
        };

        WeaponSubStat {
            value,
            stat_name: get_stat_name_from_family(family),
            attribute_key: String::from("武器副词条"),
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for WeaponSubStat {
    fn change_attribute(&self, attribute: &mut T) {
        self.stat_name.apply(attribute, self.attribute_key.as_str(), self.value);
    }
}