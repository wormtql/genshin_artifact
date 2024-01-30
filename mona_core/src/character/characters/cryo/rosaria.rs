use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeName};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::cryo::rosaria_default::RosariaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct RosariaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg51: [f64; 15],
    pub normal_dmg52: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg11: [f64; 15],
    pub elemental_skill_dmg12: [f64; 15],

    pub elemental_burst_dmg11: [f64; 15],
    pub elemental_burst_dmg12: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const ROSARIA_SKILL: RosariaSkillType = RosariaSkillType {
    normal_dmg1: [0.5246, 0.5673, 0.61, 0.671, 0.7137, 0.7625, 0.8296, 0.8967, 0.9638, 1.037, 1.1102, 1.1834, 1.2566, 1.3298, 1.403],
    normal_dmg2: [0.516, 0.558, 0.6, 0.66, 0.702, 0.75, 0.816, 0.882, 0.948, 1.02, 1.092, 1.164, 1.236, 1.308, 1.38],
    normal_dmg3: [0.3182, 0.3441, 0.37, 0.407, 0.4329, 0.4625, 0.5032, 0.5439, 0.5846, 0.629, 0.6734, 0.7178, 0.7622, 0.8066, 0.851],
    normal_dmg4: [0.6966, 0.7533, 0.81, 0.891, 0.9477, 1.0125, 1.1016, 1.1907, 1.2798, 1.377, 1.4742, 1.5714, 1.6686, 1.7658, 1.863],
    normal_dmg51: [0.4162, 0.4501, 0.484, 0.5324, 0.5663, 0.605, 0.6582, 0.7115, 0.7647, 0.8228, 0.8809, 0.939, 0.997, 1.0551, 1.1132],
    normal_dmg52: [0.43, 0.465, 0.5, 0.55, 0.585, 0.625, 0.68, 0.735, 0.79, 0.85, 0.91, 0.97, 1.03, 1.09, 1.15],
    charged_dmg1: [1.3674, 1.4787, 1.59, 1.749, 1.8603, 1.9875, 2.1624, 2.3373, 2.5122, 2.703, 2.8938, 3.0846, 3.2754, 3.4662, 3.657],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg11: [0.584, 0.6278, 0.6716, 0.73, 0.7738, 0.8176, 0.876, 0.9344, 0.9928, 1.0512, 1.1096, 1.168, 1.241, 1.314, 1.387],
    elemental_skill_dmg12: [1.36, 1.462, 1.564, 1.7, 1.802, 1.904, 2.04, 2.176, 2.312, 2.448, 2.584, 2.72, 2.89, 3.06, 3.23],
    elemental_burst_dmg11: [1.04, 1.118, 1.196, 1.3, 1.378, 1.456, 1.56, 1.664, 1.768, 1.872, 1.976, 2.08, 2.21, 2.34, 2.47],
    elemental_burst_dmg12: [1.52, 1.634, 1.748, 1.9, 2.014, 2.128, 2.28, 2.432, 2.584, 2.736, 2.888, 3.04, 3.23, 3.42, 3.61],
    elemental_burst_dmg2: [1.32, 1.419, 1.518, 1.65, 1.749, 1.848, 1.98, 2.112, 2.244, 2.376, 2.508, 2.64, 2.805, 2.97, 3.135],
};

pub const ROSARIA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Rosaria,
    internal_name: "Rosaria",
    element: Element::Cryo,
    hp: [1030, 2647, 3417, 5118, 5665, 6516, 7245, 8096, 8643, 9493, 10040, 10891, 11438, 12289],
    atk: [20, 52, 67, 100, 111, 127, 141, 158, 169, 185, 196, 213, 223, 240],
    def: [60, 153, 197, 296, 327, 376, 418, 468, 499, 548, 580, 629, 661, 710],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Polearm,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·教会枪术",
        en: "Normal Attack: Spear of the Church",
    ),
    skill_name2: locale!(
        zh_cn: "噬罪的告解",
        en: "Ravaging Confession",
    ),
    skill_name3: locale!(
        zh_cn: "终命的圣礼",
        en: "Rites of Termination",
    ),
    name_locale: locale!(
        zh_cn: "罗莎莉亚",
        en: "Rosaria",
    )
};

pub struct RosariaEffect {
    pub has_talent1: bool,
    pub e_from_behind: bool,
}

impl RosariaEffect {
    pub fn new(common_data: &CharacterCommonData, config: &CharacterConfig) -> RosariaEffect {
        let e_from_behind = match config {
            CharacterConfig::Rosaria { e_from_behind } => *e_from_behind,
            _ => false,
        };

        RosariaEffect {
            has_talent1: common_data.has_talent1,
            e_from_behind,
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for RosariaEffect {
    fn change_attribute(&self, attribute: &mut T) {
        if self.has_talent1 && self.e_from_behind {
            attribute.set_value_by(AttributeName::CriticalBase, "罗莎莉亚天赋：聆听忏悔的幽影", 0.12);
        }
    }
}

pub struct Rosaria;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum RosariaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal51,
    Normal52,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E11,
    E12,
    Q11,
    Q12,
    Q2,
}

impl RosariaDamageEnum {
    pub fn get_element(&self) -> Element {
        use RosariaDamageEnum::*;
        match *self {
            E11 | E12 | Q11 | Q12 | Q2 => Element::Cryo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use RosariaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal51 | Normal52 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E11 | E12 => SkillType::ElementalSkill,
            Q11 | Q12 | Q2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for RosariaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum RosariaRoleEnum {
    Freezing
}

impl CharacterTrait for Rosaria {
    const STATIC_DATA: CharacterStaticData = ROSARIA_STATIC_DATA;
    type SkillType = RosariaSkillType;
    const SKILL: Self::SkillType = ROSARIA_SKILL;
    type DamageEnumType = RosariaDamageEnum;
    type RoleEnum = RosariaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: RosariaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: RosariaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: RosariaDamageEnum::Normal3 as usize, text: locale!(zh_cn: "三段伤害/2", en: "3-Hit DMG/2") },
            CharacterSkillMapItem { index: RosariaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: RosariaDamageEnum::Normal51 as usize, text: hit_n_dmg!(5, 1) },
            CharacterSkillMapItem { index: RosariaDamageEnum::Normal52 as usize, text: hit_n_dmg!(5, 2) },
            CharacterSkillMapItem { index: RosariaDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: RosariaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: RosariaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: RosariaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: RosariaDamageEnum::E11 as usize, text: locale!(zh_cn: "技能伤害-1", en: "Skill DMG-1") },
            CharacterSkillMapItem { index: RosariaDamageEnum::E12 as usize, text: locale!(zh_cn: "技能伤害-2", en: "Skill DMG-2") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: RosariaDamageEnum::Q11 as usize, text: locale!(zh_cn: "技能伤害-1", en: "Skill DMG-1") },
            CharacterSkillMapItem { index: RosariaDamageEnum::Q12 as usize, text: locale!(zh_cn: "技能伤害-2", en: "Skill DMG-2") },
            CharacterSkillMapItem { index: RosariaDamageEnum::Q2 as usize, text: locale!(zh_cn: "冰枪持续伤害", en: "Ice Lance DoT") },
        ]),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_from_behind",
            title: locale!(
                zh_cn: "应用天赋「聆听忏悔的幽影」效果",
                en: "Use Talent「Regina Probationum」"
            ),
            config: ItemConfigType::Bool { default: true },
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: RosariaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use RosariaDamageEnum::*;
        let ratio = match s {
            Normal1 => ROSARIA_SKILL.normal_dmg1[s1],
            Normal2 => ROSARIA_SKILL.normal_dmg2[s1],
            Normal3 => ROSARIA_SKILL.normal_dmg3[s1],
            Normal4 => ROSARIA_SKILL.normal_dmg4[s1],
            Normal51 => ROSARIA_SKILL.normal_dmg51[s1],
            Normal52 => ROSARIA_SKILL.normal_dmg51[s1],
            Charged => ROSARIA_SKILL.charged_dmg1[s1],
            Plunging1 => ROSARIA_SKILL.plunging_dmg1[s1],
            Plunging2 => ROSARIA_SKILL.plunging_dmg2[s1],
            Plunging3 => ROSARIA_SKILL.plunging_dmg3[s1],
            E11 => ROSARIA_SKILL.elemental_skill_dmg11[s2],
            E12 => ROSARIA_SKILL.elemental_skill_dmg12[s2],
            Q11 => ROSARIA_SKILL.elemental_burst_dmg11[s3],
            Q12 => ROSARIA_SKILL.elemental_burst_dmg12[s3],
            Q2 => ROSARIA_SKILL.elemental_burst_dmg2[s3]
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        /*let e_from_behind = match *config {
            CharacterSkillConfig::Rosaria { e_from_behind } => e_from_behind,
            _ => false
        };
        if s == E12 && e_from_behind && context.character_common_data.has_talent1 {
            builder.add_extra_critical("罗莎莉亚天赋：聆听忏悔的幽影", 0.12);
        }*/

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(RosariaEffect::new(common_data, config)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: RosariaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            RosariaRoleEnum::Freezing => Box::new(RosariaDefaultTargetFunction::default())
        }
    }
}
