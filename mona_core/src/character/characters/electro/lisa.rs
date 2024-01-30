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
use crate::target_functions::target_functions::LisaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct LisaSkillType {
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
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_dmg4: [f64; 15],
    pub elemental_skill_dmg5: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
}

pub const LISA_SKILL: LisaSkillType = LisaSkillType {
    normal_dmg1: [0.396, 0.4257, 0.4554, 0.495, 0.5247, 0.5544, 0.594, 0.6336, 0.6732, 0.7128, 0.754, 0.8078, 0.8617, 0.9156, 0.9694],
    normal_dmg2: [0.3592, 0.3861, 0.4131, 0.449, 0.4759, 0.5029, 0.5388, 0.5747, 0.6106, 0.6466, 0.6839, 0.7328, 0.7816, 0.8305, 0.8793],
    normal_dmg3: [0.428, 0.4601, 0.4922, 0.535, 0.5671, 0.5992, 0.642, 0.6848, 0.7276, 0.7704, 0.8149, 0.8731, 0.9313, 0.9895, 1.0477],
    normal_dmg4: [0.5496, 0.5908, 0.632, 0.687, 0.7282, 0.7694, 0.8244, 0.8794, 0.9343, 0.9893, 1.0464, 1.1212, 1.1959, 1.2707, 1.3454],
    charged_dmg1: [1.7712, 1.904, 2.0369, 2.214, 2.3468, 2.4797, 2.6568, 2.8339, 3.011, 3.1882, 3.3724, 3.6132, 3.8541, 4.095, 4.3359],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.8, 0.86, 0.92, 1., 1.06, 1.12, 1.2, 1.28, 1.36, 1.44, 1.52, 1.6, 1.7, 1.8, 1.9],
    elemental_skill_dmg2: [3.2, 3.44, 3.68, 4., 4.24, 4.48, 4.8, 5.12, 5.44, 5.76, 6.08, 6.4, 6.8, 7.2, 7.6],
    elemental_skill_dmg3: [3.68, 3.956, 4.232, 4.6, 4.876, 5.152, 5.52, 5.888, 6.256, 6.624, 6.992, 7.36, 7.82, 8.28, 8.74],
    elemental_skill_dmg4: [4.24, 4.558, 4.876, 5.3, 5.618, 5.936, 6.36, 6.784, 7.208, 7.632, 8.056, 8.48, 9.01, 9.54, 10.07],
    elemental_skill_dmg5: [4.872, 5.2374, 5.6028, 6.09, 6.4554, 6.8208, 7.308, 7.7952, 8.2824, 8.7696, 9.2568, 9.744, 10.353, 10.962, 11.571],
    elemental_burst_dmg1: [0.3656, 0.393, 0.4204, 0.457, 0.4844, 0.5118, 0.5484, 0.585, 0.6215, 0.6581, 0.6946, 0.7312, 0.7769, 0.8226, 0.8683],
};

pub const LISA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Lisa,
    internal_name: "Lisa",
    element: Element::Electro,
    hp: [802, 2061, 2661, 3985, 4411, 5074, 5642, 6305, 6731, 7393, 7818, 8481, 8907, 9570],
    atk: [19, 50, 64, 96, 107, 123, 136, 153, 163, 179, 189, 205, 215, 232],
    def: [48, 123, 159, 239, 264, 304, 338, 378, 403, 443, 468, 508, 534, 573],
    sub_stat: CharacterSubStatFamily::ElementalMastery96,
    weapon_type: WeaponType::Catalyst,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·指尖雷暴",
        en: "Normal Attack: Lightning Touch",
    ),
    skill_name2: locale!(
        zh_cn: "苍雷",
        en: "Violet Arc",
    ),
    skill_name3: locale!(
        zh_cn: "蔷薇的雷光",
        en: "Lightning Rose",
    ),
    name_locale: locale!(
        zh_cn: "丽莎",
        en: "Lisa",
    )
};

pub struct Lisa;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum LisaDamageEnum {
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
    E3,
    E4,
    E5,
    Q1
}

impl LisaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use LisaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4 | E5 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for LisaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum LisaRoleEnum {
    Default
}

impl CharacterTrait for Lisa {
    const STATIC_DATA: CharacterStaticData = LISA_STATIC_DATA;
    type SkillType = LisaSkillType;
    const SKILL: Self::SkillType = LISA_SKILL;
    type DamageEnumType = LisaDamageEnum;
    type RoleEnum = LisaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: LisaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: LisaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: LisaDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: LisaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: LisaDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: LisaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: LisaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: LisaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: LisaDamageEnum::E1 as usize, text: locale!(zh_cn: "点按伤害", en: "Tapping DMG") },
            CharacterSkillMapItem { index: LisaDamageEnum::E2 as usize, text: locale!(zh_cn: "无引雷长按伤害", en: "Non-Conductive Hold DMG") },
            CharacterSkillMapItem { index: LisaDamageEnum::E3 as usize, text: locale!(zh_cn: "一层引雷长按伤害", en: "Stack 1 Conductive Hold DMG") },
            CharacterSkillMapItem { index: LisaDamageEnum::E4 as usize, text: locale!(zh_cn: "二层引雷长按伤害", en: "Stack 2 Conductive Hold DMG") },
            CharacterSkillMapItem { index: LisaDamageEnum::E5 as usize, text: locale!(zh_cn: "三层引雷长按伤害", en: "Stack 3 Conductive Hold DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: LisaDamageEnum::Q1 as usize, text: locale!(zh_cn: "雷光放电伤害", en: "Discharge DMG") }
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LisaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use LisaDamageEnum::*;
        let ratio = match s {
            Normal1 => LISA_SKILL.normal_dmg1[s1],
            Normal2 => LISA_SKILL.normal_dmg2[s1],
            Normal3 => LISA_SKILL.normal_dmg3[s1],
            Normal4 => LISA_SKILL.normal_dmg4[s1],
            Charged => LISA_SKILL.charged_dmg1[s1],
            Plunging1 => LISA_SKILL.plunging_dmg1[s1],
            Plunging2 => LISA_SKILL.plunging_dmg2[s1],
            Plunging3 => LISA_SKILL.plunging_dmg3[s1],
            E1 => LISA_SKILL.elemental_skill_dmg1[s2],
            E2 => LISA_SKILL.elemental_skill_dmg2[s2],
            E3 => LISA_SKILL.elemental_skill_dmg3[s2],
            E4 => LISA_SKILL.elemental_skill_dmg4[s2],
            E5 => LISA_SKILL.elemental_skill_dmg5[s2],
            Q1 => LISA_SKILL.elemental_burst_dmg1[s3]
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Electro,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: LisaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            LisaRoleEnum::Default => Box::new(LisaDefaultTargetFunction {
                recharge_demand: 1.0
            })
        }
    }
}
