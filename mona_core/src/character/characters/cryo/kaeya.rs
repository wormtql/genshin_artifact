use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::KaeyaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct KaeyaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_burst_dmg1: [f64; 15],
}

pub const KAEYA_SKILL: KaeyaSkillType = KaeyaSkillType {
    normal_dmg1: [0.5375, 0.5813, 0.625, 0.6875, 0.7313, 0.7813, 0.85, 0.9188, 0.9875, 1.0625, 1.1484, 1.2495, 1.3506, 1.4516, 1.5619],
    normal_dmg2: [0.5169, 0.5589, 0.601, 0.6611, 0.7032, 0.7513, 0.8174, 0.8835, 0.9496, 1.0217, 1.1043, 1.2015, 1.2987, 1.3959, 1.5019],
    normal_dmg3: [0.6527, 0.7059, 0.759, 0.8349, 0.888, 0.9488, 1.0322, 1.1157, 1.1992, 1.2903, 1.3947, 1.5174, 1.6401, 1.7629, 1.8967],
    normal_dmg4: [0.7086, 0.7663, 0.824, 0.9064, 0.9641, 1.03, 1.1206, 1.2113, 1.3019, 1.4008, 1.5141, 1.6473, 1.7806, 1.9138, 2.0592],
    normal_dmg5: [0.8824, 0.9542, 1.026, 1.1286, 1.2004, 1.2825, 1.3954, 1.5082, 1.6211, 1.7442, 1.8853, 2.0512, 2.2171, 2.383, 2.564],
    charged_dmg11: [0.5504, 0.5952, 0.64, 0.704, 0.7488, 0.8, 0.8704, 0.9408, 1.0112, 1.088, 1.176, 1.2795, 1.383, 1.4865, 1.5994],
    charged_dmg12: [0.731, 0.7905, 0.85, 0.935, 0.9945, 1.0625, 1.156, 1.2495, 1.343, 1.445, 1.5619, 1.6993, 1.8368, 1.9742, 2.1242],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.912, 2.0554, 2.1988, 2.39, 2.5334, 2.6768, 2.868, 3.0592, 3.2504, 3.4416, 3.6328, 3.824, 4.063, 4.302, 4.541],
    elemental_burst_dmg1: [0.776, 0.8342, 0.8924, 0.97, 1.0282, 1.0864, 1.164, 1.2416, 1.3192, 1.3968, 1.4744, 1.552, 1.649, 1.746, 1.843],
};

pub const KAEYA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Kaeya,
    internal_name: "Kaeya",
    element: Element::Cryo,
    hp: [976, 2506, 3235, 4846, 5364, 6170, 6860, 7666, 8184, 8989, 9507, 10312, 10830, 11636],
    atk: [19, 48, 62, 93, 103, 118, 131, 147, 157, 172, 182, 198, 208, 223],
    def: [66, 171, 220, 330, 365, 420, 467, 522, 557, 612, 647, 702, 737, 792],
    sub_stat: CharacterSubStatFamily::Recharge267,
    weapon_type: WeaponType::Sword,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·仪典剑术",
        en: "Normal Attack: Ceremonial Bladework",
    ),
    skill_name2: locale!(
        zh_cn: "霜袭",
        en: "Frostgnaw",
    ),
    skill_name3: locale!(
        zh_cn: "凛冽轮舞",
        en: "Glacial Waltz",
    ),
    name_locale: locale!(
        zh_cn: "凯亚",
        en: "Kaeya",
    )
};

pub struct Kaeya;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum KaeyaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Charged11,
    Charged12,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1
}

impl KaeyaDamageEnum {
    pub fn get_element(&self) -> Element {
        use KaeyaDamageEnum::*;
        match *self {
            E1 | Q1 => Element::Cryo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use KaeyaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for KaeyaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum KaeyaRoleEnum {
    MainCryo
}

impl CharacterTrait for Kaeya {
    const STATIC_DATA: CharacterStaticData = KAEYA_STATIC_DATA;
    type SkillType = KaeyaSkillType;
    const SKILL: Self::SkillType = KAEYA_SKILL;
    type DamageEnumType = KaeyaDamageEnum;
    type RoleEnum = KaeyaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: KaeyaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: KaeyaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: KaeyaDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: KaeyaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: KaeyaDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: KaeyaDamageEnum::Charged11 as usize, text: charged_dmg!(1) },
            CharacterSkillMapItem { index: KaeyaDamageEnum::Charged12 as usize, text: charged_dmg!(2) },
            CharacterSkillMapItem { index: KaeyaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: KaeyaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: KaeyaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: KaeyaDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: KaeyaDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KaeyaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KaeyaDamageEnum::*;
        let ratio = match s {
            Normal1 => KAEYA_SKILL.normal_dmg1[s1],
            Normal2 => KAEYA_SKILL.normal_dmg2[s1],
            Normal3 => KAEYA_SKILL.normal_dmg3[s1],
            Normal4 => KAEYA_SKILL.normal_dmg4[s1],
            Normal5 => KAEYA_SKILL.normal_dmg5[s1],
            Charged11 => KAEYA_SKILL.charged_dmg11[s1],
            Charged12 => KAEYA_SKILL.charged_dmg11[s1],
            Plunging1 => KAEYA_SKILL.plunging_dmg1[s1],
            Plunging2 => KAEYA_SKILL.plunging_dmg1[s1],
            Plunging3 => KAEYA_SKILL.plunging_dmg1[s1],
            E1 => KAEYA_SKILL.elemental_skill_dmg1[s2],
            Q1 => KAEYA_SKILL.elemental_burst_dmg1[s3]
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

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: KaeyaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            KaeyaRoleEnum::MainCryo => Box::new(KaeyaDefaultTargetFunction)
        }
    }
}
