use crate::attribute::{Attribute, AttributeName};
use crate::common::reaction_type::TransformativeType;

pub struct TransformativeDamage {
    pub swirl_cryo: f64,
    pub swirl_hydro: f64,
    pub swirl_pyro: f64,
    pub swirl_electro: f64,
    pub overload: f64,
    pub electro_charged: f64,
    pub shatter: f64,
    pub superconduct: f64
}

const TRANSFORMATIVE_BASE: [[usize; 5]; 90] = [
    // 超导，扩散，感电，碎冰，超载
    [9, 10, 21, 26, 34],        // 1
    [9, 11, 22, 28, 37],
    [10, 12, 24, 30, 40],
    [11, 13, 26, 32, 43],
    [11, 14, 27, 34, 45],       // 5
    [12, 15, 30, 37, 49],
    [13, 16, 32, 40, 53],
    [14, 17, 35, 43, 58],
    [16, 19, 38, 47, 63],
    [17, 20, 41, 51, 68],       // 10
    [19, 22, 45, 56, 74],
    [20, 24, 49, 61, 81],
    [22, 27, 53, 67, 89],
    [24, 29, 58, 73, 97],
    [27, 32, 64, 81, 107],      // 15
    [30, 35, 71, 89, 117],
    [32, 39, 77, 97, 129],
    [35, 42, 84, 105, 139],
    [38, 45, 90, 113, 150],
    [40, 48, 97, 121, 161],     // 20
    [43, 52, 103, 129, 172],
    [46, 55, 110, 138, 183],
    [49, 58, 117, 146, 194],
    [51, 62, 123, 154, 206],
    [54, 65, 130, 163, 217],    // 25
    [57, 68, 136, 170, 226],
    [59, 71, 142, 177, 236],
    [61, 74, 148, 184, 246],
    [65, 78, 156, 195, 259],
    [68, 82, 164, 204, 273],    // 30
    [71, 86, 171, 214, 285],
    [75, 89, 179, 224, 298],
    [78, 93, 187, 233, 311],
    [81, 97, 194, 243, 324],
    [85, 101, 203, 254, 338],   // 35
    [88, 106, 212, 265, 353],
    [92, 110, 221, 276, 368],
    [96, 115, 230, 288, 383],
    [100, 120, 239, 299, 399],
    [104, 124, 249, 311, 415],  // 40
    [108, 129, 258, 323, 431],
    [112, 134, 269, 336, 448],
    [117, 140, 280, 350, 467],
    [122, 146, 292, 365, 487],
    [128, 154, 307, 384, 512],  // 45
    [134, 161, 322, 403, 537],
    [141, 169, 338, 422, 563],
    [148, 177, 354, 443, 590],
    [155, 185, 371, 464, 618],
    [162, 194, 388, 485, 647],  // 50
    [168, 202, 404, 505, 674],
    [175, 210, 421, 526, 701],
    [182, 219, 437, 547, 729],
    [189, 227, 454, 568, 757],
    [199, 239, 478, 598, 797],  // 55
    [208, 250, 500, 625, 833],
    [217, 261, 521, 652, 869],
    [226, 272, 544, 679, 906],
    [236, 284, 567, 709, 945],
    [246, 296, 591, 739, 986],  // 60
    [257, 308, 616, 770, 1027],
    [270, 323, 647, 809, 1078],
    [283, 339, 679, 848, 1131],
    [296, 356, 711, 889, 1185],
    [312, 375, 749, 937, 1249], // 65
    [326, 391, 782, 977, 1303],
    [340, 408, 815, 1019, 1359],
    [354, 425, 849, 1062, 1416],
    [368, 442, 884, 1105, 1473],
    [383, 459, 919, 1148, 1531],    // 70
    [397, 477, 954, 1192, 1590],
    [412, 495, 990, 1237, 1649],
    [426, 511, 1021, 1277, 1702],
    [439, 527, 1053, 1317, 1755],
    [457, 549, 1097, 1371, 1828],   // 75
    [473, 568, 1136, 1420, 1893],
    [490, 588, 1175, 1469, 1959],
    [506, 607, 1213, 1517, 2022],
    [522, 627, 1254, 1567, 2090],
    [539, 646, 1293, 1616, 2155],   // 80
    [555, 666, 1332, 1665, 2155],
    [571, 686, 1372, 1714, 2286],
    [588, 706, 1412, 1765, 2353],
    [605, 726, 1452, 1815, 2420],
    [627, 752, 1505, 1881, 2420],   // 85
    [644, 773, 1547, 1933, 2578],
    [663, 795, 1591, 1988, 2651],
    [682, 818, 1636, 2045, 2727],
    [703, 843, 1686, 2108, 2810],
    [723, 868, 1736, 2170, 2894],
];

fn type_to_index(t: TransformativeType) -> usize {
    use TransformativeType::*;
    match t {
        Overload => 4,
        Superconduct => 0,
        SwirlHydro | SwirlElectro | SwirlCryo | SwirlPyro => 1,
        ElectroCharged => 2,
        Shatter => 3,
        _ => unreachable!()
    }
}

fn get_transformative_base(level: usize, t: TransformativeType) -> usize {
    TRANSFORMATIVE_BASE[level - 1][type_to_index(t)]
}

fn get_em_bonus(em: f64) -> f64 {
    16.0 * em / (em + 2000.0)
}

pub fn transformative_damage<A: Attribute>(level: usize, attribute: &A, res_ratio: f64) -> TransformativeDamage {
    let enhance_base = attribute.get_value(AttributeName::EnhanceSwirlBase);
    let enhance_swirl_pyro = attribute.get_value(AttributeName::EnhanceSwirlPyro) + enhance_base;
    let enhance_swirl_cryo = attribute.get_value(AttributeName::EnhanceSwirlCryo) + enhance_base;
    let enhance_swirl_electro = attribute.get_value(AttributeName::EnhanceSwirlElectro) + enhance_base;
    let enhance_swirl_hydro = attribute.get_value(AttributeName::EnhanceSwirlHydro) + enhance_base;
    let enhance_overload = attribute.get_value(AttributeName::EnhanceOverload);
    let enhance_superconduct = attribute.get_value(AttributeName::EnhanceSuperconduct);
    let enhance_shatter = attribute.get_value(AttributeName::EnhanceShatter);
    let enhance_electro_charged = attribute.get_value(AttributeName::EnhanceElectroCharged);

    let base_swirl = get_transformative_base(level, TransformativeType::SwirlPyro) as f64;
    let base_overload = get_transformative_base(level, TransformativeType::Overload) as f64;
    let base_superconduct = get_transformative_base(level, TransformativeType::Superconduct) as f64;
    let base_shatter = get_transformative_base(level, TransformativeType::Shatter) as f64;
    let base_electro_charged = get_transformative_base(level, TransformativeType::ElectroCharged) as f64;

    let em = attribute.get_value(AttributeName::ElementalMastery);
    let em_bonus = get_em_bonus(em);

    let dmg_swirl_pyro = base_swirl * res_ratio * (1.0 + em_bonus + enhance_swirl_pyro);
    let dmg_swirl_cryo = base_swirl * res_ratio * (1.0 + em_bonus + enhance_swirl_cryo);
    let dmg_swirl_electro = base_swirl * res_ratio * (1.0 + em_bonus + enhance_swirl_electro);
    let dmg_swirl_hydro = base_swirl * res_ratio * (1.0 + em_bonus + enhance_swirl_hydro);
    let dmg_overload = base_overload * res_ratio * (1.0 + em_bonus + enhance_overload);
    let dmg_superconduct = base_superconduct * res_ratio * (1.0 + em_bonus + enhance_superconduct);
    let dmg_shatter = base_shatter * res_ratio * (1.0 + em_bonus + enhance_shatter);
    let dmg_electro_charged = base_electro_charged * res_ratio * (1.0 + em_bonus + enhance_electro_charged);

    TransformativeDamage {
        swirl_cryo: dmg_swirl_cryo,
        swirl_hydro: dmg_swirl_hydro,
        swirl_pyro: dmg_swirl_pyro,
        swirl_electro: dmg_swirl_electro,
        overload: dmg_overload,
        electro_charged: dmg_electro_charged,
        shatter: dmg_shatter,
        superconduct: dmg_superconduct
    }
}

pub fn swirl_without_element<A: Attribute>(level: usize, attribute: &A, res_ratio: f64) -> f64 {
    let enhance_swirl_base = attribute.get_value(AttributeName::EnhanceSwirlBase);
    let em = attribute.get_value(AttributeName::ElementalMastery);
    let em_bonus = get_em_bonus(em);

    let base = get_transformative_base(level, TransformativeType::SwirlPyro) as f64;

    base * res_ratio * (1.0 + em_bonus + enhance_swirl_base)
}
