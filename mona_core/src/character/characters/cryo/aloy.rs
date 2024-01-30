use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::AloyDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct AloySkillType {
    pub normal_dmg11: [f64; 15],
    pub normal_dmg12: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_atk_minus: [f64; 15],
    pub elemental_skill_atk_bonus1: [f64; 15],
    pub elemental_skill_atk_bonus2: [f64; 15],
    pub elemental_skill_atk_bonus3: [f64; 15],
    pub elemental_skill_atk_bonus4: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
}

pub const ALOY_SKILL: AloySkillType = AloySkillType {
    normal_dmg11: [0.2112, 0.2256, 0.24, 0.2592, 0.2736, 0.2904, 0.312, 0.3336, 0.3552, 0.3768, 0.3984, 0.42, 0.4416, 0.4632, 0.4848],
    normal_dmg12: [0.2376, 0.2538, 0.27, 0.2916, 0.3078, 0.3267, 0.351, 0.3753, 0.3996, 0.4239, 0.4482, 0.4725, 0.4968, 0.5211, 0.5454],
    normal_dmg2: [0.4312, 0.4606, 0.49, 0.5292, 0.5586, 0.5929, 0.637, 0.6811, 0.7252, 0.7693, 0.8134, 0.8575, 0.9016, 0.9457, 0.9898],
    normal_dmg3: [0.528, 0.564, 0.6, 0.648, 0.684, 0.726, 0.78, 0.834, 0.888, 0.942, 0.996, 1.05, 1.104, 1.158, 1.212],
    normal_dmg4: [0.6565, 0.7012, 0.746, 0.8057, 0.8504, 0.9027, 0.9698, 1.0369, 1.1041, 1.1712, 1.2384, 1.3055, 1.3726, 1.4398, 1.5069],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [1.776, 1.9092, 2.0424, 2.22, 2.3532, 2.4864, 2.664, 2.8416, 3.0192, 3.1968, 3.3744, 3.552, 3.774, 3.996, 4.218],
    elemental_skill_dmg2: [0.4, 0.43, 0.46, 0.5, 0.53, 0.56, 0.6, 0.64, 0.68, 0.72, 0.76, 0.8, 0.85, 0.9, 0.95],
    elemental_skill_atk_minus: [0.12, 0.12, 0.12, 0.13, 0.13, 0.13, 0.14, 0.14, 0.14, 0.15, 0.15, 0.15, 0.15, 0.15, 0.15],
    elemental_skill_atk_bonus1: [0.0585, 0.062, 0.0655, 0.07, 0.0735, 0.077, 0.0816, 0.0861, 0.0907, 0.0952, 0.0998, 0.1043, 0.1089, 0.1134, 0.118],
    elemental_skill_atk_bonus2: [0.1169, 0.1239, 0.1309, 0.14, 0.147, 0.154, 0.1631, 0.1722, 0.1813, 0.1904, 0.1995, 0.2086, 0.2177, 0.2268, 0.2359],
    elemental_skill_atk_bonus3: [0.1754, 0.1859, 0.1964, 0.21, 0.2205, 0.231, 0.2447, 0.2583, 0.272, 0.2856, 0.2993, 0.3129, 0.3266, 0.3402, 0.3539],
    elemental_skill_atk_bonus4: [0.2923, 0.3098, 0.3273, 0.35, 0.3675, 0.385, 0.4078, 0.4305, 0.4533, 0.476, 0.4988, 0.5215, 0.5443, 0.567, 0.5898],
    elemental_burst_dmg1: [3.592, 3.8614, 4.1308, 4.49, 4.7594, 5.0288, 5.388, 5.7472, 6.1064, 6.4656, 6.8248, 7.184, 7.633, 8.082, 8.531],
};

pub const ALOY_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Aloy,
    internal_name: "Aloy",
    element: Element::Cryo,
    hp: [848, 2201, 2928, 4382, 4899, 5636, 6325, 7070, 7587, 8339, 8856, 9616, 10133, 10899],
    atk: [18, 47, 63, 94, 105, 121, 136, 152, 163, 179, 190, 206, 217, 234],
    def: [53, 137, 182, 272, 304, 350, 393, 439, 471, 517, 550, 597, 629, 676],
    sub_stat: CharacterSubStatFamily::Bonus288(StatName::CryoBonus),
    weapon_type: WeaponType::Bow,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·快速射击",
        en: "Normal Attack: Rapid Fire",
    ),
    skill_name2: locale!(
        zh_cn: "冰尘雪野",
        en: "Frozen Wilds",
    ),
    skill_name3: locale!(
        zh_cn: "曙光预言",
        en: "Prophecies of Dawn",
    ),
    name_locale: locale!(
        zh_cn: "埃洛伊",
        en: "Aloy",
    )
};

pub struct Aloy;

#[derive(Copy, Clone, FromPrimitive)]
#[derive(EnumString, EnumCountMacro)]
pub enum AloyDamageEnum {
    Normal11,
    Normal12,
    Normal2,
    Normal3,
    Normal4,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    Q1
}

impl AloyDamageEnum {
    pub fn get_element(&self, ice_rush: bool) -> Element {
        use AloyDamageEnum::*;
        if ice_rush {
            match *self {
                Normal11 | Normal12 | Normal2 | Normal3 | Normal4 | E1 | E2 | Q1 => Element::Cryo,
                _ => Element::Physical
            }
        } else {
            match *self {
                E1 | E2 | Q1 => Element::Cryo,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AloyDamageEnum::*;
        match *self {
            Normal11 | Normal12 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for AloyDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum AloyRoleEnum {
    Sub
}

impl CharacterTrait for Aloy {
    const STATIC_DATA: CharacterStaticData = ALOY_STATIC_DATA;
    type SkillType = AloySkillType;
    const SKILL: Self::SkillType = ALOY_SKILL;
    type DamageEnumType = AloyDamageEnum;
    type RoleEnum = AloyRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: AloyDamageEnum::Normal11 as usize, text: hit_n_dmg!(1, 1) },
            CharacterSkillMapItem { index: AloyDamageEnum::Normal12 as usize, text: hit_n_dmg!(1, 2) },
            CharacterSkillMapItem { index: AloyDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: AloyDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: AloyDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: AloyDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: AloyDamageEnum::Charged2 as usize, text: charged_dmg!("shoot2") },
            CharacterSkillMapItem { index: AloyDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: AloyDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: AloyDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: AloyDamageEnum::E1 as usize, text: locale!(zh_cn: "冰尘弹伤害", en: "Freeze Bomb DMG") },
            CharacterSkillMapItem { index: AloyDamageEnum::E2 as usize, text: locale!(zh_cn: "冷冻炸弹伤害", en: "Chillwater Bomblet DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: AloyDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "coil_count",
            title: locale!(
                zh_cn: "线圈层数",
                en: "Coil Stack"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 4 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: AloyDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use AloyDamageEnum::*;
        let ratio = match s {
            Normal11 => ALOY_SKILL.normal_dmg11[s1],
            Normal12 => ALOY_SKILL.normal_dmg12[s1],
            Normal2 => ALOY_SKILL.normal_dmg2[s1],
            Normal3 => ALOY_SKILL.normal_dmg3[s1],
            Normal4 => ALOY_SKILL.normal_dmg4[s1],
            Charged1 => ALOY_SKILL.charged_dmg1[s1],
            Charged2 => ALOY_SKILL.charged_dmg2[s1],
            Plunging1 => ALOY_SKILL.plunging_dmg1[s1],
            Plunging2 => ALOY_SKILL.plunging_dmg2[s1],
            Plunging3 => ALOY_SKILL.plunging_dmg3[s1],
            E1 => ALOY_SKILL.elemental_skill_dmg1[s2],
            E2 => ALOY_SKILL.elemental_skill_dmg2[s2],
            Q1 => ALOY_SKILL.elemental_burst_dmg1[s3]
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let coil_count = match *config {
            CharacterSkillConfig::Aloy { coil_count } => coil_count,
            _ => 0
        };

        let skill_type = s.get_skill_type();
        if coil_count > 0 && skill_type == SkillType::NormalAttack {
            let bonus = match coil_count {
                0 => 0.0,
                1 => ALOY_SKILL.elemental_skill_atk_bonus1[s2],
                2 => ALOY_SKILL.elemental_skill_atk_bonus2[s2],
                3 => ALOY_SKILL.elemental_skill_atk_bonus3[s2],
                _ => ALOY_SKILL.elemental_skill_atk_bonus4[s2],
            };
            if coil_count < 4 {
                builder.add_extra_bonus("线圈加成", bonus);
            } else {
                builder.add_extra_bonus("冰驰加成", bonus);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(coil_count >= 4),
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: AloyRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            AloyRoleEnum::Sub => Box::new(AloyDefaultTargetFunction)
        }
    }
}
