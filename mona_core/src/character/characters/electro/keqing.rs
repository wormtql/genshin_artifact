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
use crate::target_functions::target_functions::KeqingDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct KeqingSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg41: [f64; 15],
    pub normal_dmg42: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_dmg3: [f64; 15],
}

pub const KEQING_SKILL: KeqingSkillType = KeqingSkillType {
    normal_dmg1: [0.4102, 0.4436, 0.477, 0.5247, 0.5581, 0.5962, 0.6487, 0.7012, 0.7537, 0.8109, 0.8681, 0.9254, 0.9826, 1.0399, 1.0971],
    normal_dmg2: [0.4102, 0.4436, 0.477, 0.5247, 0.5581, 0.5962, 0.6487, 0.7012, 0.7537, 0.8109, 0.8681, 0.9254, 0.9826, 1.0399, 1.0971],
    normal_dmg3: [0.5444, 0.5887, 0.633, 0.6963, 0.7406, 0.7913, 0.8609, 0.9305, 1.0001, 1.0761, 1.1521, 1.228, 1.304, 1.3799, 1.4559],
    normal_dmg41: [0.3148, 0.3404, 0.366, 0.4026, 0.4282, 0.4575, 0.4978, 0.538, 0.5783, 0.6222, 0.6661, 0.71, 0.754, 0.7979, 0.8418],
    normal_dmg42: [0.344, 0.372, 0.4, 0.44, 0.468, 0.5, 0.544, 0.588, 0.632, 0.68, 0.728, 0.776, 0.824, 0.872, 0.92],
    normal_dmg5: [0.6699, 0.7245, 0.779, 0.8569, 0.9114, 0.9738, 1.0594, 1.1451, 1.2308, 1.3243, 1.4178, 1.5113, 1.6047, 1.6982, 1.7917],
    charged_dmg11: [0.768, 0.8305, 0.893, 0.9823, 1.0448, 1.1163, 1.2145, 1.3127, 1.4109, 1.5181, 1.6253, 1.7324, 1.8396, 1.9467, 2.0539],
    charged_dmg12: [0.86, 0.93, 1., 1.1, 1.17, 1.25, 1.36, 1.47, 1.58, 1.7, 1.82, 1.94, 2.06, 2.18, 2.3],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [0.504, 0.5418, 0.5796, 0.63, 0.6678, 0.7056, 0.756, 0.8064, 0.8568, 0.9072, 0.9576, 1.008, 1.071, 1.134, 1.197],
    elemental_skill_dmg2: [1.68, 1.806, 1.932, 2.1, 2.226, 2.352, 2.52, 2.688, 2.856, 3.024, 3.192, 3.36, 3.57, 3.78, 3.99],
    elemental_burst_dmg1: [0.88, 0.946, 1.012, 1.1, 1.166, 1.232, 1.32, 1.408, 1.496, 1.584, 1.672, 1.76, 1.87, 1.98, 2.09],
    elemental_burst_dmg2: [0.24, 0.258, 0.276, 0.3, 0.318, 0.336, 0.36, 0.384, 0.408, 0.432, 0.456, 0.48, 0.51, 0.54, 0.57],
    elemental_burst_dmg3: [1.888, 2.0296, 2.1712, 2.36, 2.5016, 2.6432, 2.832, 3.0208, 3.2096, 3.3984, 3.5872, 3.776, 4.012, 4.248, 4.484],
};

pub const KEQING_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Keqing,
    internal_name: "Keqing",
    element: Element::Electro,
    hp: [1020, 2646, 3521, 5268, 5889, 6776, 7604, 8500, 9121, 10025, 10647, 11561, 12182, 13103],
    atk: [25, 65, 87, 130, 145, 167, 187, 209, 225, 247, 262, 285, 300, 323],
    def: [62, 161, 215, 321, 359, 413, 464, 519, 556, 612, 649, 705, 743, 799],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·云来剑法",
        en: "Normal Attack: Yunlai Swordsmanship",
    ),
    skill_name2: locale!(
        zh_cn: "星斗归位",
        en: "Stellar Restoration",
    ),
    skill_name3: locale!(
        zh_cn: "天街巡游",
        en: "Starward Sword",
    ),
    name_locale: locale!(
        zh_cn: "刻晴",
        en: "Keqing",
    )
};

pub struct KeqingEffect {
    pub has_talent2: bool,
    pub rate: f64
}

impl KeqingEffect {
    pub fn new(character: &CharacterCommonData, config: &CharacterConfig) -> Self {
        let rate = match *config {
            CharacterConfig::Keqing { talent2_rate } => talent2_rate,
            _ => 0.0
        };

        KeqingEffect {
            has_talent2: character.has_talent2,
            rate
        }
    }
}

impl<A: Attribute> ChangeAttribute<A> for KeqingEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent2 {
            attribute.set_value_by(AttributeName::CriticalBase, "刻晴天赋：玉衡之贵", 0.15 * self.rate);
            attribute.set_value_by(AttributeName::Recharge, "刻晴天赋：玉衡之贵", 0.15 * self.rate);
        }
    }
}

pub struct Keqing;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum KeqingDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal41,
    Normal42,
    Normal5,
    Charged11,
    Charged12,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    E3Times2,
    Q1,
    Q2,
    Q2Times8,
    Q3
}

impl KeqingDamageEnum {
    pub fn get_element(&self, after_e: bool) -> Element {
        if after_e {
            Element::Electro
        } else {
            use KeqingDamageEnum::*;
            match *self {
                E1 | E2 | E3 | E3Times2 | Q1 | Q2 | Q2Times8 | Q3 => Element::Electro,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use KeqingDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal41 | Normal42 | Normal5 => SkillType::NormalAttack,
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E3Times2 => SkillType::ElementalSkill,
            Q1 | Q2 | Q2Times8 | Q3 => SkillType::ElementalBurst
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum KeqingRoleEnum {
    MainElectro,
    MainPhysical,
}

impl Into<usize> for KeqingDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl CharacterTrait for Keqing {
    const STATIC_DATA: CharacterStaticData = KEQING_STATIC_DATA;
    type SkillType = KeqingSkillType;
    const SKILL: Self::SkillType = KEQING_SKILL;
    type DamageEnumType = KeqingDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: KeqingDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Normal41 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Normal42 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Charged11 as usize, text: charged_dmg!(1) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Charged12 as usize, text: charged_dmg!(2) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: KeqingDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: KeqingDamageEnum::E1 as usize, text: locale!(zh_cn: "雷楔伤害", en: "Lightning Stiletto DMG") },
            CharacterSkillMapItem { index: KeqingDamageEnum::E2 as usize, text: locale!(zh_cn: "斩击伤害", en: "Slashing DMG") },
            CharacterSkillMapItem { index: KeqingDamageEnum::E3 as usize, text: locale!(zh_cn: "雷暴连斩伤害/2", en: "Thunderclap Slash DMG/2") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: KeqingDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: KeqingDamageEnum::Q2 as usize, text: locale!(zh_cn: "连斩伤害/8", en: "Consecutive Slash DMG/8") },
            CharacterSkillMapItem { index: KeqingDamageEnum::Q3 as usize, text: locale!(zh_cn: "最后一击伤害", en: "Last Attack DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_rate",
            title: locale!(
                zh_cn: "天赋「玉衡之贵」应用比例",
                en: "Talent「Aristocratic Dignity」Apply Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_e",
            title: locale!(
                zh_cn: "E附魔",
                en: "E附魔",
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KeqingDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KeqingDamageEnum::*;
        let ratio = match s {
            Normal1 => KEQING_SKILL.normal_dmg1[s1],
            Normal2 => KEQING_SKILL.normal_dmg2[s1],
            Normal3 => KEQING_SKILL.normal_dmg3[s1],
            Normal41 => KEQING_SKILL.normal_dmg41[s1],
            Normal42 => KEQING_SKILL.normal_dmg42[s1],
            Normal5 => KEQING_SKILL.normal_dmg5[s1],
            Charged11 => KEQING_SKILL.charged_dmg11[s1],
            Charged12 => KEQING_SKILL.charged_dmg12[s1],
            Plunging1 => KEQING_SKILL.plunging_dmg1[s1],
            Plunging2 => KEQING_SKILL.plunging_dmg2[s1],
            Plunging3 => KEQING_SKILL.plunging_dmg3[s1],
            E1 => KEQING_SKILL.elemental_skill_dmg1[s2],
            E2 | E3Times2 => KEQING_SKILL.elemental_skill_dmg2[s2],
            E3 => KEQING_SKILL.elemental_skill_dmg2[s2] / 2.0,
            Q1 => KEQING_SKILL.elemental_burst_dmg1[s3],
            Q2 => KEQING_SKILL.elemental_burst_dmg2[s3],
            Q2Times8 => KEQING_SKILL.elemental_burst_dmg2[s3] * 8.0,
            Q3 => KEQING_SKILL.elemental_burst_dmg3[s3]
        };
        let after_e = match *config {
            CharacterSkillConfig::Keqing { after_e } => after_e,
            _ => false
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(after_e),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(KeqingEffect::new(common_data, config)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        // let role: KeqingRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        // match role {
        //     KeqingRoleEnum::MainElectro => Box::new(KeqingDefaultTargetFunction),
        //     KeqingRoleEnum::MainPhysical =>
        // }
        unimplemented!()
    }
}
