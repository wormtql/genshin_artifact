use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct YelanSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4div2: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub charged_dmg3: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const YELAN_SKILL: YelanSkillType = YelanSkillType {
    normal_dmg1: [0.4068, 0.4399, 0.473, 0.5203, 0.5534, 0.5913, 0.6433, 0.6953, 0.7473, 0.8041, 0.8609, 0.9176, 0.9744, 1.0311, 1.0879],
    normal_dmg2: [0.3904, 0.4222, 0.454, 0.4994, 0.5312, 0.5675, 0.6174, 0.6674, 0.7173, 0.7718, 0.8263, 0.8808, 0.9352, 0.9897, 1.0442],
    normal_dmg3: [0.516, 0.558, 0.6, 0.66, 0.702, 0.75, 0.816, 0.882, 0.948, 1.02, 1.092, 1.164, 1.236, 1.308, 1.38],
    normal_dmg4div2: [0.3251, 0.3515, 0.378, 0.4158, 0.4423, 0.4725, 0.5141, 0.5557, 0.5972, 0.6426, 0.688, 0.7333, 0.7787, 0.824, 0.8694],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    charged_dmg3: [0.1158, 0.1244, 0.1331, 0.1447, 0.1534, 0.1621, 0.1736, 0.1852, 0.1968, 0.2084, 0.2199, 0.2315, 0.246, 0.2605, 0.2749],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.2261, 0.2431, 0.2601, 0.2827, 0.2996, 0.3166, 0.3392, 0.3618, 0.3844, 0.407, 0.4297, 0.4523, 0.4805, 0.5088, 0.5371],
    elemental_burst_dmg1: [0.0731, 0.0786, 0.084, 0.0914, 0.0968, 0.1023, 0.1096, 0.1169, 0.1242, 0.1315, 0.1389, 0.1462, 0.1553, 0.1644, 0.1736],
    elemental_burst_dmg2: [0.0487, 0.0524, 0.056, 0.0609, 0.0646, 0.0682, 0.0731, 0.078, 0.0828, 0.0877, 0.0926, 0.0974, 0.1035, 0.1096, 0.1157],
};

pub struct YelanEffect {
    pub team_element_count: usize,
}

impl<A: Attribute> ChangeAttribute<A> for YelanEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let value = match self.team_element_count {
            0 => 0.0,
            1 => 0.06,
            2 => 0.12,
            3 => 0.18,
            _ => 0.3
        };

        attribute.add_hp_percentage("天赋：猜先有方", value);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum YelanDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4Div2,
    Charged1,
    Charged2,
    Charged3,
    Charged3C6,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
    Q2,
}

impl Into<usize> for YelanDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl YelanDamageEnum {
    pub fn is_hp_ratio(&self) -> bool {
        use YelanDamageEnum::*;
        match *self {
            Charged3 | Charged3C6 | E1 | Q1 | Q2 => true,
            _ => false
        }
    }

    pub fn get_element(&self) -> Element {
        use YelanDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4Div2 | Charged1 => Element::Physical,
            _ => Element::Hydro
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use YelanDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4Div2 => SkillType::NormalAttack,
            Charged1 | Charged2 | Charged3 | Charged3C6 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum YelanRoleEnum {
    Sub
}

pub struct Yelan;

impl CharacterTrait for Yelan {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Yelan,
        internal_name: "Yelan",
        element: Element::Hydro,
        hp: [1125, 2918, 3883, 5810, 6495, 7472, 8386, 9374, 10059, 11056, 11741, 12749, 13434, 14450],
        atk: [19, 49, 66, 98, 110, 126, 142, 158, 170, 187, 198, 215, 227, 244],
        def: [43, 111, 147, 220, 246, 283, 318, 355, 381, 419, 445, 483, 509, 548],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Bow,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·潜形隐曜弓",
            en: "Normal Attack: Stealthy Bowshot",
        ),
        skill_name2: locale!(
            zh_cn: "萦络纵命索",
            en: "Lingering Lifeline",
        ),
        skill_name3: locale!(
            zh_cn: "渊图玲珑骰",
            en: "Depth-Clarion Dice",
        ),
        name_locale: locale!(
            zh_cn: "夜兰",
            en: "Yelan",
        )
    };
    type SkillType = YelanSkillType;
    const SKILL: Self::SkillType = YELAN_SKILL;
    type DamageEnumType = YelanDamageEnum;
    type RoleEnum = YelanRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: YelanDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: YelanDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: YelanDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: YelanDamageEnum::Normal4Div2 as usize, text: locale!(zh_cn: "四段伤害/2", en: "4-Hit DMG/2") },
            CharacterSkillMapItem { index: YelanDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: YelanDamageEnum::Charged2 as usize, text: charged_dmg!("shoot2") },
            CharacterSkillMapItem { index: YelanDamageEnum::Charged3 as usize, text: locale!(zh_cn: "破局矢伤害", en: "Breakthrough Barb DMG") },
            CharacterSkillMapItem { index: YelanDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: YelanDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: YelanDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: YelanDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: YelanDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: YelanDamageEnum::Q2 as usize, text: locale!(zh_cn: "玄掷玲珑伤害", en: "Exquisite Throw DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "team_element_count",
            title: locale!(
                zh_cn: "队伍不同元素数量",
                en: "Different Element Count",
            ),
            config: ItemConfigType::Int { min: 1, max: 4, default: 4 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: YelanDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use YelanDamageEnum::*;
        let ratio = match s {
            Normal1 => YELAN_SKILL.normal_dmg1[s1],
            Normal2 => YELAN_SKILL.normal_dmg2[s1],
            Normal3 => YELAN_SKILL.normal_dmg3[s1],
            Normal4Div2 => YELAN_SKILL.normal_dmg4div2[s1],
            Charged1 => YELAN_SKILL.charged_dmg1[s1],
            Charged2 => YELAN_SKILL.charged_dmg2[s1],
            Charged3 => YELAN_SKILL.charged_dmg3[s1],
            Charged3C6 => YELAN_SKILL.charged_dmg3[s1] * 1.56,
            Plunging1 => YELAN_SKILL.plunging_dmg1[s1],
            Plunging2 => YELAN_SKILL.plunging_dmg2[s1],
            Plunging3 => YELAN_SKILL.plunging_dmg3[s1],
            E1 => YELAN_SKILL.elemental_skill_dmg1[s2],
            Q1 => YELAN_SKILL.elemental_burst_dmg1[s3],
            Q2 => YELAN_SKILL.elemental_burst_dmg2[s3],
        };

        let mut builder = D::new();
        if s.is_hp_ratio() {
            builder.add_hp_ratio("技能倍率", ratio);
        } else {
            builder.add_atk_ratio("技能倍率", ratio);
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
        let team_element_count = match *config {
            CharacterConfig::Yelan { team_element_count } => team_element_count,
            _ => 1
        };

        Some(Box::new(YelanEffect {
            team_element_count
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}