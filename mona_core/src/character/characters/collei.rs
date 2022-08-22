use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use num_derive::FromPrimitive;
use strum::EnumCount;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;


pub struct ColleiSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

const COLLEI_SKILL: ColleiSkillType = ColleiSkillType {
    normal_dmg1: [0.436, 0.4715, 0.507, 0.5577, 0.5932, 0.6338, 0.6895, 0.7453, 0.8011, 0.8619, 0.9227, 0.9836, 1.0444, 1.1053, 1.1661],
    normal_dmg2: [0.4266, 0.4613, 0.496, 0.5456, 0.5803, 0.62, 0.6746, 0.7291, 0.7837, 0.8432, 0.9027, 0.9622, 1.0218, 1.0813, 1.1408],
    normal_dmg3: [0.5409, 0.585, 0.629, 0.6919, 0.7359, 0.7862, 0.8554, 0.9246, 0.9938, 1.0693, 1.1448, 1.2203, 1.2957, 1.3712, 1.4467],
    normal_dmg4: [0.6803, 0.7356, 0.791, 0.8701, 0.9255, 0.9887, 1.0758, 1.1628, 1.2498, 1.3447, 1.4396, 1.5345, 1.6295, 1.7244, 1.8193],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [1.512, 1.6254, 1.7388, 1.89, 2.0034, 2.1168, 2.268, 2.4192, 2.5704, 2.7216, 2.8728, 3.024, 3.213, 3.402, 3.591],
    elemental_burst_dmg1: [2.0182, 2.1696, 2.321, 2.5228, 2.6742, 2.8255, 3.0274, 3.2292, 3.431, 3.6328, 3.8347, 4.0365, 4.2888, 4.541, 4.7933],
    elemental_burst_dmg2: [0.4325, 0.4649, 0.4974, 0.5406, 0.573, 0.6055, 0.6487, 0.692, 0.7352, 0.7785, 0.8217, 0.865, 0.919, 0.9731, 1.0271],
};

pub struct ColleiEffect {
    pub background: bool,
    pub c1: bool,
}

impl<A: Attribute> ChangeAttribute<A> for ColleiEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.c1 {
            attribute.set_value_by(AttributeName::Recharge, "柯莱1命", 0.2);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, EnumString)]
#[derive(FromPrimitive, EnumCountMacro)]
pub enum ColleiDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
    Q2
}

impl Into<usize> for ColleiDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl ColleiDamageEnum {
    pub fn get_element(&self) -> Element {
        use ColleiDamageEnum::*;
        match *self {
            E1 | Q1 | Q2 | Charged2 => Element::Dendro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ColleiDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 | Plunging2 | Plunging3 => SkillType::PlungingAttack,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

pub struct Collei;

impl CharacterTrait for Collei {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Collei,
        internal_name: "Collei",
        chs: "科莱",
        element: Element::Dendro,
        hp: [821, 2108, 2721, 4076, 4512, 5189, 5770, 6448, 6884, 7561, 7996, 8674, 9110, 9787],
        atk: [17, 43, 56, 83, 92, 106, 118, 132, 140, 154, 163, 177, 186, 200],
        def: [50, 129, 167, 250, 277, 318, 354, 396, 422, 464, 491, 532, 559, 601],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Bow,
        star: 4,
        skill_name1: "普通攻击·祈颂射艺",
        skill_name2: "拂花偈叶",
        skill_name3: "猫猫秘宝"
    };
    type SkillType = ColleiSkillType;
    const SKILL: Self::SkillType = COLLEI_SKILL;
    type DamageEnumType = ColleiDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: ColleiDamageEnum::Normal1 as usize, chs: "一段伤害" },
            CharacterSkillMapItem { index: ColleiDamageEnum::Normal2 as usize, chs: "二段伤害" },
            CharacterSkillMapItem { index: ColleiDamageEnum::Normal3 as usize, chs: "三段伤害" },
            CharacterSkillMapItem { index: ColleiDamageEnum::Normal4 as usize, chs: "四段伤害" },
            CharacterSkillMapItem { index: ColleiDamageEnum::Charged1 as usize, chs: "瞄准射击" },
            CharacterSkillMapItem { index: ColleiDamageEnum::Charged2 as usize, chs: "满蓄力瞄准射击" },
            CharacterSkillMapItem { index: ColleiDamageEnum::Plunging1 as usize, chs: "下坠期间伤害" },
            CharacterSkillMapItem { index: ColleiDamageEnum::Plunging2 as usize, chs: "低空坠地冲击伤害" },
            CharacterSkillMapItem { index: ColleiDamageEnum::Plunging3 as usize, chs: "高空坠地冲击伤害" },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: ColleiDamageEnum::E1 as usize, chs: "技能伤害" },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: ColleiDamageEnum::Q1 as usize, chs: "爆发伤害" },
            CharacterSkillMapItem { index: ColleiDamageEnum::Q2 as usize, chs: "跃动伤害" }
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            title: "c37",
            name: "background",
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig) -> D::Result {
        let s: ColleiDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let ratio = match s {
            ColleiDamageEnum::Normal1 => COLLEI_SKILL.normal_dmg1[s1],
            ColleiDamageEnum::Normal2 => COLLEI_SKILL.normal_dmg2[s1],
            ColleiDamageEnum::Normal3 => COLLEI_SKILL.normal_dmg3[s1],
            ColleiDamageEnum::Normal4 => COLLEI_SKILL.normal_dmg4[s1],
            ColleiDamageEnum::Charged1 => COLLEI_SKILL.charged_dmg1[s1],
            ColleiDamageEnum::Charged2 => COLLEI_SKILL.charged_dmg2[s1],
            ColleiDamageEnum::Plunging1 => COLLEI_SKILL.plunging_dmg1[s1],
            ColleiDamageEnum::Plunging2 => COLLEI_SKILL.plunging_dmg2[s1],
            ColleiDamageEnum::Plunging3 => COLLEI_SKILL.plunging_dmg3[s1],
            ColleiDamageEnum::E1 => COLLEI_SKILL.elemental_skill_dmg1[s2],
            ColleiDamageEnum::Q1 => COLLEI_SKILL.elemental_burst_dmg1[s3],
            ColleiDamageEnum::Q2 => COLLEI_SKILL.elemental_burst_dmg2[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let background = match *config {
            CharacterConfig::Collei { background } => background,
            _ => false
        };

        Some(Box::new(ColleiEffect {
            background,
            c1: common_data.constellation >= 1
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}