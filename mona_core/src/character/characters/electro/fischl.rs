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
use crate::target_functions::target_functions::FischlDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct FischlSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15]
}

pub const FISCHL_SKILL: FischlSkillType = FischlSkillType {
    normal_dmg1: [0.4412, 0.4771, 0.513, 0.5643, 0.6002, 0.6413, 0.6977, 0.7541, 0.8105, 0.8721, 0.9337, 0.9952, 1.0568, 1.1183, 1.1799],
    normal_dmg2: [0.4678, 0.5059, 0.544, 0.5984, 0.6365, 0.68, 0.7398, 0.7997, 0.8595, 0.9248, 0.9901, 1.0554, 1.1206, 1.1859, 1.2512],
    normal_dmg3: [0.5814, 0.6287, 0.676, 0.7436, 0.7909, 0.845, 0.9194, 0.9937, 1.0681, 1.1492, 1.2303, 1.3114, 1.3926, 1.4737, 1.5548],
    normal_dmg4: [0.5771, 0.624, 0.671, 0.7381, 0.7851, 0.8388, 0.9126, 0.9864, 1.0602, 1.1407, 1.2212, 1.3017, 1.3823, 1.4628, 1.5433],
    normal_dmg5: [0.7207, 0.7793, 0.838, 0.9218, 0.9805, 1.0475, 1.1397, 1.2319, 1.324, 1.4246, 1.5252, 1.6257, 1.7263, 1.8268, 1.9274],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.888, 0.9546, 1.0212, 1.11, 1.1766, 1.2432, 1.332, 1.4208, 1.5096, 1.5984, 1.6872, 1.776, 1.887, 1.998, 2.109],
    elemental_skill_dmg2: [1.1544, 1.241, 1.3276, 1.443, 1.5296, 1.6162, 1.7316, 1.847, 1.9625, 2.0779, 2.1934, 2.3088, 2.4531, 2.5974, 2.7417],
    elemental_burst_dmg1: [2.08, 2.236, 2.392, 2.6, 2.756, 2.912, 3.12, 3.328, 3.536, 3.744, 3.952, 4.16, 4.42, 4.68, 4.94],
};

pub const FISCHL_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Fischl,
    internal_name: "Fischl",
    element: Element::Electro,
    hp: [770, 1979, 2555, 3827, 4236, 4872, 5418, 6054, 6463, 7099, 7508, 8144, 8553, 9189],
    atk: [20, 53, 68, 102, 113, 130, 144, 161, 172, 189, 200, 216, 227, 244],
    def: [50, 128, 165, 247, 274, 315, 350, 391, 418, 459, 485, 526, 553, 594],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Bow,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·罪灭之矢",
        en: "Normal Attack: Bolts of Downfall",
    ),
    skill_name2: locale!(
        zh_cn: "夜巡影翼",
        en: "Nightrider",
    ),
    skill_name3: locale!(
        zh_cn: "至夜幻现",
        en: "Midnight Phantasmagoria",
    ),
    name_locale: locale!(
        zh_cn: "菲谢尔",
        en: "Fischl",
    )
};

pub struct Fischl;

#[derive(Copy, Clone, FromPrimitive)]
#[derive(EnumString, EnumCountMacro)]
pub enum FischlDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    Q1
}

impl FischlDamageEnum {
    pub fn get_element(&self) -> Element {
        use FischlDamageEnum::*;
        match *self {
            Charged2 | E1 | E2 | Q1 => Element::Electro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use FischlDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for FischlDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum FischlRoleEnum {
    Damage
}

impl CharacterTrait for Fischl {
    const STATIC_DATA: CharacterStaticData = FISCHL_STATIC_DATA;
    type SkillType = FischlSkillType;
    const SKILL: Self::SkillType = FISCHL_SKILL;
    type DamageEnumType = FischlDamageEnum;
    type RoleEnum = FischlRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: FischlDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: FischlDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: FischlDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: FischlDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: FischlDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: FischlDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: FischlDamageEnum::Charged2 as usize, text: charged_dmg!("shoot2") },
            CharacterSkillMapItem { index: FischlDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: FischlDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: FischlDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: FischlDamageEnum::E1 as usize, text: locale!(zh_cn: "奥兹攻击伤害", en: "Oz’s ATK DMG") },
            CharacterSkillMapItem { index: FischlDamageEnum::E2 as usize, text: locale!(zh_cn: "召唤伤害", en: "Summoning DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: FischlDamageEnum::Q1 as usize, text: locale!(zh_cn: "落雷伤害", en: "Falling Thunder DMG") }
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: FischlDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use FischlDamageEnum::*;
        let ratio = match s {
            Normal1 => FISCHL_SKILL.normal_dmg1[s1],
            Normal2 => FISCHL_SKILL.normal_dmg2[s1],
            Normal3 => FISCHL_SKILL.normal_dmg3[s1],
            Normal4 => FISCHL_SKILL.normal_dmg4[s1],
            Normal5 => FISCHL_SKILL.normal_dmg5[s1],
            Charged1 => FISCHL_SKILL.charged_dmg1[s1],
            Charged2 => FISCHL_SKILL.charged_dmg2[s1],
            Plunging1 => FISCHL_SKILL.plunging_dmg1[s1],
            Plunging2 => FISCHL_SKILL.plunging_dmg2[s1],
            Plunging3 => FISCHL_SKILL.plunging_dmg3[s1],
            E1 => FISCHL_SKILL.elemental_skill_dmg1[s2],
            E2 => FISCHL_SKILL.elemental_skill_dmg2[s2],
            Q1 => FISCHL_SKILL.elemental_burst_dmg1[s3]
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
        let role: FischlRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            FischlRoleEnum::Damage => Box::new(FischlDefaultTargetFunction)
        }
    }
}
