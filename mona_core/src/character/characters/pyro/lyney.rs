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

pub struct LyneySkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub charged_dmg3: [f64; 15],
    pub a_hp_inheritance: [f64; 15],
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const LYNEY_SKILL: LyneySkillType = LyneySkillType {
    normal_dmg1: [0.3879, 0.4194, 0.451, 0.4961, 0.5277, 0.5638, 0.6134, 0.663, 0.7126, 0.7667, 0.8208, 0.8749, 0.9291, 0.9832, 1.0373],
    normal_dmg2: [0.3801, 0.4111, 0.442, 0.4862, 0.5171, 0.5525, 0.6011, 0.6497, 0.6984, 0.7514, 0.8044, 0.8575, 0.9105, 0.9636, 1.0166],
    normal_dmg3: [0.2726, 0.2948, 0.317, 0.3487, 0.3709, 0.3963, 0.4311, 0.466, 0.5009, 0.5389, 0.5769, 0.615, 0.653, 0.6911, 0.7291],
    normal_dmg4: [0.5693, 0.6157, 0.662, 0.7282, 0.7745, 0.8275, 0.9003, 0.9731, 1.046, 1.1254, 1.2048, 1.2843, 1.3637, 1.4432, 1.5226],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    charged_dmg3: [1.728, 1.8576, 1.9872, 2.16, 2.2896, 2.4192, 2.592, 2.7648, 2.9376, 3.1104, 3.2832, 3.456, 3.672, 3.888, 4.104],
    a_hp_inheritance: [0.64, 0.688, 0.736, 0.8, 0.848, 0.896, 0.96, 1.024, 1.088, 1.152, 1.216, 1.28, 1.36, 1.44, 1.52],
    a_dmg1: [2.12, 2.279, 2.438, 2.65, 2.809, 2.968, 3.18, 3.392, 3.604, 3.816, 4.028, 4.24, 4.505, 4.77, 5.035],
    a_dmg2: [0.2755, 0.2962, 0.3168, 0.3444, 0.3651, 0.3857, 0.4133, 0.4408, 0.4684, 0.4959, 0.5235, 0.551, 0.5855, 0.6199, 0.6544],
    e_dmg1: [1.672, 1.7974, 1.9228, 2.09, 2.2154, 2.3408, 2.508, 2.6752, 2.8424, 3.0096, 3.1768, 3.344, 3.553, 3.762, 3.971],
    e_dmg2: [0.532, 0.5719, 0.6118, 0.665, 0.7049, 0.7448, 0.798, 0.8512, 0.9044, 0.9576, 1.0108, 1.064, 1.1305, 1.197, 1.2635],
    q_dmg1: [1.54, 1.6555, 1.771, 1.925, 2.0405, 2.156, 2.31, 2.464, 2.618, 2.772, 2.926, 3.08, 3.2725, 3.465, 3.6575],
    q_dmg2: [4.14, 4.4505, 4.761, 5.175, 5.4855, 5.796, 6.21, 6.624, 7.038, 7.452, 7.866, 8.28, 8.7975, 9.315, 9.8325],
};

damage_enum!(
    LyneyDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged1
    Charged2
    Charged3
    Plunging1
    Plunging2
    Plunging3
    A1
    A2
    E1
    Q1
    Q2
);

impl LyneyDamageEnum {
    pub fn get_element(&self) -> Element {
        use LyneyDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Plunging1 | Plunging2 | Plunging3 |
                Charged1 => Element::Physical,
            Charged2 | Charged3 | A1 | A2 | E1 | Q1 | Q2 => Element::Pyro
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use LyneyDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Charged1 | Charged2 | Charged3 | A1 | A2 => SkillType::ChargedAttack,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst,
        }
    }
}

pub struct LyneyEffect {
    pub constellation: usize,
    pub c2_stack: f64,
    pub c4_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for LyneyEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.constellation >= 2 {
            let value = 0.2 * self.c2_stack;
            attribute.set_value_by(AttributeName::CriticalDamageBase, "2命「巧言贴耳的诱引」", value);
        }
        if self.constellation >= 4 {
            let value = 0.2 * self.c4_rate;
            attribute.set_value_by(AttributeName::ResMinusPyro, "4命「熟稔习练的筹谋」", value);
        }
    }
}

pub struct Lyney;

impl CharacterTrait for Lyney {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Lyney,
        internal_name: "Lyney",
        name_locale: locale!(
            zh_cn: "林尼",
            en: "Lyney"
        ),
        element: Element::Pyro,
        hp: [858, 2226, 2961, 4431, 4954, 5699, 6396, 7150, 7672, 8432, 8955, 9724, 10247, 11021],
        atk: [25, 64, 85, 128, 143, 165, 185, 206, 221, 243, 258, 281, 296, 318],
        def: [42, 109, 145, 216, 242, 278, 312, 349, 375, 412, 437, 475, 500, 538],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Bow,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·迫牌易位式",
            en: "Normal Attack: Card Force Translocation"
        ),
        skill_name2: locale!(
            zh_cn: "眩惑光戏法",
            en: "Bewildering Lights"
        ),
        skill_name3: locale!(
            zh_cn: "大魔术·灵迹巡游",
            en: "Wondrous Trick: Miracle Parade"
        )
    };
    type SkillType = LyneySkillType;
    const SKILL: Self::SkillType = LYNEY_SKILL;
    type DamageEnumType = LyneyDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LyneyDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 locale!(zh_cn: "三段伤害/2", en: "3-Hit DMG/2")
            Normal4 hit_n_dmg!(4)
            Charged1 locale!(zh_cn: "瞄准射击", en: "Aimed Shot")
            Charged2 locale!(zh_cn: "一段蓄力瞄准射击", en: "Aimed Shot Charge Level 1")
            Charged3 locale!(zh_cn: "隐具魔术箭伤害", en: "Prop Arrow DMG")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
            A1 locale!(zh_cn: "礼花术弹伤害", en: "Pyrotechnic Strike DMG")
            A2 locale!(zh_cn: "灵息之刺伤害", en: "Spiritbreath Thorn DMG")
        ),
        skill2: skill_map!(
            LyneyDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            LyneyDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "引爆礼花伤害", en: "Explosive Firework DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c2_stack",
            title: locale!(
                zh_cn: "2命「巧言贴耳的诱引」层数",
                en: "C2「Loquacious Cajoling」Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        },
        ItemConfig {
            name: "c4_rate",
            title: locale!(
                zh_cn: "4命「熟稔习练的筹谋」比例",
                en: "C4「Well-Versed, Well-Rehearsed」Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "prop_stack",
            title: locale!(
                zh_cn: "隐具余数",
                en: "Prop Surplus stacks"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 0.0 }
        },
        ItemConfig {
            name: "under_pyro",
            title: locale!(
                zh_cn: "敌人被挂火",
                en: "Under Pyro"
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "pyro_count",
            title: locale!(
                zh_cn: "队伍火元素数（除林尼）",
                en: "Pyro Count in Team (Lyney Excluded)"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 1 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LyneyDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use LyneyDamageEnum::*;

        let mut builder = D::new();
        let mut ratio = match s {
            Normal1 => LYNEY_SKILL.normal_dmg1[s1],
            Normal2 => LYNEY_SKILL.normal_dmg2[s1],
            Normal3 => LYNEY_SKILL.normal_dmg3[s1],
            Normal4 => LYNEY_SKILL.normal_dmg4[s1],
            Charged1 => LYNEY_SKILL.charged_dmg1[s1],
            Charged2 => LYNEY_SKILL.charged_dmg2[s1],
            Charged3 => LYNEY_SKILL.charged_dmg3[s1],
            Plunging1 => LYNEY_SKILL.plunging_dmg1[s1],
            Plunging2 => LYNEY_SKILL.plunging_dmg2[s1],
            Plunging3 => LYNEY_SKILL.plunging_dmg3[s1],
            A1 => LYNEY_SKILL.a_dmg1[s1],
            A2 => LYNEY_SKILL.a_dmg2[s1],
            E1 => LYNEY_SKILL.e_dmg1[s2],
            Q1 => LYNEY_SKILL.q_dmg1[s3],
            Q2 => LYNEY_SKILL.q_dmg2[s3],
        };

        let (prop_stack, under_pyro, pyro_count) = match *config {
            CharacterSkillConfig::Lyney { prop_stack, under_pyro, pyro_count } => (
                prop_stack, under_pyro, pyro_count
            ),
            _ => (0.0, false, 0)
        };

        if s == E1 {
            ratio += LYNEY_SKILL.e_dmg2[s2] * prop_stack;
        }

        builder.add_atk_ratio("技能倍率", ratio);
        if context.character_common_data.has_talent1 && s == A1 {
            builder.add_atk_ratio("天赋1「惊险演出」", 0.8);
        }

        if context.character_common_data.has_talent2 && under_pyro {
            let mut bonus = 0.6;
            bonus += pyro_count as f64 * 0.2;
            bonus = bonus.min(1.0);
            builder.add_extra_bonus("天赋2「完场喝彩」", bonus);
            // builder.add_atk_ratio("天赋2「完场喝彩」", bonus);
        }

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
        let (c2_stack, c4_rate) = match *config {
            CharacterConfig::Lyney { c2_stack, c4_rate } => (c2_stack, c4_rate),
            _ => (0.0, 0.0)
        };
        Some(Box::new(LyneyEffect {
            c2_stack, c4_rate,
            constellation: common_data.constellation as usize
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
