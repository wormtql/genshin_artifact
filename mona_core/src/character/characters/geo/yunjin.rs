use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::YunjinDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct YunjinSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg41: [f64; 15],
    pub normal_dmg42: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_shield1: [f64; 15],
    pub elemental_skill_shield1_fixed: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg_bonus: [f64; 15],
}

const YUNJIN_SKILL: YunjinSkillType = YunjinSkillType {
    normal_dmg1: [0.4051, 0.438, 0.471, 0.5181, 0.5511, 0.5888, 0.6406, 0.6924, 0.7442, 0.8007, 0.8572, 0.9137, 0.9703, 1.0268, 1.0833],
    normal_dmg2: [0.4025, 0.4352, 0.468, 0.5148, 0.5476, 0.585, 0.6365, 0.688, 0.7394, 0.7956, 0.8518, 0.9079, 0.9641, 1.0202, 1.0764],
    normal_dmg31: [0.2296, 0.2483, 0.267, 0.2937, 0.3124, 0.3338, 0.3631, 0.3925, 0.4219, 0.4539, 0.4859, 0.518, 0.55, 0.5821, 0.6141],
    normal_dmg32: [0.2752, 0.2976, 0.32, 0.352, 0.3744, 0.4, 0.4352, 0.4704, 0.5056, 0.544, 0.5824, 0.6208, 0.6592, 0.6976, 0.736],
    normal_dmg41: [0.2399, 0.2595, 0.279, 0.3069, 0.3264, 0.3488, 0.3794, 0.4101, 0.4408, 0.4743, 0.5078, 0.5413, 0.5747, 0.6082, 0.6417],
    normal_dmg42: [0.2881, 0.3116, 0.335, 0.3685, 0.392, 0.4188, 0.4556, 0.4925, 0.5293, 0.5695, 0.6097, 0.6499, 0.6901, 0.7303, 0.7705],
    normal_dmg5: [0.6734, 0.7282, 0.783, 0.8613, 0.9161, 0.9788, 1.0649, 1.151, 1.2371, 1.3311, 1.4251, 1.519, 1.613, 1.7069, 1.8009],
    charged_dmg: [1.2169, 1.316, 1.415, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.6001, 2.8289, 3.0577, 3.2865, 3.5361],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.4912, 1.603, 1.7149, 1.864, 1.9758, 2.0877, 2.2368, 2.3859, 2.535, 2.6842, 2.8333, 2.9824, 3.1688, 3.3552, 3.5416],
    elemental_skill_dmg2: [2.6096, 2.8053, 3.001, 3.262, 3.4577, 3.6534, 3.9144, 4.1754, 4.4363, 4.6973, 4.9582, 5.2192, 5.5454, 5.8716, 6.1978],
    elemental_skill_dmg3: [3.728, 4.0076, 4.2872, 4.66, 4.9396, 5.2192, 5.592, 5.9648, 6.3376, 6.7104, 7.0832, 7.456, 7.922, 8.388, 8.854],
    elemental_skill_shield1: [0.12, 0.129, 0.138, 0.15, 0.159, 0.168, 0.18, 0.192, 0.204, 0.216, 0.228, 0.24, 0.255, 0.27, 0.285],
    elemental_skill_shield1_fixed: [1155.0, 1271.0, 1396.0, 1531.0, 1675.0, 1830.0, 1993.0, 2167.0, 2350.0, 2542.0, 2744.0, 2965.0, 3178.0, 3409.0, 3650.0],
    elemental_burst_dmg1: [2.44, 2.623, 2.806, 3.05, 3.233, 3.416, 3.66, 3.904, 4.148, 4.392, 4.636, 4.88, 5.185, 5.49, 5.795],
    elemental_burst_dmg_bonus: [0.3216, 0.3457, 0.3698, 0.402, 0.4261, 0.4502, 0.4824, 0.5146, 0.5467, 0.5789, 0.611, 0.6432, 0.6834, 0.7236, 0.7638],
};

const YUNJIN_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Yunjin,
    internal_name: "Yunjin",
    element: Element::Geo,
    hp: [894, 2296, 2963, 4438, 4913, 5651, 6283, 7021, 7495, 8233, 8707, 9445, 9919, 10657],
    atk: [16, 41, 53, 80, 88, 101, 113, 126, 134, 148, 156, 169, 178, 191],
    def: [62, 158, 204, 306, 339, 389, 433, 484, 517, 567, 600, 651, 684, 734],
    sub_stat: CharacterSubStatFamily::Recharge267,
    weapon_type: WeaponType::Polearm,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·拂云出手",
        en: "Normal Attack: Cloud-Grazing Strike",
    ),
    skill_name2: locale!(
        zh_cn: "旋云开相",
        en: "Opening Flourish",
    ),
    skill_name3: locale!(
        zh_cn: "破嶂见旌仪",
        en: "Cliffbreaker's Banner",
    ),
    name_locale: locale!(
        zh_cn: "云堇",
        en: "Yunjin",
    )
};

pub struct Yunjin;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum YunjinDamageEnum {
    Normal1,
    Normal2,
    Normal31,
    Normal32,
    Normal41,
    Normal42,
    Normal5,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    Q1
}

impl YunjinDamageEnum {
    pub fn get_element(&self) -> Element {
        use YunjinDamageEnum::*;
        match *self {
            E1 | E2 | E3 | Q1 => Element::Geo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use YunjinDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal41 | Normal42 | Normal5 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for YunjinDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum YunjinRoleEnum {
    Aux
}

impl CharacterTrait for Yunjin {
    const STATIC_DATA: CharacterStaticData = YUNJIN_STATIC_DATA;
    type SkillType = YunjinSkillType;
    const SKILL: Self::SkillType = YUNJIN_SKILL;
    type DamageEnumType = YunjinDamageEnum;
    type RoleEnum = YunjinRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: YunjinDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: YunjinDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: YunjinDamageEnum::Normal31 as usize, text: hit_n_dmg!(3, 1) },
            CharacterSkillMapItem { index: YunjinDamageEnum::Normal32 as usize, text: hit_n_dmg!(3, 2) },
            CharacterSkillMapItem { index: YunjinDamageEnum::Normal41 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: YunjinDamageEnum::Normal42 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: YunjinDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: YunjinDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: YunjinDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: YunjinDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: YunjinDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: YunjinDamageEnum::E1 as usize, text: locale!(zh_cn: "点按伤害", en: "Tap DMG") },
            CharacterSkillMapItem { index: YunjinDamageEnum::E2 as usize, text: locale!(zh_cn: "一段蓄力伤害", en: "Charge Level 1 DMG") },
            CharacterSkillMapItem { index: YunjinDamageEnum::E3 as usize, text: locale!(zh_cn: "二段蓄力伤害", en: "Charge Level 2 DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: YunjinDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: YunjinDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use YunjinDamageEnum::*;
        let ratio = match s {
            Normal1 => YUNJIN_SKILL.normal_dmg1[s1],
            Normal2 => YUNJIN_SKILL.normal_dmg2[s1],
            Normal31 => YUNJIN_SKILL.normal_dmg31[s1],
            Normal32 => YUNJIN_SKILL.normal_dmg32[s1],
            Normal41 => YUNJIN_SKILL.normal_dmg41[s1],
            Normal42 => YUNJIN_SKILL.normal_dmg42[s1],
            Normal5 => YUNJIN_SKILL.normal_dmg5[s1],
            Charged => YUNJIN_SKILL.charged_dmg[s1],
            Plunging1 => YUNJIN_SKILL.plunging_dmg1[s1],
            Plunging2 => YUNJIN_SKILL.plunging_dmg2[s1],
            Plunging3 => YUNJIN_SKILL.plunging_dmg3[s1],
            E1 => YUNJIN_SKILL.elemental_skill_dmg1[s2],
            E2 => YUNJIN_SKILL.elemental_skill_dmg2[s2],
            E3 => YUNJIN_SKILL.elemental_skill_dmg3[s2],
            Q1 => YUNJIN_SKILL.elemental_burst_dmg1[s3]
        };

        let mut builder = D::new();
        let skill_type = s.get_skill_type();
        if skill_type == SkillType::ElementalSkill {
            builder.add_def_ratio("技能倍率", ratio);
        } else {
            builder.add_atk_ratio("技能倍率", ratio);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: YunjinRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            YunjinRoleEnum::Aux => Box::new(YunjinDefaultTargetFunction {
                recharge_demand: 1.4
            })
        }
    }
}
