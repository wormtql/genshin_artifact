use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::arlecchino::ArlecchinoDamageEnum;
use crate::character::characters::arlecchino::ArlecchinoDamageEnum::{Plunging1, Plunging2, Plunging3};
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

type SKILL = [f64; 15];

pub struct CitlaliSkillType {
    pub normal_dmg1: SKILL,
    pub normal_dmg2: SKILL,
    pub normal_dmg3: SKILL,
    pub charged_dmg: SKILL,
    pub plunging_dmg1: SKILL,
    pub plunging_dmg2: SKILL,
    pub plunging_dmg3: SKILL,

    pub e_dmg1: SKILL,
    pub e_shield: SKILL,
    pub e_shield_fixed: SKILL,
    pub e_dmg2: SKILL,

    pub q_dmg1: SKILL,
    pub q_dmg2: SKILL,
}

pub const CITLALI_SKILL: CitlaliSkillType = CitlaliSkillType {
    normal_dmg1: [0.4341, 0.4666, 0.4992, 0.5426, 0.5751, 0.6077, 0.6511, 0.6945, 0.7379, 0.7813, 0.8247, 0.8681, 0.9224, 0.9767, 1.0309],
    normal_dmg2: [0.3881, 0.4172, 0.4464, 0.4852, 0.5143, 0.5434, 0.5822, 0.621, 0.6598, 0.6986, 0.7375, 0.7763, 0.8248, 0.8733, 0.9218],
    normal_dmg3: [0.5377, 0.578, 0.6184, 0.6721, 0.7125, 0.7528, 0.8066, 0.8603, 0.9141, 0.9679, 1.0217, 1.0754, 1.1426, 1.2099, 1.2771],
    charged_dmg: [0.992, 1.0664, 1.1408, 1.24, 1.3144, 1.3888, 1.488, 1.5872, 1.6864, 1.7856, 1.8848, 1.984, 2.108, 2.232, 2.356],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [0.7296, 0.7843, 0.839, 0.912, 0.9667, 1.0214, 1.0944, 1.1674, 1.2403, 1.3133, 1.3862, 1.4592, 1.5504, 1.6416, 1.7328],
    e_shield: [5.76, 6.192, 6.624, 7.2, 7.632, 8.064, 8.64, 9.216, 9.792, 10.368, 10.944, 11.52, 12.24, 12.96, 13.68],
    e_shield_fixed: [1386.68, 1525.36, 1675.61, 1837.41, 2010.77, 2195.68, 2392.16, 2600.19, 2819.77, 3050.92, 3293.62, 3547.88, 3813.7, 4091.07, 4380.0],
    e_dmg2: [0.1702, 0.183, 0.1958, 0.2128, 0.2256, 0.2383, 0.2554, 0.2724, 0.2894, 0.3064, 0.3235, 0.3405, 0.3618, 0.383, 0.4043],
    q_dmg1: [5.376, 5.7792, 6.1824, 6.72, 7.1232, 7.5264, 8.064, 8.6016, 9.1392, 9.6768, 10.2144, 10.752, 11.424, 12.096, 12.768],
    q_dmg2: [1.344, 1.4448, 1.5456, 1.68, 1.7808, 1.8816, 2.016, 2.1504, 2.2848, 2.4192, 2.5536, 2.688, 2.856, 3.024, 3.192],
};

struct CitlaliEffect {
    pub constellation: usize,
    pub c6_stack: f64,
    pub talent1_rate: f64,
    pub has_talent1: bool,
}

impl<A: Attribute> ChangeAttribute<A> for CitlaliEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.constellation >= 2 {
            attribute.set_value_by(AttributeName::ElementalMastery, "C2「吞心者的巡行」", 125.0);
        }

        if self.constellation >= 6 {
            attribute.set_value_by(AttributeName::BonusBase, "C6「原动天的密契」", 0.025 * self.c6_stack);
        }

        if self.has_talent1 && self.talent1_rate > 0.0 {
            let mut down = self.talent1_rate * 0.2;
            if self.constellation >= 2 {
                down += self.talent1_rate * 0.2;
            }

            attribute.set_value_by(AttributeName::ResMinusPyro, "天赋「五重天的寒雨」", down);
            attribute.set_value_by(AttributeName::ResMinusHydro, "天赋「五重天的寒雨」", down);
        }
    }
}

damage_enum!(
    CitlaliDamageEnum
    Normal1
    Normal2
    Normal3
    Charged
    Plunging1
    Plunging2
    Plunging3
    EShield
    E1
    E2
    Q1
    Q2
);

impl CitlaliDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use CitlaliDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            EShield | E1 | E2 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

pub struct Citlali;

impl CharacterTrait for Citlali {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Citlali,
        internal_name: "Citlali",
        name_locale: locale!(
            zh_cn: "茜特菈莉",
            en: "Citlali"
        ),
        element: Element::Cryo,
        hp: [906, 2349, 3126, 4677, 5229, 6016, 6752, 7547, 8098, 8901, 9453, 10264, 10816, 11634],
        atk: [10, 26, 34, 51, 57, 66, 74, 82, 88, 97, 103, 112, 118, 127],
        def: [59, 154, 205, 307, 343, 395, 443, 495, 531, 584, 620, 673, 710, 763],
        sub_stat: CharacterSubStatFamily::ElementalMastery115,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·宿灵捕影",
            en: "Normal Attack: Shadow-Stealing Spirit Vessel"
        ),
        skill_name2: locale!(
            zh_cn: "霜昼黑星",
            en: "Dawnfrost Darkstar"
        ),
        skill_name3: locale!(
            zh_cn: "诸曜饬令",
            en: "Edict of Entwined Splendor"
        ),
    };
    type SkillType = CitlaliSkillType;
    const SKILL: Self::SkillType = CITLALI_SKILL;
    type DamageEnumType = CitlaliDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            CitlaliDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            CitlaliDamageEnum
            E1 locale!(zh_cn: "黑曜星魔伤害", en: "Obsidian Tzitzimitl DMG")
            EShield locale!(zh_cn: "护盾吸收量", en: "Shield DMG Absorption")
            E2 locale!(zh_cn: "霜陨风暴伤害", en: "Frostfall Storm DMG")
        ),
        skill3: skill_map!(
            CitlaliDamageEnum
            Q1 locale!(zh_cn: "冰风暴伤害", en: "Ice Storm DMG")
            Q1 locale!(zh_cn: "宿灵之髑伤害", en: "Spiritvessel Skull DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_rate",
            title: locale!(
                zh_cn: "天赋「五重天的寒雨」",
                en: "Talent 1"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "c6_stack",
            title: locale!(
                zh_cn: "C6「秘律之数」",
                en: "C6 \"Cifra of the Secret Law\""
            ),
            config: ItemConfigType::Float { min: 0.0, max: 40.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: CitlaliDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let mut builder = D::new();
        use CitlaliDamageEnum::*;

        if s == EShield {
            let ratio = CITLALI_SKILL.e_shield[s2];
            let fixed = CITLALI_SKILL.e_shield_fixed[s2];

            builder.add_em_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);

            return builder.shield(&context.attribute, Element::Cryo);
        }

        let ratio = match s {
            Normal1 => CITLALI_SKILL.normal_dmg1[s1],
            Normal2 => CITLALI_SKILL.normal_dmg2[s1],
            Normal3 => CITLALI_SKILL.normal_dmg3[s1],
            Charged => CITLALI_SKILL.charged_dmg[s1],
            Plunging1 => CITLALI_SKILL.plunging_dmg1[s1],
            Plunging2 => CITLALI_SKILL.plunging_dmg2[s1],
            Plunging3 => CITLALI_SKILL.plunging_dmg3[s1],
            E1 => CITLALI_SKILL.e_dmg1[s2],
            E2 => CITLALI_SKILL.e_dmg2[s2],
            Q1 => CITLALI_SKILL.q_dmg1[s3],
            Q2 => CITLALI_SKILL.q_dmg2[s3],
            EShield => 0.0
        };

        builder.add_atk_ratio("技能倍率", ratio);

        if context.character_common_data.has_talent2 {
            if s == E2 {
                builder.add_em_ratio("天赋「白燧蝶的星衣」", 0.9);
            } else if s == Q1 {
                builder.add_em_ratio("天赋「白燧蝶的星衣」", 12.0);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Cryo,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Citlali { talent1_rate, c6_stack } => Some(Box::new(
                CitlaliEffect {
                    talent1_rate,
                    has_talent1: common_data.has_talent1,
                    c6_stack,
                    constellation: common_data.constellation as usize
                }
            )),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
