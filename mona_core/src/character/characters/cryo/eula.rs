use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::EulaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct EulaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg51: [f64; 15],
    pub normal_dmg52: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_def_bonus: [f64; 15],
    pub elemental_skill_res_cryo_minus: [f64; 15],
    pub elemental_skill_res_physical_minus: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_dmg3: [f64; 15]
}

pub const EULA_SKILL: EulaSkillType = EulaSkillType {
    normal_dmg1: [0.8973, 0.9704, 1.0434, 1.1477, 1.2208, 1.3043, 1.419, 1.5338, 1.6486, 1.7738, 1.9172, 2.086, 2.2547, 2.4234, 2.6075],
    normal_dmg2: [0.9355, 1.0117, 1.0878, 1.1966, 1.2727, 1.3598, 1.4794, 1.5991, 1.7187, 1.8493, 1.9988, 2.1747, 2.3506, 2.5265, 2.7184],
    normal_dmg31: [0.568, 0.6142, 0.6605, 0.7265, 0.7727, 0.8256, 0.8982, 0.9709, 1.0435, 1.1228, 1.2136, 1.3204, 1.4272, 1.534, 1.6505],
    normal_dmg32: [0.568, 0.6142, 0.6605, 0.7265, 0.7727, 0.8256, 0.8982, 0.9709, 1.0435, 1.1228, 1.2136, 1.3204, 1.4272, 1.534, 1.6505],
    normal_dmg4: [1.1264, 1.2181, 1.3098, 1.4408, 1.5325, 1.6373, 1.7813, 1.9254, 2.0695, 2.2267, 2.4068, 2.6186, 2.8303, 3.0421, 3.2732],
    normal_dmg51: [0.7183, 0.7768, 0.8353, 0.9188, 0.9773, 1.0441, 1.136, 1.2279, 1.3197, 1.42, 1.5348, 1.6699, 1.8049, 1.94, 2.0874],
    normal_dmg52: [0.7183, 0.7768, 0.8353, 0.9188, 0.9773, 1.0441, 1.136, 1.2279, 1.3197, 1.42, 1.5348, 1.6699, 1.8049, 1.94, 2.0874],
    charged_dmg1: [0.688, 0.744, 0.8, 0.88, 0.936, 1., 1.088, 1.176, 1.264, 1.36, 1.47, 1.5994, 1.7287, 1.8581, 1.9992],
    charged_dmg2: [1.244, 1.3452, 1.4465, 1.5912, 1.6924, 1.8081, 1.9672, 2.1264, 2.2855, 2.4591, 2.6579, 2.8918, 3.1257, 3.3596, 3.6148],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5937, 1.7339, 1.8741, 2.0144, 2.1674],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1867, 3.4671, 3.7475, 4.0279, 4.3338],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9803, 4.3306, 4.6808, 5.0311, 5.4132],
    elemental_skill_dmg1: [1.464, 1.5738, 1.6836, 1.83, 1.9398, 2.0496, 2.196, 2.3424, 2.4888, 2.6352, 2.7816, 2.928, 3.111, 3.294, 3.477],
    elemental_skill_dmg2: [2.456, 2.6402, 2.8244, 3.07, 3.2542, 3.4384, 3.684, 3.9296, 4.1752, 4.4208, 4.6664, 4.912, 5.219, 5.526, 5.833],
    elemental_skill_dmg3: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    elemental_skill_def_bonus: [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
    elemental_skill_res_cryo_minus: [0.16, 0.17, 0.18, 0.19, 0.2, 0.21, 0.22, 0.23, 0.24, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25],
    elemental_skill_res_physical_minus: [0.16, 0.17, 0.18, 0.19, 0.2, 0.21, 0.22, 0.23, 0.24, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25],
    elemental_burst_dmg1: [2.456, 2.6402, 2.8244, 3.07, 3.2542, 3.4384, 3.684, 3.9296, 4.1752, 4.4208, 4.6664, 4.912, 5.219, 5.526, 5.833],
    elemental_burst_dmg2: [3.6705, 3.9692, 4.268, 4.6948, 4.9936, 5.335, 5.8045, 6.274, 6.7434, 7.2556, 7.8425, 8.5326, 9.2227, 9.9129, 10.6657],
    elemental_burst_dmg3: [0.7499, 0.811, 0.872, 0.9592, 1.0202, 1.09, 1.1859, 1.2818, 1.3778, 1.4824, 1.6023, 1.7433, 1.8843, 2.0253, 2.1791]
};

pub const EULA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Eula,
    internal_name: "Eula",
    element: Element::Cryo,
    hp: [1030, 2671, 3554, 5317, 5944, 6839, 7675, 8579, 9207, 10119, 10746, 11699, 12296, 13226],
    atk: [27, 69, 92, 138, 154, 177, 198, 222, 238, 262, 278, 302, 318, 342],
    def: [58, 152, 202, 302, 337, 388, 436, 487, 523, 574, 610, 662, 698, 751],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Claymore,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·西风剑术·宗室",
        en: "Normal Attack: Favonius Bladework - Edel",
    ),
    skill_name2: locale!(
        zh_cn: "冰潮的涡旋",
        en: "Icetide Vortex",
    ),
    skill_name3: locale!(
        zh_cn: "凝浪之光剑",
        en: "Glacial Illumination",
    ),
    name_locale: locale!(
        zh_cn: "优菈",
        en: "Eula",
    )
};

pub struct Eula;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq)]
#[derive(EnumString, EnumCountMacro)]
pub enum EulaDamageEnum {
    Normal1,
    Normal2,
    Normal31,
    Normal32,
    Normal4,
    Normal51,
    Normal52,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    EShatteredLightfall,
    Q1,
    QLightfall
}

impl EulaDamageEnum {
    pub fn get_element(&self) -> Element {
        use EulaDamageEnum::*;
        match *self {
            E1 | E2 | E3 | Q1 => Element::Cryo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use EulaDamageEnum::*;
        match *self {
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 | QLightfall | EShatteredLightfall => SkillType::ElementalBurst,
            _ => SkillType::NormalAttack
        }
    }
}

impl Into<usize> for EulaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum EulaRoleEnum {
    Main
}

impl CharacterTrait for Eula {
    const STATIC_DATA: CharacterStaticData = EULA_STATIC_DATA;
    type SkillType = EulaSkillType;
    const SKILL: Self::SkillType = EULA_SKILL;
    type DamageEnumType = EulaDamageEnum;
    type RoleEnum = EulaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: EulaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: EulaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: EulaDamageEnum::Normal31 as usize, text: hit_n_dmg!(3, 1) },
            CharacterSkillMapItem { index: EulaDamageEnum::Normal32 as usize, text: hit_n_dmg!(3, 2) },
            CharacterSkillMapItem { index: EulaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: EulaDamageEnum::Normal51 as usize, text: hit_n_dmg!(5, 1) },
            CharacterSkillMapItem { index: EulaDamageEnum::Normal52 as usize, text: hit_n_dmg!(5, 2) },
            CharacterSkillMapItem { index: EulaDamageEnum::Charged1 as usize, text: charged_dmg!("loop1") },
            CharacterSkillMapItem { index: EulaDamageEnum::Charged2 as usize, text: charged_dmg!("loop2") },
            CharacterSkillMapItem { index: EulaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: EulaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: EulaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: EulaDamageEnum::E1 as usize, text: locale!(zh_cn: "点按伤害", en: "Tapping DMG") },
            CharacterSkillMapItem { index: EulaDamageEnum::E2 as usize, text: locale!(zh_cn: "长按伤害", en: "Hold DMG") },
            CharacterSkillMapItem { index: EulaDamageEnum::E3 as usize, text: locale!(zh_cn: "冰涡之剑伤害", en: "Icewhirl Brand DMG") },
            CharacterSkillMapItem { index: EulaDamageEnum::EShatteredLightfall as usize, text: locale!(zh_cn: "残缺光降之剑", en: "Shattered Lightfall Sword DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: EulaDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: EulaDamageEnum::QLightfall as usize, text: locale!(zh_cn: "光降之剑", en: "Lightfall Sword DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "lightfall_stack",
            title: locale!(
                zh_cn: "光降之剑能量层数",
                en: "Lightfall Sword Energy Stack",
            ),
            config: ItemConfigType::Int { min: 0, max: 30, default: 0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: EulaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use EulaDamageEnum::*;
        let ratio = match s {
            Normal1 => EULA_SKILL.normal_dmg1[s1],
            Normal2 => EULA_SKILL.normal_dmg2[s1],
            Normal31 => EULA_SKILL.normal_dmg31[s1],
            Normal32 => EULA_SKILL.normal_dmg32[s1],
            Normal4 => EULA_SKILL.normal_dmg4[s1],
            Normal51 => EULA_SKILL.normal_dmg51[s1],
            Normal52 => EULA_SKILL.normal_dmg52[s1],
            Charged1 => EULA_SKILL.charged_dmg1[s1],
            Charged2 => EULA_SKILL.charged_dmg2[s1],
            Plunging1 => EULA_SKILL.plunging_dmg1[s1],
            Plunging2 => EULA_SKILL.plunging_dmg2[s1],
            Plunging3 => EULA_SKILL.plunging_dmg3[s1],
            E1 => EULA_SKILL.elemental_skill_dmg1[s2],
            E2 => EULA_SKILL.elemental_skill_dmg2[s2],
            E3 => EULA_SKILL.elemental_skill_dmg3[s2],
            EShatteredLightfall => EULA_SKILL.elemental_burst_dmg2[s3] / 2.0,
            Q1 => EULA_SKILL.elemental_burst_dmg1[s3],
            QLightfall => EULA_SKILL.elemental_burst_dmg2[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        if s == QLightfall {
            let stack = match *config {
                CharacterSkillConfig::Eula { lightfall_stack } => lightfall_stack,
                _ => 0
            };
            builder.add_atk_ratio("光降之剑能量", (stack as f64) * EULA_SKILL.elemental_burst_dmg3[s3]);
        }

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

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: EulaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            EulaRoleEnum::Main => Box::new(EulaDefaultTargetFunction {
                is_c2: c.constellation >= 2
            })
        }
    }
}
