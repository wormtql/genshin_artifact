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
use crate::target_functions::target_functions::ChongyunDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct ChongyunSkillType {
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

    pub elemental_burst_dmg1: [f64; 15]
}

pub const CHONGYUN_SKILL: ChongyunSkillType = ChongyunSkillType {
    normal_dmg1: [0.7, 0.757, 0.814, 0.8954, 0.9524, 1.0175, 1.107, 1.1966, 1.2861, 1.3838, 1.4815, 1.5792, 1.6768, 1.7745, 1.8722],
    normal_dmg2: [0.6312, 0.6826, 0.734, 0.8074, 0.8588, 0.9175, 0.9982, 1.079, 1.1597, 1.2478, 1.3359, 1.424, 1.512, 1.6001, 1.6882],
    normal_dmg3: [0.8032, 0.8686, 0.934, 1.0274, 1.0928, 1.1675, 1.2702, 1.373, 1.4757, 1.5878, 1.6999, 1.812, 1.924, 2.0361, 2.1482],
    normal_dmg4: [1.0122, 1.0946, 1.177, 1.2947, 1.3771, 1.4713, 1.6007, 1.7302, 1.8597, 2.0009, 2.1421, 2.2834, 2.4246, 2.5659, 2.7071],
    charged_dmg1: [0.5629, 0.6087, 0.6545, 0.7199, 0.7657, 0.8181, 0.8901, 0.9621, 1.0341, 1.1126, 1.1912, 1.2697, 1.3482, 1.4268, 1.5053],
    charged_dmg2: [1.0178, 1.1007, 1.1835, 1.3019, 1.3847, 1.4794, 1.6096, 1.7397, 1.8699, 2.012, 2.154, 2.296, 2.438, 2.58, 2.7221],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    elemental_skill_dmg1: [1.7204, 1.8494, 1.9785, 2.1505, 2.2795, 2.4086, 2.5806, 2.7526, 2.9247, 3.0967, 3.2688, 3.4408, 3.6559, 3.8709, 4.086],
    elemental_burst_dmg1: [1.424, 1.5308, 1.6376, 1.78, 1.8868, 1.9936, 2.136, 2.2784, 2.4208, 2.5632, 2.7056, 2.848, 3.026, 3.204, 3.382],
};

pub const CHONGYUN_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Chongyun,
    internal_name: "Chongyun",
    element: Element::Cryo,
    hp: [921, 2366, 3054, 4574, 5063, 5824, 6475, 7236, 7725, 8485, 8974, 9734, 10223, 10984],
    atk: [19, 48, 62, 93, 103, 118, 131, 147, 157, 172, 182, 197, 208, 223],
    def: [54, 140, 180, 270, 299, 344, 382, 427, 456, 501, 530, 575, 603, 648],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Claymore,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·灭邪四式",
        en: "Normal Attack: Demonbane",
    ),
    skill_name2: locale!(
        zh_cn: "灵刃·重华叠霜",
        en: "Spirit Blade: Chonghua's Layered Frost",
    ),
    skill_name3: locale!(
        zh_cn: "灵刃·云开星落",
        en: "Spirit Blade: Cloud-Parting Star",
    ),
    name_locale: locale!(
        zh_cn: "重云",
        en: "Chongyun",
    )
};

pub struct Chongyun;

#[derive(Copy, Clone)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum ChongyunDamageEnum {
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
    Q1
}

impl ChongyunDamageEnum {
    pub fn get_element(&self) -> Element {
        use ChongyunDamageEnum::*;
        match *self {
            E1 | Q1 => Element::Cryo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ChongyunDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 | Plunging2 | Plunging3 => SkillType::ChargedAttack,
            E1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for ChongyunDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum ChongyunRoleEnum {
    Sub
}

impl CharacterTrait for Chongyun {
    const STATIC_DATA: CharacterStaticData = CHONGYUN_STATIC_DATA;
    type SkillType = ChongyunSkillType;
    const SKILL: Self::SkillType = CHONGYUN_SKILL;
    type DamageEnumType = ChongyunDamageEnum;
    type RoleEnum = ChongyunRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: ChongyunDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: ChongyunDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: ChongyunDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: ChongyunDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: ChongyunDamageEnum::Charged1 as usize, text: charged_dmg!("loop1") },
            CharacterSkillMapItem { index: ChongyunDamageEnum::Charged2 as usize, text: charged_dmg!("loop2") },
            CharacterSkillMapItem { index: ChongyunDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: ChongyunDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: ChongyunDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: ChongyunDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: ChongyunDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ChongyunDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use ChongyunDamageEnum::*;
        let ratio = match s {
            Normal1 => CHONGYUN_SKILL.normal_dmg1[s1],
            Normal2 => CHONGYUN_SKILL.normal_dmg2[s1],
            Normal3 => CHONGYUN_SKILL.normal_dmg3[s1],
            Normal4 => CHONGYUN_SKILL.normal_dmg4[s1],
            Charged1 => CHONGYUN_SKILL.charged_dmg1[s1],
            Charged2 => CHONGYUN_SKILL.charged_dmg2[s1],
            Plunging1 => CHONGYUN_SKILL.plunging_dmg1[s1],
            Plunging2 => CHONGYUN_SKILL.plunging_dmg2[s1],
            Plunging3 => CHONGYUN_SKILL.plunging_dmg3[s1],
            E1 => CHONGYUN_SKILL.elemental_skill_dmg1[s2],
            Q1 => CHONGYUN_SKILL.elemental_burst_dmg1[s3]
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
        let role: ChongyunRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            ChongyunRoleEnum::Sub => Box::new(ChongyunDefaultTargetFunction)
        }
    }
}
