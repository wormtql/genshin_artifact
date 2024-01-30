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
use crate::target_functions::target_functions::hydro::sangonomiya_kokomi_default::SangonomiyaKokomiDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct SangonomiyaKokomiSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_heal1: [f64; 15],
    pub elemental_skill_heal1_fixed: [f64; 15],
    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_a_bonus: [f64; 15],
    pub elemental_burst_b_bonus: [f64; 15],
    pub elemental_burst_e_bonus: [f64; 15],
    pub elemental_burst_heal1: [f64; 15],
    pub elemental_burst_heal1_fixed: [f64; 15],
}

pub const SANGONOMIYA_KOKOMI_SKILL: SangonomiyaKokomiSkillType = SangonomiyaKokomiSkillType {
    normal_dmg1: [0.6838, 0.735, 0.7863, 0.8547, 0.906, 0.9573, 1.0256, 1.094, 1.1624, 1.2308, 1.2991, 1.3675, 1.453, 1.5385, 1.6239],
    normal_dmg2: [0.6154, 0.6615, 0.7077, 0.7692, 0.8154, 0.8615, 0.9231, 0.9846, 1.0462, 1.1077, 1.1692, 1.2308, 1.3077, 1.3846, 1.4615],
    normal_dmg3: [0.9431, 1.0138, 1.0845, 1.1788, 1.2495, 1.3203, 1.4146, 1.5089, 1.6032, 1.6975, 1.7918, 1.8861, 2.004, 2.1219, 2.2398],
    charged_dmg1: [1.4832, 1.5944, 1.7057, 1.854, 1.9652, 2.0765, 2.2248, 2.3731, 2.5214, 2.6698, 2.8181, 2.9664, 3.1518, 3.3372, 3.5226],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_heal1: [0.044, 0.0473, 0.0506, 0.055, 0.0583, 0.0616, 0.066, 0.0704, 0.0748, 0.0792, 0.0836, 0.088, 0.0935, 0.099, 0.1045],
    elemental_skill_heal1_fixed: [424.0, 466.0, 512.0, 561.0, 614.0, 671.0, 731.0, 795.0, 862.0, 932.0, 1006.0, 1084.0, 1165.0, 1250.0, 1338.0],
    elemental_skill_dmg1: [1.0919, 1.1738, 1.2557, 1.3649, 1.4468, 1.5287, 1.6379, 1.747, 1.8562, 1.9654, 2.0746, 2.1838, 2.3203, 2.4568, 2.5933],
    elemental_burst_dmg1: [0.1042, 0.112, 0.1198, 0.1302, 0.138, 0.1458, 0.1562, 0.1667, 0.1771, 0.1875, 0.1979, 0.2083, 0.2213, 0.2344, 0.2474],
    elemental_burst_a_bonus: [0.0484, 0.052, 0.0557, 0.0605, 0.0641, 0.0678, 0.0726, 0.0774, 0.0823, 0.0871, 0.092, 0.0968, 0.1029, 0.1089, 0.115],
    elemental_burst_b_bonus: [0.0678, 0.0728, 0.0779, 0.0847, 0.0898, 0.0949, 0.1016, 0.1084, 0.1152, 0.122, 0.1287, 0.1355, 0.144, 0.1525, 0.1609],
    elemental_burst_e_bonus: [0.071, 0.0763, 0.0816, 0.0887, 0.094, 0.0993, 0.1064, 0.1135, 0.1206, 0.1277, 0.1348, 0.1419, 0.1508, 0.1597, 0.1685],
    elemental_burst_heal1: [0.0081, 0.0087, 0.0093, 0.0101, 0.0107, 0.0113, 0.0121, 0.0129, 0.0137, 0.0145, 0.0154, 0.0162, 0.0172, 0.0182, 0.0192],
    elemental_burst_heal1_fixed: [77.0, 85.0, 93.0, 102.0, 112.0, 122.0, 133.0, 144.0, 157.0, 169.0, 183.0, 197.0, 212.0, 227.0, 243.0],
};

pub const SANGONOMIYA_KOKOMI_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::SangonomiyaKokomi,
    internal_name: "Kokomi",
    element: Element::Hydro,
    hp: [1049, 2720, 3619, 5416, 6055, 6966, 7818, 8738, 9377, 10306, 10945, 11885, 12524, 13471],
    atk: [18, 47, 63, 94, 105, 121, 136, 152, 163, 179, 190, 207, 218, 234],
    def: [51, 133, 177, 264, 295, 340, 381, 426, 457, 503, 534, 580, 611, 657],
    sub_stat: CharacterSubStatFamily::Bonus288(StatName::HydroBonus),
    weapon_type: WeaponType::Catalyst,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击•水有常形",
        en: "Normal Attack: The Shape of Water",
    ),
    skill_name2: locale!(
        zh_cn: "海月之誓",
        en: "Kurage's Oath",
    ),
    skill_name3: locale!(
        zh_cn: "海人化羽",
        en: "Nereid's Ascension",
    ),
    name_locale: locale!(
        zh_cn: "珊瑚宫心海",
        en: "Sangonomiya Kokomi",
    )
};

pub struct SangonomiyaKokomiEffect;

impl SangonomiyaKokomiEffect {
    pub fn new() -> SangonomiyaKokomiEffect {
        SangonomiyaKokomiEffect {}
    }
}

impl<T: Attribute> ChangeAttribute<T> for SangonomiyaKokomiEffect {
    fn change_attribute(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "珊瑚宫心海天赋：庙算无遗", -1.0);
        attribute.set_value_by(AttributeName::HealingBonus, "珊瑚宫心海天赋：庙算无遗", 0.25);
    }
}

pub struct SangonomiyaKokomi;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum SangonomiyaKokomiDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    EHeal1,
    E1,
    Q1,
    QHeal1
}

impl SangonomiyaKokomiDamageEnum {
    pub fn is_heal(&self) -> bool {
        use SangonomiyaKokomiDamageEnum::*;
        match *self {
            EHeal1 | QHeal1 => true,
            _ => false
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use SangonomiyaKokomiDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            EHeal1 | E1 => SkillType::ElementalSkill,
            Q1 | QHeal1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for SangonomiyaKokomiDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum SangonomiyaKokomiRoleEnum {
    Normal
}

impl CharacterTrait for SangonomiyaKokomi {
    const STATIC_DATA: CharacterStaticData = SANGONOMIYA_KOKOMI_STATIC_DATA;
    type SkillType = SangonomiyaKokomiSkillType;
    const SKILL: Self::SkillType = SANGONOMIYA_KOKOMI_SKILL;
    type DamageEnumType = SangonomiyaKokomiDamageEnum;
    type RoleEnum = SangonomiyaKokomiRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::E1 as usize, text: locale!(zh_cn: "波纹伤害", en: "Ripple DMG") },
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::EHeal1 as usize, text: locale!(zh_cn: "治疗量", en: "Regeneration") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: SangonomiyaKokomiDamageEnum::QHeal1 as usize, text: locale!(zh_cn: "命中治疗量", en: "HP Regeneration Per Hit") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_q",
            title: locale!(
                zh_cn: "「仪来羽衣」状态",
                en: "Under 「Ceremonial Garment」",
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: SangonomiyaKokomiDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use SangonomiyaKokomiDamageEnum::*;
        if s.is_heal() {
            let ratio = match s {
                EHeal1 => SANGONOMIYA_KOKOMI_SKILL.elemental_skill_heal1[s2],
                QHeal1 => SANGONOMIYA_KOKOMI_SKILL.elemental_burst_heal1[s3],
                _ => 0.0
            };
            let fixed = match s {
                EHeal1 => SANGONOMIYA_KOKOMI_SKILL.elemental_skill_heal1_fixed[s2],
                QHeal1 => SANGONOMIYA_KOKOMI_SKILL.elemental_burst_heal1_fixed[s3],
                _ => 0.0
            };
            let mut builder = D::new();
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => SANGONOMIYA_KOKOMI_SKILL.normal_dmg1[s1],
                Normal2 => SANGONOMIYA_KOKOMI_SKILL.normal_dmg2[s1],
                Normal3 => SANGONOMIYA_KOKOMI_SKILL.normal_dmg3[s1],
                Charged => SANGONOMIYA_KOKOMI_SKILL.charged_dmg1[s1],
                Plunging1 => SANGONOMIYA_KOKOMI_SKILL.plunging_dmg1[s1],
                Plunging2 => SANGONOMIYA_KOKOMI_SKILL.plunging_dmg2[s1],
                Plunging3 => SANGONOMIYA_KOKOMI_SKILL.plunging_dmg3[s1],
                E1 => SANGONOMIYA_KOKOMI_SKILL.elemental_skill_dmg1[s2],
                Q1 => SANGONOMIYA_KOKOMI_SKILL.elemental_burst_dmg1[s3],
                _ => 0.0
            };
            let mut builder = D::new();
            if s == Q1 {
                builder.add_hp_ratio("技能倍率", ratio);
            } else {
                builder.add_atk_ratio("技能倍率", ratio);
            }

            let skill_type = s.get_skill_type();
            let after_q = match *config {
                CharacterSkillConfig::SangonomiyaKokomi { after_q } => after_q,
                _ => false
            };
            if after_q {
                if skill_type == SkillType::NormalAttack {
                    let bonus = SANGONOMIYA_KOKOMI_SKILL.elemental_burst_a_bonus[s3];
                    builder.add_hp_ratio("海人化羽伤害提升", bonus);
                    if context.character_common_data.has_talent2 {
                        let healing_bonus = context.attribute.get_value(AttributeName::HealingBonus);
                        builder.add_hp_ratio("珊瑚宫心海天赋「真珠御呗」", healing_bonus * 0.15);
                    }
                } else if skill_type == SkillType::ChargedAttack {
                    let bonus = SANGONOMIYA_KOKOMI_SKILL.elemental_burst_b_bonus[s3];
                    builder.add_hp_ratio("海人化羽伤害提升", bonus);
                    if context.character_common_data.has_talent2 {
                        let healing_bonus = context.attribute.get_value(AttributeName::HealingBonus);
                        builder.add_hp_ratio("珊瑚宫心海天赋「真珠御呗」", healing_bonus * 0.15);
                    }
                } else if skill_type == SkillType::ElementalSkill {
                    let bonus = SANGONOMIYA_KOKOMI_SKILL.elemental_burst_e_bonus[s3];
                    builder.add_hp_ratio("海人化羽伤害提升", bonus);
                }
            }

            builder.damage(
                &context.attribute,
                &context.enemy,
                Element::Hydro,
                skill_type,
                context.character_common_data.level,
                fumo,
            )
        }
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(SangonomiyaKokomiEffect::new()))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: SangonomiyaKokomiRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            SangonomiyaKokomiRoleEnum::Normal => Box::new(SangonomiyaKokomiDefaultTargetFunction)
        }
    }
}
