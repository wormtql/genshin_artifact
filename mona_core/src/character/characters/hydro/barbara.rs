use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::BarbaraDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct BarbaraSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_heal1: [f64; 15],
    pub elemental_skill_heal1_fixed: [f64; 15],
    pub elemental_skill_heal2: [f64; 15],
    pub elemental_skill_heal2_fixed: [f64; 15],
    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_heal1: [f64; 15],
    pub elemental_burst_heal1_fixed: [f64; 15],
}

pub const BARBARA_SKILL: BarbaraSkillType = BarbaraSkillType {
    normal_dmg1: [0.3784, 0.4068, 0.4352, 0.473, 0.5014, 0.5298, 0.5676, 0.6054, 0.6433, 0.6811, 0.7205, 0.7719, 0.8234, 0.8749, 0.9263],
    normal_dmg2: [0.3552, 0.3818, 0.4085, 0.444, 0.4706, 0.4973, 0.5328, 0.5683, 0.6038, 0.6394, 0.6763, 0.7246, 0.7729, 0.8212, 0.8695],
    normal_dmg3: [0.4104, 0.4412, 0.472, 0.513, 0.5438, 0.5746, 0.6156, 0.6566, 0.6977, 0.7387, 0.7814, 0.8372, 0.893, 0.9488, 1.0047],
    normal_dmg4: [0.552, 0.5934, 0.6348, 0.69, 0.7314, 0.7728, 0.828, 0.8832, 0.9384, 0.9936, 1.051, 1.1261, 1.2012, 1.2762, 1.3513],
    charged_dmg1: [1.6624, 1.7871, 1.9118, 2.078, 2.2027, 2.3274, 2.4936, 2.6598, 2.8261, 2.9923, 3.1652, 3.3913, 3.6174, 3.8435, 4.0696],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_heal1: [0.0075, 0.0081, 0.0086, 0.0094, 0.0099, 0.0105, 0.0113, 0.012, 0.0127, 0.0135, 0.0143, 0.015, 0.0159, 0.0169, 0.0178],
    elemental_skill_heal1_fixed: [72.0, 79.0, 87.0, 96.0, 105.0, 114.0, 125.0, 135.0, 147.0, 159.0, 172.0, 185.0, 199.0, 213.0, 228.0],
    elemental_skill_heal2: [0.04, 0.043, 0.046, 0.05, 0.053, 0.056, 0.06, 0.064, 0.068, 0.072, 0.076, 0.08, 0.085, 0.09, 0.095],
    elemental_skill_heal2_fixed: [385.0, 424.0, 465.0, 510.0, 559.0, 610.0, 664.0, 722.0, 783.0, 847.0, 915.0, 986.0, 1059.0, 1136.0, 1217.0],
    elemental_skill_dmg1: [0.584, 0.6278, 0.6716, 0.73, 0.7738, 0.8176, 0.876, 0.9344, 0.9928, 1.0512, 1.1096, 1.168, 1.241, 1.314, 1.387],
    elemental_burst_heal1: [0.176, 0.1892, 0.2024, 0.22, 0.2332, 0.2464, 0.264, 0.2816, 0.2992, 0.3168, 0.3344, 0.352, 0.374, 0.396, 0.418],
    elemental_burst_heal1_fixed: [1694.0, 1864.0, 2047.0, 2245.0, 2457.0, 2683.0, 2923.0, 3177.0, 3445.0, 3728.0, 4024.0, 4335.0, 4660.0, 4999.0, 5352.0]
};

pub const BARBARA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Barbara,
    internal_name: "Barbara",
    element: Element::Hydro,
    hp: [821, 2108, 2721, 4076, 4512, 5189, 5770, 6448, 6884, 7561, 7996, 8674, 9110, 9787],
    atk: [13, 34, 44, 66, 73, 84, 94, 105, 112, 123, 130, 141, 148, 159],
    def: [56, 144, 186, 279, 308, 355, 394, 441, 470, 517, 546, 593, 623, 669],
    sub_stat: CharacterSubStatFamily::HP240,
    weapon_type: WeaponType::Catalyst,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·水之浅唱",
        en: "Normal Attack: Whisper of Water",
    ),
    skill_name2: locale!(
        zh_cn: "演唱，开始♪",
        en: "Let the Show Begin♪",
    ),
    skill_name3: locale!(
        zh_cn: "闪耀奇迹♪",
        en: "Shining Miracle♪",
    ),
    name_locale: locale!(
        zh_cn: "芭芭拉",
        en: "Barbara",
    )
};

pub struct Barbara;

#[derive(Copy, Clone)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum BarbaraDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    EHeal1,
    EDmg,
    EHeal2,
    QHeal
}

impl BarbaraDamageEnum {
    pub fn is_heal(&self) -> bool {
        use BarbaraDamageEnum::*;
        match *self {
            EHeal1 | EHeal2 | QHeal => true,
            _ => false
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use BarbaraDamageEnum::*;
        match *self {
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            EDmg | EHeal1 | EHeal2 => SkillType::ElementalSkill,
            QHeal => SkillType::ElementalBurst,
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack
        }
    }
}

impl Into<usize> for BarbaraDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum BarbaraRoleEnum {
    Heal
}

impl CharacterTrait for Barbara {
    const STATIC_DATA: CharacterStaticData = BARBARA_STATIC_DATA;
    type SkillType = BarbaraSkillType;
    const SKILL: Self::SkillType = BARBARA_SKILL;
    type DamageEnumType = BarbaraDamageEnum;
    type RoleEnum = BarbaraRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: BarbaraDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: BarbaraDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: BarbaraDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: BarbaraDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: BarbaraDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: BarbaraDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: BarbaraDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: BarbaraDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: BarbaraDamageEnum::EHeal1 as usize, text: locale!(zh_cn: "命中治疗量", en: "HP Regeneration Per Hit") },
            CharacterSkillMapItem { index: BarbaraDamageEnum::EHeal2 as usize, text: locale!(zh_cn: "持续治疗量", en: "Continuous Regeneration") },
            CharacterSkillMapItem { index: BarbaraDamageEnum::EDmg as usize, text: locale!(zh_cn: "水珠伤害", en: "Droplet DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: BarbaraDamageEnum::QHeal as usize, text: locale!(zh_cn: "治疗量", en: "Regeneration") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: BarbaraDamageEnum = num::FromPrimitive::from_usize(s).unwrap();

        let s1 = context.character_common_data.skill1;
        let s2 = context.character_common_data.skill2;
        let s3 = context.character_common_data.skill3;

        use BarbaraDamageEnum::*;
        if s.is_heal() {
            let ratio = match s {
                EHeal1 => BARBARA_SKILL.elemental_skill_heal1[s2],
                EHeal2 => BARBARA_SKILL.elemental_skill_heal2[s2],
                QHeal => BARBARA_SKILL.elemental_burst_heal1[s3],
                _ => 0.0
            };
            let fixed = match s {
                EHeal1 => BARBARA_SKILL.elemental_skill_heal1_fixed[s2],
                EHeal2 => BARBARA_SKILL.elemental_skill_heal2_fixed[s2],
                QHeal => BARBARA_SKILL.elemental_burst_heal1_fixed[s3],
                _ => 0.0
            };

            let mut builder = D::new();
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);

            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => BARBARA_SKILL.normal_dmg1[s1],
                Normal2 => BARBARA_SKILL.normal_dmg2[s1],
                Normal3 => BARBARA_SKILL.normal_dmg3[s1],
                Normal4 => BARBARA_SKILL.normal_dmg4[s1],
                Charged => BARBARA_SKILL.charged_dmg1[s1],
                Plunging1 => BARBARA_SKILL.plunging_dmg1[s1],
                Plunging2 => BARBARA_SKILL.plunging_dmg2[s1],
                Plunging3 => BARBARA_SKILL.plunging_dmg3[s1],
                EDmg => BARBARA_SKILL.elemental_skill_dmg1[s2],
                _ => 0.0
            };
            let mut builder = D::new();
            builder.add_atk_ratio("技能倍率", ratio);

            builder.damage(
                &context.attribute,
                &context.enemy,
                Element::Hydro,
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        }
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: BarbaraRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            BarbaraRoleEnum::Heal => Box::new(BarbaraDefaultTargetFunction)
        }
    }
}