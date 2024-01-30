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
use crate::target_functions::target_functions::ShenheDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct ShenheSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg41: [f64; 15],
    pub normal_dmg42: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_damage_bonus: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_res_minus: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

const SHENHE_SKILL: ShenheSkillType = ShenheSkillType {
    normal_dmg1: [0.4326, 0.4678, 0.503, 0.5533, 0.5885, 0.6288, 0.6841, 0.7394, 0.7947, 0.8551, 0.9155, 0.9758, 1.0362, 1.0965, 1.1569],
    normal_dmg2: [0.4025, 0.4352, 0.468, 0.5148, 0.5476, 0.585, 0.6365, 0.688, 0.7394, 0.7956, 0.8518, 0.9079, 0.9641, 1.0202, 1.0764],
    normal_dmg3: [0.5332, 0.5766, 0.62, 0.682, 0.7254, 0.775, 0.8432, 0.9114, 0.9796, 1.054, 1.1284, 1.2028, 1.2772, 1.3516, 1.426],
    normal_dmg41: [0.2632, 0.2846, 0.306, 0.3366, 0.358, 0.3825, 0.4162, 0.4498, 0.4835, 0.5202, 0.5569, 0.5936, 0.6304, 0.6671, 0.7038],
    normal_dmg42: [0.2632, 0.2846, 0.306, 0.3366, 0.358, 0.3825, 0.4162, 0.4498, 0.4835, 0.5202, 0.5569, 0.5936, 0.6304, 0.6671, 0.7038],
    normal_dmg5: [0.6562, 0.7096, 0.763, 0.8393, 0.8927, 0.9538, 1.0377, 1.1216, 1.2055, 1.2971, 1.3887, 1.4802, 1.5718, 1.6633, 1.7549],
    charged_dmg1: [1.1067, 1.1968, 1.2869, 1.4156, 1.5057, 1.6086, 1.7502, 1.8917, 2.0333, 2.1877, 2.3422, 2.4966, 2.651, 2.8054, 2.9599],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.392, 1.4964, 1.6008, 1.74, 1.8444, 1.9488, 2.088, 2.2272, 2.3664, 2.5056, 2.6448, 2.784, 2.958, 3.132, 3.306],
    elemental_skill_dmg2: [1.888, 2.0296, 2.1712, 2.36, 2.5016, 2.6432, 2.832, 3.0208, 3.2096, 3.3984, 3.5872, 3.776, 4.012, 4.248, 4.484],
    elemental_skill_damage_bonus: [0.4566, 0.4908, 0.525, 0.5707, 0.6049, 0.6392, 0.6848, 0.7305, 0.7762, 0.8218, 0.8675, 0.9131, 0.9702, 1.0273, 1.0843],
    elemental_burst_dmg1: [1.008, 1.0836, 1.1592, 1.26, 1.3356, 1.4112, 1.512, 1.6128, 1.7136, 1.8144, 1.9152, 2.016, 2.142, 2.268, 2.394],
    elemental_burst_res_minus: [0.06, 0.07, 0.08, 0.09, 0.1, 0.11, 0.12, 0.13, 0.14, 0.15, 0.15, 0.15, 0.15, 0.15, 0.15],
    elemental_burst_dmg2: [0.3312, 0.356, 0.3809, 0.414, 0.4388, 0.4637, 0.4968, 0.5299, 0.563, 0.5962, 0.6293, 0.6624, 0.7038, 0.7452, 0.7866],
};

const SHENHE_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Shenhe,
    internal_name: "Shenhe",
    element: Element::Cryo,
    hp: [1011, 2624, 3491, 5224, 5840, 6719, 7540, 8429, 9045, 9941, 10557, 11463, 12080, 12993],
    atk: [24, 61, 82, 122, 137, 157, 176, 197, 211, 232, 247, 268, 282, 304],
    def: [65, 168, 223, 334, 373, 429, 482, 538, 578, 635, 674, 732, 772, 830],
    sub_stat: CharacterSubStatFamily::ATK288,
    weapon_type: WeaponType::Polearm,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击•踏辰摄斗",
        en: "Normal Attack: Dawnstar Piercer",
    ),
    skill_name2: locale!(
        zh_cn: "仰灵威召将役咒",
        en: "Spring Spirit Summoning",
    ),
    skill_name3: locale!(
        zh_cn: "神女遣灵真诀",
        en: "Divine Maiden's Deliverance",
    ),
    name_locale: locale!(
        zh_cn: "申鹤",
        en: "Shenhe",
    )
};

pub struct Shenhe;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum ShenheDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal41,
    Normal42,
    Normal5,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    Q1,
    Q2
}

impl ShenheDamageEnum {
    pub fn get_element(&self) -> Element {
        use ShenheDamageEnum::*;
        match *self {
            E1 | E2 | Q1 | Q2 => Element::Cryo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ShenheDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal41 | Normal42 | Normal5 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for ShenheDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum ShenheRoleEnum {
    Aux
}

impl CharacterTrait for Shenhe {
    const STATIC_DATA: CharacterStaticData = SHENHE_STATIC_DATA;
    type SkillType = ShenheSkillType;
    const SKILL: Self::SkillType = SHENHE_SKILL;
    type DamageEnumType = ShenheDamageEnum;
    type RoleEnum = ShenheRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: ShenheDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: ShenheDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: ShenheDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: ShenheDamageEnum::Normal41 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: ShenheDamageEnum::Normal42 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: ShenheDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: ShenheDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: ShenheDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: ShenheDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: ShenheDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: ShenheDamageEnum::E1 as usize, text: locale!(zh_cn: "点按技能伤害", en: "Tap Skill DMG") },
            CharacterSkillMapItem { index: ShenheDamageEnum::E2 as usize, text: locale!(zh_cn: "长按技能伤害", en: "Hold Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: ShenheDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: ShenheDamageEnum::Q2 as usize, text: locale!(zh_cn: "持续伤害", en: "DoT") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ShenheDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use ShenheDamageEnum::*;
        let ratio = match s {
            Normal1 => SHENHE_SKILL.normal_dmg1[s1],
            Normal2 => SHENHE_SKILL.normal_dmg2[s1],
            Normal3 => SHENHE_SKILL.normal_dmg3[s1],
            Normal41 => SHENHE_SKILL.normal_dmg41[s1],
            Normal42 => SHENHE_SKILL.normal_dmg42[s1],
            Normal5 => SHENHE_SKILL.normal_dmg5[s1],
            Charged => SHENHE_SKILL.charged_dmg1[s1],
            Plunging1 => SHENHE_SKILL.plunging_dmg1[s1],
            Plunging2 => SHENHE_SKILL.plunging_dmg2[s1],
            Plunging3 => SHENHE_SKILL.plunging_dmg3[s1],
            E1 => SHENHE_SKILL.elemental_skill_dmg1[s2],
            E2 => SHENHE_SKILL.elemental_skill_dmg2[s2],
            Q1 => SHENHE_SKILL.elemental_burst_dmg1[s3],
            Q2 => SHENHE_SKILL.elemental_burst_dmg2[s3],
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
        let role: ShenheRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            ShenheRoleEnum::Aux => Box::new(ShenheDefaultTargetFunction {
                recharge_demand: 1.6
            })
        }
    }
}
