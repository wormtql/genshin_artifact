use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::JeanDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{hit_n_dmg, locale, plunging_dmg};

pub struct JeanSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_heal1: [f64; 15],
    pub elemental_burst_heal1_fixed: [f64; 15],
    pub elemental_burst_heal2: [f64; 15],
    pub elemental_burst_heal2_fixed: [f64; 15],
}

pub const JEAN_SKILL: JeanSkillType = JeanSkillType {
    normal_dmg1: [0.4833, 0.5227, 0.562, 0.6182, 0.6575, 0.7025, 0.7643, 0.8261, 0.888, 0.9554, 1.0327, 1.1236, 1.2144, 1.3053, 1.4044],
    normal_dmg2: [0.4558, 0.4929, 0.53, 0.583, 0.6201, 0.6625, 0.7208, 0.7791, 0.8374, 0.901, 0.9739, 1.0596, 1.1453, 1.231, 1.3245],
    normal_dmg3: [0.6029, 0.6519, 0.701, 0.7711, 0.8202, 0.8763, 0.9534, 1.0305, 1.1076, 1.1917, 1.2881, 1.4014, 1.5148, 1.6281, 1.7518],
    normal_dmg4: [0.6588, 0.7124, 0.766, 0.8426, 0.8962, 0.9575, 1.0418, 1.126, 1.2103, 1.3022, 1.4075, 1.5314, 1.6552, 1.7791, 1.9142],
    normal_dmg5: [0.7921, 0.8565, 0.921, 1.0131, 1.0776, 1.1513, 1.2526, 1.3539, 1.4552, 1.5657, 1.6923, 1.8413, 1.9902, 2.1391, 2.3016],
    charged_dmg1: [1.6202, 1.7521, 1.884, 2.0724, 2.2043, 2.355, 2.5622, 2.7695, 2.9767, 3.2028, 3.4619, 3.7665, 4.0711, 4.3758, 4.7081],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [2.92, 3.139, 3.358, 3.65, 3.869, 4.088, 4.38, 4.672, 4.964, 5.256, 5.548, 5.84, 6.205, 6.57, 6.935],
    elemental_burst_dmg1: [4.248, 4.5666, 4.8852, 5.31, 5.6286, 5.9472, 6.372, 6.7968, 7.2216, 7.6464, 8.0712, 8.496, 9.027, 9.558, 10.089],
    elemental_burst_dmg2: [0.784, 0.8428, 0.9016, 0.98, 1.0388, 1.0976, 1.176, 1.2544, 1.3328, 1.4112, 1.4896, 1.568, 1.666, 1.764, 1.862],
    elemental_burst_heal1: [2.512, 2.7004, 2.8888, 3.14, 3.3284, 3.5168, 3.768, 4.0192, 4.2704, 4.5216, 4.7728, 5.024, 5.338, 5.652, 5.966],
    elemental_burst_heal1_fixed: [1540.0, 1694.0, 1861.0, 2041.0, 2234.0, 2439.0, 2657.0, 2888.0, 3132.0, 3389.0, 3659.0, 3941.0, 4236.0, 4544.0, 4865.0],
    elemental_burst_heal2: [0.2512, 0.27, 0.2889, 0.314, 0.3328, 0.3517, 0.3768, 0.4019, 0.427, 0.4522, 0.4773, 0.5024, 0.5338, 0.5652, 0.5966],
    elemental_burst_heal2_fixed: [154.0, 169.0, 186.0, 204.0, 223.0, 244.0, 266.0, 289.0, 313.0, 339.0, 366.0, 394.0, 424.0, 454.0, 487.0],
};

pub const JEAN_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Jean,
    internal_name: "Qin",
    element: Element::Anemo,
    hp: [1144, 2967, 3948, 5908, 6605, 7599, 8528, 9533, 10230, 11243, 11940, 12965, 13662, 14695],
    atk: [19, 48, 64, 96, 108, 124, 139, 155, 166, 183, 194, 211, 222, 239],
    def: [60, 155, 206, 309, 345, 397, 446, 499, 535, 588, 624, 678, 715, 769],
    sub_stat: CharacterSubStatFamily::HealingBonus222,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·西风剑术",
        en: "Normal Attack: Favonius Bladework",
    ),
    skill_name2: locale!(
        zh_cn: "风压剑",
        en: "Gale Blade",
    ),
    skill_name3: locale!(
        zh_cn: "蒲公英之风",
        en: "Dandelion Breeze",
    ),
    name_locale: locale!(
        zh_cn: "琴",
        en: "Jean",
    )
};

pub struct Jean;

#[derive(Copy, Clone, FromPrimitive)]
#[derive(EnumString, EnumCountMacro)]
pub enum JeanDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
    Q2,
    QHeal1,
    QHeal2
}

impl JeanDamageEnum {
    pub fn is_heal(&self) -> bool {
        use JeanDamageEnum::*;
        match *self {
            QHeal1 | QHeal2 => true,
            _ => false
        }
    }

    pub fn get_element(&self) -> Element {
        use JeanDamageEnum::*;
        match *self {
            E1 | Q1 | Q2 => Element::Anemo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use JeanDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 | QHeal1 | QHeal2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for JeanDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum JeanRoleEnum {
    Default
}

impl CharacterTrait for Jean {
    const STATIC_DATA: CharacterStaticData = JEAN_STATIC_DATA;
    type SkillType = JeanSkillType;
    const SKILL: Self::SkillType = JEAN_SKILL;
    type DamageEnumType = JeanDamageEnum;
    type RoleEnum = JeanRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: JeanDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: JeanDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: JeanDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: JeanDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: JeanDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: JeanDamageEnum::Charged as usize, text: locale!(zh_cn: "重击伤害", en: "Charged Attack DMG") },
            CharacterSkillMapItem { index: JeanDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: JeanDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: JeanDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: JeanDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: JeanDamageEnum::Q1 as usize, text: locale!(zh_cn: "爆发伤害", en: "Burst DMG") },
            CharacterSkillMapItem { index: JeanDamageEnum::Q2 as usize, text: locale!(zh_cn: "出入领域伤害", en: "Field Entering/Exiting DMG") },
            CharacterSkillMapItem { index: JeanDamageEnum::QHeal1 as usize, text: locale!(zh_cn: "领域发动治疗量", en: "Field Activation Healing") },
            CharacterSkillMapItem { index: JeanDamageEnum::QHeal2 as usize, text: locale!(zh_cn: "持续治疗", en: "Continuous Regeneration") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: JeanDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use JeanDamageEnum::*;
        if s.is_heal() {
            let ratio = match s {
                QHeal1 => JEAN_SKILL.elemental_burst_heal1[s3],
                QHeal2 => JEAN_SKILL.elemental_burst_heal2[s3],
                _ => 0.0
            };
            let fixed = match s {
                QHeal1 => JEAN_SKILL.elemental_burst_heal1_fixed[s3],
                QHeal2 => JEAN_SKILL.elemental_burst_heal2_fixed[s3],
                _ => 0.0
            };
            let mut builder = D::new();
            builder.add_atk_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => JEAN_SKILL.normal_dmg1[s1],
                Normal2 => JEAN_SKILL.normal_dmg2[s1],
                Normal3 => JEAN_SKILL.normal_dmg3[s1],
                Normal4 => JEAN_SKILL.normal_dmg4[s1],
                Normal5 => JEAN_SKILL.normal_dmg5[s1],
                Charged => JEAN_SKILL.charged_dmg1[s1],
                Plunging1 => JEAN_SKILL.plunging_dmg1[s1],
                Plunging2 => JEAN_SKILL.plunging_dmg2[s1],
                Plunging3 => JEAN_SKILL.plunging_dmg3[s1],
                E1 => JEAN_SKILL.elemental_skill_dmg1[s2],
                Q1 => JEAN_SKILL.elemental_burst_dmg1[s3],
                Q2 => JEAN_SKILL.elemental_burst_dmg2[s3],
                _ => 0.0
            };
            let mut builder = D::new();
            builder.add_atk_ratio("技能倍率", ratio);
            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        }
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: JeanRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            JeanRoleEnum::Default => Box::new(JeanDefaultTargetFunction {
                recharge_demand: 1.6,
                damage_weight: 0.8,
            })
        }
    }
}
