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
use crate::target_functions::target_functions::DilucDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct DilucSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_dmg3: [f64; 15]
}

pub const DILUC_SKILL: DilucSkillType = DilucSkillType {
    normal_dmg1: [0.897, 0.97, 1.043, 1.1473, 1.2203, 1.3038, 1.4185, 1.5332, 1.6479, 1.7731, 1.9165, 2.0852, 2.2538, 2.4225, 2.6065],
    normal_dmg2: [0.8763, 0.9477, 1.019, 1.1209, 1.1922, 1.2738, 1.3858, 1.4979, 1.61, 1.7323, 1.8724, 2.0372, 2.202, 2.3667, 2.5465],
    normal_dmg3: [0.9881, 1.0686, 1.149, 1.2639, 1.3443, 1.4363, 1.5626, 1.689, 1.8154, 1.9533, 2.1113, 2.2971, 2.4829, 2.6687, 2.8714],
    normal_dmg4: [1.3399, 1.4489, 1.558, 1.7138, 1.8229, 1.9475, 2.1189, 2.2903, 2.4616, 2.6486, 2.8628, 3.1148, 3.3667, 3.6186, 3.8934],
    charged_dmg1: [0.688, 0.744, 0.8, 0.88, 0.936, 1., 1.088, 1.176, 1.264, 1.36, 1.47, 1.5994, 1.7287, 1.8581, 1.9992],
    charged_dmg2: [1.247, 1.3485, 1.45, 1.595, 1.6965, 1.8125, 1.972, 2.1315, 2.291, 2.465, 2.6644, 2.8988, 3.1333, 3.3678, 3.6236],
    plunging_dmg1: [0.8951, 0.9679, 1.0408, 1.1448, 1.2177, 1.301, 1.4154, 1.5299, 1.6444, 1.7693, 1.8942, 2.0191, 2.144, 2.2689, 2.3937],
    plunging_dmg2: [1.7897, 1.9354, 2.0811, 2.2892, 2.4349, 2.6013, 2.8303, 3.0592, 3.2881, 3.5378, 3.7876, 4.0373, 4.287, 4.5368, 4.7865],
    plunging_dmg3: [2.2355, 2.4174, 2.5994, 2.8593, 3.0413, 3.2492, 3.5352, 3.8211, 4.107, 4.4189, 4.7309, 5.0428, 5.3547, 5.6666, 5.9786],
    elemental_skill_dmg1: [0.944, 1.0148, 1.0856, 1.18, 1.2508, 1.3216, 1.416, 1.5104, 1.6048, 1.6992, 1.7936, 1.888, 2.006, 2.124, 2.242],
    elemental_skill_dmg2: [0.976, 1.0492, 1.1224, 1.22, 1.2932, 1.3664, 1.464, 1.5616, 1.6592, 1.7568, 1.8544, 1.952, 2.074, 2.196, 2.318],
    elemental_skill_dmg3: [1.288, 1.3846, 1.4812, 1.61, 1.7066, 1.8032, 1.932, 2.0608, 2.1896, 2.3184, 2.4472, 2.576, 2.737, 2.898, 3.059],
    elemental_burst_dmg1: [2.04, 2.193, 2.346, 2.55, 2.703, 2.856, 3.06, 3.264, 3.468, 3.672, 3.876, 4.08, 4.335, 4.59, 4.845],
    elemental_burst_dmg2: [0.6, 0.645, 0.69, 0.75, 0.795, 0.84, 0.9, 0.96, 1.02, 1.08, 1.14, 1.2, 1.275, 1.35, 1.425],
    elemental_burst_dmg3: [2.04, 2.193, 2.346, 2.55, 2.703, 2.856, 3.06, 3.264, 3.468, 3.672, 3.876, 4.08, 4.335, 4.59, 4.845]
};

const DILUC_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Diluc,
    internal_name: "Diluc",
    element: Element::Pyro,
    hp: [1011, 2621, 3488, 5219, 5834, 6712, 7533, 8421, 9036, 9932, 10547, 11453, 12068, 12981],
    atk: [26, 68, 90, 135, 151, 173, 194, 217, 233, 256, 272, 295, 311, 335],
    def: [61, 158, 211, 315, 352, 405, 455, 509, 546, 600, 637, 692, 729, 784],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Claymore,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·淬炼之剑",
        en: "Normal Attack: Tempered Sword",
    ),
    skill_name2: locale!(
        zh_cn: "逆焰之刃",
        en: "Searing Onslaught",
    ),
    skill_name3: locale!(
        zh_cn: "黎明",
        en: "Dawn",
    ),
    name_locale: locale!(
        zh_cn: "迪卢克",
        en: "Diluc",
    )
};

pub struct Diluc;

#[derive(Copy, Clone)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum DilucDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    Q1,
    Q2,
    Q3
}

impl DilucDamageEnum {
    pub fn get_element(&self, pyro: bool) -> Element {
        use DilucDamageEnum::*;

        if pyro {
            Element::Pyro
        } else {
            match *self {
                E1 | E2 | E3 | Q1 | Q2 | Q3 => Element::Pyro,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use DilucDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 | Q2 | Q3 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for DilucDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum DilucRoleEnum {
    Main
}

impl CharacterTrait for Diluc {
    const STATIC_DATA: CharacterStaticData = DILUC_STATIC_DATA;
    type SkillType = DilucSkillType;
    const SKILL: Self::SkillType = DILUC_SKILL;
    type DamageEnumType = DilucDamageEnum;
    type RoleEnum = DilucRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: DilucDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: DilucDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: DilucDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: DilucDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: DilucDamageEnum::Charged1 as usize, text: charged_dmg!("loop1") },
            CharacterSkillMapItem { index: DilucDamageEnum::Charged2 as usize, text: charged_dmg!("loop2") },
            CharacterSkillMapItem { index: DilucDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: DilucDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: DilucDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: DilucDamageEnum::E1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: DilucDamageEnum::E2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: DilucDamageEnum::E3 as usize, text: hit_n_dmg!(3) },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: DilucDamageEnum::Q1 as usize, text: locale!(zh_cn: "斩击伤害", en: "Slashing DMG") },
            CharacterSkillMapItem { index: DilucDamageEnum::Q2 as usize, text: locale!(zh_cn: "持续伤害", en: "DoT") },
            CharacterSkillMapItem { index: DilucDamageEnum::Q3 as usize, text: locale!(zh_cn: "爆裂伤害", en: "Explosion DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "pyro",
            title: locale!(
                zh_cn: "是否被大招附魔",
                en: "是否被大招附魔",
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: DilucDamageEnum = num::FromPrimitive::from_usize(s).expect("wrong skill index");
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use DilucDamageEnum::*;
        let ratio = match s {
            Normal1 => DILUC_SKILL.normal_dmg1[s1],
            Normal2 => DILUC_SKILL.normal_dmg2[s1],
            Normal3 => DILUC_SKILL.normal_dmg3[s1],
            Normal4 => DILUC_SKILL.normal_dmg4[s1],
            Charged1 => DILUC_SKILL.charged_dmg1[s1],
            Charged2 => DILUC_SKILL.charged_dmg2[s1],
            Plunging1 => DILUC_SKILL.plunging_dmg1[s1],
            Plunging2 => DILUC_SKILL.plunging_dmg2[s1],
            Plunging3 => DILUC_SKILL.plunging_dmg3[s1],
            E1 => DILUC_SKILL.elemental_skill_dmg1[s2],
            E2 => DILUC_SKILL.elemental_skill_dmg2[s2],
            E3 => DILUC_SKILL.elemental_skill_dmg3[s2],
            Q1 => DILUC_SKILL.elemental_burst_dmg1[s3],
            Q2 => DILUC_SKILL.elemental_burst_dmg2[s3],
            Q3 => DILUC_SKILL.elemental_burst_dmg3[s3]
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let pyro = match *config {
            CharacterSkillConfig::Diluc { pyro } => pyro,
            _ => false
        };
        let skill_type = s.get_skill_type();
        if pyro && context.character_common_data.has_talent2 {
            builder.add_extra_bonus("天赋：「熔毁之翼」", 0.2);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(pyro),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: DilucRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            DilucRoleEnum::Main => Box::new(DilucDefaultTargetFunction {
                melt_rate: 0.0,
                vaporize_rate: 0.0
            }),
        }
    }
}
