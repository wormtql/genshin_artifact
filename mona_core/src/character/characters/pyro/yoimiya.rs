use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::YoimiyaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct YoimiyaSkill {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub charged_dmg3: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const YOIMIYA_SKILL: YoimiyaSkill = YoimiyaSkill {
    normal_dmg1: [0.3564, 0.3807, 0.405, 0.4374, 0.4617, 0.4901, 0.5265, 0.5629, 0.5994, 0.6359, 0.6723, 0.7088, 0.7452, 0.7817, 0.8181],
    normal_dmg2: [0.6838, 0.7304, 0.777, 0.8392, 0.8858, 0.9402, 1.0101, 1.08, 1.15, 1.2199, 1.2898, 1.3598, 1.4297, 1.4996, 1.5695],
    normal_dmg3: [0.8889, 0.9495, 1.0101, 1.0909, 1.1515, 1.2222, 1.3131, 1.404, 1.4949, 1.5859, 1.6768, 1.7677, 1.8586, 1.9495, 2.0404],
    normal_dmg4: [0.4642, 0.4959, 0.5275, 0.5697, 0.6014, 0.6383, 0.6858, 0.7332, 0.7807, 0.8282, 0.8757, 0.9231, 0.9706, 1.0181, 1.0656],
    normal_dmg5: [1.0586, 1.1308, 1.203, 1.2992, 1.3714, 1.4556, 1.5639, 1.6722, 1.7804, 1.8887, 1.997, 2.1053, 2.2135, 2.3218, 2.4301],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    charged_dmg3: [0.164, 0.1763, 0.1886, 0.205, 0.2173, 0.2296, 0.246, 0.2624, 0.2788, 0.2952, 0.3116, 0.328, 0.3485, 0.369, 0.3895],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [1.3791, 1.4018, 1.4245, 1.454, 1.4767, 1.4994, 1.5289, 1.5584, 1.5879, 1.6174, 1.647, 1.6765, 1.706, 1.7355, 1.765],
    elemental_burst_dmg1: [1.272, 1.3674, 1.4628, 1.59, 1.6854, 1.7808, 1.908, 2.0352, 2.1624, 2.2896, 2.4168, 2.544, 2.703, 2.862, 3.021],
    elemental_burst_dmg2: [1.22, 1.3115, 1.403, 1.525, 1.6165, 1.708, 1.83, 1.952, 2.074, 2.196, 2.318, 2.44, 2.5925, 2.745, 2.8975],
};

pub const YOIMIYA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Yoimiya,
    internal_name: "Yoimiya",
    element: Element::Pyro,
    hp: [791, 2053, 2731, 4086, 4568, 5256, 5899, 6593, 7075, 7777, 8259, 8968, 9450, 10164],
    atk: [25, 65, 87, 130, 145, 167, 187, 209, 225, 247, 262, 285, 300, 323],
    def: [48, 124, 165, 247, 276, 318, 357, 399, 428, 470, 500, 542, 572, 615],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Bow,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·烟火打扬",
        en: "Normal Attack: Firework Flare-Up",
    ),
    skill_name2: locale!(
        zh_cn: "焰硝庭火舞",
        en: "Niwabi Fire-Dance",
    ),
    skill_name3: locale!(
        zh_cn: "琉金云间草",
        en: "Ryuukin Saxifrage",
    ),
    name_locale: locale!(
        zh_cn: "宵宫",
        en: "Yoimiya",
    )
};

pub struct YoimiyaEffect {
    pub has_talent1: bool,
    pub talent1_level: f64,
}

impl YoimiyaEffect {
    pub fn new(common_data: &CharacterCommonData, config: &CharacterConfig) -> YoimiyaEffect {
        let level = (match config {
            CharacterConfig::Yoimiya { talent1_level } => *talent1_level,
            _ => 0.0,
        }).clamp(0.0, 10.0);
        YoimiyaEffect {
            has_talent1: common_data.has_talent1,
            talent1_level: level
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for YoimiyaEffect {
    fn change_attribute(&self, attribute: &mut T) {
        if self.has_talent1 {
            attribute.set_value_by(AttributeName::BonusPyro, "宵宫天赋：袖火百景图", self.talent1_level * 0.02);
        }
    }
}

pub struct Yoimiya;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum YoimiyaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Charged1,
    Charged2,
    Charged3,
    Plunging1,
    Plunging2,
    Plunging3,
    Q1,
    Q2
}

impl YoimiyaDamageEnum {
    pub fn get_element(&self, after_e: bool) -> Element {
        use YoimiyaDamageEnum::*;
        if after_e {
            match *self {
                Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
                _ => Element::Pyro
            }
        } else {
            match *self {
                Charged2 | Charged3 | Q1 | Q2 => Element::Pyro,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use YoimiyaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged1 | Charged2 | Charged3 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for YoimiyaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum YoimiyaRoleEnum {
    Main
}

impl CharacterTrait for Yoimiya {
    const STATIC_DATA: CharacterStaticData = YOIMIYA_STATIC_DATA;
    type SkillType = YoimiyaSkill;
    const SKILL: Self::SkillType = YOIMIYA_SKILL;
    type DamageEnumType = YoimiyaDamageEnum;
    type RoleEnum = YoimiyaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Normal1 as usize, text: locale!(zh_cn: "一段伤害/2", en: "1-Hit DMG/2") },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Normal4 as usize, text: locale!(zh_cn: "四段伤害/2", en: "4-Hit DMG/2") },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Charged2 as usize, text: charged_dmg!("shoot2") },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Charged3 as usize, text: locale!(zh_cn: "焰硝矢伤害", en: "Kindling Arrow DMG") },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: None,
        skill3: Some(&[
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: YoimiyaDamageEnum::Q2 as usize, text: locale!(zh_cn: "琉金火光爆炸伤害", en: "Aurous Blaze Explosion DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_level",
            title: locale!(
                zh_cn: "天赋「袖火百景图」应用层数",
                en: "Talent「Tricks of the Trouble-Maker」Apply Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 10.0, default: 8.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_e",
            title: locale!(
                zh_cn: "庭火焰硝",
                en: "Niwabi Enshou"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: YoimiyaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use YoimiyaDamageEnum::*;
        let mut ratio = match s {
            Normal1 => YOIMIYA_SKILL.normal_dmg1[s1],
            Normal2 => YOIMIYA_SKILL.normal_dmg2[s1],
            Normal3 => YOIMIYA_SKILL.normal_dmg3[s1],
            Normal4 => YOIMIYA_SKILL.normal_dmg4[s1],
            Normal5 => YOIMIYA_SKILL.normal_dmg5[s1],
            Charged1 => YOIMIYA_SKILL.charged_dmg1[s1],
            Charged2 => YOIMIYA_SKILL.charged_dmg2[s1],
            Charged3 => YOIMIYA_SKILL.charged_dmg3[s1],
            Plunging1 => YOIMIYA_SKILL.plunging_dmg1[s1],
            Plunging2 => YOIMIYA_SKILL.plunging_dmg2[s1],
            Plunging3 => YOIMIYA_SKILL.plunging_dmg3[s1],
            Q1 => YOIMIYA_SKILL.elemental_burst_dmg1[s3],
            Q2 => YOIMIYA_SKILL.elemental_burst_dmg2[s3],
        };

        let skill_type = s.get_skill_type();
        let after_e = match *config {
            CharacterSkillConfig::Yoimiya { after_e } => after_e,
            _ => false
        };

        if after_e && skill_type == SkillType::NormalAttack {
            let times = YOIMIYA_SKILL.elemental_skill_dmg1[s2];
            ratio *= times;
        }

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(after_e),
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(YoimiyaEffect::new(common_data, config)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: YoimiyaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            YoimiyaRoleEnum::Main => Box::new(YoimiyaDefaultTargetFunction {
                melt_rate: 0.0,
                vaporize_rate: 0.0
            })
        }
    }
}
