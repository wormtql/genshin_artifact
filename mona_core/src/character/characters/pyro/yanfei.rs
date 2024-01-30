use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::YanfeiDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{hit_n_dmg, locale, plunging_dmg};

pub struct YanfeiSkillType {
    pub normal1: [f64; 15],
    pub normal2: [f64; 15],
    pub normal3: [f64; 15],
    pub charged1: [f64; 15],
    pub charged2: [f64; 15],
    pub charged3: [f64; 15],
    pub charged4: [f64; 15],
    pub charged5: [f64; 15],
    pub plunging1: [f64; 15],
    pub plunging2: [f64; 15],
    pub plunging3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_b_bonus: [f64; 15],
}

pub const YANFEI_SKILL: YanfeiSkillType = YanfeiSkillType {
    normal1: [0.5834, 0.6272, 0.6709, 0.7293, 0.773, 0.8168, 0.8751, 0.9335, 0.9918, 1.0501, 1.1085, 1.1668, 1.2398, 1.3127, 1.3856],
    normal2: [0.5213, 0.5604, 0.5994, 0.6516, 0.6907, 0.7298, 0.7819, 0.834, 0.8861, 0.9383, 0.9904, 1.0425, 1.1077, 1.1728, 1.238],
    normal3: [0.7601, 0.8171, 0.8741, 0.9502, 1.0072, 1.0642, 1.1402, 1.2162, 1.2922, 1.3682, 1.4442, 1.5203, 1.6153, 1.7103, 1.8053],
    charged1: [0.9823, 1.0411, 1.0999, 1.1764, 1.2352, 1.294, 1.3705, 1.447, 1.5234, 1.5999, 1.6764, 1.7528, 1.8293, 1.9058, 1.9822],
    charged2: [1.1556, 1.2248, 1.294, 1.384, 1.4532, 1.5224, 1.6124, 1.7023, 1.7923, 1.8822, 1.9722, 2.0622, 2.1521, 2.2421, 2.332],
    charged3: [1.329, 1.4086, 1.4881, 1.5916, 1.6712, 1.7508, 1.8542, 1.9577, 2.0611, 2.1646, 2.268, 2.3715, 2.4749, 2.5784, 2.6818],
    charged4: [1.5023, 1.5923, 1.6823, 1.7992, 1.8892, 1.9791, 2.0961, 2.213, 2.33, 2.4469, 2.5639, 2.6808, 2.7978, 2.9147, 3.0317],
    charged5: [1.6757, 1.776, 1.8764, 2.0068, 2.1071, 2.2075, 2.3379, 2.4684, 2.5988, 2.7292, 2.8597, 2.9901, 3.1206, 3.251, 3.3815],
    plunging1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [1.696, 1.8232, 1.9504, 2.12, 2.2472, 2.3744, 2.544, 2.7136, 2.8832, 3.0528, 3.2224, 3.392, 3.604, 3.816, 4.028],
    elemental_burst_dmg1: [1.824, 1.9608, 2.0976, 2.28, 2.4168, 2.5536, 2.736, 2.9184, 3.1008, 3.2832, 3.4656, 3.648, 3.876, 4.104, 4.332],
    elemental_burst_b_bonus: [0.334, 0.354, 0.374, 0.4, 0.42, 0.44, 0.466, 0.492, 0.518, 0.544, 0.57, 0.596, 0.622, 0.648, 0.674]
};

pub const YANFEI_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Yanfei,
    internal_name: "Feiyan",
    element: Element::Pyro,
    hp: [784, 2014, 2160, 3895, 4311, 4959, 5514, 6161, 6578, 7225, 7641, 8289, 8705, 9352],
    atk: [20, 52, 67, 100, 111, 127, 141, 158, 169, 185, 196, 213, 223, 240],
    def: [49, 126, 163, 244, 271, 311, 346, 387, 413, 453, 480, 520, 546, 587],
    sub_stat: CharacterSubStatFamily::Bonus240(StatName::PyroBonus),
    weapon_type: WeaponType::Catalyst,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·火漆制印",
        en: "Normal Attack: Seal of Approval",
    ),
    skill_name2: locale!(
        zh_cn: "丹书立约",
        en: "Signed Edict",
    ),
    skill_name3: locale!(
        zh_cn: "凭此结契",
        en: "Done Deal",
    ),
    name_locale: locale!(
        zh_cn: "烟绯",
        en: "Yanfei",
    )
};

pub struct Yanfei;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum YanfeiDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Charged1,
    Charged2,
    Charged3,
    Charged4,
    Charged5,
    DmgTalent2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
}

impl YanfeiDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use YanfeiDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged1 | Charged2 | Charged3 | Charged4 | Charged5 | DmgTalent2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for YanfeiDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum YanfeiRoleEnum {
    Main
}

impl CharacterTrait for Yanfei {
    const STATIC_DATA: CharacterStaticData = YANFEI_STATIC_DATA;
    type SkillType = YanfeiSkillType;
    const SKILL: Self::SkillType = YANFEI_SKILL;
    type DamageEnumType = YanfeiDamageEnum;
    type RoleEnum = YanfeiRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: YanfeiDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Charged1 as usize, text: locale!(zh_cn: "重击-无印伤害", en: "Charged Attack-No Scarlet Seals") },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Charged2 as usize, text: locale!(zh_cn: "重击-1层伤害", en: "Charged Attack-1 Scarlet Seals") },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Charged3 as usize, text: locale!(zh_cn: "重击-2层伤害", en: "Charged Attack-2 Scarlet Seals") },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Charged4 as usize, text: locale!(zh_cn: "重击-3层伤害", en: "Charged Attack-3 Scarlet Seals") },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Charged5 as usize, text: locale!(zh_cn: "重击-4层伤害", en: "Charged Attack-4 Scarlet Seals") },
            CharacterSkillMapItem { index: YanfeiDamageEnum::DmgTalent2 as usize, text: locale!(zh_cn: "天赋2额外伤害", en: "Talent2 Additional DMG") },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: YanfeiDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: YanfeiDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: YanfeiDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_q",
            title: locale!(
                zh_cn: "灼灼",
                en: "Brilliance"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: YanfeiDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use YanfeiDamageEnum::*;
        let ratio = match s {
            Normal1 => YANFEI_SKILL.normal1[s1],
            Normal2 => YANFEI_SKILL.normal2[s1],
            Normal3 => YANFEI_SKILL.normal3[s1],
            Charged1 => YANFEI_SKILL.charged1[s1],
            Charged2 => YANFEI_SKILL.charged2[s1],
            Charged3 => YANFEI_SKILL.charged3[s1],
            Charged4 => YANFEI_SKILL.charged4[s1],
            Charged5 => YANFEI_SKILL.charged5[s1],
            DmgTalent2 => 0.8,
            Plunging1 => YANFEI_SKILL.plunging1[s1],
            Plunging2 => YANFEI_SKILL.plunging2[s1],
            Plunging3 => YANFEI_SKILL.plunging3[s1],
            E1 => YANFEI_SKILL.elemental_skill_dmg1[s2],
            Q1 => YANFEI_SKILL.elemental_burst_dmg1[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let after_q = match *config {
            CharacterSkillConfig::Yanfei { after_q } => after_q,
            _ => false
        };
        let skill_type = s.get_skill_type();
        if skill_type == SkillType::ChargedAttack && after_q {
            let bonus = YANFEI_SKILL.elemental_burst_b_bonus[s3];
            builder.add_extra_bonus("灼灼", bonus);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Pyro,
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: YanfeiRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            YanfeiRoleEnum::Main => Box::new(YanfeiDefaultTargetFunction)
        }
    }
}