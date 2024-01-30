use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use num_derive::FromPrimitive;
use strum::{EnumCount};
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct ShikanoinHeizouSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg41: [f64; 15],
    pub normal_dmg42: [f64; 15],
    pub normal_dmg43: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_bonus1: [f64; 15],
    pub elemental_skill_bonus2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const SHIKANOIN_HEIZOU_SKILL: ShikanoinHeizouSkillType = ShikanoinHeizouSkillType {
    normal_dmg1: [0.3747, 0.4028, 0.4309, 0.4684, 0.4965, 0.5246, 0.5621, 0.5996, 0.6371, 0.6745, 0.712, 0.7495, 0.7963, 0.8432, 0.89],
    normal_dmg2: [0.3685, 0.3962, 0.4238, 0.4607, 0.4883, 0.5159, 0.5528, 0.5896, 0.6265, 0.6633, 0.7002, 0.737, 0.7831, 0.8292, 0.8752],
    normal_dmg3: [0.5106, 0.5489, 0.5872, 0.6383, 0.6765, 0.7148, 0.7659, 0.817, 0.868, 0.9191, 0.9701, 1.0212, 1.085, 1.1489, 1.2127],
    normal_dmg41: [0.1478, 0.1589, 0.17, 0.1848, 0.1959, 0.207, 0.2217, 0.2365, 0.2513, 0.2661, 0.2809, 0.2956, 0.3141, 0.3326, 0.3511],
    normal_dmg42: [0.1626, 0.1748, 0.187, 0.2033, 0.2155, 0.2277, 0.2439, 0.2602, 0.2764, 0.2927, 0.309, 0.3252, 0.3455, 0.3659, 0.3862],
    normal_dmg43: [0.1922, 0.2066, 0.221, 0.2402, 0.2546, 0.269, 0.2883, 0.3075, 0.3267, 0.3459, 0.3651, 0.3844, 0.4084, 0.4324, 0.4564],
    normal_dmg5: [0.6145, 0.6606, 0.7067, 0.7681, 0.8142, 0.8603, 0.9217, 0.9832, 1.0446, 1.1061, 1.1675, 1.229, 1.3058, 1.3826, 1.4594],
    charged_dmg1: [0.73, 0.7848, 0.8395, 0.9125, 0.9673, 1.022, 1.095, 1.168, 1.241, 1.314, 1.387, 1.46, 1.5513, 1.6425, 1.7338],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [2.2752, 2.4458, 2.6165, 2.844, 3.0146, 3.1853, 3.4128, 3.6403, 3.8678, 4.0954, 4.3229, 4.5504, 4.8348, 5.1192, 5.4036],
    elemental_skill_bonus1: [0.5688, 0.6115, 0.6541, 0.711, 0.7537, 0.7963, 0.8532, 0.9101, 0.967, 1.0238, 1.0807, 1.1376, 1.2087, 1.2798, 1.3509],
    elemental_skill_bonus2: [1.1376, 1.2229, 1.3082, 1.422, 1.5073, 1.5926, 1.7064, 1.8202, 1.9339, 2.0477, 2.1614, 2.2752, 2.4174, 2.5596, 2.7018],
    elemental_burst_dmg1: [3.1469, 3.3829, 3.6189, 3.9336, 4.1696, 4.4056, 4.7203, 5.035, 5.3497, 5.6644, 5.9791, 6.2938, 6.6871, 7.0805, 7.4738],
    elemental_burst_dmg2: [0.2146, 0.2307, 0.2467, 0.2682, 0.2843, 0.3004, 0.3218, 0.3433, 0.3648, 0.3862, 0.4077, 0.4291, 0.4559, 0.4828, 0.5096],
};

#[derive(Copy, Clone, FromPrimitive)]
#[derive(EnumString, EnumCountMacro)]
pub enum ShikanoinHeizouDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal41,
    Normal42,
    Normal43,
    Normal4,
    Normal5,
    Charged1,
    Plunging1,
    Plunging2,
    Plunging3,
    E0,
    E1,
    E2,
    E3,
    E4,
    Q1,
    Q2Pyro,
    Q2Hydro,
    Q2Cryo,
    Q2Electro,
}

impl Into<usize> for ShikanoinHeizouDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl ShikanoinHeizouDamageEnum {
    pub fn get_element(&self) -> Element {
        use ShikanoinHeizouDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal41 | Normal42 | Normal43 | Normal4 | Normal5 | Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Anemo,
            E0 | E1 | E2 | E3 | E4 | Q1 => Element::Anemo,
            Q2Pyro => Element::Pyro,
            Q2Hydro => Element::Hydro,
            Q2Cryo => Element::Cryo,
            Q2Electro => Element::Electro,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ShikanoinHeizouDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal41 | Normal42 | Normal43 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E0 | E1 | E2 | E3 | E4 => SkillType::ElementalSkill,
            Q1 | Q2Cryo | Q2Hydro | Q2Pyro | Q2Electro => SkillType::ElementalBurst
        }
    }
}

pub struct ShikanoinHeizou;

impl CharacterTrait for ShikanoinHeizou {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::ShikanoinHeizou,
        internal_name: "Heizo",
        element: Element::Anemo,
        hp: [894, 2296, 2963, 4438, 4913, 5651, 6283, 7021, 7495, 8233, 8707, 9445, 9919, 10657],
        atk: [19, 48, 63, 94, 104, 119, 133, 148, 158, 174, 184, 200, 210, 225],
        def: [57, 147, 190, 285, 315, 363, 403, 451, 481, 528, 559, 606, 637, 684],
        sub_stat: CharacterSubStatFamily::Bonus240(StatName::AnemoBonus),
        weapon_type: WeaponType::Catalyst,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·不动流格斗术",
            en: "Normal Attack: Fudou Style Martial Arts",
        ),
        skill_name2: locale!(
            zh_cn: "勠心拳",
            en: "Heartstopper Strike",
        ),
        skill_name3: locale!(
            zh_cn: "聚风蹴",
            en: "Windmuster Kick",
        ),
        name_locale: locale!(
            zh_cn: "鹿野院平藏",
            en: "Shikanoin Heizou",
        )
    };
    type SkillType = ShikanoinHeizouSkillType;
    const SKILL: Self::SkillType = SHIKANOIN_HEIZOU_SKILL;
    type DamageEnumType = ShikanoinHeizouDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Normal41 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Normal42 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Normal43 as usize, text: hit_n_dmg!(4, 3) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Charged1 as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::E0 as usize, text: locale!(zh_cn: "技能伤害-0层", en: "Skill DMG-0") },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害-1层", en: "Skill DMG-1") },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::E2 as usize, text: locale!(zh_cn: "技能伤害-2层", en: "Skill DMG-2") },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::E3 as usize, text: locale!(zh_cn: "技能伤害-3层", en: "Skill DMG-3") },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::E4 as usize, text: locale!(zh_cn: "技能伤害-4层", en: "Skill DMG-4") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Q1 as usize, text: locale!(zh_cn: "不动流·真空弹伤害", en: "Fudou Style Vacuum Slugger DMG") },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Q2Pyro as usize, text: locale!(zh_cn: "聚风真眼伤害-火", en: "Windmuster Iris DMG-Pyro") },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Q2Cryo as usize, text: locale!(zh_cn: "聚风真眼伤害-冰", en: "Windmuster Iris DMG-Cryo") },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Q2Electro as usize, text: locale!(zh_cn: "聚风真眼伤害-雷", en: "Windmuster Iris DMG-Electro") },
            CharacterSkillMapItem { index: ShikanoinHeizouDamageEnum::Q2Hydro as usize, text: locale!(zh_cn: "聚风真眼伤害-水", en: "Windmuster Iris DMG-Hydro") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ShikanoinHeizouDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use ShikanoinHeizouDamageEnum::*;
        let ratio = match s {
            Normal1 => SHIKANOIN_HEIZOU_SKILL.normal_dmg1[s1],
            Normal2 => SHIKANOIN_HEIZOU_SKILL.normal_dmg2[s1],
            Normal3 => SHIKANOIN_HEIZOU_SKILL.normal_dmg3[s1],
            Normal41 => SHIKANOIN_HEIZOU_SKILL.normal_dmg41[s1],
            Normal42 => SHIKANOIN_HEIZOU_SKILL.normal_dmg42[s1],
            Normal43 => SHIKANOIN_HEIZOU_SKILL.normal_dmg43[s1],
            Normal4 => SHIKANOIN_HEIZOU_SKILL.normal_dmg41[s1] + SHIKANOIN_HEIZOU_SKILL.normal_dmg42[s1] + SHIKANOIN_HEIZOU_SKILL.normal_dmg43[s2],
            Normal5 => SHIKANOIN_HEIZOU_SKILL.normal_dmg5[s1],
            Charged1 => SHIKANOIN_HEIZOU_SKILL.charged_dmg1[s1],
            Plunging1 => SHIKANOIN_HEIZOU_SKILL.plunging_dmg1[s1],
            Plunging2 => SHIKANOIN_HEIZOU_SKILL.plunging_dmg2[s1],
            Plunging3 => SHIKANOIN_HEIZOU_SKILL.plunging_dmg3[s1],
            E0 => SHIKANOIN_HEIZOU_SKILL.elemental_skill_dmg1[s2],
            E1 => SHIKANOIN_HEIZOU_SKILL.elemental_skill_dmg1[s2] + SHIKANOIN_HEIZOU_SKILL.elemental_skill_bonus1[s2],
            E2 => SHIKANOIN_HEIZOU_SKILL.elemental_skill_dmg1[s2] + SHIKANOIN_HEIZOU_SKILL.elemental_skill_bonus1[s2] * 2.0,
            E3 => SHIKANOIN_HEIZOU_SKILL.elemental_skill_dmg1[s2] + SHIKANOIN_HEIZOU_SKILL.elemental_skill_bonus1[s2] * 3.0,
            E4 => SHIKANOIN_HEIZOU_SKILL.elemental_skill_dmg1[s2] + SHIKANOIN_HEIZOU_SKILL.elemental_skill_bonus1[s2] * 4.0 + SHIKANOIN_HEIZOU_SKILL.elemental_skill_bonus2[s2],
            Q1 => SHIKANOIN_HEIZOU_SKILL.elemental_burst_dmg1[s3],
            Q2Pyro | Q2Hydro | Q2Cryo | Q2Electro => SHIKANOIN_HEIZOU_SKILL.elemental_burst_dmg2[s3],
        };

        let element = s.get_element();
        let skill = s.get_skill_type();

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        if context.character_common_data.constellation >= 6 && skill == SkillType::ElementalSkill {
            let stack = match s {
                E1 => 1,
                E2 => 2,
                E3 => 3,
                E4 => 4,
                _ => 0,
            };

            if stack > 0 {
                builder.add_extra_critical("6命：奇想天开捕物帐", stack as f64 * 0.04);
            }
            if stack == 4 {
                builder.add_extra_critical_damage("6命：奇想天开捕物帐", 0.32);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            element,
            skill,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}