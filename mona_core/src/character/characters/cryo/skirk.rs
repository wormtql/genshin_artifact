use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

type SKILL = [f64; 15];

pub struct SkirkSkillType {

    // normal attack
    pub normal_dmg1: SKILL,
    pub normal_dmg2: SKILL,
    pub normal_dmg3: SKILL,
    pub normal_dmg4: SKILL,
    pub normal_dmg5: SKILL,

    // charged attack
    pub charged_dmg: SKILL,

    // xialuo
    pub plunging_dmg1: SKILL,
    pub plunging_dmg2: SKILL,
    pub plunging_dmg3: SKILL,

    // e
    pub e_dmg_normal_1: SKILL,
    pub e_dmg_normal_2: SKILL,
    pub e_dmg_normal_3: SKILL,
    pub e_dmg_normal_4: SKILL,
    pub e_dmg_normal_5: SKILL,
    pub e_dmg_charged: SKILL,
    pub e_dmg_plunging_1: SKILL,
    pub e_dmg_plunging_2: SKILL,
    pub e_dmg_plunging_3: SKILL,

    // q
    pub q_dmg1: SKILL,
    pub q_dmg2: SKILL,
    pub q_bonus_attack: SKILL,
    pub q_bonus0: SKILL,
    pub q_bonus1: SKILL,
    pub q_bonus2: SKILL,
    pub q_bonus3: SKILL,
}

pub const SKIRK_SKILL: SkirkSkillType = SkirkSkillType {
    // TODO: From this on index 11 through 14 are made up (-4)
    normal_dmg1: [0.545, 0.59, 0.634, 0.697, 0.742, 0.793, 0.862, 0.932, 1.002, 1.078, 1.154, 1.154, 1.154, 1.154, 1.154],
    normal_dmg2: [0.498, 0.538, 0.579, 0.637, 0.677, 0.724, 0.787, 0.851, 0.915, 0.984, 1.054, 1.054, 1.054, 1.054, 1.054],
    normal_dmg3: [0.324, 0.351, 0.377, 0.415, 0.441, 0.471, 0.513, 0.554, 0.596, 0.641, 0.686, 0.686, 0.686, 0.686, 0.686],
    normal_dmg4: [0.608, 0.658, 0.707, 0.778, 0.827, 0.884, 0.962, 1.039, 1.117, 1.202, 1.287, 1.287, 1.287, 1.287, 1.287],
    normal_dmg5: [0.829, 0.897, 0.964, 1.060, 1.128, 1.195, 1.311, 1.417, 1.523, 1.639, 1.754, 1.754, 1.754, 1.754, 1.754],

    charged_dmg: [0.668, 0.723, 0.777, 0.854, 0.909, 0.971, 1.057, 1.142, 1.228, 1.321, 1.414, 1.414, 1.414, 1.414, 1.414],
    
    plunging_dmg1: [0.639, 0.691, 0.743, 0.818, 0.870, 0.929, 1.011, 1.093, 1.175, 1.264, 1.353, 1.353, 1.353, 1.353, 1.353],
    plunging_dmg2:[1.28, 1.38, 1.49, 1.64, 1.74, 1.86, 2.02, 2.19, 2.35, 2.53, 2.71, 2.71, 2.71, 2.71, 2.71],
    plunging_dmg3: [1.60, 1.73, 1.86, 2.04, 2.17, 2.32, 2.53, 2.73, 2.93, 3.16, 3.38, 3.38, 3.38, 3.38, 3.38],

    // -1
    e_dmg_normal_1: [1.328, 1.436, 1.544, 1.699, 1.807, 1.931, 2.100, 2.27, 2.44, 2.626, 2.811, 2.996, 3.182, 3.367, 3.367],
    e_dmg_normal_2: [1.198, 1.296, 1.393, 1.532, 1.630, 1.741, 1.895, 2.048, 2.201, 2.368, 2.535, 2.702, 2.870, 3.037, 3.037],
    e_dmg_normal_3: [0.757, 0.819, 0.881, 0.969, 1.030, 1.101, 1.197, 1.294, 1.391, 1.497, 1.603, 1.708, 1.814, 1.919, 1.919],
    e_dmg_normal_4: [0.805, 0.871, 0.937, 1.030, 1.096, 1.171, 1.274, 1.377, 1.480, 1.592, 1.704, 1.817, 1.929, 2.042, 2.042],
    e_dmg_normal_5: [1.966, 2.216, 2.286, 2.515, 2.675, 2.858, 3.109, 3.361, 3.612, 3.887, 4.161, 4.435, 4.710, 4.984, 4.984],
    e_dmg_charged: [0.445, 0.482, 0.518, 0.57, 0.606, 0.648, 0.704, 0.761, 0.818, 0.881, 0.943, 1.005, 1.067, 1.129, 1.129],
    e_dmg_plunging_1: [0.639, 0.691, 0.743, 0.818, 0.870, 0.929, 1.011, 1.093, 1.175, 1.264, 1.353, 1.442, 1.531, 1.621, 1.621],
    e_dmg_plunging_2:[1.28, 1.38, 1.49, 1.64, 1.74, 1.86, 2.02, 2.19, 2.35, 2.53, 2.71, 2.88, 3.06, 3.24, 3.24],
    e_dmg_plunging_3: [1.60, 1.73, 1.86, 2.04, 2.17, 2.32, 2.53, 2.73, 2.93, 3.16, 3.38, 3.6, 3.82, 4.05, 4.05],

    // -2
    q_dmg1: [1.228, 1.320, 1.412, 1.535, 1.627, 1.719, 1.841, 1.964, 2.087, 2.21, 2.332, 2.455, 2.609, 2.609, 2.609],
    q_dmg2: [2.046, 2.191, 2.353, 2.558, 2.711, 2.864, 3.069, 3.274, 3.478, 3.683, 3.887, 4.092, 4.348, 4.348, 4.348],
    q_bonus_attack: [0.1932, 0.2077, 0.2222, 0.2415, 0.256, 0.2705, 0.2899, 0.3092, 0.3285, 0.3478, 0.3671, 0.3865, 0.4106, 0.4106, 0.4106],
    q_bonus0: [0.035, 0.040, 0.045, 0.050, 0.055, 0.060, 0.065, 0.070, 0.075, 0.080, 0.085, 0.090, 0.095, 0.095, 0.095],
    q_bonus1: [0.066, 0.072, 0.078, 0.084, 0.090, 0.096, 0.102, 0.108, 0.114, 0.120, 0.126, 0.132, 0.138, 0.138, 0.138],
    q_bonus2: [0.088, 0.096, 0.104, 0.112, 0.120, 0.128, 0.136, 0.144, 0.152, 0.160, 0.168, 0.176, 0.184, 0.184, 0.184],
    q_bonus3: [0.110, 0.120, 0.130, 0.140, 0.150, 0.160, 0.170, 0.180, 0.190, 0.200, 0.210, 0.220, 0.230, 0.230, 0.230],
};

pub struct SkirkEffect {
    // Void Rift: 虚境裂隙
    // Serpent's Subtlety: 蛇之狡谋
    pub valid_teamate_count: i32, // 水冰队友数量
    pub void_rift_count: i32, // 虚境裂隙数量 0~3
    pub serpents_subtlety_count: i32, // 蛇之狡谋数量 50~100

    pub constellation: usize, // 命座

    pub has_talent1: bool,
    pub has_talent2: bool,
}

damage_enum!(
    SkirkDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Normal5

    Charged

    Plunging1
    Plunging2
    Plunging3

    ENormal1
    ENormal2
    ENormal3
    ENormal4
    ENormal5
    ECharged
    EPlunging1
    EPlunging2
    EPlunging3

    Q1
    Q2

    C1_Void_Rift
    C6_Q
    C6_E
);

impl SkirkDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use SkirkDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            ENormal1 | ENormal2 | ENormal3 | ENormal4 | ENormal5 => SkillType::NormalAttack,
            ECharged => SkillType::ChargedAttack,
            EPlunging1 => SkillType::PlungingAttackInAction,
            EPlunging2 | EPlunging3=> SkillType::PlungingAttackOnGround,
            Q1 | Q2 => SkillType::ElementalBurst,
            C1_Void_Rift => SkillType::ChargedAttack,
            C6_Q => SkillType::ElementalBurst,
            C6_E => SkillType::NormalAttack,
        }
    }

    pub fn get_element(&self) -> Element {
        use SkirkDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 | Charged | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            ENormal1 | ENormal2 | ENormal3 | ENormal4 | ENormal5 | ECharged | EPlunging1 | EPlunging2 | EPlunging3 | C1_Void_Rift => Element::Cryo,
            Q1 | Q2 => Element::Cryo,
            C6_Q | C6_E => Element::Cryo,
        }
    }

    pub fn is_e_normal(&self) -> bool {
        use SkirkDamageEnum::*;
        match *self {
            ENormal1 | ENormal2 | ENormal3 | ENormal4 | ENormal5 | C6_E => true,
            _ => false,
        }
    }
}

impl<A: Attribute> ChangeAttribute<A> for SkirkEffect {
    fn change_attribute(&self, attribute: &mut A) {
        
        let mult1 = if self.has_talent1 {1.0} else {0.0};
        let mult2 = if self.has_talent2 {1.0} else {0.0};

        if self.constellation >= 2 {
            attribute.add_atk_percentage("C2「坠渊」", 0.7);
        }

        if self.constellation >= 4 {
            let this_list = [0.0, 0.1, 0.2, 0.4];
            attribute.add_atk_percentage("C4「流断」", mult2 * this_list[self.valid_teamate_count as usize] as f64);
        }

        attribute.set_value_by(AttributeName::USER1, "水冰队友数量", self.valid_teamate_count as f64 * mult2);
        attribute.set_value_by(AttributeName::USER2, "虚境裂隙数量", self.void_rift_count as f64 * mult1);
        attribute.set_value_by(AttributeName::USER3, "蛇之狡谋数量", self.serpents_subtlety_count as f64);
    }
}

pub struct Skirk;

impl CharacterTrait for Skirk {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Skirk,
        internal_name: "Skirk",
        name_locale: locale!(
            zh_cn: "丝柯克",
            en: "Skirk"
        ),
        element: Element::Cryo,
        hp: [967, 2508, 3336, 4992, 5581, 6421, 7206, 8055, 8644, 9501, 10089, 10956, 11544, 12417],
        atk: [28, 72, 96, 144, 161, 186, 208, 233, 250, 274, 292, 317, 334, 359],
        def: [63, 163, 217, 324, 362, 417, 468, 523, 561, 617, 655, 711, 750, 806],

        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·极恶技·断",
            en: "Normal Attack: Havoc: Sunder"
        ),
        skill_name2: locale!(
            zh_cn: "极恶技·闪",
            en: "Havoc: Wrap"
        ),
        skill_name3: locale!(
            zh_cn: "极恶技·灭",
            en: "Havoc: Ruin"
        ),
    };
    type SkillType = SkirkSkillType;
    const SKILL: Self::SkillType = SKIRK_SKILL;
    type DamageEnumType = SkirkDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            SkirkDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Normal5 hit_n_dmg!(5)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            SkirkDamageEnum
            ENormal1 locale!(zh_cn: "七相一闪普通攻击一段伤害", en: "Seven-Phase Flash Normal Attack 1-Hit DMG")
            ENormal2 locale!(zh_cn: "七相一闪普通攻击二段伤害", en: "Seven-Phase Flash Normal Attack 2-Hit DMG")
            ENormal3 locale!(zh_cn: "七相一闪普通攻击三段伤害", en: "Seven-Phase Flash Normal Attack 3-Hit DMG")
            ENormal4 locale!(zh_cn: "七相一闪普通攻击四段伤害", en: "Seven-Phase Flash Normal Attack 4-Hit DMG")
            ENormal5 locale!(zh_cn: "七相一闪普通攻击五段伤害", en: "Seven-Phase Flash Normal Attack 5-Hit DMG")
            ECharged locale!(zh_cn: "七相一闪重击伤害", en: "Seven-Phase Flash Charged Attack DMG")
            EPlunging1 locale!(zh_cn: "七相一闪下坠期间伤害", en: "Seven-Phase Flash Plunge DMG")
            EPlunging2 locale!(zh_cn: "七相一闪低空坠地冲击伤害", en: "Seven-Phase Flash Low Plunge DMG")
            EPlunging3 locale!(zh_cn: "七相一闪高空坠地冲击伤害", en: "Seven-Phase Flash High Plunge DMG")
        ),
        skill3: skill_map!(
            SkirkDamageEnum
            Q1 locale!(zh_cn: "斩击伤害", en: "Slash DMG")
            Q2 locale!(zh_cn: "最终段斩击伤害", en: "Last-Hit DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "valid_teamate_count",
            title: locale!(
                zh_cn: "水冰队友数量",
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 3 }
        },
        ItemConfig {
            name: "void_rift_count",
            title: locale!(
                zh_cn: "虚境裂隙数量",
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 3 }
        },
        ItemConfig {
            name: "serpents_subtlety_count",
            title: locale!(
                zh_cn: "蛇之狡谋数量",
            ),
            config: ItemConfigType::Int { min: 50, max: 100, default: 50 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_q",
            title: locale!(
                zh_cn: "极恶技·尽",
                en: "Havoc: Extinction"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: SkirkDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let mut builder = D::new();
        use SkirkDamageEnum::*;

        let after_q = match *config {
            CharacterSkillConfig::Skirk { after_q } => after_q,
            _ => false
        };

        let valid_teamate_count = context.attribute.get_value(AttributeName::USER1);
        let mut void_rift_count = context.attribute.get_value(AttributeName::USER2) as f64;
        let serpents_subtlety_count = context.attribute.get_value(AttributeName::USER3) as f64;

        let has_c1 = if context.character_common_data.constellation >= 1 {1.0} else {0.0};
        let has_c6 = if context.character_common_data.constellation >= 6 {1.0} else {0.0};

        let talent_2_normal = [1.1, 1.1, 1.2, 1.7]; // 倍率
        let talent_2_q = [1.1, 1.05, 1.15, 1.6]; // 倍率
        let e_normal_mult = talent_2_normal[valid_teamate_count as usize];
        let q_mult = talent_2_q[valid_teamate_count as usize];

        let skill_type = s.get_skill_type();

        let ratio = match s {
            Normal1 => SKIRK_SKILL.normal_dmg1[s1],
            Normal2 => SKIRK_SKILL.normal_dmg2[s1],
            Normal3 => SKIRK_SKILL.normal_dmg3[s1] * 2.0,
            Normal4 => SKIRK_SKILL.normal_dmg4[s1],
            Normal5 => SKIRK_SKILL.normal_dmg5[s1],
            Charged => SKIRK_SKILL.charged_dmg[s1] * 2.0,
            Plunging1 => SKIRK_SKILL.plunging_dmg1[s1],
            Plunging2 => SKIRK_SKILL.plunging_dmg2[s1],
            Plunging3 => SKIRK_SKILL.plunging_dmg3[s1],
            ENormal1 => SKIRK_SKILL.e_dmg_normal_1[s2] * e_normal_mult,
            ENormal2 => SKIRK_SKILL.e_dmg_normal_2[s2] * e_normal_mult,
            ENormal3 => SKIRK_SKILL.e_dmg_normal_3[s2] * e_normal_mult * 2.0,
            ENormal4 => SKIRK_SKILL.e_dmg_normal_4[s2] * e_normal_mult * 2.0,
            ENormal5 => SKIRK_SKILL.e_dmg_normal_5[s2] * e_normal_mult,
            ECharged => SKIRK_SKILL.e_dmg_charged[s2] * 3.0,
            EPlunging1 => SKIRK_SKILL.e_dmg_plunging_1[s2],
            EPlunging2 => SKIRK_SKILL.e_dmg_plunging_2[s2],
            EPlunging3 => SKIRK_SKILL.e_dmg_plunging_3[s2],
            Q1 => SKIRK_SKILL.q_dmg1[s3] * q_mult * 5.0,
            Q2 => SKIRK_SKILL.q_dmg2[s3] * q_mult,
            C1_Void_Rift => has_c1 * 5.0 * void_rift_count,
            C6_Q => has_c6 * 750.0 * void_rift_count,
            C6_E => has_c6 * 180.0,
        };

        builder.add_atk_ratio("技能倍率", ratio);

        if skill_type == SkillType::ElementalBurst {
            let mut q_bonus_stacks = serpents_subtlety_count as f64 - 50.0;
            if q_bonus_stacks > 12.0 {
                q_bonus_stacks = 12.0;
            }
            if context.character_common_data.constellation >= 2 {
                q_bonus_stacks += 10.0;
            }
            
            builder.add_extra_atk("蛇之狡谋大招攻击力加成", q_bonus_stacks * SKIRK_SKILL.q_bonus_attack[s3]);
        }

        if after_q && s.is_e_normal() {
            let e_bonus = match void_rift_count as f64 {
                1.0 => SKIRK_SKILL.q_bonus1[s3],
                2.0 => SKIRK_SKILL.q_bonus2[s3],
                3.0 => SKIRK_SKILL.q_bonus3[s3],
                _ => SKIRK_SKILL.q_bonus0[s3],
            };
            builder.add_extra_bonus("汲取虚境裂隙伤害提升", e_bonus);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            skill_type,
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Skirk { valid_teamate_count, void_rift_count, serpents_subtlety_count } => Some(Box::new(SkirkEffect {
                valid_teamate_count, void_rift_count, serpents_subtlety_count,
                has_talent1: common_data.has_talent1,
                has_talent2: common_data.has_talent2,
                constellation: common_data.constellation as usize,
            })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}