use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::clorinde::ClorindeDamageEnum;
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct IansanSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub high_ratio: f64,
    pub low_ratio: f64,
    pub max_atk_bonus: [f64; 15],
}

pub const IANSAN_SKILL: IansanSkillType = IansanSkillType {
    normal_dmg1: [0.4698, 0.508, 0.5462, 0.6009, 0.6391, 0.6828, 0.7429, 0.803, 0.863, 0.9286, 0.9941, 1.0597, 1.1252, 1.1908, 1.2563],
    normal_dmg2: [0.4276, 0.4625, 0.4973, 0.547, 0.5818, 0.6216, 0.6763, 0.731, 0.7857, 0.8453, 0.905, 0.9647, 1.0244, 1.084, 1.1437],
    normal_dmg3: [0.6439, 0.6963, 0.7487, 0.8236, 0.876, 0.9359, 1.0182, 1.1006, 1.1829, 1.2728, 1.3626, 1.4525, 1.5423, 1.6322, 1.722],
    charged_dmg1: [1.0028, 1.0844, 1.166, 1.2826, 1.3642, 1.4575, 1.5858, 1.714, 1.8423, 1.9822, 2.1221, 2.262, 2.402, 2.5419, 2.6818],
    charged_dmg2: [0.8419, 0.9105, 0.979, 1.0769, 1.1454, 1.2238, 1.3314, 1.4391, 1.5468, 1.6643, 1.7818, 1.8993, 2.0167, 2.1342, 2.2517],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [2.864, 3.0788, 3.2936, 3.58, 3.7948, 4.0096, 4.296, 4.5824, 4.8688, 5.1552, 5.4416, 5.728, 6.086, 6.444, 6.802],
    q_dmg1: [4.304, 4.6268, 4.9496, 5.38, 5.7028, 6.0256, 6.456, 6.8864, 7.3168, 7.7472, 8.1776, 8.608, 9.146, 9.684, 10.222],
    high_ratio: 0.27,
    low_ratio: 0.005,
    max_atk_bonus: [330.0, 370.0, 410.0, 450.0, 490.0, 530.0, 570.0, 610.0, 650.0, 690.0, 730.0, 770.0, 810.0, 850.0, 890.0]
};

damage_enum!(
    IansanDamageEnum
    Normal1
    Normal2
    Normal3
    Charged1
    Charged2
    Plunging1
    Plunging2
    Plunging3
    E1
    Q1
);

impl IansanDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use IansanDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self) -> Element {
        use IansanDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            Charged2 | E1 | Q1 => Element::Electro
        }
    }
}

struct IansanEffect {
    pub talent1_rate: f64,
    pub has_talent1: bool
}

impl<A: Attribute> ChangeAttribute<A> for IansanEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent1 {
            attribute.add_atk_percentage("天赋「强化抗阻练习」", 0.2 * self.talent1_rate);
        }
    }
}

pub struct Iansan;

impl CharacterTrait for Iansan {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Iansan,
        internal_name: "Iansan",
        name_locale: locale!(
            zh_cn: "伊安珊",
            en: "Iansan"
        ),
        element: Element::Electro,
        hp: [894, 2296, 2963, 4438, 4913, 5651, 6283, 7021, 7495, 8233, 8707, 9445, 9919, 10657],
        atk: [22, 55, 71, 107, 118, 136, 152, 169, 181, 199, 210, 228, 239, 257],
        def: [54, 137, 177, 266, 294, 338, 376, 420, 449, 493, 521, 566, 594, 638],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Polearm,
        star: 4,
        skill_name1: locale!(
            zh_cn: "负重锥击",
            en: "Weighted Spike"
        ),
        skill_name2: locale!(
            zh_cn: "电掣雷驰",
            en: "Thunderbolt Rush"
        ),
        skill_name3: locale!(
            zh_cn: "力的三原理",
            en: "The Three Principles of Power"
        ),
    };
    type SkillType = IansanSkillType;
    const SKILL: Self::SkillType = IANSAN_SKILL;
    type DamageEnumType = IansanDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            IansanDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged1 charged_dmg!()
            Charged2 locale!(zh_cn: "雷霆飞缒伤害", en: "Swift Stormflight DMG")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            IansanDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            IansanDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_rate",
            title: locale!(
                zh_cn: "天赋「强化抗阻练习」比例",
                en: "Talent 'Enhanced Resistance Training' Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: IansanDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use IansanDamageEnum::*;
        let mut builder = D::new();

        let ratio = match s {
            Normal1 => IANSAN_SKILL.normal_dmg1[s1],
            Normal2 => IANSAN_SKILL.normal_dmg2[s1],
            Normal3 => IANSAN_SKILL.normal_dmg3[s1],
            Charged1 => IANSAN_SKILL.charged_dmg1[s1],
            Charged2 => IANSAN_SKILL.charged_dmg2[s1],
            Plunging1 => IANSAN_SKILL.plunging_dmg1[s1],
            Plunging2 => IANSAN_SKILL.plunging_dmg2[s1],
            Plunging3 => IANSAN_SKILL.plunging_dmg3[s1],
            E1 => IANSAN_SKILL.e_dmg1[s2],
            Q1 => IANSAN_SKILL.q_dmg1[s3],
        };
        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Iansan { talent1_rate } => Some(Box::new(IansanEffect {
                talent1_rate,
                has_talent1: common_data.has_talent1
            })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
