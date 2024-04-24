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

pub struct ArlecchinoSkill {
    pub a_bonus: [f64; 15],
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub normal_dmg6: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_heal1_hp_atk: [f64; 15],
}

pub const ARLECCHINO_SKILL: ArlecchinoSkill = ArlecchinoSkill {
    a_bonus: [1.204, 1.302, 1.4, 1.54, 1.638, 1.75, 1.904, 2.058, 2.212, 2.38, 2.548, 2.716, 2.884, 3.052, 3.22],
    normal_dmg1: [0.475, 0.5137, 0.5523, 0.6076, 0.6462, 0.6904, 0.7512, 0.8119, 0.8727, 0.939, 1.0052, 1.0715, 1.1378, 1.2041, 1.2704],
    normal_dmg2: [0.5211, 0.5635, 0.6059, 0.6665, 0.7089, 0.7574, 0.824, 0.8906, 0.9573, 1.03, 1.1027, 1.1754, 1.2481, 1.3208, 1.3935],
    normal_dmg3: [0.6539, 0.7071, 0.7603, 0.8363, 0.8896, 0.9504, 1.034, 1.1176, 1.2013, 1.2925, 1.3837, 1.475, 1.5662, 1.6575, 1.7487],
    normal_dmg4: [0.3715, 0.4017, 0.4319, 0.4751, 0.5053, 0.5399, 0.5874, 0.6349, 0.6824, 0.7343, 0.7861, 0.8379, 0.8898, 0.9416, 0.9934],
    normal_dmg5: [0.6998, 0.7568, 0.8137, 0.8951, 0.9521, 1.0172, 1.1067, 1.1962, 1.2857, 1.3834, 1.481, 1.5787, 1.6763, 1.774, 1.8716],
    normal_dmg6: [0.8538, 0.9233, 0.9928, 1.092, 1.1615, 1.241, 1.3502, 1.4594, 1.5686, 1.6877, 1.8068, 1.926, 2.0451, 2.1642, 2.2834],
    charged_dmg1: [0.9082, 0.9821, 1.056, 1.1616, 1.2355, 1.32, 1.4362, 1.5523, 1.6685, 1.7952, 1.9219, 2.0486, 2.1754, 2.3021, 2.4288],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [0.1484, 0.1595, 0.1707, 0.1855, 0.1966, 0.2078, 0.2226, 0.2374, 0.2523, 0.2671, 0.282, 0.2968, 0.3153, 0.3339, 0.3525],
    e_dmg2: [1.3356, 1.4358, 1.5359, 1.6695, 1.7697, 1.8698, 2.0034, 2.137, 2.2705, 2.4041, 2.5376, 2.6712, 2.8382, 3.0051, 3.172],
    e_dmg3: [0.318, 0.3419, 0.3657, 0.3975, 0.4214, 0.4452, 0.477, 0.5088, 0.5406, 0.5724, 0.6042, 0.636, 0.6758, 0.7155, 0.7552],
    q_dmg1: [3.704, 3.9818, 4.2596, 4.63, 4.9078, 5.1856, 5.556, 5.9264, 6.2968, 6.6672, 7.0376, 7.408, 7.871, 8.334, 8.797],
    q_heal1_hp_atk: [1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5],
};

damage_enum!(
    ArlecchinoDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Normal5
    Normal6
    Charged1
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    Q1
    QHeal1
);

impl ArlecchinoDamageEnum {
    pub fn get_element(&self, is_pyro: bool) -> Element {
        if is_pyro {
            return Element::Pyro;
        }
        use ArlecchinoDamageEnum::*;
        match *self {
            E1 | E2 | E3 | Q1 => Element::Pyro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ArlecchinoDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 | Normal6 => SkillType::NormalAttack,
            Charged1 | Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 | QHeal1 => SkillType::ElementalBurst
        }
    }
}

pub struct ArlecchinoEffect {
    pub has_talent2: bool,
    pub c6_ratio: f64,
    pub constellation: usize,
}

impl<A: Attribute> ChangeAttribute<A> for ArlecchinoEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // attribute.add_edge1(
        //     AttributeName::ATK,
        //     AttributeName::,
        //     Box::new(|atk, _| {
        //         0.2_f64.min(((atk - 1000.0).max(0.0) / 100.0).floor() * 0.01)
        //     }),
        //     Box::new(|_x, _y, _v| (0.0, 0.0)),
        //     "天赋「唯力量可守护」"
        // );
        attribute.set_value_by(AttributeName::BonusPyro, "天赋「唯厄月可知晓」", 0.4);
        if self.constellation >= 6 {
            attribute.set_value_by(AttributeName::CriticalNormalAttack, "C6加成", 0.1 * self.c6_ratio);
            attribute.set_value_by(AttributeName::CriticalElementalBurst, "C6加成", 0.1 * self.c6_ratio);
            attribute.set_value_by(AttributeName::CriticalDamageNormalAttack, "C6加成", 0.7 * self.c6_ratio);
            attribute.set_value_by(AttributeName::CriticalDamageElementalBurst, "C6加成", 0.7 * self.c6_ratio);
        }
        // attribute.set_value_by(AttributeName::CriticalNormalAttack)
    }
}

pub struct Arlecchino;

impl CharacterTrait for Arlecchino {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Arlecchino,
        internal_name: "Arlecchino",
        name_locale: locale!(
            zh_cn: "阿蕾奇诺",
            en: "Arlecchino"
        ),
        element: Element::Pyro,
        hp: [1020, 2646, 3521, 5268, 5889, 6776, 7604, 8500, 9121, 10025, 10647, 11561, 12182, 13103],
        atk: [27, 69, 92, 138, 154, 177, 198, 222, 238, 262, 278, 302, 318, 342],
        def: [60, 154, 205, 307, 344, 395, 444, 496, 532, 585, 621, 675, 711, 765],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Polearm,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·斩首之邀",
            en: "Normal Attack: Invitation to a Beheading"
        ),
        skill_name2: locale!(
            zh_cn: "万相化灰",
            en: "All is Ash"
        ),
        skill_name3: locale!(
            zh_cn: "厄月将升",
            en: "Balemoon Rising"
        ),
    };
    type SkillType = ArlecchinoSkill;
    const SKILL: Self::SkillType = ARLECCHINO_SKILL;
    type DamageEnumType = ArlecchinoDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ArlecchinoDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Normal5 hit_n_dmg!(5)
            Normal6 hit_n_dmg!(6)
            Charged1 charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ArlecchinoDamageEnum
            E1 locale!(zh_cn: "尖刺伤害", en: "Spike DMG")
            E2 locale!(zh_cn: "切斩伤害", en: "Cleave DMG")
            E3 locale!(zh_cn: "血偿勒令伤害", en: "Blood-Debt Directive DMG")
        ),
        skill3: skill_map!(
            ArlecchinoDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            QHeal1 locale!(zh_cn: "回复量", en: "Amount of HP Restored")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c6_ratio",
            title: locale!(zh_cn: "6命效果比例", en: "C6 Ratio"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "bond_of_life",
            title: locale!(zh_cn: "生命之契百分比", en: "Bond of Life percentage"),
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 0.0 },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ArlecchinoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use ArlecchinoDamageEnum::*;
        let bond_of_life = match *config {
            CharacterSkillConfig::Arlecchino { bond_of_life } => bond_of_life,
            _ => 0.0
        };

        let mut builder = D::new();

        if s == QHeal1 {
            let hp = context.attribute.get_hp();
            if bond_of_life > 0.0 {
                builder.add_hp_ratio("生命之契", 1.5 * bond_of_life);
            }
            builder.add_atk_ratio("技能倍率", 1.5);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => ARLECCHINO_SKILL.normal_dmg1[s1],
                Normal2 => ARLECCHINO_SKILL.normal_dmg2[s1],
                Normal3 => ARLECCHINO_SKILL.normal_dmg3[s1],
                Normal4 => ARLECCHINO_SKILL.normal_dmg4[s1] * 2.0,
                Normal5 => ARLECCHINO_SKILL.normal_dmg5[s1],
                Normal6 => ARLECCHINO_SKILL.normal_dmg6[s1],
                Charged | Charged1 => ARLECCHINO_SKILL.charged_dmg1[s1],
                Plunging1 => ARLECCHINO_SKILL.plunging_dmg1[s1],
                Plunging2 => ARLECCHINO_SKILL.plunging_dmg2[s1],
                Plunging3 => ARLECCHINO_SKILL.plunging_dmg3[s1],
                E1 => ARLECCHINO_SKILL.e_dmg1[s2],
                E2 => ARLECCHINO_SKILL.e_dmg2[s2],
                E3 => ARLECCHINO_SKILL.e_dmg3[s2],
                Q1 => ARLECCHINO_SKILL.q_dmg1[s3],
                _ => 0.0
            };
            let is_pyro = bond_of_life >= 0.3;
            let skill_type = s.get_skill_type();
            builder.add_atk_ratio("技能倍率", ratio);
            if is_pyro && skill_type == SkillType::NormalAttack {
                builder.add_atk_ratio("红死之宴", ARLECCHINO_SKILL.a_bonus[s1] * bond_of_life);
                if context.character_common_data.constellation >= 1 {
                    builder.add_atk_ratio("红死之宴-1命", bond_of_life);
                }
            }
            if s == Q1 && context.character_common_data.constellation >= 6 {
                if bond_of_life > 0.0 {
                    builder.add_atk_ratio("C6加成", 7.0 * bond_of_life);
                }

            }
            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(is_pyro),
                skill_type,
                context.character_common_data.level,
                fumo
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let c6_ratio = match *config {
            CharacterConfig::Arlecchino {c6_ratio} => c6_ratio,
            _ => 0.0
        };
        Some(Box::new(ArlecchinoEffect {
            has_talent2: common_data.has_talent2,
            c6_ratio,
            constellation: common_data.constellation as usize
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
