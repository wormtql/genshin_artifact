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
use crate::target_functions::target_functions::XianglingDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct XianglingSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_dmg3: [f64; 15],
    pub elemental_burst_dmg4: [f64; 15],
}

pub const XIANGLING_SKILL: XianglingSkillType = XianglingSkillType {
    normal_dmg1: [0.4205, 0.4548, 0.489, 0.5379, 0.5721, 0.6113, 0.665, 0.7188, 0.7726, 0.8313, 0.8985, 0.9776, 1.0567, 1.1358, 1.222],
    normal_dmg2: [0.4214, 0.4557, 0.49, 0.539, 0.5733, 0.6125, 0.6664, 0.7203, 0.7742, 0.833, 0.9004, 0.9796, 1.0588, 1.1381, 1.2245],
    normal_dmg31: [0.2606, 0.2818, 0.303, 0.3333, 0.3545, 0.3787, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568, 0.6058, 0.6548, 0.7037, 0.7572],
    normal_dmg32: [0.2606, 0.2818, 0.303, 0.3333, 0.3545, 0.3787, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568, 0.6058, 0.6548, 0.7037, 0.7572],
    normal_dmg4: [0.141, 0.1525, 0.164, 0.1804, 0.1919, 0.205, 0.223, 0.2411, 0.2591, 0.2788, 0.3014, 0.3279, 0.3544, 0.3809, 0.4098],
    normal_dmg5: [0.7104, 0.7682, 0.826, 0.9086, 0.9664, 1.0325, 1.1234, 1.2142, 1.3051, 1.4042, 1.5178, 1.6513, 1.7849, 1.9185, 2.0642],
    charged_dmg1: [1.2169, 1.316, 1.415, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.6001, 2.8289, 3.0577, 3.2865, 3.5361],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.1128, 1.1963, 1.2797, 1.391, 1.4745, 1.5579, 1.6692, 1.7805, 1.8918, 2.003, 2.1143, 2.2256, 2.3647, 2.5038, 2.6429],
    elemental_burst_dmg1: [0.72, 0.774, 0.828, 0.9, 0.954, 1.008, 1.08, 1.152, 1.224, 1.296, 1.368, 1.44, 1.53, 1.62, 1.71],
    elemental_burst_dmg2: [0.88, 0.946, 1.012, 1.1, 1.166, 1.232, 1.32, 1.408, 1.496, 1.584, 1.672, 1.76, 1.87, 1.98, 2.09],
    elemental_burst_dmg3: [1.096, 1.1782, 1.2604, 1.37, 1.4522, 1.5344, 1.644, 1.7536, 1.8632, 1.9728, 2.0824, 2.192, 2.329, 2.466, 2.603],
    elemental_burst_dmg4: [1.12, 1.204, 1.288, 1.4, 1.484, 1.568, 1.68, 1.792, 1.904, 2.016, 2.128, 2.24, 2.38, 2.52, 2.66]
};

pub const XIANGLING_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Xiangling,
    internal_name: "Xiangling",
    element: Element::Pyro,
    hp: [912, 2342, 3024, 4529, 5013, 5766, 6411, 7164, 7648, 8401, 8885, 9638, 10122, 10875],
    atk: [19, 48, 63, 94, 104, 119, 133, 148, 158, 174, 184, 200, 210, 225],
    def: [56, 144, 186, 279, 308, 355, 394, 441, 470, 517, 546, 593, 623, 669],
    sub_stat: CharacterSubStatFamily::ElementalMastery96,
    weapon_type: WeaponType::Polearm,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·白案功夫",
        en: "Normal Attack: Dough-Fu",
    ),
    skill_name2: locale!(
        zh_cn: "锅巴出击",
        en: "Guoba Attack",
    ),
    skill_name3: locale!(
        zh_cn: "旋火轮",
        en: "Pyronado",
    ),
    name_locale: locale!(
        zh_cn: "香菱",
        en: "Xiangling",
    )
};

pub struct Xiangling;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum XianglingDamageEnum {
    Normal1,
    Normal2,
    Normal31,
    Normal32,
    Normal3,
    Normal4,
    Normal4Times4,
    Normal5,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
    Q2,
    Q3,
    Q4
}

impl XianglingDamageEnum {
    pub fn get_element(&self) -> Element {
        use XianglingDamageEnum::*;
        match *self {
            E1 | Q1 | Q2 | Q3 | Q4 => Element::Pyro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use XianglingDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal3 | Normal4 | Normal4Times4 | Normal5 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 | Q3 | Q4 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for XianglingDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum XianglingRoleEnum {
    Sub
}

impl CharacterTrait for Xiangling {
    const STATIC_DATA: CharacterStaticData = XIANGLING_STATIC_DATA;
    type SkillType = XianglingSkillType;
    const SKILL: Self::SkillType = XIANGLING_SKILL;
    type DamageEnumType = XianglingDamageEnum;
    type RoleEnum = XianglingRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: XianglingDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: XianglingDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: XianglingDamageEnum::Normal31 as usize, text: hit_n_dmg!(3, 1) },
            CharacterSkillMapItem { index: XianglingDamageEnum::Normal32 as usize, text: hit_n_dmg!(3, 2) },
            CharacterSkillMapItem { index: XianglingDamageEnum::Normal4 as usize, text: locale!(zh_cn: "四段伤害/4", en: "4-Hit DMG/4") },
            CharacterSkillMapItem { index: XianglingDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: XianglingDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: XianglingDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: XianglingDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: XianglingDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: XianglingDamageEnum::E1 as usize, text: locale!(zh_cn: "喷火伤害", en: "Flame DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: XianglingDamageEnum::Q1 as usize, text: locale!(zh_cn: "一段挥舞伤害", en: "1-Hit Swing DMG") },
            CharacterSkillMapItem { index: XianglingDamageEnum::Q2 as usize, text: locale!(zh_cn: "二段挥舞伤害", en: "2-Hit Swing DMG") },
            CharacterSkillMapItem { index: XianglingDamageEnum::Q3 as usize, text: locale!(zh_cn: "三段挥舞伤害", en: "3-Hit Swing DMG") },
            CharacterSkillMapItem { index: XianglingDamageEnum::Q4 as usize, text: locale!(zh_cn: "旋火轮伤害", en: "Pyronado DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: XianglingDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use XianglingDamageEnum::*;
        let ratio = match s {
            Normal1 => XIANGLING_SKILL.normal_dmg1[s1],
            Normal2 => XIANGLING_SKILL.normal_dmg2[s1],
            Normal31 => XIANGLING_SKILL.normal_dmg31[s1],
            Normal32 => XIANGLING_SKILL.normal_dmg32[s1],
            Normal3 => XIANGLING_SKILL.normal_dmg31[s1] + XIANGLING_SKILL.normal_dmg32[s1],
            Normal4 => XIANGLING_SKILL.normal_dmg4[s1],
            Normal4Times4 => XIANGLING_SKILL.normal_dmg4[s1] * 4.0,
            Normal5 => XIANGLING_SKILL.normal_dmg5[s1],
            Charged => XIANGLING_SKILL.charged_dmg1[s1],
            Plunging1 => XIANGLING_SKILL.plunging_dmg1[s1],
            Plunging2 => XIANGLING_SKILL.plunging_dmg2[s1],
            Plunging3 => XIANGLING_SKILL.plunging_dmg3[s1],
            E1 => XIANGLING_SKILL.elemental_skill_dmg1[s2],
            Q1 => XIANGLING_SKILL.elemental_burst_dmg1[s3],
            Q2 => XIANGLING_SKILL.elemental_burst_dmg2[s3],
            Q3 => XIANGLING_SKILL.elemental_burst_dmg3[s3],
            Q4 => XIANGLING_SKILL.elemental_burst_dmg4[s3],
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
        let role: XianglingRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            XianglingRoleEnum::Sub => Box::new(XianglingDefaultTargetFunction {
                recharge_demand: 1.8,
                melt_rate: 0.0,
                vaporize_rate: 0.0,
                overload_rate: 0.0,
            })
        }
    }
}
