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
use crate::target_functions::target_functions::GorouDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct GorouSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_def_bonus: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const GOROU_SKILL: GorouSkillType = GorouSkillType {
    normal_dmg1: [0.3775, 0.4083, 0.439, 0.4829, 0.5136, 0.5488, 0.597, 0.6453, 0.6936, 0.7463, 0.799, 0.8517, 0.9043, 0.957, 1.0097],
    normal_dmg2: [0.3715, 0.4018, 0.432, 0.4752, 0.5054, 0.54, 0.5875, 0.635, 0.6826, 0.7344, 0.7862, 0.8381, 0.8899, 0.9418, 0.9936],
    normal_dmg3: [0.4945, 0.5348, 0.575, 0.6325, 0.6728, 0.7188, 0.782, 0.8453, 0.9085, 0.9775, 1.0465, 1.1155, 1.1845, 1.2535, 1.3225],
    normal_dmg4: [0.59, 0.638, 0.686, 0.7546, 0.8026, 0.8575, 0.933, 1.0084, 1.0839, 1.1662, 1.2485, 1.3308, 1.4132, 1.4955, 1.5778],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [1.072, 1.1524, 1.2328, 1.34, 1.4204, 1.5008, 1.608, 1.7152, 1.8224, 1.9296, 2.0368, 2.144, 2.278, 2.412, 2.546],
    elemental_skill_def_bonus: [206.0, 222.0, 237.0, 258.0, 273.0, 289.0, 309.0, 330.0, 350.0, 371.0, 392.0, 412.0, 438.0, 464.0, 490.0],
    elemental_burst_dmg1: [0.9822, 1.0558, 1.1295, 1.2277, 1.3014, 1.375, 1.4732, 1.5715, 1.6697, 1.7679, 1.8661, 1.9643, 2.0871, 2.2099, 2.3326],
    elemental_burst_dmg2: [0.613, 0.659, 0.705, 0.7663, 0.8122, 0.8582, 0.9195, 0.9808, 1.0421, 1.1034, 1.1647, 1.226, 1.3026, 1.3793, 1.4559],
};

pub const GOROU_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Gorou,
    internal_name: "Gorou",
    element: Element::Geo,
    hp: [802, 2061, 2661, 3985, 4411, 5074, 5642, 6305, 6731, 7393, 7818, 8481, 8907, 9570],
    atk: [15, 39, 51, 76, 84, 97, 108, 120, 128, 141, 149, 162, 170, 183],
    def: [54, 140, 180, 270, 299, 344, 382, 427, 456, 501, 530, 575, 603, 648],
    sub_stat: CharacterSubStatFamily::Bonus240(StatName::GeoBonus),
    weapon_type: WeaponType::Bow,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击•呲牙裂扇箭",
        en: "Normal Attack: Ripping Fang Fletching",
    ),
    skill_name2: locale!(
        zh_cn: "犬坂吠吠方圆阵",
        en: "Inuzaka All-Round Defense",
    ),
    skill_name3: locale!(
        zh_cn: "兽牙逐突形胜战法",
        en: "Juuga: Forward Unto Victory",
    ),
    name_locale: locale!(
        zh_cn: "五郎",
        en: "Gorou",
    )
};

pub struct Gorou;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq)]
#[derive(EnumString, EnumCountMacro)]
pub enum GorouDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
    Q2
}

impl GorouDamageEnum {
    pub fn get_element(&self) -> Element {
        use GorouDamageEnum::*;
        match *self {
            Charged2 | E1 | Q1 | Q2 => Element::Geo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use GorouDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for GorouDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum GorouRoleEnum {
    Default
}

impl CharacterTrait for Gorou {
    const STATIC_DATA: CharacterStaticData = GOROU_STATIC_DATA;
    type SkillType = GorouSkillType;
    const SKILL: Self::SkillType = GOROU_SKILL;
    type DamageEnumType = GorouDamageEnum;
    type RoleEnum = GorouRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: GorouDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: GorouDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: GorouDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: GorouDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: GorouDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: GorouDamageEnum::Charged2 as usize, text: charged_dmg!("shoot2") },
            CharacterSkillMapItem { index: GorouDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: GorouDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: GorouDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: GorouDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: GorouDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: GorouDamageEnum::Q2 as usize, text: locale!(zh_cn: "岩晶崩破伤害", en: "Crystal Collapse DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: GorouDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use GorouDamageEnum::*;
        let ratio = match s {
            Normal1 => GOROU_SKILL.normal_dmg1[s1],
            Normal2 => GOROU_SKILL.normal_dmg2[s1],
            Normal3 => GOROU_SKILL.normal_dmg3[s1],
            Normal4 => GOROU_SKILL.normal_dmg4[s1],
            Charged1 => GOROU_SKILL.charged_dmg1[s1],
            Charged2 => GOROU_SKILL.charged_dmg2[s1],
            Plunging1 => GOROU_SKILL.plunging_dmg1[s1],
            Plunging2 => GOROU_SKILL.plunging_dmg2[s1],
            Plunging3 => GOROU_SKILL.plunging_dmg3[s1],
            E1 => GOROU_SKILL.elemental_skill_dmg1[s2],
            Q1 => GOROU_SKILL.elemental_burst_dmg1[s3],
            Q2 => GOROU_SKILL.elemental_burst_dmg2[s3]
        };

        let skill_type = s.get_skill_type();

        let mut builder = D::new();

        if skill_type == SkillType::ElementalBurst {
            builder.add_def_ratio("技能倍率", ratio);
        } else {
            builder.add_atk_ratio("技能倍率", ratio);
        }

        if context.character_common_data.has_talent2 {
            if skill_type == SkillType::ElementalSkill {
                builder.add_def_ratio("五郎天赋「报恩之守」", 1.56);
            } else if skill_type == SkillType::ElementalBurst {
                builder.add_def_ratio("五郎天赋「报恩之守」", 0.156);
            }
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
        let role: GorouRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            GorouRoleEnum::Default => Box::new(GorouDefaultTargetFunction { recharge_demand: 2.0 })
        }
    }
}
