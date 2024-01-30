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
use crate::target_functions::target_functions::DionaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct DionaSkillType {
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
    pub elemental_skill_shield1: [f64; 15],
    pub elemental_skill_shield1_fixed: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_heal1: [f64; 15],
    pub elemental_burst_heal1_fixed: [f64; 15]
}

pub const DIONA_SKILL: DionaSkillType = DionaSkillType {
    normal_dmg1: [0.3612, 0.3906, 0.42, 0.462, 0.4914, 0.525, 0.5712, 0.6174, 0.6636, 0.714, 0.7718, 0.8397, 0.9076, 0.9755, 1.0496],
    normal_dmg2: [0.3354, 0.3627, 0.39, 0.429, 0.4563, 0.4875, 0.5304, 0.5733, 0.6162, 0.663, 0.7166, 0.7797, 0.8428, 0.9058, 0.9746],
    normal_dmg3: [0.4558, 0.4929, 0.53, 0.583, 0.6201, 0.6625, 0.7208, 0.7791, 0.8374, 0.901, 0.9739, 1.0596, 1.1453, 1.231, 1.3245],
    normal_dmg4: [0.43, 0.465, 0.5, 0.55, 0.585, 0.625, 0.68, 0.735, 0.79, 0.85, 0.9188, 0.9996, 1.0805, 1.1613, 1.2495],
    normal_dmg5: [0.5375, 0.5813, 0.625, 0.6875, 0.7313, 0.7813, 0.85, 0.9188, 0.9875, 1.0625, 1.1484, 1.2495, 1.3506, 1.4516, 1.5619],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9371, 1.0196, 1.1021, 1.1845, 1.2745],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.361, 2.5296, 2.6982, 2.8669, 3.0355],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.4192, 0.4506, 0.4821, 0.524, 0.5554, 0.5869, 0.6288, 0.6707, 0.7126, 0.7546, 0.7965, 0.8384, 0.8908, 0.9432, 0.9956],
    elemental_skill_shield1: [0.072, 0.0774, 0.0828, 0.09, 0.0954, 0.1008, 0.108, 0.1152, 0.1224, 0.1296, 0.1368, 0.144, 0.153, 0.162, 0.171],
    elemental_skill_shield1_fixed: [693.0, 762.0, 837.0, 918.0, 1005.0, 1097.0, 1195.0, 1299.0, 1409.0, 1524.0, 1646.0, 1773.0, 1905.0, 2044.0, 2188.0],
    elemental_burst_dmg1: [0.8, 0.86, 0.92, 1., 1.06, 1.12, 1.2, 1.28, 1.36, 1.44, 1.52, 1.6, 1.7, 1.8, 1.9],
    elemental_burst_dmg2: [0.5264, 0.5659, 0.6054, 0.658, 0.6975, 0.737, 0.7896, 0.8422, 0.8949, 0.9475, 1.0002, 1.0528, 1.1186, 1.1844, 1.2502],
    elemental_burst_heal1: [0.0534, 0.0574, 0.0614, 0.0667, 0.0707, 0.0747, 0.08, 0.0854, 0.0907, 0.096, 0.1014, 0.1067, 0.1134, 0.1201, 0.1267],
    elemental_burst_heal1_fixed: [513.0, 565.0, 620.0, 680.0, 744.0, 813.0, 885.0, 962.0, 1044.0, 1129.0, 1219.0, 1313.0, 1411.0, 1514.0, 1621.0]
};

pub const DIONA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Diona,
    internal_name: "Diona",
    element: Element::Cryo,
    hp: [802, 2061, 2661, 3985, 4411, 5074, 5642, 6305, 6731, 7393, 7818, 8481, 8907, 9570],
    atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212],
    def: [50, 129, 167, 250, 277, 318, 354, 396, 422, 464, 491, 532, 559, 601],
    sub_stat: CharacterSubStatFamily::Bonus240(StatName::CryoBonus),
    weapon_type: WeaponType::Bow,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·猎人射术",
        en: "Normal Attack: Kätzlein Style",
    ),
    skill_name2: locale!(
        zh_cn: "猫爪冻冻",
        en: "Icy Paws",
    ),
    skill_name3: locale!(
        zh_cn: "最烈特调",
        en: "Signature Mix",
    ),
    name_locale: locale!(
        zh_cn: "迪奥娜",
        en: "Diona",
    )
};

pub struct Diona;

#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum DionaDamageEnum {
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
    Q1,
    Q2,
    QHeal
}

impl DionaDamageEnum {
    pub fn is_heal(&self) -> bool {
        *self == DionaDamageEnum::QHeal
    }

    pub fn get_element(&self) -> Element {
        use DionaDamageEnum::*;
        match *self {
            Charged2 | E1 | Q1 | Q2 => Element::Cryo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use DionaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 | QHeal => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for DionaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum DionaRoleEnum {
    Aux
}

impl CharacterTrait for Diona {
    const STATIC_DATA: CharacterStaticData = DIONA_STATIC_DATA;
    type SkillType = DionaSkillType;
    const SKILL: Self::SkillType = DIONA_SKILL;
    type DamageEnumType = DionaDamageEnum;
    type RoleEnum = DionaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: DionaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: DionaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: DionaDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: DionaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: DionaDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: DionaDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: DionaDamageEnum::Charged2 as usize, text: charged_dmg!("shoot2") },
            CharacterSkillMapItem { index: DionaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: DionaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: DionaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: DionaDamageEnum::E1 as usize, text: locale!(zh_cn: "猫爪伤害", en: "Icy Paw DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: DionaDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: DionaDamageEnum::Q2 as usize, text: locale!(zh_cn: "领域持续伤害", en: "Continuous Field DMG") },
            CharacterSkillMapItem { index: DionaDamageEnum::QHeal as usize, text: locale!(zh_cn: "持续治疗量", en: "HP Regeneration Over Time") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: DionaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use DionaDamageEnum::*;
        if s.is_heal() {
            let ratio = match s {
                QHeal => DIONA_SKILL.elemental_burst_heal1[s3],
                _ => 0.0
            };
            let fixed = match s {
                QHeal => DIONA_SKILL.elemental_burst_heal1_fixed[s3],
                _ => 0.0
            };
            let mut builder = D::new();
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => DIONA_SKILL.normal_dmg1[s1],
                Normal2 => DIONA_SKILL.normal_dmg2[s1],
                Normal3 => DIONA_SKILL.normal_dmg3[s1],
                Normal4 => DIONA_SKILL.normal_dmg4[s1],
                Normal5 => DIONA_SKILL.normal_dmg5[s1],
                Charged1 => DIONA_SKILL.charged_dmg1[s1],
                Charged2 => DIONA_SKILL.charged_dmg2[s1],
                Plunging1 => DIONA_SKILL.plunging_dmg1[s1],
                Plunging2 => DIONA_SKILL.plunging_dmg2[s1],
                Plunging3 => DIONA_SKILL.plunging_dmg3[s1],
                E1 => DIONA_SKILL.elemental_skill_dmg1[s2],
                Q1 => DIONA_SKILL.elemental_burst_dmg1[s3],
                Q2 => DIONA_SKILL.elemental_burst_dmg2[s3],
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
        let role: DionaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            DionaRoleEnum::Aux => Box::new(DionaDefaultTargetFunction {
                recharge_demand: 1.5
            })
        }
    }
}
