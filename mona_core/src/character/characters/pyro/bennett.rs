use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::BennettDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct BennettSkillType {
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

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg21: [f64; 15],
    pub elemental_skill_dmg22: [f64; 15],
    pub elemental_skill_dmg31: [f64; 15],
    pub elemental_skill_dmg32: [f64; 15],
    pub elemental_skill_dmg4: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_heal1: [f64; 15],
    pub elemental_burst_heal1_fixed: [f64; 15],
    pub elemental_burst_atk_bonus: [f64; 15]
}

pub const BENNETT_SKILL: BennettSkillType = BennettSkillType {
    normal_dmg1: [0.4455, 0.4817, 0.518, 0.5698, 0.6061, 0.6475, 0.7045, 0.7615, 0.8184, 0.8806, 0.9428, 1.0049, 1.0671, 1.1292, 1.1914],
    normal_dmg2: [0.4274, 0.4622, 0.497, 0.5467, 0.5815, 0.6213, 0.6759, 0.7306, 0.7853, 0.8449, 0.9045, 0.9642, 1.0238, 1.0835, 1.1431],
    normal_dmg3: [0.5461, 0.5906, 0.635, 0.6985, 0.743, 0.7938, 0.8636, 0.9335, 1.0033, 1.0795, 1.1557, 1.2319, 1.3081, 1.3843, 1.4605],
    normal_dmg4: [0.5968, 0.6454, 0.694, 0.7634, 0.812, 0.8675, 0.9438, 1.0202, 1.0965, 1.1798, 1.2631, 1.3464, 1.4296, 1.5129, 1.5962],
    normal_dmg5: [0.719, 0.7775, 0.836, 0.9196, 0.9781, 1.045, 1.137, 1.2289, 1.3209, 1.4212, 1.5215, 1.6218, 1.7222, 1.8225, 1.9228],
    charged_dmg1: [0.559, 0.6045, 0.65, 0.715, 0.7605, 0.8125, 0.884, 0.9555, 1.027, 1.105, 1.183, 1.261, 1.339, 1.417, 1.495],
    charged_dmg2: [0.6072, 0.6566, 0.706, 0.7766, 0.826, 0.8825, 0.9602, 1.0378, 1.1155, 1.2002, 1.2849, 1.3696, 1.4544, 1.5391, 1.6238],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.376, 1.4792, 1.5824, 1.72, 1.8232, 1.9264, 2.064, 2.2016, 2.3392, 2.4768, 2.6144, 2.752, 2.924, 3.096, 3.268],
    elemental_skill_dmg21: [0.84, 0.903, 0.966, 1.05, 1.113, 1.176, 1.26, 1.344, 1.428, 1.512, 1.596, 1.68, 1.785, 1.89, 1.995],
    elemental_skill_dmg22: [0.92, 0.989, 1.058, 1.15, 1.219, 1.288, 1.38, 1.472, 1.564, 1.656, 1.748, 1.84, 1.955, 2.07, 2.185],
    elemental_skill_dmg31: [0.88, 0.946, 1.012, 1.1, 1.166, 1.232, 1.32, 1.408, 1.496, 1.584, 1.672, 1.76, 1.87, 1.98, 2.09],
    elemental_skill_dmg32: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    elemental_skill_dmg4: [1.32, 1.419, 1.518, 1.65, 1.749, 1.848, 1.98, 2.112, 2.244, 2.376, 2.508, 2.64, 2.805, 2.97, 3.135],
    elemental_burst_dmg1: [2.328, 2.5026, 2.6772, 2.91, 3.0846, 3.2592, 3.492, 3.7248, 3.9576, 4.1904, 4.4232, 4.656, 4.947, 5.238, 5.529],
    elemental_burst_heal1: [0.06, 0.0645, 0.069, 0.075, 0.0795, 0.084, 0.09, 0.096, 0.102, 0.108, 0.114, 0.12, 0.1275, 0.135, 0.1425],
    elemental_burst_heal1_fixed: [577.0, 635.0, 698.0, 765.0, 837.0, 914.0, 996.0, 1083.0, 1174.0, 1270.0, 1371.0, 1477.0, 1588.0, 1703.0, 1824.0],
    elemental_burst_atk_bonus: [0.56, 0.602, 0.644, 0.7, 0.742, 0.784, 0.84, 0.896, 0.952, 1.008, 1.064, 1.12, 1.19, 1.26, 1.33]
};

pub const BENNETT_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Bennett,
    internal_name: "Bennett",
    element: Element::Pyro,
    hp: [1039, 2670, 3447, 5163, 5715, 6573, 7309, 8186, 8719, 9577, 10129, 10987, 11539, 12397],
    atk: [16, 41, 53, 80, 88, 101, 113, 126, 134, 148, 156, 169, 178, 191],
    def: [65, 166, 214, 321, 356, 409, 455, 508, 542, 596, 630, 684, 718, 771],
    sub_stat: CharacterSubStatFamily::Recharge267,
    weapon_type: WeaponType::Sword,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·好运剑",
        en: "Normal Attack: Strike of Fortune",
    ),
    skill_name2: locale!(
        zh_cn: "热情过载",
        en: "Passion Overload",
    ),
    skill_name3: locale!(
        zh_cn: "美妙旅程",
        en: "Fantastic Voyage",
    ),
    name_locale: locale!(
        zh_cn: "班尼特",
        en: "Bennett",
    )
};

pub struct Bennett;

impl Bennett {
    pub fn atk_bonus<A: Attribute>(common_data: &CharacterCommonData, attribute: &A) -> f64 {
        let base_atk = attribute.get_value(AttributeName::ATKBase);
        let s3 = common_data.skill3;

        let bonus = BENNETT_SKILL.elemental_burst_atk_bonus[s3] + (if common_data.constellation >= 1 { 0.2 } else { 0.0 });
        bonus * base_atk
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(FromPrimitive, EnumCountMacro, EnumString)]
pub enum BennettDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Charged11,
    Charged12,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E21,
    E22,
    E31,
    E32,
    E4,
    Q1,
    QHeal
}

impl BennettDamageEnum {
    pub fn is_heal(&self) -> bool {
        *self == BennettDamageEnum::QHeal
    }

    pub fn get_element(&self) -> Element {
        use BennettDamageEnum::*;
        match *self {
            E1 | E21 | E22 | E31 | E32 | E4 | Q1 | QHeal => Element::Pyro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use BennettDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E21 | E22 | E31 | E32 | E4 => SkillType::ElementalSkill,
            Q1 | QHeal => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for BennettDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(FromPrimitive)]
pub enum BennettRoleEnum {
    Aux
}

impl CharacterTrait for Bennett {
    const STATIC_DATA: CharacterStaticData = BENNETT_STATIC_DATA;
    type SkillType = BennettSkillType;
    const SKILL: Self::SkillType = BENNETT_SKILL;
    type DamageEnumType = BennettDamageEnum;
    type RoleEnum = BennettRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: BennettDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: BennettDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: BennettDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: BennettDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: BennettDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: BennettDamageEnum::Charged11 as usize, text: charged_dmg!(1) },
            CharacterSkillMapItem { index: BennettDamageEnum::Charged12 as usize, text: charged_dmg!(2) },
            CharacterSkillMapItem { index: BennettDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: BennettDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: BennettDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: BennettDamageEnum::E1 as usize, text: locale!(zh_cn: "点按伤害", en: "Tapping DMG") },
            CharacterSkillMapItem { index: BennettDamageEnum::E21 as usize, text: locale!(zh_cn: "一段蓄力伤害-1", en: "Charge Level 1 DMG-1") },
            CharacterSkillMapItem { index: BennettDamageEnum::E22 as usize, text: locale!(zh_cn: "一段蓄力伤害-2", en: "Charge Level 1 DMG-2") },
            CharacterSkillMapItem { index: BennettDamageEnum::E31 as usize, text: locale!(zh_cn: "二段蓄力伤害-1", en: "Charge Level 2 DMG-1") },
            CharacterSkillMapItem { index: BennettDamageEnum::E32 as usize, text: locale!(zh_cn: "二段蓄力伤害-2", en: "Charge Level 2 DMG-2") },
            CharacterSkillMapItem { index: BennettDamageEnum::E4 as usize, text: locale!(zh_cn: "爆炸伤害", en: "Explosion DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: BennettDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: BennettDamageEnum::QHeal as usize, text: locale!(zh_cn: "持续治疗", en: "Continuous Regeneration Per Sec") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: BennettDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        if s.is_heal() {
            let ratio = match s {
                BennettDamageEnum::QHeal => BENNETT_SKILL.elemental_burst_heal1[s3],
                _ => 0.0
            };
            let fixed = BENNETT_SKILL.elemental_burst_heal1_fixed[s3];

            let mut builder = D::new();
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);

            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                BennettDamageEnum::Normal1 => BENNETT_SKILL.normal_dmg1[s1],
                BennettDamageEnum::Normal2 => BENNETT_SKILL.normal_dmg2[s1],
                BennettDamageEnum::Normal3 => BENNETT_SKILL.normal_dmg3[s1],
                BennettDamageEnum::Normal4 => BENNETT_SKILL.normal_dmg4[s1],
                BennettDamageEnum::Normal5 => BENNETT_SKILL.normal_dmg5[s1],
                BennettDamageEnum::Charged11 => BENNETT_SKILL.charged_dmg1[s1],
                BennettDamageEnum::Charged12 => BENNETT_SKILL.charged_dmg2[s1],
                BennettDamageEnum::Plunging1 => BENNETT_SKILL.plunging_dmg1[s1],
                BennettDamageEnum::Plunging2 => BENNETT_SKILL.plunging_dmg2[s1],
                BennettDamageEnum::Plunging3 => BENNETT_SKILL.plunging_dmg3[s1],
                BennettDamageEnum::E1 => BENNETT_SKILL.elemental_skill_dmg1[s2],
                BennettDamageEnum::E21 => BENNETT_SKILL.elemental_skill_dmg21[s2],
                BennettDamageEnum::E22 => BENNETT_SKILL.elemental_skill_dmg22[s2],
                BennettDamageEnum::E31 => BENNETT_SKILL.elemental_skill_dmg31[s2],
                BennettDamageEnum::E32 => BENNETT_SKILL.elemental_skill_dmg32[s2],
                BennettDamageEnum::E4 => BENNETT_SKILL.elemental_skill_dmg4[s2],
                BennettDamageEnum::Q1 => BENNETT_SKILL.elemental_burst_dmg1[s3],
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
        let role: BennettRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            BennettRoleEnum::Aux => Box::new(BennettDefaultTargetFunction {
                recharge_demand: 2.0
            })
        }
    }
}
