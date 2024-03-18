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

pub struct NeuvilletteSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const NEUVILLETTE_SKILL: NeuvilletteSkillType = NeuvilletteSkillType {
    normal_dmg1: [0.5458, 0.5867, 0.6276, 0.6822, 0.7231, 0.7641, 0.8187, 0.8732, 0.9278, 0.9824, 1.037, 1.0915, 1.1598, 1.228, 1.2962],
    normal_dmg2: [0.4625, 0.4971, 0.5318, 0.5781, 0.6128, 0.6474, 0.6937, 0.7399, 0.7862, 0.8324, 0.8787, 0.9249, 0.9827, 1.0405, 1.0983],
    normal_dmg3: [0.7234, 0.7776, 0.8319, 0.9042, 0.9585, 1.0127, 1.0851, 1.1574, 1.2297, 1.3021, 1.3744, 1.4468, 1.5372, 1.6276, 1.718],
    charged_dmg1: [1.368, 1.4706, 1.5732, 1.71, 1.8126, 1.9152, 2.052, 2.1888, 2.3256, 2.4624, 2.5992, 2.736, 2.907, 3.078, 3.249],
    charged_dmg2: [0.0732, 0.0791, 0.0851, 0.0936, 0.0996, 0.1064, 0.1157, 0.1251, 0.1345, 0.1447, 0.1549, 0.1651, 0.1753, 0.1855, 0.1957],
    // [0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 0.16, 0.16],
    // [0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [0.1286, 0.1383, 0.1479, 0.1608, 0.1704, 0.1801, 0.193, 0.2058, 0.2187, 0.2316, 0.2444, 0.2573, 0.2734, 0.2894, 0.3055],
    e_dmg2: [0.208, 0.2236, 0.2392, 0.26, 0.2756, 0.2912, 0.312, 0.3328, 0.3536, 0.3744, 0.3952, 0.416, 0.442, 0.468, 0.494],
    q_dmg1: [0.2226, 0.2393, 0.256, 0.2782, 0.2949, 0.3116, 0.3339, 0.3561, 0.3784, 0.4006, 0.4229, 0.4452, 0.473, 0.5008, 0.5286],
    q_dmg2: [0.0911, 0.0979, 0.1047, 0.1138, 0.1206, 0.1275, 0.1366, 0.1457, 0.1548, 0.1639, 0.173, 0.1821, 0.1935, 0.2049, 0.2163],
};

damage_enum!(
    NeuvilletteDamageEnum
    Normal1
    Normal2
    Normal3
    Charged1
    Charged2
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    Q1
    Q2
);

impl NeuvilletteDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use NeuvilletteDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }

    pub fn is_hp_ratio(&self) -> bool {
        use NeuvilletteDamageEnum::*;
        match *self {
            Charged2 | E1 | Q1 | Q2 => true,
            _ => false
        }
    }
}

pub struct NeuvilletteEffect {
    pub has_talent2: bool,
    pub current_hp: usize,
}

impl<A: Attribute> ChangeAttribute<A> for NeuvilletteEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let current_hp = self.current_hp;
        if self.has_talent2 {
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::BonusHydro,
                Box::new(move |hp, _| {
                    // let bonus = (current_hp as i32 - 30_i32).max(0) as f64 * 0.006;
                    let bonus = (current_hp - 30).max(0) as f64 * 0.006;
                    let bonus = bonus.min(0.3);
                    bonus
                }),
                Box::new(|dx, dy, grad| (0.0, 0.0)),
                "天赋「至高仲裁的纪律」"
            );
        }
    }
}

pub struct Neuvillette;

impl CharacterTrait for Neuvillette {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Neuvillette,
        internal_name: "Neuvillette",
        name_locale: locale!(
            zh_cn: "那维莱特",
            en: "Neuvillette"
        ),
        element: Element::Hydro,
        hp: [1144, 2967, 3948, 5908, 6605, 7599, 8528, 9533, 10230, 11243, 11940, 12965, 13662, 14695],
        atk: [16, 42, 56, 84, 94, 108, 121, 135, 145, 159, 169, 184, 194, 208],
        def: [45, 116, 155, 232, 259, 298, 335, 374, 401, 441, 468, 509, 536, 576],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·如水从平",
            en: "Normal Attack: As Water Seeks Equilibrium"
        ),
        skill_name2: locale!(
            zh_cn: "泪水啊，我必偿还",
            en: "O Tears, I Shall Repay"
        ),
        skill_name3: locale!(
            zh_cn: "潮水啊，我已归来",
            en: "O Tides, I Have Returned"
        )
    };
    type SkillType = NeuvilletteSkillType;
    const SKILL: Self::SkillType = NEUVILLETTE_SKILL;
    type DamageEnumType = NeuvilletteDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            NeuvilletteDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged1 locale!(zh_cn: "重击伤害", en: "Charged Attack DMG")
            Charged2 locale!(zh_cn: "重击·衡平推裁持续伤害", en: "Charged Attack: Equitable Judgment")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            NeuvilletteDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E2 locale!(zh_cn: "灵息之刺伤害", en: "Spiritbreath Thorn DMG")
        ),
        skill3: skill_map!(
            NeuvilletteDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "水瀑伤害", en: "Waterfall DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "current_hp",
            title: locale!(
                zh_cn: "当前生命百分比",
                en: "Current HP Percentage"
            ),
            config: ItemConfigType::Int { min: 1, max: 100, default: 100 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_stack",
            title: locale!(
                zh_cn: "「遗龙之荣」层数",
                en: "Past Draconic Glories Stack"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 3 }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: NeuvilletteDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use NeuvilletteDamageEnum::*;

        let ratio = match s {
            Normal1 => NEUVILLETTE_SKILL.normal_dmg1[s1],
            Normal2 => NEUVILLETTE_SKILL.normal_dmg2[s1],
            Normal3 => NEUVILLETTE_SKILL.normal_dmg3[s1],
            Charged1 => NEUVILLETTE_SKILL.charged_dmg1[s1],
            Charged2 => NEUVILLETTE_SKILL.charged_dmg2[s1],
            Plunging1 => NEUVILLETTE_SKILL.plunging_dmg1[s1],
            Plunging2 => NEUVILLETTE_SKILL.plunging_dmg2[s1],
            Plunging3 => NEUVILLETTE_SKILL.plunging_dmg3[s1],
            E1 => NEUVILLETTE_SKILL.e_dmg1[s2],
            E2 => NEUVILLETTE_SKILL.e_dmg2[s2],
            Q1 => NEUVILLETTE_SKILL.q_dmg1[s3],
            Q2 => NEUVILLETTE_SKILL.q_dmg2[s3],
        };
        let mut builder = D::new();
        let is_hp_ratio = s.is_hp_ratio();

        if is_hp_ratio {
            builder.add_hp_ratio("技能倍率", ratio);
        } else {
            builder.add_atk_ratio("技能倍率", ratio);
        }

        if s == Charged2 {
            let talent1_stack = match *config {
                CharacterSkillConfig::Neuvillette { talent1_stack } => talent1_stack,
                _ => 0
            };
            let mul = if talent1_stack == 1 {
                1.1
            } else if talent1_stack == 2 {
                1.25
            } else if talent1_stack == 3 {
                1.6
            } else {
                1.0
            };
            builder.add_hp_ratio("天赋「古海孑遗的权柄」额外倍率", ratio * (mul - 1.0));

            if context.character_common_data.constellation >= 2 {
                let cd = 0.42_f64.min(0.14 * talent1_stack as f64);
                builder.add_extra_critical_damage("2命「律法的命诫」", cd);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Hydro,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let current_hp = match *config {
            CharacterConfig::Neuvillette { current_hp } => current_hp,
            _ => 1
        };
        Some(Box::new(NeuvilletteEffect {
            current_hp,
            has_talent2: common_data.has_talent2
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
