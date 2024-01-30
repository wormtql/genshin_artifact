use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::NoelleDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct NoelleSkillType {
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
    pub elemental_skill_shield1: [f64; 15],
    pub elemental_skill_shield1_fixed: [f64; 15],
    pub elemental_skill_heal1: [f64; 15],
    pub elemental_skill_heal1_fixed: [f64; 15],
    pub elemental_skill_heal1_prob: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_atk_bonus: [f64; 15],
}

pub const NOELLE_SKILL: NoelleSkillType = NoelleSkillType {
    normal_dmg1: [0.7912, 0.8556, 0.92, 1.012, 1.0764, 1.15, 1.2512, 1.3524, 1.4536, 1.564, 1.6744, 1.7848, 1.8952, 2.0056, 2.116],
    normal_dmg2: [0.7336, 0.7933, 0.853, 0.9383, 0.998, 1.0663, 1.1601, 1.2539, 1.3477, 1.4501, 1.5525, 1.6548, 1.7572, 1.8595, 1.9619],
    normal_dmg3: [0.8626, 0.9328, 1.003, 1.1033, 1.1735, 1.2538, 1.3641, 1.4744, 1.5847, 1.7051, 1.8255, 1.9458, 2.0662, 2.1865, 2.3069],
    normal_dmg4: [1.1343, 1.2267, 1.319, 1.4509, 1.5432, 1.6488, 1.7938, 1.9389, 2.084, 2.2423, 2.4006, 2.5589, 2.7171, 2.8754, 3.0337],
    charged_dmg1: [0.5074, 0.5487, 0.59, 0.649, 0.6903, 0.7375, 0.8024, 0.8673, 0.9322, 1.003, 1.0738, 1.1446, 1.2154, 1.2862, 1.357],
    charged_dmg2: [0.9047, 0.9784, 1.052, 1.1572, 1.2308, 1.315, 1.4307, 1.5464, 1.6622, 1.7884, 1.9146, 2.0409, 2.1671, 2.2934, 2.4196],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    elemental_skill_dmg1: [1.2, 1.29, 1.38, 1.5, 1.59, 1.68, 1.8, 1.92, 2.04, 2.16, 2.28, 2.4, 2.55, 2.7, 2.85],
    elemental_skill_shield1: [1.6, 1.72, 1.84, 2., 2.12, 2.24, 2.4, 2.56, 2.72, 2.88, 3.04, 3.2, 3.4, 3.6, 3.8],
    elemental_skill_shield1_fixed: [770.0, 847.0, 930.0, 1020.0, 1116.0, 1219.0, 1328.0, 1443.0, 1565.0, 1694.0, 1828.0, 1970.0, 2117.0, 2271.0, 2431.0],
    elemental_skill_heal1: [0.2128, 0.2288, 0.2447, 0.266, 0.282, 0.2979, 0.3192, 0.3405, 0.3618, 0.383, 0.4043, 0.4256, 0.4522, 0.4788, 0.5054],
    elemental_skill_heal1_fixed: [103.0, 113.0, 124.0, 136.0, 149.0, 163.0, 177.0, 193.0, 209.0, 226.0, 244.0, 263.0, 282.0, 303.0, 324.0],
    elemental_skill_heal1_prob: [0.5, 0.51, 0.52, 0.53, 0.54, 0.55, 0.56, 0.57, 0.58, 0.59, 0.59, 0.6, 0.6, 0.6, 0.6],
    elemental_burst_dmg1: [0.672, 0.7224, 0.7728, 0.84, 0.8904, 0.9408, 1.008, 1.0752, 1.1424, 1.2096, 1.2768, 1.344, 1.428, 1.512, 1.596],
    elemental_burst_dmg2: [0.928, 0.9976, 1.0672, 1.16, 1.2296, 1.2992, 1.392, 1.4848, 1.5776, 1.6704, 1.7632, 1.856, 1.972, 2.088, 2.204],
    elemental_burst_atk_bonus: [0.4, 0.43, 0.46, 0.5, 0.53, 0.56, 0.6, 0.64, 0.68, 0.72, 0.76, 0.8, 0.85, 0.9, 0.95],
};

pub const NOELLE_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Noelle,
    internal_name: "Noel",
    element: Element::Geo,
    hp: [1012, 2600, 3356, 5027, 5564, 6400, 7117, 7953, 8490, 9325, 9862, 10698, 11235, 12071],
    atk: [16, 41, 53, 80, 88, 101, 113, 126, 134, 148, 156, 169, 178, 191],
    def: [67, 172, 222, 333, 368, 423, 471, 526, 562, 617, 652, 708, 743, 799],
    sub_stat: CharacterSubStatFamily::DEF300,
    weapon_type: WeaponType::Claymore,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·西风剑术·女仆",
        en: "Normal Attack: Favonius Bladework - Maid",
    ),
    skill_name2: locale!(
        zh_cn: "护心铠",
        en: "Breastplate",
    ),
    skill_name3: locale!(
        zh_cn: "大扫除",
        en: "Sweeping Time",
    ),
    name_locale: locale!(
        zh_cn: "诺艾尔",
        en: "Noelle",
    )
};

pub struct Noelle;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum NoelleDamageEnum {
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
    EHeal1,
    Q1,
    Q2
}

impl NoelleDamageEnum {
    pub fn is_heal(&self) -> bool {
        *self == NoelleDamageEnum::EHeal1
    }

    pub fn is_def_ratio(&self) -> bool {
        *self == NoelleDamageEnum::E1
    }

    pub fn get_element(&self, after_q: bool) -> Element {
        if after_q {
            Element::Geo
        } else {
            use NoelleDamageEnum::*;
            match *self {
                E1 | Q1 | Q2 => Element::Geo,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use NoelleDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | EHeal1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for NoelleDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum NoelleRoleEnum {
    Main
}

impl CharacterTrait for Noelle {
    const STATIC_DATA: CharacterStaticData = NOELLE_STATIC_DATA;
    type SkillType = NoelleSkillType;
    const SKILL: Self::SkillType = NOELLE_SKILL;
    type DamageEnumType = NoelleDamageEnum;
    type RoleEnum = NoelleRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: NoelleDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: NoelleDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: NoelleDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: NoelleDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: NoelleDamageEnum::Charged1 as usize, text: charged_dmg!("loop1") },
            CharacterSkillMapItem { index: NoelleDamageEnum::Charged2 as usize, text: charged_dmg!("loop2") },
            CharacterSkillMapItem { index: NoelleDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: NoelleDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: NoelleDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: NoelleDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: NoelleDamageEnum::EHeal1 as usize, text: locale!(zh_cn: "治疗量", en: "Healing") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: NoelleDamageEnum::Q1 as usize, text: locale!(zh_cn: "爆发伤害", en: "Burst DMG") },
            CharacterSkillMapItem { index: NoelleDamageEnum::Q2 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_q",
            title: locale!(
                zh_cn: "Q技能之后",
                en: "After Q"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: NoelleDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use NoelleDamageEnum::*;

        let mut builder = D::new();
        if s.is_heal() {
            let ratio = match s {
                EHeal1 => NOELLE_SKILL.elemental_skill_heal1[s2],
                _ => 0.0
            };
            let fixed = match s {
                EHeal1 => NOELLE_SKILL.elemental_skill_heal1_fixed[s2],
                _ => 0.0
            };
            builder.add_def_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => NOELLE_SKILL.normal_dmg1[s1],
                Normal2 => NOELLE_SKILL.normal_dmg2[s1],
                Normal3 => NOELLE_SKILL.normal_dmg3[s1],
                Normal4 => NOELLE_SKILL.normal_dmg4[s1],
                Charged1 => NOELLE_SKILL.charged_dmg1[s1],
                Charged2 => NOELLE_SKILL.charged_dmg2[s1],
                Plunging1 => NOELLE_SKILL.plunging_dmg1[s1],
                Plunging2 => NOELLE_SKILL.plunging_dmg2[s1],
                Plunging3 => NOELLE_SKILL.plunging_dmg3[s1],
                E1 => NOELLE_SKILL.elemental_skill_dmg1[s2],
                Q1 => NOELLE_SKILL.elemental_burst_dmg1[s3],
                Q2 => NOELLE_SKILL.elemental_burst_dmg2[s3],
                _ => 0.0
            };

            if s.is_def_ratio() {
                builder.add_def_ratio("技能倍率", ratio);
            } else {
                builder.add_atk_ratio("技能倍率", ratio);
            }

            let after_q = match *config {
                CharacterSkillConfig::Noelle { after_q } => after_q,
                _ => false
            };
            if after_q {
                let def = context.attribute.get_value(AttributeName::DEF);
                let is_conste6 = context.character_common_data.constellation == 6;
                let atk_bonus_ratio = NOELLE_SKILL.elemental_burst_atk_bonus[s3] + if is_conste6 {
                    0.5
                } else {
                    0.0
                };
                let atk_bonus = def * atk_bonus_ratio;
                builder.add_extra_atk("大扫除加成", atk_bonus);
            }

            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(after_q),
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
        let role: NoelleRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            NoelleRoleEnum::Main => Box::new(NoelleDefaultTargetFunction)
        }
    }
}
