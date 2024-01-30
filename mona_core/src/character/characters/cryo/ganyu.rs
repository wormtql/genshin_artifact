use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeName};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::GanyuDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct GanyuSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub normal_dmg6: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub charged_dmg3: [f64; 15],
    pub charged_dmg4: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_hp: [f64; 15],
    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
}

pub const GANYU_SKILL: GanyuSkillType = GanyuSkillType {
    normal_dmg1: [0.3173, 0.3432, 0.369, 0.4059, 0.4317, 0.4613, 0.5018, 0.5424, 0.583, 0.6273, 0.678, 0.7377, 0.7974, 0.857, 0.9221],
    normal_dmg2: [0.356, 0.385, 0.414, 0.4554, 0.4844, 0.5175, 0.563, 0.6086, 0.6541, 0.7038, 0.7607, 0.8277, 0.8946, 0.9616, 1.0346],
    normal_dmg3: [0.4549, 0.492, 0.529, 0.5819, 0.6189, 0.6613, 0.7194, 0.7776, 0.8358, 0.8993, 0.972, 1.0576, 1.1431, 1.2287, 1.322],
    normal_dmg4: [0.4549, 0.492, 0.529, 0.5819, 0.6189, 0.6613, 0.7194, 0.7776, 0.8358, 0.8993, 0.972, 1.0576, 1.1431, 1.2287, 1.322],
    normal_dmg5: [0.4825, 0.5217, 0.561, 0.6171, 0.6564, 0.7013, 0.763, 0.8247, 0.8864, 0.9537, 1.0308, 1.1216, 1.2123, 1.303, 1.4019],
    normal_dmg6: [0.5762, 0.6231, 0.67, 0.737, 0.7839, 0.8375, 0.9112, 0.9849, 1.0586, 1.139, 1.2311, 1.3395, 1.4478, 1.5561, 1.6743],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    charged_dmg3: [1.28, 1.376, 1.472, 1.6, 1.696, 1.792, 1.92, 2.048, 2.176, 2.304, 2.432, 2.56, 2.72, 2.88, 3.04],
    charged_dmg4: [2.176, 2.3392, 2.5024, 2.72, 2.8832, 3.0464, 3.264, 3.4816, 3.6992, 3.9168, 4.1344, 4.352, 4.624, 4.896, 5.168],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_hp: [1.2, 1.29, 1.38, 1.5, 1.59, 1.68, 1.8, 1.92, 2.04, 2.16, 2.28, 2.4, 2.55, 2.7, 2.85],
    elemental_skill_dmg1: [1.32, 1.419, 1.518, 1.65, 1.749, 1.848, 1.98, 2.112, 2.244, 2.376, 2.508, 2.64, 2.805, 2.97, 3.135],
    elemental_burst_dmg1: [0.7027, 0.7554, 0.8081, 0.8784, 0.9311, 0.9838, 1.0541, 1.1244, 1.1946, 1.2649, 1.3352, 1.4054, 1.4933, 1.5811, 1.669]
};

pub const GANYU_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Ganyu,
    internal_name: "Ganyu",
    element: Element::Cryo,
    hp: [763, 1978, 2632, 3939, 4403, 5066, 5686, 6355, 6820, 7495, 7960, 8643, 9108, 9797],
    atk: [26, 68, 90, 135, 151, 173, 194, 217, 233, 256, 272, 295, 311, 335],
    def: [49, 127, 169, 253, 283, 326, 366, 409, 439, 482, 512, 556, 586, 630],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Bow,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·流天射术",
        en: "Normal Attack: Liutian Archery",
    ),
    skill_name2: locale!(
        zh_cn: "山泽麟迹",
        en: "Trail of the Qilin",
    ),
    skill_name3: locale!(
        zh_cn: "降众天华",
        en: "Celestial Shower",
    ),
    name_locale: locale!(
        zh_cn: "甘雨",
        en: "Ganyu",
    )
};

pub struct GanyuEffect {
    // talent1_rate: f64,
    talent2_rate: f64
}

impl<T: Attribute> ChangeAttribute<T> for GanyuEffect {
    fn change_attribute(&self, attribute: &mut T) {
        // attribute.set_value_by(AttributeName::CriticalBase, "甘雨天赋：唯此一心", 0.2 * self.talent1_rate);
        attribute.set_value_by(AttributeName::BonusCryo, "甘雨天赋：天地交泰", 0.2 * self.talent2_rate);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum GanyuDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Normal6,
    Charged1,
    Charged2,
    Charged3,
    Charged4,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1
}

impl Into<usize> for GanyuDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl GanyuDamageEnum {
    pub fn get_element(&self) -> Element {
        use GanyuDamageEnum::*;

        match *self {
            Charged2 | Charged3 | Charged4 | E1 | Q1 => Element::Cryo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use GanyuDamageEnum::*;

        match *self {
            Charged1 | Charged2 | Charged3 | Charged4 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
            _ => SkillType::NormalAttack
        }
    }
}

pub struct Ganyu;

#[derive(Copy, Clone, FromPrimitive)]
pub enum GanyuRoleEnum {
    Main
}

impl CharacterTrait for Ganyu {
    const STATIC_DATA: CharacterStaticData = GANYU_STATIC_DATA;
    type SkillType = GanyuSkillType;
    const SKILL: Self::SkillType = GANYU_SKILL;
    type DamageEnumType = GanyuDamageEnum;
    type RoleEnum = GanyuRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: GanyuDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: GanyuDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: GanyuDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: GanyuDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: GanyuDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: GanyuDamageEnum::Normal6 as usize, text: hit_n_dmg!(6) },
            CharacterSkillMapItem { index: GanyuDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: GanyuDamageEnum::Charged2 as usize, text: locale!(zh_cn: "一段蓄力瞄准射击", en: "Aimed Shot Charge Level 1") },
            CharacterSkillMapItem { index: GanyuDamageEnum::Charged3 as usize, text: locale!(zh_cn: "霜华矢命中伤害", en: "Frostflake Arrow DMG") },
            CharacterSkillMapItem { index: GanyuDamageEnum::Charged4 as usize, text: locale!(zh_cn: "霜华矢·霜华绽发伤害", en: "Frostflake Arrow Bloom DMG") },
            CharacterSkillMapItem { index: GanyuDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: GanyuDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: GanyuDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: GanyuDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: GanyuDamageEnum::Q1 as usize, text: locale!(zh_cn: "冰棱伤害", en: "Ice Shard DMG") }
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_rate",
            title: locale!(
                zh_cn: "天赋「天地交泰」应用比例",
                en: "Talent「Harmony Between Heaven and Earth」Apply Ratio"
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_rate",
            title: locale!(
                zh_cn: "天赋「唯此一心」应用比例",
                en: "Talent「Undivided Heart」Apply Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) ->D::Result {
        let s: GanyuDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let talent1_rate = match *config {
            CharacterSkillConfig::Ganyu { talent1_rate } => talent1_rate,
            _ => 0.0
        };

        use GanyuDamageEnum::*;
        let ratio = match s {
            Normal1 => GANYU_SKILL.normal_dmg1[s1],
            Normal2 => GANYU_SKILL.normal_dmg2[s1],
            Normal3 => GANYU_SKILL.normal_dmg3[s1],
            Normal4 => GANYU_SKILL.normal_dmg4[s1],
            Normal5 => GANYU_SKILL.normal_dmg5[s1],
            Normal6 => GANYU_SKILL.normal_dmg6[s1],
            Charged1 => GANYU_SKILL.charged_dmg1[s1],
            Charged2 => GANYU_SKILL.charged_dmg2[s1],
            Charged3 => GANYU_SKILL.charged_dmg3[s1],
            Charged4 => GANYU_SKILL.charged_dmg4[s1],
            Plunging1 => GANYU_SKILL.plunging_dmg1[s1],
            Plunging2 => GANYU_SKILL.plunging_dmg2[s1],
            Plunging3 => GANYU_SKILL.plunging_dmg3[s1],
            E1 => GANYU_SKILL.elemental_skill_dmg1[s2],
            Q1 => GANYU_SKILL.elemental_burst_dmg1[s3]
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        if s == GanyuDamageEnum::Charged3 || s == GanyuDamageEnum::Charged4 {
            builder.add_extra_critical("甘雨天赋：唯此一心", talent1_rate * 0.2);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(match *config {
            CharacterConfig::Ganyu { talent2_rate } => GanyuEffect {
                // talent1_rate,
                talent2_rate
            },
            _ => GanyuEffect {
                // talent1_rate: 0.0,
                talent2_rate: 0.0
            }
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: GanyuRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            GanyuRoleEnum::Main => Box::new(GanyuDefaultTargetFunction {
                melt_rate: 0.5
            })
        }
    }
}
