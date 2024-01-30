use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::TartagliaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::macros::{skill_type, damage_enum, skill_map, damage_ratio};
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};

pub struct CandaceSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_shield1: [f64; 15],
    pub e_shield1_fixed: [f64; 15],
    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],

    pub q_dmg1_2: [f64; 15],
}

pub const CANDACE_SKILL: CandaceSkillType = CandaceSkillType {
    normal_dmg1: [0.608, 0.6575, 0.707, 0.7777, 0.8272, 0.8838, 0.9615, 1.0393, 1.1171, 1.2019, 1.2867, 1.3716, 1.4564, 1.5413, 1.6261],
    normal_dmg2: [0.6115, 0.6612, 0.711, 0.7821, 0.8319, 0.8888, 0.967, 1.0452, 1.1234, 1.2087, 1.294, 1.3793, 1.4647, 1.55, 1.6353],
    normal_dmg31: [0.3549, 0.3838, 0.4126, 0.4539, 0.4828, 0.5158, 0.5612, 0.6066, 0.652, 0.7015, 0.751, 0.8005, 0.8501, 0.8996, 0.9491],
    normal_dmg32: [0.4337, 0.469, 0.5044, 0.5548, 0.5901, 0.6304, 0.6859, 0.7414, 0.7969, 0.8574, 0.9179, 0.9784, 1.039, 1.0995, 1.16],
    normal_dmg4: [0.9494, 1.0267, 1.104, 1.2144, 1.2917, 1.38, 1.5014, 1.6229, 1.7443, 1.8768, 2.0093, 2.1418, 2.2742, 2.4067, 2.5392],
    charged_dmg: [1.2418, 1.3429, 1.444, 1.5884, 1.6895, 1.805, 1.9638, 2.1227, 2.2815, 2.4548, 2.6281, 2.8014, 2.9746, 3.1479, 3.3212],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_shield1: [0.12, 0.129, 0.138, 0.15, 0.159, 0.168, 0.18, 0.192, 0.204, 0.216, 0.228, 0.24, 0.255, 0.27, 0.285],
    e_shield1_fixed: [1155.56, 1271.14, 1396.34, 1531.17, 1675.64, 1829.73, 1993.46, 2166.82, 2349.81, 2542.43, 2744.68, 2956.57, 3178.08, 3409.22, 3650.0],
    e_dmg1: [0.12, 0.129, 0.138, 0.15, 0.159, 0.168, 0.18, 0.192, 0.204, 0.216, 0.228, 0.24, 0.255, 0.27, 0.285],
    e_dmg2: [0.1904, 0.2047, 0.219, 0.238, 0.2523, 0.2666, 0.2856, 0.3046, 0.3237, 0.3427, 0.3618, 0.3808, 0.4046, 0.4284, 0.4522],
    q_dmg1_2: [0.0661, 0.0711, 0.076, 0.0826, 0.0876, 0.0925, 0.0992, 0.1058, 0.1124, 0.119, 0.1256, 0.1322, 0.1405, 0.1487, 0.157],
};

pub struct CandaceEffect {
    pub c2: bool,
    pub c2_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for CandaceEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.c2 {
            attribute.add_hp_percentage("坎蒂丝天赋命座2「贯月的耀锋」", 0.2 * self.c2_rate);
        }
    }
}

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum CandaceDamageEnum {
    Normal1,
    Normal2,
    Normal31,
    Normal32,
    Normal4,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    Q1,
    Q2,
}

impl CandaceDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use CandaceDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst,
        }
    }

    pub fn get_element(&self, crown: bool) -> Element {
        use CandaceDamageEnum::*;
        if crown {
            return Element::Hydro
        }
        match *self {
            E1 | E2 | Q2 | Q1 => Element::Hydro,
            _ => Element::Physical
        }
    }
}

impl Into<usize> for CandaceDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

pub struct Candace;

impl CharacterTrait for Candace {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Candace,
        internal_name: "Candace",
        element: Element::Hydro,
        hp: [912, 2342, 3024, 4529, 5013, 5766, 6411, 7164, 7648, 8401, 8885, 9638, 10122, 10875],
        atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212],
        def: [57, 147, 190, 284, 315, 362, 402, 450, 480, 527, 558, 605, 635, 683],
        sub_stat: CharacterSubStatFamily::HP240,
        weapon_type: WeaponType::Polearm,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·流耀枪术·守势",
            en: "Normal Attack: Gleaming Spear – Guardian Stance",
        ),
        skill_name2: locale!(
            zh_cn: "圣仪·苍鹭庇卫",
            en: "Sacred Rite: Heron’s Sanctum",
        ),
        skill_name3: locale!(
            zh_cn: "圣仪·灰鸰衒潮",
            en: "Sacred Rite: Wagtail’s Tide",
        ),
        name_locale: locale!(
            zh_cn: "坎蒂丝",
            en: "Candace",
        )
    };
    type SkillType = CandaceSkillType;
    const SKILL: Self::SkillType = CANDACE_SKILL;
    type DamageEnumType = CandaceDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            CandaceDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal31 hit_n_dmg!(3, 1)
            Normal32 hit_n_dmg!(3, 2)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            CandaceDamageEnum
            E1 locale!(zh_cn: "基础伤害", en: "Basic DMG")
            E2 locale!(zh_cn: "蓄力完成伤害", en: "Charged Up DMG")
        ),
        skill3: skill_map!(
            CandaceDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "水波冲击伤害", en: "Wave Impact DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c2_rate",
            title: locale!(
                zh_cn: "命座2「贯月的耀锋」比例",
                en: "C2 Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "crown",
            title: locale!(
                zh_cn: "赤冕祝祷",
                en: "Prayer of the Crimson Crown"
            ),
            config: ItemConfigType::Bool { default: true },
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: CandaceDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let ratio = match s {
            CandaceDamageEnum::Normal1 => CANDACE_SKILL.normal_dmg1[s1],
            CandaceDamageEnum::Normal2 => CANDACE_SKILL.normal_dmg2[s1],
            CandaceDamageEnum::Normal31 => CANDACE_SKILL.normal_dmg31[s1],
            CandaceDamageEnum::Normal32 => CANDACE_SKILL.normal_dmg32[s1],
            CandaceDamageEnum::Normal4 => CANDACE_SKILL.normal_dmg4[s1],
            CandaceDamageEnum::Charged => CANDACE_SKILL.charged_dmg[s1],
            CandaceDamageEnum::Plunging1 => CANDACE_SKILL.plunging_dmg1[s1],
            CandaceDamageEnum::Plunging2 => CANDACE_SKILL.plunging_dmg2[s1],
            CandaceDamageEnum::Plunging3 => CANDACE_SKILL.plunging_dmg3[s1],
            CandaceDamageEnum::E1 => CANDACE_SKILL.e_dmg1[s2],
            CandaceDamageEnum::E2 => CANDACE_SKILL.e_dmg2[s2],
            CandaceDamageEnum::Q1 | CandaceDamageEnum::Q2 => CANDACE_SKILL.q_dmg1_2[s2],
        };

        let crown = match *config {
            CharacterSkillConfig::Candace { crown } => crown,
            _ => false
        };

        let skill_type = s.get_skill_type();
        let element = s.get_element(crown);

        let mut builder = D::new();
        if skill_type == SkillType::ElementalSkill || skill_type == SkillType::ElementalBurst {
            builder.add_hp_ratio("技能倍率", ratio);
        } else {
            builder.add_atk_ratio("技能倍率", ratio);
        }

        if crown {
            if skill_type == SkillType::NormalAttack {
                builder.add_extra_bonus("赤冕祝祷加成", 0.2);
                if context.character_common_data.has_talent2 {
                    let hp = context.attribute.get_value(AttributeName::HP);
                    let value = (hp / 1000.0).floor() * 0.005;
                    builder.add_extra_bonus("天赋2「漫沙陨穹」加成", value);
                }
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            element,
            skill_type,
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let c2_rate = match *config {
            CharacterConfig::Candace { c2_rate } => c2_rate,
            _ => 0.0
        };
        Some(Box::new(CandaceEffect {
            c2_rate,
            c2: common_data.constellation >= 2,
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}