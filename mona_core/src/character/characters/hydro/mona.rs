use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::MonaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct MonaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_bonus: [f64; 15],
}

pub const MONA_SKILL: MonaSkillType = MonaSkillType {
    normal_dmg1: [0.376, 0.4042, 0.4324, 0.47, 0.4982, 0.5264, 0.564, 0.6016, 0.6392, 0.6768, 0.7144, 0.752, 0.799, 0.846, 0.893],
    normal_dmg2: [0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765, 0.81, 0.855],
    normal_dmg3: [0.448, 0.4816, 0.5152, 0.56, 0.5936, 0.6272, 0.672, 0.7168, 0.7616, 0.8064, 0.8512, 0.896, 0.952, 1.008, 1.064],
    normal_dmg4: [0.5616, 0.6037, 0.6458, 0.702, 0.7441, 0.7862, 0.8424, 0.8986, 0.9547, 1.0109, 1.067, 1.1232, 1.1934, 1.2636, 1.3338],
    charged_dmg1: [1.4972, 1.6095, 1.7218, 1.8715, 1.9838, 2.0961, 2.2458, 2.3955, 2.5452, 2.695, 2.8507, 3.0543, 3.2579, 3.4615, 3.6651],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.32, 0.344, 0.368, 0.4, 0.424, 0.448, 0.48, 0.512, 0.544, 0.576, 0.608, 0.64, 0.68, 0.72, 0.76],
    elemental_skill_dmg2: [1.328, 1.4276, 1.5272, 1.66, 1.7596, 1.8592, 1.992, 2.1248, 2.2576, 2.3904, 2.5232, 2.656, 2.822, 2.988, 3.154],
    elemental_burst_dmg1: [4.424, 4.7558, 5.0876, 5.53, 5.8618, 6.1936, 6.636, 7.0784, 7.5208, 7.9632, 8.4056, 8.848, 9.401, 9.954, 10.507],
    elemental_burst_bonus: [0.42, 0.44, 0.46, 0.48, 0.5, 0.52, 0.54, 0.56, 0.58, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6],
};

pub const MONA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Mona,
    internal_name: "Mona",
    element: Element::Hydro,
    hp: [810, 2102, 2797, 4185, 4678, 5383, 6041, 6752, 7246, 7964, 8458, 9184, 9677, 10409],
    atk: [22, 58, 77, 115, 129, 148, 167, 186, 200, 220, 233, 253, 267, 287],
    def: [51, 132, 176, 263, 294, 338, 379, 424, 455, 500, 531, 576, 607, 653],
    sub_stat: CharacterSubStatFamily::Recharge320,
    weapon_type: WeaponType::Catalyst,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·因果点破",
        en: "Normal Attack: Ripple of Fate",
    ),
    skill_name2: locale!(
        zh_cn: "水中幻愿",
        en: "Mirror Reflection of Doom",
    ),
    skill_name3: locale!(
        zh_cn: "星命定轨",
        en: "Stellaris Phantasm",
    ),
    name_locale: locale!(
        zh_cn: "莫娜",
        en: "Mona",
    )
};

pub struct MonaEffect {
    has_talent2: bool
}

impl MonaEffect {
    pub fn new(common_data: &CharacterCommonData) -> MonaEffect {
        MonaEffect {
            has_talent2: common_data.has_talent2,
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for MonaEffect {
    fn change_attribute(&self, attribute: &mut T) {
        if self.has_talent2 {
            attribute.add_edge1(
                AttributeName::Recharge,
                AttributeName::BonusHydro,
                Box::new(|recharge, _| recharge * 0.2),
                Box::new(|grad, _x1, _x2| (grad * 0.2, 0.0)),
                "莫娜天赋：「托付于命运吧!」"
            );
        }
    }
}

pub struct Mona;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum MonaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    Q1
}

impl MonaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use MonaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for MonaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum MonaRoleEnum {
    General,
    Aux, // 增伤辅助
    Nu, // 核弹流
}

impl CharacterTrait for Mona {
    const STATIC_DATA: CharacterStaticData = MONA_STATIC_DATA;
    type SkillType = MonaSkillType;
    const SKILL: Self::SkillType = MONA_SKILL;
    type DamageEnumType = MonaDamageEnum;
    type RoleEnum = MonaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: MonaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: MonaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: MonaDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: MonaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: MonaDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: MonaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: MonaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: MonaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: MonaDamageEnum::E1 as usize, text: locale!(zh_cn: "持续伤害", en: "DoT") },
            CharacterSkillMapItem { index: MonaDamageEnum::E2 as usize, text: locale!(zh_cn: "爆裂伤害", en: "Explosion DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: MonaDamageEnum::Q1 as usize, text: locale!(zh_cn: "泡影破裂伤害", en: "Illusory Bubble Explosion DMG") }
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: MonaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use MonaDamageEnum::*;
        let ratio = match s {
            Normal1 => MONA_SKILL.normal_dmg1[s1],
            Normal2 => MONA_SKILL.normal_dmg2[s1],
            Normal3 => MONA_SKILL.normal_dmg3[s1],
            Normal4 => MONA_SKILL.normal_dmg4[s1],
            Charged => MONA_SKILL.charged_dmg1[s1],
            Plunging1 => MONA_SKILL.plunging_dmg1[s1],
            Plunging2 => MONA_SKILL.plunging_dmg2[s1],
            Plunging3 => MONA_SKILL.plunging_dmg3[s1],
            E1 => MONA_SKILL.elemental_skill_dmg1[s2],
            E2 => MONA_SKILL.elemental_skill_dmg2[s2],
            Q1 => MONA_SKILL.elemental_burst_dmg1[s3]
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Hydro,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(MonaEffect::new(common_data)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        // let role: MonaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        // match role {
        //     MonaRoleEnum::General => Box::new(MonaDefaultTargetFunction {
        //         recharge_demand: 1.4
        //     }),
        //     _ =>
        // }
        unimplemented!()
    }
}
