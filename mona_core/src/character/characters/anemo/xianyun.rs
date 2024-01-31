use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
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

pub struct XianyunSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],
    pub e_dmg4: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
    pub q_heal1: [f64; 15],
    pub q_heal1_fixed: [f64; 15],
    pub q_heal2: [f64; 15],
    pub q_heal2_fixed: [f64; 15],
}

pub const XIANYUN_SKILL: XianyunSkillType = XianyunSkillType {
    normal_dmg1: [0.403, 0.4333, 0.4635, 0.5038, 0.534, 0.5642, 0.6045, 0.6448, 0.6851, 0.7254, 0.7657, 0.806, 0.8564, 0.9068, 0.9572],
    normal_dmg2: [0.3886, 0.4177, 0.4468, 0.4857, 0.5148, 0.544, 0.5828, 0.6217, 0.6605, 0.6994, 0.7382, 0.7771, 0.8257, 0.8742, 0.9228],
    normal_dmg3: [0.4888, 0.5254, 0.5621, 0.611, 0.6476, 0.6843, 0.7332, 0.782, 0.8309, 0.8798, 0.9287, 0.9776, 1.0386, 1.0997, 1.1608],
    normal_dmg4: [0.6492, 0.6979, 0.7465, 0.8115, 0.8601, 0.9088, 0.9738, 1.0387, 1.1036, 1.1685, 1.2334, 1.2983, 1.3795, 1.4606, 1.5418],
    charged_dmg1: [1.2312, 1.3235, 1.4159, 1.539, 1.6313, 1.7237, 1.8468, 1.9699, 2.093, 2.2162, 2.3393, 2.4624, 2.6163, 2.7702, 2.9241],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [0.248, 0.2666, 0.2852, 0.31, 0.3286, 0.3472, 0.372, 0.3968, 0.4216, 0.4464, 0.4712, 0.496, 0.527, 0.558, 0.589],
    e_dmg2: [1.16, 1.247, 1.334, 1.45, 1.537, 1.624, 1.74, 1.856, 1.972, 2.088, 2.204, 2.32, 2.465, 2.61, 2.755],
    e_dmg3: [1.48, 1.591, 1.702, 1.85, 1.961, 2.072, 2.22, 2.368, 2.516, 2.664, 2.812, 2.96, 3.145, 3.33, 3.515],
    e_dmg4: [3.376, 3.6292, 3.8824, 4.22, 4.4732, 4.7264, 5.064, 5.4016, 5.7392, 6.0768, 6.4144, 6.752, 7.174, 7.596, 8.018],
    q_dmg1: [1.08, 1.161, 1.242, 1.35, 1.431, 1.512, 1.62, 1.728, 1.836, 1.944, 2.052, 2.16, 2.295, 2.43, 2.565],
    q_dmg2: [0.392, 0.4214, 0.4508, 0.49, 0.5194, 0.5488, 0.588, 0.6272, 0.6664, 0.7056, 0.7448, 0.784, 0.833, 0.882, 0.931],
    q_heal1: [0.9216, 0.9907, 1.0598, 1.152, 1.2211, 1.2902, 1.3824, 1.4746, 1.5667, 1.6589, 1.751, 1.8432, 1.9584, 2.0736, 2.1888],
    q_heal1_fixed: [577.78, 635.57, 698.17, 765.59, 837.82, 914.87, 996.73, 1083.41, 1174.91, 1271.22, 1372.34, 1478.28, 1589.04, 1704.61, 1825.0],
    q_heal2: [0.4301, 0.4623, 0.4946, 0.5376, 0.5699, 0.6021, 0.6451, 0.6881, 0.7311, 0.7741, 0.8172, 0.8602, 0.9139, 0.9677, 1.0214],
    q_heal2_fixed: [269.63, 296.6, 325.81, 357.27, 390.98, 426.94, 465.14, 505.59, 548.29, 593.23, 640.43, 689.87, 741.55, 795.49, 851.67],
};

damage_enum!(
    XianyunDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Charged1 // same as Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    E4
    E5
    Q1
    Q2
    QHeal1
    QHeal2
);

impl XianyunDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use XianyunDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged | Charged1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 | E2 | E3 | E4 => SkillType::PlungingAttackOnGround,
            E1 | E5 => SkillType::ElementalSkill,
            Q1 | Q2 | QHeal1 | QHeal2 => SkillType::ElementalBurst
        }
    }

    pub fn is_heal(&self) -> bool {
        use XianyunDamageEnum::*;
        match *self {
            QHeal1 | QHeal2 => true,
            _ => false
        }
    }
}

pub struct XianyunEffect {
    pub talent1_stack: f64,
    pub talent2_rate: f64,
    pub constellation: usize,
    pub butianti_count: usize,
}

impl<A: Attribute> ChangeAttribute<A> for XianyunEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus_crit = if self.talent1_stack > 1e-6 && self.talent1_stack <= 1.0 {
            0.04 * self.talent1_stack
        } else if self.talent1_stack <= 1e-6 {
            0.0
        } else {
            0.02 + 0.02 * self.talent1_stack
        };
        attribute.set_value_by(AttributeName::CriticalPlungingAttack, "天赋「霜翎高逐祥风势」", bonus_crit);

        let rate = self.talent2_rate;
        let factor = if self.constellation >= 2 { 2.0 } else { 1.0 };
        attribute.add_edge1(
            AttributeName::ATK,
            AttributeName::ExtraDmgPlungingAttackLowHigh,
            Box::new(move |atk, _| (9000.0_f64 * factor).min(atk * 2.0 * rate * factor)),
            Box::new(|_x, _y, _z| (0.0, 0.0)),
            "天赋「细想应是洞中仙」"
        );

        // c2 atk bonus
        if self.constellation >= 2 {
            let bonus_atk = if self.butianti_count > 0 { 0.2 } else { 0.0 };
            attribute.add_atk_percentage("C2「鹤唳远人间」", bonus_atk);
        }

        // c6 bonus
        if self.constellation >= 6 {
            let bonus_cd = match self.butianti_count {
                0 => 0.0,
                1 => 0.15,
                2 => 0.35,
                _ => 0.7
            };
            attribute.set_value_by(AttributeName::USER1, "C6「知是留云僊」", bonus_cd);
        }
    }
}

pub struct Xianyun;

impl CharacterTrait for Xianyun {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Xianyun,
        internal_name: "Xianyun",
        name_locale: locale!(
            zh_cn: "闲云",
            en: "Xianyun"
        ),
        element: Element::Anemo,
        hp: [810, 2102, 2797, 4185, 4678, 5383, 6041, 6752, 7246, 7964, 8458, 9184, 9677, 10409],
        atk: [26, 68, 90, 135, 150, 173, 194, 217, 233, 256, 272, 295, 311, 335],
        def: [45, 116, 154, 230, 257, 296, 332, 371, 399, 438, 465, 505, 532, 573],
        sub_stat: CharacterSubStatFamily::ATK288,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·清风散花词",
            en: "Normal Attack: Word of Wind and Flower"
        ),
        skill_name2: locale!(
            zh_cn: "朝起鹤云",
            en: "White Clouds at Dawn"
        ),
        skill_name3: locale!(
            zh_cn: "暮集竹星",
            en: "Stars Gather at Dusk"
        )
    };
    type SkillType = XianyunSkillType;
    const SKILL: Self::SkillType = XIANYUN_SKILL;
    type DamageEnumType = XianyunDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            XianyunDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            XianyunDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E2 locale!(zh_cn: "闲云冲击波伤害-1", en: "Driftcloud Wave DMG-1")
            E3 locale!(zh_cn: "闲云冲击波伤害-2", en: "Driftcloud Wave DMG-2")
            E4 locale!(zh_cn: "闲云冲击波伤害-3", en: "Driftcloud Wave DMG-3")
            E5 locale!(zh_cn: "三段跳·鹤形追击伤害", en: "Chasing Crane: Third Leap")
        ),
        skill3: skill_map!(
            XianyunDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "竹星伤害", en: "Starwicker DMG")
            QHeal1 locale!(zh_cn: "治疗量", en: "Healing")
            QHeal2 locale!(zh_cn: "持续治疗量", en: "Continuous Healing")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_stack",
            title: locale!(zh_cn: "「风翎」层数", en: "Storm Pinion Stack"),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 4.0 }
        },
        ItemConfig {
            name: "talent2_rate",
            title: locale!(zh_cn: "天赋2「细想应是洞中仙」", en: "Talent 2"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "butianti_count",
            title: locale!(zh_cn: "步天梯次数", en: "Skyladder Count"),
            config: ItemConfigType::Int { min: 0, max: 3, default: 3 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: XianyunDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use XianyunDamageEnum::*;

        let mut builder = D::new();
        if s.is_heal() {
            let (ratio, fixed) = match s {
                QHeal1 => (XIANYUN_SKILL.q_heal1[s3], XIANYUN_SKILL.q_heal1_fixed[s3]),
                QHeal2 => (XIANYUN_SKILL.q_heal2[s3], XIANYUN_SKILL.q_heal2_fixed[s3]),
                _ => (0.0, 0.0)
            };
            builder.add_atk_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => XIANYUN_SKILL.normal_dmg1[s1],
                Normal2 => XIANYUN_SKILL.normal_dmg2[s1],
                Normal3 => XIANYUN_SKILL.normal_dmg3[s1],
                Normal4 => XIANYUN_SKILL.normal_dmg4[s1],
                Charged | Charged1 => XIANYUN_SKILL.charged_dmg1[s1],
                Plunging1 => XIANYUN_SKILL.plunging_dmg1[s1],
                Plunging2 => XIANYUN_SKILL.plunging_dmg2[s1],
                Plunging3 => XIANYUN_SKILL.plunging_dmg3[s1],
                E1 => XIANYUN_SKILL.e_dmg1[s2],
                E2 => XIANYUN_SKILL.e_dmg2[s2],
                E3 => XIANYUN_SKILL.e_dmg3[s2],
                E4 | E5 => XIANYUN_SKILL.e_dmg4[s2],
                Q1 => XIANYUN_SKILL.q_dmg1[s3],
                Q2 => XIANYUN_SKILL.q_dmg2[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);

            if s == E2 || s == E3 || s == E4 {
                builder.add_extra_critical_damage("C6「知是留云僊」", context.attribute.get_value(AttributeName::USER1));
            }
            builder.damage(
                &context.attribute,
                &context.enemy,
                Element::Anemo,
                s.get_skill_type(),
                context.character_common_data.level,
                fumo
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Xianyun {talent1_stack, talent2_rate, butianti_count} => {
                Some(Box::new(XianyunEffect {
                    talent1_stack,
                    talent2_rate,
                    butianti_count,
                    constellation: common_data.constellation as usize,
                }))
            },
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
