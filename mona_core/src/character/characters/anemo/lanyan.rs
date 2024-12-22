use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::anemo::chasca::ChascaDamageEnum;
use crate::character::characters::anemo::chasca::ChascaDamageEnum::{Plunging1, Plunging2, Plunging3};
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

pub struct LanyanSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg21: [f64; 15],
    pub normal_dmg22: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],
    pub e_shield: [f64; 15],
    pub e_shield_fixed: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const LANYAN_SKILL: LanyanSkillType = LanyanSkillType {
    normal_dmg1: [0.4144, 0.4455, 0.4766, 0.518, 0.5491, 0.5802, 0.6216, 0.663, 0.7045, 0.7459, 0.7874, 0.8288, 0.8806, 0.9324, 0.9842],
    normal_dmg21: [0.2041, 0.2194, 0.2347, 0.2552, 0.2705, 0.2858, 0.3062, 0.3266, 0.347, 0.3674, 0.3878, 0.4082, 0.4338, 0.4593, 0.4848],
    normal_dmg22: [0.2495, 0.2682, 0.2869, 0.3119, 0.3306, 0.3493, 0.3742, 0.3992, 0.4241, 0.4491, 0.474, 0.499, 0.5301, 0.5613, 0.5925],
    normal_dmg3: [0.2692, 0.2894, 0.3096, 0.3365, 0.3567, 0.3769, 0.4038, 0.4307, 0.4576, 0.4846, 0.5115, 0.5384, 0.5721, 0.6057, 0.6394],
    normal_dmg4: [0.6456, 0.694, 0.7424, 0.807, 0.8554, 0.9038, 0.9684, 1.033, 1.0975, 1.1621, 1.2266, 1.2912, 1.3719, 1.4526, 1.5333],
    charged_dmg: [0.3784, 0.4068, 0.4352, 0.473, 0.5014, 0.5298, 0.5676, 0.6054, 0.6433, 0.6811, 0.719, 0.7568, 0.8041, 0.8514, 0.8987],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg: [0.9626, 1.0348, 1.1069, 1.2032, 1.2754, 1.3476, 1.4438, 1.5401, 1.6364, 1.7326, 1.8289, 1.9251, 2.0454, 2.1658, 2.2861],
    e_shield: [2.7648, 2.9722, 3.1795, 3.456, 3.6634, 3.8707, 4.1472, 4.4237, 4.7002, 4.9766, 5.2531, 5.5296, 5.8752, 6.2208, 6.5664],
    e_shield_fixed: [1155.56, 1271.14, 1396.34, 1531.17, 1675.64, 1829.73, 1993.46, 2166.82, 2349.81, 2542.43, 2744.68, 2956.57, 3178.08, 3409.22, 3650.0],
    q_dmg1: [2.4106, 2.5914, 2.7722, 3.0133, 3.1941, 3.3749, 3.616, 3.857, 4.0981, 4.3392, 4.5802, 4.8213, 5.1226, 5.4239, 5.7253],
};

damage_enum!(
    LanyanDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E1Pyro
    E1Hydro
    E1Electro
    E1Cryo
    EShield
    Q1
);

impl LanyanDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use LanyanDamageEnum::*;

        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | EShield | E1Pyro | E1Cryo | E1Electro | E1Hydro => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self) -> Element {
        use LanyanDamageEnum::*;

        match *self {
            E1Pyro => Element::Pyro,
            E1Cryo => Element::Cryo,
            E1Hydro => Element::Hydro,
            E1Electro => Element::Electro,
            _ => Element::Anemo
        }
    }
}

pub struct Lanyan;

impl CharacterTrait for Lanyan {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Lanyan,
        internal_name: "Lanyan",
        name_locale: locale!(
            zh_cn: "蓝砚",
            en: "Lan Yan"
        ),
        element: Element::Anemo,
        hp: [775, 1991, 2570, 3850, 4261, 4901, 5450, 6090, 6501, 7141, 7552, 8192, 8604, 9244],
        atk: [21, 54, 70, 104, 116, 133, 148, 165, 176, 194, 205, 222, 233, 251],
        def: [49, 125, 161, 242, 267, 308, 342, 382, 408, 448, 474, 514, 540, 580],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Catalyst,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·玄鸾画水",
            en: "Normal Attack: Black Pheasant Strides on Water"
        ),
        skill_name2: locale!(
            zh_cn: "凤缕随翦舞",
            en: "Swallow-Wisp Pinion Dance"
        ),
        skill_name3: locale!(
            zh_cn: "鹍弦踏月出",
            en: "Lustrous Moonrise"
        ),
    };
    type SkillType = LanyanSkillType;
    const SKILL: Self::SkillType = LANYAN_SKILL;
    type DamageEnumType = LanyanDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LanyanDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            LanyanDamageEnum
            E1 locale!(zh_cn: "翦月环伤害", en: "Feathermoon Ring DMG")
            EShield locale!(zh_cn: "护盾吸收量", en: "Shield DMG Absorption")
        ),
        skill3: skill_map!(
            LanyanDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LanyanDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use LanyanDamageEnum::*;

        let mut builder = D::new();

        if s == EShield {
            let ratio = LANYAN_SKILL.e_shield[s2];
            let fixed = LANYAN_SKILL.e_shield_fixed[s2];

            builder.add_atk_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);

            builder.shield(&context.attribute, Element::Anemo)
        } else {
            let ratio = match s {
                Normal1 => LANYAN_SKILL.normal_dmg1[s1],
                Normal2 => LANYAN_SKILL.normal_dmg21[s1] + LANYAN_SKILL.normal_dmg22[s1],
                Normal3 => LANYAN_SKILL.normal_dmg3[s1] * 2.0,
                Normal4 => LANYAN_SKILL.normal_dmg4[s1],
                Charged => LANYAN_SKILL.charged_dmg[s1] * 3.0,
                Plunging1 => LANYAN_SKILL.plunging_dmg1[s1],
                Plunging2 => LANYAN_SKILL.plunging_dmg2[s1],
                Plunging3 => LANYAN_SKILL.plunging_dmg3[s1],
                E1 => LANYAN_SKILL.e_dmg[s2],
                E1Pyro | E1Cryo | E1Electro | E1Hydro => LANYAN_SKILL.e_dmg[s2] * 0.5,
                Q1 => LANYAN_SKILL.q_dmg1[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);

            let skill_type = s.get_skill_type();
            if context.character_common_data.has_talent2 {
                if skill_type == SkillType::ElementalSkill {
                    builder.add_em_ratio("天赋「苍翎镇邪敕符」", 3.09);
                } else if skill_type == SkillType::ElementalBurst {
                    builder.add_em_ratio("天赋「苍翎镇邪敕符」", 7.74);
                }
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
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
