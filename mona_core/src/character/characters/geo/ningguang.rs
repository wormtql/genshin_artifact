use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::NingguangDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, locale, plunging_dmg};

pub struct NingguangSkillType {
    pub normal_dmg1: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_hp: [f64; 15],
    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
}

pub const NINGGUANG_SKILL: NingguangSkillType = NingguangSkillType {
    normal_dmg1: [0.28, 0.301, 0.322, 0.35, 0.371, 0.392, 0.42, 0.448, 0.476, 0.504, 0.5331, 0.5712, 0.6093, 0.6474, 0.6854],
    charged_dmg1: [1.7408, 1.8714, 2.0019, 2.176, 2.3066, 2.4371, 2.6112, 2.7853, 2.9594, 3.1334, 3.3145, 3.5512, 3.788, 4.0247, 4.2615],
    charged_dmg2: [0.496, 0.5332, 0.5704, 0.62, 0.6572, 0.6944, 0.744, 0.7936, 0.8432, 0.8928, 0.9444, 1.0118, 1.0793, 1.1468, 1.2142],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_hp: [0.501, 0.531, 0.561, 0.6, 0.63, 0.66, 0.699, 0.738, 0.777, 0.816, 0.855, 0.894, 0.933, 0.972, 1.011],
    elemental_skill_dmg1: [2.304, 2.4768, 2.6496, 2.88, 3.0528, 3.2256, 3.456, 3.6864, 3.9168, 4.1472, 4.3776, 4.608, 4.896, 5.184, 5.472],
    elemental_burst_dmg1: [0.8696, 0.9348, 1., 1.087, 1.1522, 1.2174, 1.3044, 1.3914, 1.4783, 1.5653, 1.6522, 1.7392, 1.8479, 1.9566, 2.0653],
};

pub const NINGGUANG_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Ningguang,
    internal_name: "Ningguang",
    element: Element::Geo,
    hp: [821, 2108, 2721, 4076, 4512, 5189, 5770, 6448, 6884, 7561, 7996, 8674, 9110, 9787],
    atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212],
    def: [48, 123, 159, 239, 264, 304, 338, 378, 403, 443, 468, 508, 534, 573],
    sub_stat: CharacterSubStatFamily::Bonus240(StatName::GeoBonus),
    weapon_type: WeaponType::Catalyst,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·千金掷",
        en: "Normal Attack: Sparkling Scatter",
    ),
    skill_name2: locale!(
        zh_cn: "璇玑屏",
        en: "Jade Screen",
    ),
    skill_name3: locale!(
        zh_cn: "天权崩玉",
        en: "Starshatter",
    ),
    name_locale: locale!(
        zh_cn: "凝光",
        en: "Ningguang",
    )
};

pub struct NingguangEffect {
    pub has_talent2: bool,
    pub rate: f64
}

impl NingguangEffect {
    pub fn new(common_data: &CharacterCommonData, config: &CharacterConfig) -> NingguangEffect {
        let rate = match *config {
            CharacterConfig::Ningguang { talent2_rate } => talent2_rate,
            _ => 0.0
        };
        NingguangEffect {
            has_talent2: common_data.has_talent2,
            rate
        }
    }
}

impl<A: Attribute> ChangeAttribute<A> for NingguangEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent2 {
            attribute.set_value_by(AttributeName::BonusGeo, "凝光天赋：储之千日，用之一刻", self.rate * 0.12);
        }
    }
}

pub struct Ningguang;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum NingguangDamageEnum {
    Normal,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
}

impl NingguangDamageEnum {
    pub fn get_skill_type(&self) ->SkillType {
        use NingguangDamageEnum::*;
        match *self {
            Normal => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
                        Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for NingguangDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum NingguangRoleEnum {
    MainAB, // 主打平A重击
}

impl CharacterTrait for Ningguang {
    const STATIC_DATA: CharacterStaticData = NINGGUANG_STATIC_DATA;
    type SkillType = NingguangSkillType;
    const SKILL: Self::SkillType = NINGGUANG_SKILL;
    type DamageEnumType = NingguangDamageEnum;
    type RoleEnum = NingguangRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: NingguangDamageEnum::Normal as usize, text: locale!(zh_cn: "普通攻击伤害", en: "Normal Attack DMG") },
            CharacterSkillMapItem { index: NingguangDamageEnum::Charged1 as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: NingguangDamageEnum::Charged2 as usize, text: locale!(zh_cn: "星璇伤害", en: "DMG per Star Jade") },
            CharacterSkillMapItem { index: NingguangDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: NingguangDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: NingguangDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: NingguangDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: NingguangDamageEnum::Q1 as usize, text: locale!(zh_cn: "每颗宝石伤害", en: "DMG Per Gem") }
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_rate",
            title: locale!(
                zh_cn: "天赋「储之千日，用之一刻」应用比例",
                en: "Talent「Strategic Reserve」Apply Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: NingguangDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use NingguangDamageEnum::*;
        let ratio = match s {
            Normal => NINGGUANG_SKILL.normal_dmg1[s1],
            Charged1 => NINGGUANG_SKILL.charged_dmg1[s1],
            Charged2 => NINGGUANG_SKILL.charged_dmg2[s1],
            Plunging1 => NINGGUANG_SKILL.plunging_dmg1[s1],
            Plunging2 => NINGGUANG_SKILL.plunging_dmg2[s1],
            Plunging3 => NINGGUANG_SKILL.plunging_dmg3[s1],
            E1 => NINGGUANG_SKILL.elemental_skill_dmg1[s2],
            Q1 => NINGGUANG_SKILL.elemental_burst_dmg1[s3]
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Geo,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(NingguangEffect::new(common_data, config)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: NingguangRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            NingguangRoleEnum::MainAB => Box::new(NingguangDefaultTargetFunction)
        }
    }
}
