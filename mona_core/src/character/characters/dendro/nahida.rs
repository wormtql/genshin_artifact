use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait, CharacterSkillMapItem};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::macros::{skill_type, damage_enum, skill_map, damage_ratio};
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};


pub struct NahidaSkillType {
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
    pub e_dmg3_1: [f64; 15],
    pub e_dmg3_2: [f64; 15],

    pub q_bonus1: [f64; 15],
    pub q_bonus2: [f64; 15],
}

pub const NAHIDA_SKILL: NahidaSkillType=  NahidaSkillType {
    normal_dmg1: [0.403, 0.4333, 0.4635, 0.5038, 0.534, 0.5643, 0.6046, 0.6449, 0.6852, 0.7255, 0.7658, 0.8061, 0.8565, 0.9069, 0.9572],
    normal_dmg2: [0.3697, 0.3975, 0.4252, 0.4622, 0.4899, 0.5176, 0.5546, 0.5916, 0.6286, 0.6655, 0.7025, 0.7395, 0.7857, 0.8319, 0.8781],
    normal_dmg3: [0.4587, 0.4931, 0.5276, 0.5734, 0.6078, 0.6422, 0.6881, 0.734, 0.7799, 0.8257, 0.8716, 0.9175, 0.9748, 1.0322, 1.0895],
    normal_dmg4: [0.5841, 0.6279, 0.6717, 0.7301, 0.7739, 0.8177, 0.8761, 0.9345, 0.9929, 1.0513, 1.1097, 1.1681, 1.2411, 1.3141, 1.3872],
    charged_dmg1: [1.32, 1.419, 1.518, 1.65, 1.749, 1.848, 1.98, 2.112, 2.244, 2.376, 2.508, 2.64, 2.805, 2.97, 3.135],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [0.984, 1.0578, 1.1316, 1.23, 1.3038, 1.3776, 1.476, 1.5744, 1.6728, 1.7712, 1.8696, 1.968, 2.091, 2.214, 2.337],
    e_dmg2: [1.304, 1.4018, 1.4996, 1.63, 1.7278, 1.8256, 1.956, 2.0864, 2.2168, 2.3472, 2.4776, 2.608, 2.771, 2.934, 3.097],
    e_dmg3_1: [1.032, 1.1094, 1.1868, 1.29, 1.3674, 1.4448, 1.548, 1.6512, 1.7544, 1.8576, 1.9608, 2.064, 2.193, 2.322, 2.451],
    e_dmg3_2: [2.064, 2.2188, 2.3736, 2.58, 2.7348, 2.8896, 3.096, 3.3024, 3.5088, 3.7152, 3.9216, 4.128, 4.386, 4.644, 4.902],
    q_bonus1: [0.1488, 0.16, 0.1711, 0.186, 0.1972, 0.2083, 0.2232, 0.2381, 0.253, 0.2678, 0.2827, 0.2976, 0.3162, 0.3348, 0.3534],
    q_bonus2: [0.2232, 0.2399, 0.2567, 0.279, 0.2957, 0.3125, 0.3348, 0.3571, 0.3794, 0.4018, 0.4241, 0.4464, 0.4743, 0.5022, 0.5301],
};

damage_enum!(
    NahidaDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
);

impl NahidaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use NahidaDamageEnum::*;
        match *self {
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Charged => SkillType::ChargedAttack,
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack
        }
    }
}

pub struct NahidaEffect {
    pub c4: bool,
    pub e_count: usize,
}

impl<A: Attribute> ChangeAttribute<A> for NahidaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.c4 {
            let em = 20.0 * self.e_count as f64 + 80.0;
            attribute.set_value_by(AttributeName::ElementalMastery, "C4: 比量现行之茎", em);
        }
    }
}

pub struct Nahida;

impl CharacterTrait for Nahida {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Nahida,
        internal_name: "Nahida",
        element: Element::Dendro,
        hp: [807, 2092, 2784, 4165, 4656, 5357, 6012, 6721, 7212, 7926, 8418, 9140, 9632, 10360],
        atk: [23, 60, 80, 120, 134, 155, 174, 194, 208, 229, 243, 264, 278, 299],
        def: [49, 127, 169, 253, 283, 326, 366, 409, 439, 482, 512, 556, 586, 630],
        sub_stat: CharacterSubStatFamily::ElementalMastery115,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·行相",
            en: "Normal Attack: Akara",
        ),
        skill_name2: locale!(
            zh_cn: "所闻遍计",
            en: "All Schemes to Know",
        ),
        skill_name3: locale!(
            zh_cn: "心景幻成",
            en: "Illusory Heart",
        ),
        name_locale: locale!(
            zh_cn: "纳西妲",
            en: "Nahida",
        )
    };
    type SkillType = NahidaSkillType;
    const SKILL: Self::SkillType = NAHIDA_SKILL;
    type DamageEnumType = NahidaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            NahidaDamageEnum
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
            NahidaDamageEnum
            E1 locale!(zh_cn: "点按伤害", en: "Tapping DMG")
            E2 locale!(zh_cn: "长按伤害", en: "Hold DMG")
            E3 locale!(zh_cn: "灭净三业伤害", en: "Tri-Karma Purification DMG")
        ),
        skill3: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c4_e_count",
            title: locale!(
                zh_cn: "（4命）蕴种印状态敌人数量",
                en: "(C4) Schemes to Know‘s Seeds of Skandha Enemy Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 4 },
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "q_bonus",
            title: locale!(
                zh_cn: "Q技能火元素加伤",
                en: "Q Pyro Bonus",
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "q_bonus_count",
            title: locale!(
                zh_cn: "Q技能火元素个数",
                en: "Q Pyro Count",
            ),
            config: ItemConfigType::Int { min: 1, max: 2, default: 2 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: NahidaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use NahidaDamageEnum::*;

        let mut builder = D::new();

        if s != NahidaDamageEnum::E3 {
            let ratio = match s {
                Normal1 => NAHIDA_SKILL.normal_dmg1[s1],
                Normal2 => NAHIDA_SKILL.normal_dmg2[s1],
                Normal3 => NAHIDA_SKILL.normal_dmg3[s1],
                Normal4 => NAHIDA_SKILL.normal_dmg4[s1],
                Charged => NAHIDA_SKILL.charged_dmg1[s1],
                Plunging1 => NAHIDA_SKILL.plunging_dmg1[s1],
                Plunging2 => NAHIDA_SKILL.plunging_dmg2[s1],
                Plunging3 => NAHIDA_SKILL.plunging_dmg3[s1],
                E1 => NAHIDA_SKILL.e_dmg1[s2],
                E2 => NAHIDA_SKILL.e_dmg2[s2],
                _ => 0.0
            };
            builder.add_atk_ratio("技能倍率", ratio);
        } else {
            let ratio_atk = NAHIDA_SKILL.e_dmg3_1[s2];
            let ratio_em = NAHIDA_SKILL.e_dmg3_2[s2];

            builder.add_atk_ratio("技能倍率", ratio_atk);
            builder.add_em_ratio("技能倍率", ratio_em);

            if context.character_common_data.has_talent2 {
                let em = context.attribute.get_em_all();
                if em > 200.0 {
                    let bonus = 0.8_f64.min((em - 200.0) * 0.001);
                    let bonus_crit = 0.24_f64.min((em - 200.0) * 0.0003);

                    builder.add_extra_critical("慧明缘觉智论", bonus_crit);
                    builder.add_extra_bonus("慧明缘觉智论", bonus);
                }
            }

            let (q_bonus, q_bonus_count) = match *config {
                CharacterSkillConfig::Nahida { q_bonus, q_bonus_count } => (q_bonus, q_bonus_count),
                _ => (false, 0)
            };
            if q_bonus {
                let bonus = if q_bonus_count == 1 {
                    NAHIDA_SKILL.q_bonus1[s3]
                } else {
                    NAHIDA_SKILL.q_bonus2[s3]
                };
                builder.add_extra_bonus("Q技能加成", bonus);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Dendro,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let e_count = match *config {
            CharacterConfig::Nahida { c4_e_count } => c4_e_count,
            _ => 0
        };
        Some(Box::new(NahidaEffect {
            c4: common_data.constellation >= 4,
            e_count
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}