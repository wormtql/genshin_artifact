use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::RazorDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct RazorSkillType {
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
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_a_speedup: [f64; 15],
}

pub const RAZOR_SKILL: RazorSkillType = RazorSkillType {
    normal_dmg1: [0.9592, 1.0246, 1.09, 1.1772, 1.2426, 1.3189, 1.417, 1.5151, 1.6132, 1.7113, 1.8094, 1.9075, 2.0056, 2.1037, 2.2018],
    normal_dmg2: [0.8263, 0.8827, 0.939, 1.0141, 1.0705, 1.1362, 1.2207, 1.3052, 1.3897, 1.4742, 1.5587, 1.6433, 1.7278, 1.8123, 1.8968],
    normal_dmg3: [1.0331, 1.1036, 1.174, 1.2679, 1.3384, 1.4205, 1.5262, 1.6319, 1.7375, 1.8432, 1.9488, 2.0545, 2.1602, 2.2658, 2.3715],
    normal_dmg4: [1.3605, 1.4532, 1.546, 1.6697, 1.7624, 1.8707, 2.0098, 2.1489, 2.2881, 2.4272, 2.5664, 2.7055, 2.8446, 2.9838, 3.1229],
    charged_dmg1: [0.6254, 0.6763, 0.7272, 0.7999, 0.8508, 0.909, 0.989, 1.069, 1.149, 1.2362, 1.3235, 1.4108, 1.498, 1.5853, 1.6726],
    charged_dmg2: [1.1309, 1.223, 1.315, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933, 2.5511, 2.7089, 2.8667, 3.0245],
    plunging_dmg1: [0.8205, 0.8872, 0.954, 1.0494, 1.1162, 1.1925, 1.2975, 1.4024, 1.5074, 1.6219, 1.7363, 1.8508, 1.9653, 2.0798, 2.1943],
    plunging_dmg2: [1.6406, 1.7741, 1.9077, 2.0984, 2.232, 2.3846, 2.5944, 2.8043, 3.0141, 3.243, 3.4719, 3.7009, 3.9298, 4.1587, 4.3876],
    plunging_dmg3: [2.0492, 2.216, 2.3828, 2.621, 2.7878, 2.9785, 3.2406, 3.5027, 3.7648, 4.0507, 4.3366, 4.6226, 4.9085, 5.1944, 5.4804],
    elemental_skill_dmg1: [1.992, 2.1414, 2.2908, 2.49, 2.6394, 2.7888, 2.988, 3.1872, 3.3864, 3.5856, 3.7848, 3.984, 4.233, 4.482, 4.731],
    elemental_skill_dmg2: [2.952, 3.1734, 3.3948, 3.69, 3.9114, 4.1328, 4.428, 4.7232, 5.0184, 5.3136, 5.6088, 5.904, 6.273, 6.642, 7.011],
    elemental_burst_dmg1: [1.6, 1.72, 1.84, 2., 2.12, 2.24, 2.4, 2.56, 2.72, 2.88, 3.04, 3.2, 3.4, 3.6, 3.8],
    elemental_burst_dmg2: [0.24, 0.258, 0.276, 0.3, 0.318, 0.336, 0.36, 0.384, 0.408, 0.432, 0.456, 0.48, 0.51, 0.54, 0.57],
    elemental_burst_a_speedup: [0.26, 0.28, 0.3, 0.32, 0.34, 0.36, 0.37, 0.38, 0.39, 0.4, 0.4, 0.4, 0.4, 0.4, 0.4]
};

pub const RAZOR_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Razor,
    internal_name: "Razor",
    element: Element::Electro,
    hp: [1003, 2577, 3326, 4982, 5514, 6343, 7052, 7881, 8413, 9241, 9773, 10602, 11134, 11962],
    atk: [20, 50, 65, 97, 108, 124, 138, 154, 164, 180, 191, 207, 217, 234],
    def: [63, 162, 209, 313, 346, 398, 443, 495, 528, 580, 613, 665, 699, 751],
    sub_stat: CharacterSubStatFamily::Bonus300(StatName::PhysicalBonus),
    weapon_type: WeaponType::Claymore,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·钢脊",
        en: "Normal Attack: Steel Fang",
    ),
    skill_name2: locale!(
        zh_cn: "利爪与苍雷",
        en: "Claw and Thunder",
    ),
    skill_name3: locale!(
        zh_cn: "雷牙",
        en: "Lightning Fang",
    ),
    name_locale: locale!(
        zh_cn: "雷泽",
        en: "Razor",
    )
};

pub struct RazorEffect {
    pub stack: f64,
    pub talent2_ratio: f64,
    pub has_talent2: bool,
}

impl RazorEffect {
    pub fn new(common_data: &CharacterCommonData, config: &CharacterConfig) -> RazorEffect {
        let (stack, talent2_ratio) = match *config {
            CharacterConfig::Razor { e_stack, talent2_ratio } => (e_stack, talent2_ratio),
            _ => (0.0, 0.0)
        };
        RazorEffect {
            stack,
            talent2_ratio,
            has_talent2: common_data.has_talent2
        }
    }
}

impl<A: Attribute> ChangeAttribute<A> for RazorEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let recharge_bonus = 0.2 * self.stack;
        attribute.set_value_by(AttributeName::Recharge, "雷泽：雷之印加成", recharge_bonus);
        if self.has_talent2 {
            attribute.set_value_by(AttributeName::Recharge, "雷泽天赋：饥饿", self.talent2_ratio * 0.3);
        }
    }
}

pub struct Razor;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum RazorDamageEnum {
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
    E2,
    Q1,
    QNormal1,
    QNormal2,
    QNormal3,
    QNormal4,
}

impl RazorDamageEnum {
    pub fn get_element(&self) -> Element {
        use RazorDamageEnum::*;
        match *self {
            E1 | E2 | Q1 | QNormal1 | QNormal2 | QNormal3 | QNormal4 => Element::Electro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use RazorDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | QNormal1 | QNormal2 | QNormal3 | QNormal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for RazorDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum RazorRoleEnum {
    Main
}

impl CharacterTrait for Razor {
    const STATIC_DATA: CharacterStaticData = RAZOR_STATIC_DATA;
    type SkillType = RazorSkillType;
    const SKILL: Self::SkillType = RAZOR_SKILL;
    type DamageEnumType = RazorDamageEnum;
    type RoleEnum = RazorRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: RazorDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: RazorDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: RazorDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: RazorDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: RazorDamageEnum::Charged1 as usize, text: charged_dmg!("loop1") },
            CharacterSkillMapItem { index: RazorDamageEnum::Charged2 as usize, text: charged_dmg!("loop2") },
            CharacterSkillMapItem { index: RazorDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: RazorDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: RazorDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: RazorDamageEnum::E1 as usize, text: locale!(zh_cn: "点按技能伤害", en: "Tapping Skill DMG") },
            CharacterSkillMapItem { index: RazorDamageEnum::E2 as usize, text: locale!(zh_cn: "长按技能伤害", en: "Hold Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: RazorDamageEnum::Q1 as usize, text: locale!(zh_cn: "爆发伤害", en: "Burst DMG") },
            CharacterSkillMapItem { index: RazorDamageEnum::QNormal1 as usize, text: locale!(zh_cn: "狼魂-一段伤害", en: "Soul Companion 1-Hit DMG") },
            CharacterSkillMapItem { index: RazorDamageEnum::QNormal2 as usize, text: locale!(zh_cn: "狼魂-二段伤害", en: "Soul Companion 2-Hit DMG") },
            CharacterSkillMapItem { index: RazorDamageEnum::QNormal3 as usize, text: locale!(zh_cn: "狼魂-三段伤害", en: "Soul Companion 3-Hit DMG") },
            CharacterSkillMapItem { index: RazorDamageEnum::QNormal4 as usize, text: locale!(zh_cn: "狼魂-四段伤害", en: "Soul Companion 4-Hit DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_stack",
            title: locale!(
                zh_cn: "雷之印层数",
                en: "Electro Sigil Count"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        },
        ItemConfig {
            name: "talent2_ratio",
            title: locale!(
                zh_cn: "天赋「饥饿」应用比例",
                en: "Talent「Hunger」Apply Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: RazorDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use RazorDamageEnum::*;
        let ratio = match s {
            Normal1 => RAZOR_SKILL.normal_dmg1[s1],
            Normal2 => RAZOR_SKILL.normal_dmg2[s1],
            Normal3 => RAZOR_SKILL.normal_dmg3[s1],
            Normal4 => RAZOR_SKILL.normal_dmg4[s1],
            Charged1 => RAZOR_SKILL.charged_dmg1[s1],
            Charged2 => RAZOR_SKILL.charged_dmg2[s1],
            Plunging1 => RAZOR_SKILL.plunging_dmg1[s1],
            Plunging2 => RAZOR_SKILL.plunging_dmg2[s1],
            Plunging3 => RAZOR_SKILL.plunging_dmg3[s1],
            E1 => RAZOR_SKILL.elemental_skill_dmg1[s2],
            E2 => RAZOR_SKILL.elemental_skill_dmg2[s2],
            Q1 => RAZOR_SKILL.elemental_burst_dmg1[s3],
            QNormal1 => RAZOR_SKILL.elemental_burst_dmg2[s3] * RAZOR_SKILL.normal_dmg1[s1],
            QNormal2 => RAZOR_SKILL.elemental_burst_dmg2[s3] * RAZOR_SKILL.normal_dmg2[s1],
            QNormal3 => RAZOR_SKILL.elemental_burst_dmg2[s3] * RAZOR_SKILL.normal_dmg3[s1],
            QNormal4 => RAZOR_SKILL.elemental_burst_dmg2[s3] * RAZOR_SKILL.normal_dmg4[s1],
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

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(RazorEffect::new(common_data, config)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: RazorRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            RazorRoleEnum::Main => Box::new(RazorDefaultTargetFunction)
        }
    }
}