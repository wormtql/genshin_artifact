#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum WeaponBaseATKFamily {
    ATK185,
    ATK243,
    ATK354,
    ATK401,
    ATK440,
    ATK448,
    ATK454,
    ATK510,
    ATK542,
    ATK565,
    ATK620,
    ATK608,
    ATK674,
    ATK741,
}

pub fn get_array(family: WeaponBaseATKFamily) -> [i32; 14] {
    match family {
        WeaponBaseATKFamily::ATK185 => [23, 56, 68, 102, 113, 130, 141, 158, 169, 185, 185, 185, 185, 185],
        WeaponBaseATKFamily::ATK243 => [33, 80, 91, 139, 151, 174, 186, 209, 220, 243, 243, 243, 243, 243],
        WeaponBaseATKFamily::ATK354 => [38, 86, 105, 151, 171, 193, 212, 234, 253, 274, 294, 314, 334, 354],
        WeaponBaseATKFamily::ATK401 => [39, 94, 113, 169, 189, 216, 236, 263, 282, 309, 329, 355, 375, 401],
        WeaponBaseATKFamily::ATK440 => [39, 94, 113, 169, 189, 216, 236, 263, 282, 309, 329, 355, 375, 440],
        WeaponBaseATKFamily::ATK448 => [40, 102, 121, 187, 207, 239, 259, 292, 311, 344, 363, 396, 415, 448],
        WeaponBaseATKFamily::ATK454 => [41, 99, 125, 184, 210, 238, 264, 293, 319, 347, 373, 401, 427, 454],
        WeaponBaseATKFamily::ATK510 => [42, 109, 135, 205, 231, 266, 292, 327, 353, 388, 414, 449, 475, 510],
        WeaponBaseATKFamily::ATK542 => [44, 110, 141, 210, 241, 275, 307, 341, 373, 408, 439, 475, 506, 542],
        WeaponBaseATKFamily::ATK565 => [44, 119, 144, 226, 252, 293, 319, 361, 387, 429, 455, 497, 523, 565],
        WeaponBaseATKFamily::ATK620 => [45, 134, 154, 261, 287, 340, 366, 419, 445, 499, 525, 579, 605, 620],
        WeaponBaseATKFamily::ATK608 => [46, 122, 153, 235, 266, 308, 340, 382, 414, 457, 488, 532, 563, 608],
        WeaponBaseATKFamily::ATK674 => [48, 133, 164, 261, 292, 341, 373, 423, 455, 506, 537, 590, 621, 674],
        WeaponBaseATKFamily::ATK741 => [49, 145, 176, 286, 317, 374, 406, 464, 495, 555, 586, 648, 679, 741],
    }
}

impl WeaponBaseATKFamily {
    pub fn get_base_atk(&self, level: i32, ascend: bool) -> f64 {
        let array = get_array(*self);

        if level < 20 || (level == 20 && !ascend) {
            (level - 1) as f64 * (array[1] - array[0]) as f64 / 19.0 + array[0] as f64
        } else if level < 40 || (level == 40 && !ascend) {
            (level - 20) as f64 * (array[3] - array[2]) as f64 / 20.0 + array[2] as f64
        } else if level < 50 || (level == 50 && !ascend) {
            (level - 40) as f64 * (array[5] - array[4]) as f64 / 10.0 + array[4] as f64
        } else if level < 60 || (level == 60 && !ascend) {
            (level - 50) as f64 * (array[7] - array[6]) as f64 / 10.0 + array[6] as f64
        } else if level < 70 || (level == 70 && !ascend) {
            (level - 60) as f64 * (array[9] - array[8]) as f64 / 10.0 + array[8] as f64
        } else if level < 80 || (level == 80 && !ascend) {
            (level - 70) as f64 * (array[11] - array[10]) as f64 / 10.0 + array[10] as f64
        } else {
            (level - 80) as f64 * (array[13] - array[12]) as f64 / 10.0 + array[12] as f64
        }
    }
}