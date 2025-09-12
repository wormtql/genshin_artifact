use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::clorinde::ClorindeDamageEnum;
use crate::character::characters::electro::iansan::IansanDamageEnum;
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

pub struct VaresaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    pub passion_normal_dmg1: [f64; 15],
    pub passion_normal_dmg2: [f64; 15],
    pub passion_normal_dmg3: [f64; 15],
    pub passion_charged_dmg: [f64; 15],
    pub passion_plunging_dmg1: [f64; 15],
    pub passion_plunging_dmg2: [f64; 15],
    pub passion_plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub passion_e_dmg1: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub passion_q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const VARESA_SKILL: VaresaSkillType = VaresaSkillType {
    normal_dmg1: [0.4678, 0.5029, 0.538, 0.5847, 0.6198, 0.6549, 0.7017, 0.7485, 0.7952, 0.842, 0.8888, 0.9356, 0.994, 1.0525, 1.111],
    normal_dmg2: [0.4003, 0.4303, 0.4603, 0.5004, 0.5304, 0.5604, 0.6004, 0.6404, 0.6805, 0.7205, 0.7605, 0.8006, 0.8506, 0.9006, 0.9507],
    normal_dmg3: [0.5631, 0.6054, 0.6476, 0.7039, 0.7461, 0.7884, 0.8447, 0.901, 0.9573, 1.0136, 1.0699, 1.1263, 1.1966, 1.267, 1.3374],
    charged_dmg: [0.8928, 0.9598, 1.0267, 1.116, 1.183, 1.2499, 1.3392, 1.4285, 1.5178, 1.607, 1.6963, 1.7856, 1.8972, 2.0088, 2.1204],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    passion_normal_dmg1: [0.5441, 0.5849, 0.6257, 0.6801, 0.7209, 0.7617, 0.8161, 0.8705, 0.9249, 0.9793, 1.0337, 1.0881, 1.1562, 1.2242, 1.2922],
    passion_normal_dmg2: [0.5203, 0.5593, 0.5983, 0.6504, 0.6894, 0.7284, 0.7804, 0.8325, 0.8845, 0.9365, 0.9885, 1.0406, 1.1056, 1.1706, 1.2357],
    passion_normal_dmg3: [0.7359, 0.7911, 0.8462, 0.9198, 0.975, 1.0302, 1.1038, 1.1774, 1.251, 1.3246, 1.3981, 1.4717, 1.5637, 1.6557, 1.7477],
    passion_charged_dmg: [0.9264, 0.9959, 1.0654, 1.158, 1.2275, 1.297, 1.3896, 1.4822, 1.5749, 1.6675, 1.7602, 1.8528, 1.9686, 2.0844, 2.2002],
    passion_plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    passion_plunging_dmg2: [2.2372, 2.4193, 2.6013, 2.8615, 3.0436, 3.2517, 3.5378, 3.824, 4.1101, 4.4223, 4.7345, 5.0466, 5.3588, 5.6709, 5.9831],
    passion_plunging_dmg3: [2.7943, 3.0218, 3.2492, 3.5741, 3.8016, 4.0615, 4.4189, 4.7764, 5.1338, 5.5237, 5.9136, 6.3035, 6.6934, 7.0833, 7.4732],
    e_dmg1: [0.7448, 0.8007, 0.8565, 0.931, 0.9869, 1.0427, 1.1172, 1.1917, 1.2662, 1.3406, 1.4151, 1.4896, 1.5827, 1.6758, 1.7689],
    passion_e_dmg1: [1.064, 1.1438, 1.2236, 1.33, 1.4098, 1.4896, 1.596, 1.7024, 1.8088, 1.9152, 2.0216, 2.128, 2.261, 2.394, 2.527],
    q_dmg1: [3.4512, 3.71, 3.9689, 4.314, 4.5728, 4.8317, 5.1768, 5.5219, 5.867, 6.2122, 6.5573, 6.9024, 7.3338, 7.7652, 8.1966],
    passion_q_dmg1: [5.752, 6.1834, 6.6148, 7.19, 7.6214, 8.0528, 8.628, 9.2032, 9.7784, 10.3536, 10.9288, 11.504, 12.223, 12.942, 13.661],
    q_dmg2: [4.0264, 4.3284, 4.6304, 5.033, 5.335, 5.637, 6.0396, 6.4422, 6.8449, 7.2475, 7.6502, 8.0528, 8.5561, 9.0594, 9.5627],
};

damage_enum!(
    VaresaDamageEnum
    Normal1
    Normal2
    Normal3
    Charged
    Plunging1
    Plunging2
    Plunging3
    PassionNormal1
    PassionNormal2
    PassionNormal3
    PassionCharged
    PassionPlunging1
    PassionPlunging2
    PassionPlunging3
    E1
    PassionE1
    Q1
    PassionQ1
    Q2
);

impl VaresaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use VaresaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | PassionNormal1 | PassionNormal2 | PassionNormal3 => SkillType::NormalAttack,
            Charged | PassionCharged => SkillType::ChargedAttack,
            Plunging1 | PassionPlunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 | PassionPlunging2 | PassionPlunging3 => SkillType::PlungingAttackOnGround,
            E1 | PassionE1 => SkillType::ElementalSkill,
            Q1 | PassionQ1 => SkillType::ElementalBurst,
            Q2 => SkillType::PlungingAttackOnGround
        }
    }
}

struct VaresaEffect {
    pub talent2_stack: f64,
    pub has_talent2: bool,
    pub talent1_rate: f64,
    pub has_talent1: bool,
    pub passion: bool,
}

impl<A: Attribute> ChangeAttribute<A> for VaresaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent2 {
            attribute.add_atk_percentage("天赋「英雄，二度归来！」", 0.35 * self.talent2_stack);
        }

        if self.has_talent1 {
            let bonus = if self.passion { 1.8 } else { 0.5 };
            attribute.set_value_by(AttributeName::ATKRatioPlungingAttack, "天赋「连势，三重腾跃！」", bonus * self.talent1_rate);
        }

        attribute.set_value_by(AttributeName::USER1, "", if self.passion { 1.0 } else { 0.0 });
    }
}

pub struct Varesa;

impl CharacterTrait for Varesa {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Varesa,
        internal_name: "Varesa",
        name_locale: locale!(
            zh_cn: "瓦雷莎",
            en: "Varesa"
        ),
        element: Element::Electro,
        hp: [989, 2564, 3412, 5106, 5708, 6567, 7370, 8238, 8840, 9716, 10318, 11204, 11806, 12699, 13602],
        atk: [28, 72, 96, 143, 160, 184, 207, 231, 248, 273, 290, 314, 331, 356, 437],
        def: [61, 158, 210, 314, 351, 404, 454, 507, 544, 598, 635, 690, 727, 782, 837],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "角力搏摔",
            en: "By the Horns"
        ),
        skill_name2: locale!(
            zh_cn: "夜虹逐跃",
            en: "Riding the Night-Rainbow"
        ),
        skill_name3: locale!(
            zh_cn: "闪烈降临！",
            en: "Guardian Vent!"
        ),
    };
    type SkillType = VaresaSkillType;
    const SKILL: Self::SkillType = VARESA_SKILL;
    type DamageEnumType = VaresaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            VaresaDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            VaresaDamageEnum
            E1 locale!(zh_cn: "突进伤害", en: "Rush DMG")
        ),
        skill3: skill_map!(
            VaresaDamageEnum
            Q1 locale!(zh_cn: "飞踢伤害", en: "Flying Kick DMG")
            Q2 locale!(zh_cn: "「大火山崩落」伤害", en: "Volcano Kablam DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_rate",
            title: locale!(
                zh_cn: "天赋「连势，三重腾跃！」比例",
                en: "Talent 1 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "talent2_stack",
            title: locale!(
                zh_cn: "天赋「英雄，二度归来！」层数",
                en: "Talent 2 Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 }
        },
        ItemConfig {
            name: "passion",
            title: locale!(
                zh_cn: "炽热激情状态",
                en: "Fiery Passion"
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: VaresaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use VaresaDamageEnum::*;
        let mut builder = D::new();

        let passion = context.attribute.get_value(AttributeName::USER1) > 0.0;
        let ratio = match s {
            Normal1 => if passion { VARESA_SKILL.passion_normal_dmg1[s1] } else { VARESA_SKILL.normal_dmg1[s1] },
            Normal2 => if passion { VARESA_SKILL.passion_normal_dmg2[s1] } else { VARESA_SKILL.normal_dmg2[s1] },
            Normal3 => if passion { VARESA_SKILL.passion_normal_dmg3[s1] } else { VARESA_SKILL.normal_dmg3[s1] },
            Charged => if passion { VARESA_SKILL.passion_charged_dmg[s1] } else { VARESA_SKILL.charged_dmg[s1] },
            Plunging1 => if passion { VARESA_SKILL.passion_plunging_dmg1[s1] } else { VARESA_SKILL.plunging_dmg1[s1] },
            Plunging2 => if passion { VARESA_SKILL.passion_plunging_dmg2[s1] } else { VARESA_SKILL.plunging_dmg2[s1] },
            Plunging3 => if passion { VARESA_SKILL.passion_plunging_dmg3[s1] } else { VARESA_SKILL.plunging_dmg3[s1] },
            E1 => if passion { VARESA_SKILL.passion_e_dmg1[s2] } else { VARESA_SKILL.e_dmg1[s2] },
            Q1 => if passion { VARESA_SKILL.passion_q_dmg1[s3] } else { VARESA_SKILL.q_dmg1[s3] },
            Q2 => VARESA_SKILL.q_dmg2[s3],

            PassionNormal1 => VARESA_SKILL.passion_normal_dmg1[s1],
            PassionNormal2 => VARESA_SKILL.passion_normal_dmg2[s1],
            PassionNormal3 => VARESA_SKILL.passion_normal_dmg3[s1],
            PassionCharged => VARESA_SKILL.passion_charged_dmg[s1],
            PassionPlunging1 => VARESA_SKILL.passion_plunging_dmg1[s1],
            PassionPlunging2 => VARESA_SKILL.passion_plunging_dmg2[s1],
            PassionPlunging3 => VARESA_SKILL.passion_plunging_dmg3[s1],
            PassionE1 => VARESA_SKILL.passion_e_dmg1[s2],
            PassionQ1 => VARESA_SKILL.passion_q_dmg1[s3]
        };
        builder.add_atk_ratio("技能倍率", ratio);

        let skill_type = s.get_skill_type();
        if context.character_common_data.constellation >= 6 {
            if skill_type == SkillType::ElementalBurst || skill_type == SkillType::PlungingAttackOnGround {
                builder.add_extra_critical("C6「正义英雄的凯旋」", 0.1);
                builder.add_extra_critical_damage("C6「正义英雄的凯旋」", 1.0);
            }
        }

        // TODO C4

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Electro,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match config {
            &CharacterConfig::Varesa { talent1_rate, talent2_stack, passion } => Some(Box::new(
                VaresaEffect {
                    talent1_rate, talent2_stack, passion,
                    has_talent1: common_data.has_talent1,
                    has_talent2: common_data.has_talent2
                }
            )),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
