use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::XiaoDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct XiaoSkillType {
    pub normal_dmg11: [f64; 15],
    pub normal_dmg12: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg41: [f64; 15],
    pub normal_dmg42: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub normal_dmg6: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_bonus: [f64; 15],
    pub elemental_burst_hp_drain: [f64; 15],
}

pub const XIAO_SKILL: XiaoSkillType = XiaoSkillType {
    normal_dmg11: [0.2754, 0.2942, 0.313, 0.338, 0.3568, 0.3787, 0.4069, 0.4351, 0.4632, 0.4914, 0.5196, 0.5478, 0.5759, 0.6041, 0.6323],
    normal_dmg12: [0.2754, 0.2942, 0.313, 0.338, 0.3568, 0.3787, 0.4069, 0.4351, 0.4632, 0.4914, 0.5196, 0.5478, 0.5759, 0.6041, 0.6323],
    normal_dmg2: [0.5694, 0.6082, 0.647, 0.6988, 0.7376, 0.7829, 0.8411, 0.8993, 0.9576, 1.0158, 1.074, 1.1323, 1.1905, 1.2487, 1.3069],
    normal_dmg3: [0.6855, 0.7323, 0.779, 0.8413, 0.8881, 0.9426, 1.0127, 1.0828, 1.1529, 1.223, 1.2931, 1.3633, 1.4334, 1.5035, 1.5736],
    normal_dmg41: [0.3766, 0.4023, 0.428, 0.4622, 0.4879, 0.5179, 0.5564, 0.5949, 0.6334, 0.672, 0.7105, 0.749, 0.7875, 0.826, 0.8646],
    normal_dmg42: [0.3766, 0.4023, 0.428, 0.4622, 0.4879, 0.5179, 0.5564, 0.5949, 0.6334, 0.672, 0.7105, 0.749, 0.7875, 0.826, 0.8646],
    normal_dmg5: [0.7154, 0.7642, 0.813, 0.878, 0.9268, 0.9837, 1.0569, 1.1301, 1.2032, 1.2764, 1.3496, 1.4228, 1.4959, 1.5691, 1.6423],
    normal_dmg6: [0.9583, 1.0237, 1.089, 1.1761, 1.2415, 1.3177, 1.4157, 1.5137, 1.6117, 1.7097, 1.8077, 1.9058, 2.0038, 2.1018, 2.1998],
    charged_dmg1: [1.2109, 1.2934, 1.376, 1.4861, 1.5686, 1.665, 1.7888, 1.9126, 2.0365, 2.1603, 2.2842, 2.408, 2.5318, 2.6557, 2.7795],
    plunging_dmg1: [0.8183, 0.8849, 0.9516, 1.0467, 1.1133, 1.1894, 1.2941, 1.3988, 1.5035, 1.6176, 1.7318, 1.846, 1.9602, 2.0744, 2.1886],
    plunging_dmg2: [1.6363, 1.7695, 1.9027, 2.093, 2.2262, 2.3784, 2.5877, 2.797, 3.0063, 3.2346, 3.4629, 3.6912, 3.9196, 4.1479, 4.3762],
    plunging_dmg3: [2.0439, 2.2102, 2.3766, 2.6142, 2.7806, 2.9707, 3.2321, 3.4936, 3.755, 4.0402, 4.3254, 4.6106, 4.8957, 5.1809, 5.4661],
    elemental_skill_dmg1: [2.528, 2.7176, 2.9072, 3.16, 3.3496, 3.5392, 3.792, 4.0448, 4.2976, 4.5504, 4.8032, 5.056, 5.372, 5.688, 6.004],
    elemental_burst_bonus: [0.5845, 0.6195, 0.6545, 0.7, 0.735, 0.77, 0.8155, 0.861, 0.9065, 0.952, 0.9975, 1.043, 1.0885, 1.134, 1.1795],
    elemental_burst_hp_drain: [0.03, 0.03, 0.03, 0.025, 0.025, 0.025, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02, 0.02]
};

pub const XIAO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Xiao,
    internal_name: "Xiao",
    element: Element::Anemo,
    hp: [991, 2572, 3422, 5120, 5724, 6586, 7391, 8262, 8866, 9744, 10348, 11236, 11840, 12736],
    atk: [27, 71, 94, 140, 157, 181, 203, 227, 243, 267, 284, 308, 325, 349],
    def: [62, 161, 215, 321, 359, 413, 464, 519, 556, 612, 649, 705, 743, 799],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Polearm,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·卷积微尘",
        en: "Normal Attack: Whirlwind Thrust",
    ),
    skill_name2: locale!(
        zh_cn: "风轮两立",
        en: "Lemniscatic Wind Cycling",
    ),
    skill_name3: locale!(
        zh_cn: "靖妖傩舞",
        en: "Bane of All Evil",
    ),
    name_locale: locale!(
        zh_cn: "魈",
        en: "Xiao",
    )
};

pub struct Xiao;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum XiaoDamageEnum {
    Normal11,
    Normal12,
    Normal1,
    Normal2,
    Normal3,
    Normal41,
    Normal42,
    Normal4,
    Normal5,
    Normal6,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1
}

impl XiaoDamageEnum {
    pub fn is_affected_by_q(&self) -> bool {
        *self != XiaoDamageEnum::E1
    }

    pub fn get_element(&self, after_q: bool) -> Element {
        if after_q {
            Element::Anemo
        } else {
            use XiaoDamageEnum::*;
            match *self {
                E1 => Element::Anemo,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use XiaoDamageEnum::*;
        match *self {
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            _ => SkillType::NormalAttack
        }
    }
}

impl Into<usize> for XiaoDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum XiaoRoleEnum {
    Main
}

impl CharacterTrait for Xiao {
    const STATIC_DATA: CharacterStaticData = XIAO_STATIC_DATA;
    type SkillType = XiaoSkillType;
    const SKILL: Self::SkillType = XIAO_SKILL;
    type DamageEnumType = XiaoDamageEnum;
    type RoleEnum = XiaoRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: XiaoDamageEnum::Normal11 as usize, text: hit_n_dmg!(1, 1) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Normal12 as usize, text: hit_n_dmg!(1, 2) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Normal41 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Normal42 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Normal6 as usize, text: hit_n_dmg!(6) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: XiaoDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: XiaoDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: XiaoDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ]),
        skill3: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_q",
            title: locale!(
                zh_cn: "靖妖傩舞",
                en: "Bane of All Evil",
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "talent1_stack",
            title: locale!(
                zh_cn: "天赋「降魔·平妖大圣」应用层数",
                en: "Talent「Conqueror of Evil: Tamer of Demons」Apply Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 4.0 },
        },
        ItemConfig {
            name: "talent2_stack",
            title: locale!(
                zh_cn: "天赋「坏劫·国土碾尘」应用层数",
                en: "Talent「Dissolution Eon: Heaven Fall」Apply Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 },
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: XiaoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use XiaoDamageEnum::*;
        let ratio = match s {
            Normal11 => XIAO_SKILL.normal_dmg11[s1],
            Normal12 => XIAO_SKILL.normal_dmg12[s1],
            Normal1 => XIAO_SKILL.normal_dmg11[s1] + XIAO_SKILL.normal_dmg12[s1],
            Normal2 => XIAO_SKILL.normal_dmg2[s1],
            Normal3 => XIAO_SKILL.normal_dmg3[s1],
            Normal41 => XIAO_SKILL.normal_dmg41[s1],
            Normal42 => XIAO_SKILL.normal_dmg42[s1],
            Normal4 => XIAO_SKILL.normal_dmg41[s1] + XIAO_SKILL.normal_dmg42[s1],
            Normal5 => XIAO_SKILL.normal_dmg5[s1],
            Normal6 => XIAO_SKILL.normal_dmg6[s1],
            Charged => XIAO_SKILL.charged_dmg1[s1],
            Plunging1 => XIAO_SKILL.plunging_dmg1[s1],
            Plunging2 => XIAO_SKILL.plunging_dmg2[s1],
            Plunging3 => XIAO_SKILL.plunging_dmg3[s1],
            E1 => XIAO_SKILL.elemental_skill_dmg1[s2]
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let (after_q, talent1_stack, talent2_stack) = match *config {
            CharacterSkillConfig::Xiao { after_q, talent1_stack, talent2_stack } => (after_q, talent1_stack, talent2_stack),
            _ => (false, 0.0, 0.0)
        };

        if after_q && s.is_affected_by_q() {
            let bonus = XIAO_SKILL.elemental_burst_bonus[s3];
            builder.add_extra_bonus("夜叉傩面加成", bonus);
        }

        if after_q && context.character_common_data.has_talent1 {
            builder.add_extra_bonus("魈天赋：降魔·平妖大圣", 0.05 + talent1_stack * 0.05);
        }

        if s == E1 && context.character_common_data.has_talent2 {
            builder.add_extra_bonus("魈天赋：坏劫·国土碾尘", 0.15 * talent2_stack);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(after_q),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: XiaoRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            XiaoRoleEnum::Main => Box::new(XiaoDefaultTargetFunction)
        }
    }
}
