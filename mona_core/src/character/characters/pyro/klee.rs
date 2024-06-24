use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::KleeDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct KleeSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
}

pub const KLEE_SKILL: KleeSkillType = KleeSkillType {
    normal_dmg1: [0.7216, 0.7757, 0.8298, 0.902, 0.9561, 1.0102, 1.0824, 1.1546, 1.2267, 1.2989, 1.3739, 1.4721, 1.5702, 1.6683, 1.7665],
    normal_dmg2: [0.624, 0.6708, 0.7176, 0.78, 0.8268, 0.8736, 0.936, 0.9984, 1.0608, 1.1232, 1.1881, 1.273, 1.3578, 1.4427, 1.5276],
    normal_dmg3: [0.8992, 0.9666, 1.0341, 1.124, 1.1914, 1.2589, 1.3488, 1.4387, 1.5286, 1.6186, 1.7121, 1.8344, 1.9567, 2.079, 2.2012],
    charged_dmg1: [1.5736, 1.6916, 1.8096, 1.967, 2.085, 2.203, 2.3604, 2.5178, 2.6751, 2.8325, 2.9961, 3.2101, 3.4242, 3.6382, 3.8522],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.952, 1.0234, 1.0948, 1.19, 1.2614, 1.3328, 1.428, 1.5232, 1.6184, 1.7136, 1.8088, 1.904, 2.023, 2.142, 2.261],
    elemental_skill_dmg2: [0.328, 0.3526, 0.3772, 0.41, 0.4346, 0.4592, 0.492, 0.5248, 0.5576, 0.5904, 0.6232, 0.656, 0.697, 0.738, 0.779],
    elemental_burst_dmg1: [0.4264, 0.4584, 0.4904, 0.533, 0.565, 0.597, 0.6396, 0.6822, 0.7249, 0.7675, 0.8102, 0.8528, 0.9061, 0.9594, 1.0127],
};

pub const KLEE_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Klee,
    internal_name: "Klee",
    element: Element::Pyro,
    hp: [801, 2077, 2764, 4136, 4623, 5319, 5970, 6673, 7161, 7870, 8358, 9076, 9563, 10287],
    atk: [24, 63, 84, 125, 140, 161, 180, 202, 216, 238, 253, 274, 289, 311],
    def: [48, 124, 165, 247, 276, 318, 357, 399, 428, 470, 500, 542, 572, 615],
    sub_stat: CharacterSubStatFamily::Bonus288(StatName::PyroBonus),
    weapon_type: WeaponType::Catalyst,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·砰砰",
        en: "Normal Attack: Kaboom!",
    ),
    skill_name2: locale!(
        zh_cn: "蹦蹦炸弹",
        en: "Jumpy Dumpty",
    ),
    skill_name3: locale!(
        zh_cn: "轰轰火花",
        en: "Sparks 'n' Splash",
    ),
    name_locale: locale!(
        zh_cn: "可莉",
        en: "Klee",
    )
};

pub struct Klee;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq)]
#[derive(EnumString, EnumCountMacro)]
pub enum KleeDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Charged,
    ChargedWithTalent,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    Q1
}

impl KleeDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use KleeDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged | ChargedWithTalent => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for KleeDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum KleeRoleEnum {
    MainPyro
}

impl CharacterTrait for Klee {
    const STATIC_DATA: CharacterStaticData = KLEE_STATIC_DATA;
    type SkillType = KleeSkillType;
    const SKILL: Self::SkillType = KLEE_SKILL;
    type DamageEnumType = KleeDamageEnum;
    type RoleEnum = KleeRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: KleeDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: KleeDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: KleeDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: KleeDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: KleeDamageEnum::ChargedWithTalent as usize, text: locale!(zh_cn: "重击伤害（带天赋）", en: "Charged Attack DMG(With talent)") },
            CharacterSkillMapItem { index: KleeDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: KleeDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: KleeDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: KleeDamageEnum::E1 as usize, text: locale!(zh_cn: "蹦蹦炸弹伤害", en: "Jumpy Dumpty DMG") },
            CharacterSkillMapItem { index: KleeDamageEnum::E2 as usize, text: locale!(zh_cn: "诡雷伤害", en: "Mine DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: KleeDamageEnum::Q1 as usize, text: locale!(zh_cn: "轰轰火花伤害", en: "Sparks 'n' Splash DMG") }
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KleeDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KleeDamageEnum::*;
        let ratio = match s {
            Normal1 => KLEE_SKILL.normal_dmg1[s1],
            Normal2 => KLEE_SKILL.normal_dmg2[s1],
            Normal3 => KLEE_SKILL.normal_dmg3[s1],
            Charged => KLEE_SKILL.charged_dmg1[s1],
            ChargedWithTalent => KLEE_SKILL.charged_dmg1[s1],
            Plunging1 => KLEE_SKILL.plunging_dmg1[s1],
            Plunging2 => KLEE_SKILL.plunging_dmg2[s1],
            Plunging3 => KLEE_SKILL.plunging_dmg3[s1],
            E1 => KLEE_SKILL.elemental_skill_dmg1[s2],
            E2 => KLEE_SKILL.elemental_skill_dmg2[s2],
            Q1 => KLEE_SKILL.elemental_burst_dmg1[s3]
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        if s == ChargedWithTalent {
            builder.add_extra_bonus("可莉天赋：砰砰礼物", 0.5);
        }
        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Pyro,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: KleeRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            KleeRoleEnum::MainPyro => Box::new(KleeDefaultTargetFunction {
                recharge_demand: 1.0
            })
        }
    }
}
