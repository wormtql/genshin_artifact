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
use crate::target_functions::target_functions::ThomaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct ThomaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_shield1: [f64; 15],
    pub elemental_skill_shield1_fixed: [f64; 15],
    pub elemental_skill_shield2: [f64; 15],
    pub elemental_skill_shield2_fixed: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_shield1: [f64; 15],
    pub elemental_burst_shield1_fixed: [f64; 15],
}

pub const THOMA_SKILL: ThomaSkillType = ThomaSkillType {
    normal_dmg1: [0.4439, 0.4801, 0.5162, 0.5678, 0.604, 0.6453, 0.702, 0.7588, 0.8156, 0.8775, 0.9395, 1.0014, 1.0634, 1.1253, 1.1873],
    normal_dmg2: [0.4363, 0.4718, 0.5073, 0.558, 0.5935, 0.6341, 0.6899, 0.7457, 0.8015, 0.8624, 0.9233, 0.9842, 1.045, 1.1059, 1.1668],
    normal_dmg3: [0.2679, 0.2897, 0.3115, 0.3427, 0.3645, 0.3894, 0.4236, 0.4579, 0.4922, 0.5296, 0.5669, 0.6043, 0.6417, 0.6791, 0.7165],
    normal_dmg4: [0.6736, 0.7284, 0.7832, 0.8615, 0.9163, 0.979, 1.0652, 1.1513, 1.2375, 1.3314, 1.4254, 1.5194, 1.6134, 1.7074, 1.8014],
    charged_dmg1: [1.1275, 1.2192, 1.311, 1.4421, 1.5339, 1.6388, 1.783, 1.9272, 2.0714, 2.2287, 2.386, 2.5433, 2.7007, 2.858, 3.0153],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.464, 1.5738, 1.6836, 1.83, 1.9398, 2.0496, 2.196, 2.3424, 2.4888, 2.6352, 2.7816, 2.928, 3.111, 3.294, 3.477],
    elemental_skill_shield1: [0.072, 0.0774, 0.0828, 0.09, 0.0954, 0.1008, 0.108, 0.1152, 0.1224, 0.1296, 0.1368, 0.144, 0.153, 0.162, 0.171],
    elemental_skill_shield1_fixed: [693.0, 763.0, 838.0, 919.0, 1005.0, 1098.0, 1196.0, 1300.0, 1410.0, 1525.0, 1647.0, 1774.0, 1907.0, 2046.0, 2190.0],
    elemental_skill_shield2: [0.196, 0.2107, 0.2254, 0.245, 0.2597, 0.2744, 0.294, 0.3136, 0.3332, 0.3528, 0.3724, 0.392, 0.4165, 0.441, 0.4655],
    elemental_skill_shield2_fixed: [1887.0, 2076.0, 2281.0, 2501.0, 2737.0, 2989.0, 3256.0, 3539.0, 3838.0, 4153.0, 4483.0, 4829.0, 5191.0, 5568.0, 5962.0],
    elemental_burst_dmg1: [0.88, 0.946, 1.012, 1.1, 1.166, 1.232, 1.32, 1.408, 1.496, 1.584, 1.672, 1.76, 1.87, 1.98, 2.09],
    elemental_burst_dmg2: [0.58, 0.6235, 0.667, 0.725, 0.7685, 0.812, 0.87, 0.928, 0.986, 1.044, 1.102, 1.16, 1.2325, 1.305, 1.3775],
    elemental_burst_shield1: [0.0114, 0.0123, 0.0132, 0.0143, 0.0152, 0.016, 0.0172, 0.0183, 0.0194, 0.0206, 0.0217, 0.0229, 0.0243, 0.0257, 0.0272],
    elemental_burst_shield1_fixed: [110.0, 121.0, 133.0, 146.0, 160.0, 174.0, 190.0, 206.0, 224.0, 242.0, 261.0, 282.0, 303.0, 325.0, 348.0],
};

pub const THOMA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Thoma,
    internal_name: "Tohma",
    element: Element::Pyro,
    hp: [866, 2225, 2872, 4302, 4762, 5478, 6091, 6806, 7266, 7981, 8440, 9156, 9616, 10331],
    atk: [17, 43, 56, 84, 93, 107, 119, 133, 142, 156, 165, 179, 188, 202],
    def: [63, 162, 209, 313, 346, 398, 443, 495, 528, 580, 613, 665, 699, 751],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Polearm,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·迅破枪势",
        en: "Normal Attack: Swiftshatter Spear",
    ),
    skill_name2: locale!(
        zh_cn: "烈烧佑命之侍护",
        en: "Blazing Blessing",
    ),
    skill_name3: locale!(
        zh_cn: "真红炽火之大铠",
        en: "Crimson Ooyoroi",
    ),
    name_locale: locale!(
        zh_cn: "托马",
        en: "Thoma",
    )
};

pub struct Thoma;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum ThomaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal3Times2,
    Normal4,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
    Q2
}

impl ThomaDamageEnum {
    pub fn get_element(&self) -> Element {
        use ThomaDamageEnum::*;
        match *self {
            E1 | Q1 | Q2 => Element::Pyro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ThomaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal3Times2 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for ThomaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum ThomaRoleEnum {
    AuxShield
}

impl CharacterTrait for Thoma {
    const STATIC_DATA: CharacterStaticData = THOMA_STATIC_DATA;
    type SkillType = ThomaSkillType;
    const SKILL: Self::SkillType = THOMA_SKILL;
    type DamageEnumType = ThomaDamageEnum;
    type RoleEnum = ThomaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: ThomaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: ThomaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: ThomaDamageEnum::Normal3 as usize, text: locale!(zh_cn: "三段伤害/2", en: "3-Hit DMG/2") },
            CharacterSkillMapItem { index: ThomaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: ThomaDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: ThomaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: ThomaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: ThomaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: ThomaDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: ThomaDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: ThomaDamageEnum::Q2 as usize, text: locale!(zh_cn: "炽火崩破伤害", en: "Fiery Collapse DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ThomaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use ThomaDamageEnum::*;
        let ratio = match s {
            Normal1 => THOMA_SKILL.normal_dmg1[s1],
            Normal2 => THOMA_SKILL.normal_dmg2[s1],
            Normal3 => THOMA_SKILL.normal_dmg3[s1],
            Normal3Times2 => THOMA_SKILL.normal_dmg3[s1] * 2.0,
            Normal4 => THOMA_SKILL.normal_dmg4[s1],
            Charged => THOMA_SKILL.charged_dmg1[s1],
            Plunging1 => THOMA_SKILL.plunging_dmg1[s1],
            Plunging2 => THOMA_SKILL.plunging_dmg2[s1],
            Plunging3 => THOMA_SKILL.plunging_dmg3[s1],
            E1 => THOMA_SKILL.elemental_skill_dmg1[s2],
            Q1 => THOMA_SKILL.elemental_burst_dmg1[s3],
            Q2 => THOMA_SKILL.elemental_burst_dmg2[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        if s == Q2 && context.character_common_data.has_talent2 {
            builder.add_hp_ratio("托马天赋：烈火攻燔", 0.022);
        }
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
        let role: ThomaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            ThomaRoleEnum::AuxShield => Box::new(ThomaDefaultTargetFunction {
                recharge_demand: 2.0
            })
        }
    }
}
