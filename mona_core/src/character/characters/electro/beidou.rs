use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::BeidouDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct BeidouSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_shield1: [f64; 15],
    pub elemental_skill_shield1_fixed: [f64; 15],
    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg_delta: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_res_minus: [f64; 15]
}

pub const BEIDOU_SKILL: BeidouSkillType = BeidouSkillType {
    normal_dmg1: [0.7112, 0.7691, 0.827, 0.9097, 0.9676, 1.0338, 1.1247, 1.2157, 1.3067, 1.4059, 1.5196, 1.6533, 1.7871, 1.9208, 2.0667],
    normal_dmg2: [0.7086, 0.7663, 0.824, 0.9064, 0.9641, 1.03, 1.1206, 1.2113, 1.3019, 1.4008, 1.5141, 1.6473, 1.7806, 1.9138, 2.0592],
    normal_dmg3: [0.8832, 0.9551, 1.027, 1.1297, 1.2016, 1.2838, 1.3967, 1.5097, 1.6227, 1.7459, 1.8871, 2.0532, 2.2192, 2.3853, 2.5665],
    normal_dmg4: [0.8652, 0.9356, 1.006, 1.1066, 1.177, 1.2575, 1.3682, 1.4788, 1.5895, 1.7102, 1.8485, 2.0112, 2.1739, 2.3365, 2.514],
    normal_dmg5: [1.1214, 1.2127, 1.304, 1.4344, 1.5257, 1.63, 1.7734, 1.9169, 2.0603, 2.2168, 2.3961, 2.607, 2.8178, 3.0287, 3.2587],
    charged_dmg1: [0.5624, 0.6082, 0.654, 0.7194, 0.7652, 0.8175, 0.8894, 0.9614, 1.0333, 1.1118, 1.2017, 1.3075, 1.4132, 1.519, 1.6343],
    charged_dmg2: [1.0182, 1.1011, 1.184, 1.3024, 1.3853, 1.48, 1.6102, 1.7405, 1.8707, 2.0128, 2.1756, 2.3671, 2.5585, 2.75, 2.9588],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    elemental_skill_shield1: [0.144, 0.1548, 0.1656, 0.18, 0.1908, 0.2016, 0.216, 0.2304, 0.2448, 0.2592, 0.2736, 0.288, 0.306, 0.324, 0.342],
    elemental_skill_shield1_fixed: [1386.0, 1525.0, 1675.0, 1837.0, 2010.0, 2195.0, 2392.0, 2600.0, 2819.0, 3050.0, 3293.0, 3547.0, 3813.0, 4090.0, 4379.0],
    elemental_skill_dmg1: [1.216, 1.3072, 1.3984, 1.52, 1.6112, 1.7024, 1.824, 1.9456, 2.0672, 2.1888, 2.3104, 2.432, 2.584, 2.736, 2.888],
    elemental_skill_dmg_delta: [1.6, 1.72, 1.84, 2., 2.12, 2.24, 2.4, 2.56, 2.72, 2.88, 3.04, 3.2, 3.4, 3.6, 3.8],
    elemental_burst_dmg1: [1.216, 1.3072, 1.3984, 1.52, 1.6112, 1.7024, 1.824, 1.9456, 2.0672, 2.1888, 2.3104, 2.432, 2.584, 2.736, 2.888],
    elemental_burst_dmg2: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    elemental_burst_res_minus: [0.2, 0.21, 0.22, 0.24, 0.25, 0.26, 0.28, 0.3, 0.32, 0.34, 0.35, 0.36, 0.37, 0.38, 0.39]
};

const BEIDOU_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Beidou,
    internal_name: "Beidou",
    element: Element::Electro,
    hp: [1094, 2811, 3628, 5435, 6015, 6919, 7694, 8597, 9178, 10081, 10662, 11565, 12146, 13050],
    atk: [19, 48, 63, 94, 104, 119, 133, 148, 158, 174, 184, 200, 210, 225],
    def: [54, 140, 180, 270, 299, 344, 382, 427, 456, 501, 530, 575, 603, 648],
    sub_stat: CharacterSubStatFamily::Bonus240(StatName::ElectroBonus),
    weapon_type: WeaponType::Claymore,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·征涛",
        en: "Normal Attack: Oceanborne",
    ),
    skill_name2: locale!(
        zh_cn: "捉浪",
        en: "Tidecaller",
    ),
    skill_name3: locale!(
        zh_cn: "斫雷",
        en: "Stormbreaker",
    ),
    name_locale: locale!(
        zh_cn: "北斗",
        en: "Beidou",
    )
};

pub struct Beidou;

#[derive(Copy, Clone)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum BeidouDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    Q1,
    Q2
}

impl Into<usize> for BeidouDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl BeidouDamageEnum {
    pub fn get_element(&self) -> Element {
        use BeidouDamageEnum::*;
        match *self {
            E1 | E2 | E3 | Q1 | Q2 => Element::Electro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use BeidouDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum BeidouRoleEnum {
    EMain,
}

impl CharacterTrait for Beidou {
    const STATIC_DATA: CharacterStaticData = BEIDOU_STATIC_DATA;
    type SkillType = BeidouSkillType;
    const SKILL: Self::SkillType = BEIDOU_SKILL;
    type DamageEnumType = BeidouDamageEnum;
    type RoleEnum = BeidouRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: BeidouDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: BeidouDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: BeidouDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: BeidouDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: BeidouDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: BeidouDamageEnum::Charged1 as usize, text: charged_dmg!("loop1") },
            CharacterSkillMapItem { index: BeidouDamageEnum::Charged2 as usize, text: charged_dmg!("loop2") },
            CharacterSkillMapItem { index: BeidouDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: BeidouDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: BeidouDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: BeidouDamageEnum::E1 as usize, text: locale!(zh_cn: "基础伤害", en: "Base DMG") },
            CharacterSkillMapItem { index: BeidouDamageEnum::E2 as usize, text: locale!(zh_cn: "一层伤害", en: "1-Stack DMG") },
            CharacterSkillMapItem { index: BeidouDamageEnum::E3 as usize, text: locale!(zh_cn: "二层伤害", en: "2-Stack DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: BeidouDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: BeidouDamageEnum::Q2 as usize, text: locale!(zh_cn: "闪电伤害", en: "Lightning DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: BeidouDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use BeidouDamageEnum::*;
        let ratio = match s {
            Normal1 => BEIDOU_SKILL.normal_dmg1[s1],
            Normal2 => BEIDOU_SKILL.normal_dmg2[s1],
            Normal3 => BEIDOU_SKILL.normal_dmg3[s1],
            Normal4 => BEIDOU_SKILL.normal_dmg4[s1],
            Normal5 => BEIDOU_SKILL.normal_dmg5[s1],
            Charged1 => BEIDOU_SKILL.charged_dmg1[s1],
            Charged2 => BEIDOU_SKILL.charged_dmg2[s1],
            Plunging1 => BEIDOU_SKILL.plunging_dmg1[s1],
            Plunging2 => BEIDOU_SKILL.plunging_dmg2[s1],
            Plunging3 => BEIDOU_SKILL.plunging_dmg3[s1],
            E1 => BEIDOU_SKILL.elemental_skill_dmg1[s2],
            E2 => BEIDOU_SKILL.elemental_skill_dmg1[s2] + BEIDOU_SKILL.elemental_skill_dmg_delta[s2],
            E3 => BEIDOU_SKILL.elemental_skill_dmg1[s2] + 2.0 * BEIDOU_SKILL.elemental_skill_dmg_delta[s2],
            Q1 => BEIDOU_SKILL.elemental_burst_dmg1[s3],
            Q2 => BEIDOU_SKILL.elemental_burst_dmg2[s3]
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

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: BeidouRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            BeidouRoleEnum::EMain => Box::new(BeidouDefaultTargetFunction)
        }
    }
}
