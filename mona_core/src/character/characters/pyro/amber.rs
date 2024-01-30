use num_derive::FromPrimitive;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::common::{Element, WeaponType, ChangeAttribute, SkillType};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::character_common_data::CharacterCommonData;
use crate::attribute::{Attribute, AttributeName};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::AmberDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{locale, plunging_dmg};

pub struct AmberSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_hp: [f64; 15],
    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const AMBER_SKILL: AmberSkillType = AmberSkillType {
    normal_dmg1: [0.3612, 0.3906, 0.42, 0.462, 0.4914, 0.525, 0.5712, 0.6174, 0.6636, 0.714, 0.7644, 0.8148, 0.8652, 0.9156, 0.966],
    normal_dmg2: [0.3612, 0.3906, 0.42, 0.462, 0.4914, 0.525, 0.5712, 0.6174, 0.6636, 0.714, 0.7644, 0.8148, 0.8652, 0.9156, 0.966],
    normal_dmg3: [0.4644, 0.5022, 0.54, 0.594, 0.6318, 0.675, 0.7344, 0.7938, 0.8532, 0.918, 0.9828, 1.0476, 1.1124, 1.1772, 1.242],
    normal_dmg4: [0.473, 0.5115, 0.55, 0.605, 0.6435, 0.6875, 0.748, 0.8085, 0.869, 0.935, 1.001, 1.067, 1.133, 1.199, 1.265],
    normal_dmg5: [0.5934, 0.6417, 0.69, 0.759, 0.8073, 0.8625, 0.9384, 1.0143, 1.0902, 1.173, 1.2558, 1.3386, 1.4214, 1.5042, 1.587],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_hp: [0.4136, 0.4446, 0.4756, 0.517, 0.548, 0.579, 0.6204, 0.6618, 0.7031, 0.7445, 0.7858, 0.8272, 0.8789, 0.9306, 0.9823],
    elemental_skill_dmg1: [1.232, 1.3244, 1.4168, 1.54, 1.6324, 1.7248, 1.848, 1.9712, 2.0944, 2.2176, 2.3408, 2.464, 2.618, 2.772, 2.926],
    elemental_burst_dmg1: [0.2808, 0.3019, 0.3229, 0.351, 0.3721, 0.3931, 0.4212, 0.4493, 0.4774, 0.5054, 0.5335, 0.5616, 0.5967, 0.6318, 0.6669],
    elemental_burst_dmg2: [5.0544, 5.4335, 5.8126, 6.318, 6.6971, 7.0762, 7.5816, 8.087, 8.5925, 9.0979, 9.6034, 10.1088, 10.7406, 11.3724, 12.0042],
};

pub const AMBER_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Amber,
    internal_name: "Ambor",
    element: Element::Pyro,
    hp: [793, 2038, 2630, 3940, 4361, 5016, 5578, 6233, 6654, 7309, 7730, 8358, 8806, 9461],
    atk: [19, 48, 62, 93, 103, 118, 131, 147, 157, 172, 182, 198, 208, 223],
    def: [50, 129, 167, 250, 277, 318, 354, 396, 422, 464, 491, 532, 559, 601],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Bow,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·神射手",
        en: "Normal Attack: Sharpshooter",
    ),
    skill_name2: locale!(
        zh_cn: "爆弹玩偶",
        en: "Explosive Puppet",
    ),
    skill_name3: locale!(
        zh_cn: "箭雨",
        en: "Fiery Rain",
    ),
    name_locale: locale!(
        zh_cn: "安柏",
        en: "Amber",
    )
};

pub struct AmberEffect {
    has_talent1: bool,
}

impl AmberEffect {
    pub fn new(common_data: &CharacterCommonData) -> AmberEffect {
        AmberEffect {
            has_talent1: common_data.has_talent1,
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for AmberEffect {
    fn change_attribute(&self, attribute: &mut T) {
        if self.has_talent1 {
            attribute.set_value_by(AttributeName::CriticalElementalBurst, "安柏天赋1", 0.1);
        }
    }
}

#[derive(Copy, Clone)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum AmberDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
    Q2
}

impl Into<usize> for AmberDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl AmberDamageEnum {
    pub fn get_element(&self) -> Element {
        use AmberDamageEnum::*;

        match *self {
            Charged2 | E1 | Q1 | Q2 => Element::Pyro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AmberDamageEnum::*;

        match *self {
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst,
            _ => SkillType::NormalAttack
        }
    }
}

pub struct Amber;

#[derive(Copy, Clone, FromPrimitive)]
pub enum AmberRoleEnum {
    Main
}

impl CharacterTrait for Amber {
    const STATIC_DATA: CharacterStaticData = AMBER_STATIC_DATA;
    type SkillType = AmberSkillType;
    const SKILL: Self::SkillType = AMBER_SKILL;
    type DamageEnumType = AmberDamageEnum;
    type RoleEnum = AmberRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: AmberDamageEnum::Normal1 as usize, text: locale!(zh_cn: "一段伤害", en: "1-Hit DMG") },
            CharacterSkillMapItem { index: AmberDamageEnum::Normal2 as usize, text: locale!(zh_cn: "二段伤害", en: "2-Hit DMG") },
            CharacterSkillMapItem { index: AmberDamageEnum::Normal3 as usize, text: locale!(zh_cn: "三段伤害", en: "3-Hit DMG") },
            CharacterSkillMapItem { index: AmberDamageEnum::Normal4 as usize, text: locale!(zh_cn: "四段伤害", en: "4-Hit DMG") },
            CharacterSkillMapItem { index: AmberDamageEnum::Normal5 as usize, text: locale!(zh_cn: "五段伤害", en: "5-Hit DMG") },
            CharacterSkillMapItem { index: AmberDamageEnum::Charged1 as usize, text: locale!(zh_cn: "瞄准射击", en: "Aimed Shot") },
            CharacterSkillMapItem { index: AmberDamageEnum::Charged2 as usize, text: locale!(zh_cn: "满蓄力瞄准射击", en: "Fully-Charged Aimed Shot") },
            CharacterSkillMapItem { index: AmberDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: AmberDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: AmberDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: AmberDamageEnum::E1 as usize, text: locale!(zh_cn: "爆炸伤害", en: "") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: AmberDamageEnum::Q1 as usize, text: locale!(zh_cn: "箭雨单次伤害", en: "") },
            CharacterSkillMapItem { index: AmberDamageEnum::Q2 as usize, text: locale!(zh_cn: "箭雨总伤害", en: "") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        use AmberDamageEnum::*;
        let s: AmberDamageEnum = num::FromPrimitive::from_usize(s).unwrap();

        let s1 = context.character_common_data.skill1;
        let s2 = context.character_common_data.skill2;
        let s3 = context.character_common_data.skill3;

        let ratio = match s {
            Normal1 => AMBER_SKILL.normal_dmg1[s1],
            Normal2 => AMBER_SKILL.normal_dmg2[s1],
            Normal3 => AMBER_SKILL.normal_dmg3[s1],
            Normal4 => AMBER_SKILL.normal_dmg4[s1],
            Normal5 => AMBER_SKILL.normal_dmg5[s1],
            Charged1 => AMBER_SKILL.charged_dmg1[s1],
            Charged2 => AMBER_SKILL.charged_dmg2[s1],
            Plunging1 => AMBER_SKILL.plunging_dmg1[s1],
            Plunging2 => AMBER_SKILL.plunging_dmg2[s1],
            Plunging3 => AMBER_SKILL.plunging_dmg3[s1],
            E1 => AMBER_SKILL.elemental_skill_dmg1[s2],
            Q1 => AMBER_SKILL.elemental_burst_dmg1[s3],
            Q2 => AMBER_SKILL.elemental_burst_dmg2[s3]
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(AmberEffect::new(common_data)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: AmberRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            AmberRoleEnum::Main => Box::new(AmberDefaultTargetFunction)
        }
    }
}
