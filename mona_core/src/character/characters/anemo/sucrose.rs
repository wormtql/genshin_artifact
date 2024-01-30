use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::anemo::sucrose_default::SucroseDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct SucroseSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const SUCROSE_SKILL: SucroseSkillType = SucroseSkillType {
    normal_dmg1: [0.3346, 0.3597, 0.3848, 0.4183, 0.4434, 0.4685, 0.502, 0.5354, 0.5689, 0.6024, 0.6358, 0.6693, 0.7111, 0.7529, 0.7948],
    normal_dmg2: [0.3062, 0.3291, 0.3521, 0.3827, 0.4057, 0.4286, 0.4592, 0.4899, 0.5205, 0.5511, 0.5817, 0.6123, 0.6506, 0.6889, 0.7271],
    normal_dmg3: [0.3845, 0.4133, 0.4422, 0.4806, 0.5094, 0.5383, 0.5767, 0.6152, 0.6536, 0.6921, 0.7305, 0.769, 0.817, 0.8651, 0.9131],
    normal_dmg4: [0.4792, 0.5151, 0.5511, 0.599, 0.6349, 0.6708, 0.7188, 0.7667, 0.8146, 0.8625, 0.9104, 0.9584, 1.0182, 1.0781, 1.138],
    charged_dmg1: [1.2016, 1.2917, 1.3818, 1.502, 1.5921, 1.6822, 1.8024, 1.9226, 2.0427, 2.1629, 2.283, 2.4032, 2.5534, 2.7036, 2.8538],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [2.112, 2.2704, 2.4288, 2.64, 2.7984, 2.9568, 3.168, 3.3792, 3.5904, 3.8016, 4.0128, 4.224, 4.488, 4.752, 5.016],
    elemental_burst_dmg1: [1.48, 1.591, 1.702, 1.85, 1.961, 2.072, 2.22, 2.368, 2.516, 2.664, 2.812, 2.96, 3.145, 3.33, 3.515],
    elemental_burst_dmg2: [0.44, 0.473, 0.506, 0.55, 0.583, 0.616, 0.66, 0.704, 0.748, 0.792, 0.836, 0.88, 0.935, 0.99, 1.045]
};

pub const SUCROSE_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Sucrose,
    internal_name: "Sucrose",
    element: Element::Anemo,
    hp: [775, 1991, 2570, 3850, 4261, 4901, 5450, 6090, 6501, 7141, 7552, 8192, 8604, 9244],
    atk: [14, 37, 47, 71, 78, 90, 100, 112, 120, 131, 139, 151, 158, 170],
    def: [59, 151, 195, 293, 324, 373, 414, 463, 494, 543, 574, 623, 654, 703],
    sub_stat: CharacterSubStatFamily::Bonus240(StatName::AnemoBonus),
    weapon_type: WeaponType::Catalyst,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·简式风灵作成",
        en: "Normal Attack: Wind Spirit Creation",
    ),
    skill_name2: locale!(
        zh_cn: "风灵作成·陆叁零捌",
        en: "Astable Anemohypostasis Creation - 6308",
    ),
    skill_name3: locale!(
        zh_cn: "禁·风灵作成·柒伍同构贰型",
        en: "Forbidden Creation - Isomer 75 / Type II",
    ),
    name_locale: locale!(
        zh_cn: "砂糖",
        en: "Sucrose",
    )
};

pub struct Sucrose;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum SucroseDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
    Q2Pyro,
    Q2Hydro,
    Q2Electro,
    Q2Cryo
}

impl SucroseDamageEnum {
    pub fn get_element(&self) -> Element {
        use SucroseDamageEnum::*;
        match *self {
            Q2Pyro => Element::Pyro,
            Q2Hydro => Element::Hydro,
            Q2Electro => Element::Electro,
            Q2Cryo => Element::Cryo,
            _ => Element::Anemo
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use SucroseDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2Cryo | Q2Hydro | Q2Pyro | Q2Electro => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for SucroseDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum SucroseRoleEnum {
    Aux
}

impl CharacterTrait for Sucrose {
    const STATIC_DATA: CharacterStaticData = SUCROSE_STATIC_DATA;
    type SkillType = SucroseSkillType;
    const SKILL: Self::SkillType = SUCROSE_SKILL;
    type DamageEnumType = SucroseDamageEnum;
    type RoleEnum = SucroseRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: SucroseDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: SucroseDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: SucroseDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: SucroseDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: SucroseDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: SucroseDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: SucroseDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: SucroseDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: SucroseDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: SucroseDamageEnum::Q1 as usize, text: locale!(zh_cn: "持续伤害", en: "DoT") },
            CharacterSkillMapItem { index: SucroseDamageEnum::Q2Pyro as usize, text: locale!(zh_cn: "附加火元素伤害", en: "Additional Pyro DMG") },
            CharacterSkillMapItem { index: SucroseDamageEnum::Q2Hydro as usize, text: locale!(zh_cn: "附加水元素伤害", en: "Additional Hydro DMG") },
            CharacterSkillMapItem { index: SucroseDamageEnum::Q2Cryo as usize, text: locale!(zh_cn: "附加冰元素伤害", en: "Additional Cryo DMG") },
            CharacterSkillMapItem { index: SucroseDamageEnum::Q2Electro as usize, text: locale!(zh_cn: "附加雷元素伤害", en: "Additional Electro DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: SucroseDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use SucroseDamageEnum::*;
        let ratio = match s {
            Normal1 => SUCROSE_SKILL.normal_dmg1[s1],
            Normal2 => SUCROSE_SKILL.normal_dmg2[s1],
            Normal3 => SUCROSE_SKILL.normal_dmg3[s1],
            Normal4 => SUCROSE_SKILL.normal_dmg4[s1],
            Charged => SUCROSE_SKILL.charged_dmg1[s1],
            Plunging1 => SUCROSE_SKILL.plunging_dmg1[s1],
            Plunging2 => SUCROSE_SKILL.plunging_dmg2[s1],
            Plunging3 => SUCROSE_SKILL.plunging_dmg3[s1],
            E1 => SUCROSE_SKILL.elemental_skill_dmg1[s2],
            Q1 => SUCROSE_SKILL.elemental_burst_dmg1[s3],
            Q2Electro | Q2Pyro | Q2Hydro | Q2Cryo => SUCROSE_SKILL.elemental_burst_dmg2[s3]
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
        let role: SucroseRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            SucroseRoleEnum::Aux => Box::new(SucroseDefaultTargetFunction {
                recharge_demand: 1.4
            })
        }
    }
}
