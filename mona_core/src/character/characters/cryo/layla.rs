use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait, CharacterSkillMapItem};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::macros::{skill_type, damage_enum, skill_map, damage_ratio};
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};

pub struct LaylaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_shield1: [f64; 15],
    pub e_shield1_fixed: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const LAYLA_SKILL: LaylaSkillType = LaylaSkillType {
    normal_dmg1: [0.5122, 0.5539, 0.5956, 0.6551, 0.6968, 0.7444, 0.8099, 0.8755, 0.941, 1.0124, 1.0839, 1.1554, 1.2268, 1.2983, 1.3698],
    normal_dmg2: [0.4848, 0.5243, 0.5638, 0.6201, 0.6596, 0.7047, 0.7667, 0.8287, 0.8907, 0.9584, 1.026, 1.0937, 1.1613, 1.229, 1.2966],
    normal_dmg3: [0.7297, 0.7891, 0.8485, 0.9334, 0.9928, 1.0607, 1.154, 1.2473, 1.3407, 1.4425, 1.5443, 1.6461, 1.748, 1.8498, 1.9516],
    charged_dmg11: [0.4773, 0.5161, 0.555, 0.6105, 0.6493, 0.6938, 0.7548, 0.8159, 0.8769, 0.9435, 1.0101, 1.0767, 1.1433, 1.2099, 1.2765],
    charged_dmg12: [0.5255, 0.5682, 0.611, 0.6721, 0.7149, 0.7638, 0.831, 0.8982, 0.9654, 1.0387, 1.112, 1.1853, 1.2587, 1.332, 1.4053],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [0.128, 0.1376, 0.1472, 0.16, 0.1696, 0.1792, 0.192, 0.2048, 0.2176, 0.2304, 0.2432, 0.256, 0.272, 0.288, 0.304],
    e_dmg2: [0.1472, 0.1582, 0.1693, 0.184, 0.195, 0.2061, 0.2208, 0.2355, 0.2502, 0.265, 0.2797, 0.2944, 0.3128, 0.3312, 0.3496],
    e_shield1: [0.108, 0.1161, 0.1242, 0.135, 0.1431, 0.1512, 0.162, 0.1728, 0.1836, 0.1944, 0.2052, 0.216, 0.2295, 0.243, 0.2565],
    e_shield1_fixed: [1040.01, 1144.02, 1256.71, 1378.06, 1508.08, 1646.76, 1794.12, 1950.14, 2114.83, 2288.19, 2470.22, 2660.91, 2860.27, 3068.3, 3285.0],
    q_dmg1: [0.0465, 0.05, 0.0535, 0.0581, 0.0616, 0.0651, 0.0697, 0.0744, 0.079, 0.0837, 0.0883, 0.093, 0.0988, 0.1046, 0.1104],
};

damage_enum!(
    LaylaDamageEnum
    Normal1
    Normal2
    Normal3
    Charged11
    Charged12
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    Q1
);

impl LaylaDamageEnum {
    pub fn get_element(&self) -> Element {
        use LaylaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Charged11 | Charged12 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            E1 | E2 | Q1 => Element::Cryo
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use LaylaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

pub struct Layla;

impl CharacterTrait for Layla {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Layla,
        internal_name: "Layla",
        element: Element::Cryo,
        hp: [930, 2389, 3084, 4619, 5113, 5881, 6540, 7308, 7801, 8569, 9062, 9831, 10324, 11092],
        atk: [18, 47, 60, 90, 100, 115, 128, 143, 152, 167, 177, 192, 202, 217],
        def: [55, 141, 182, 273, 302, 347, 386, 432, 461, 506, 535, 581, 610, 655],
        sub_stat: CharacterSubStatFamily::HP240,
        weapon_type: WeaponType::Sword,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·熠辉轨度剑",
            en: "Normal Attack: Sword of the Radiant Path",
        ),
        skill_name2: locale!(
            zh_cn: "垂裳端凝之夜",
            en: "Nights of Formal Focus",
        ),
        skill_name3: locale!(
            zh_cn: "星流摇床之梦",
            en: "Dream of the Star-Stream Shaker",
        ),
        name_locale: locale!(
            zh_cn: "莱依拉",
            en: "Layla",
        )
    };
    type SkillType = LaylaSkillType;
    const SKILL: Self::SkillType = LAYLA_SKILL;
    type DamageEnumType = LaylaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LaylaDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged11 charged_dmg!(1)
            Charged12 charged_dmg!(2)
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            LaylaDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E2 locale!(zh_cn: "飞星伤害", en: "Shooting Star DMG")
        ),
        skill3: skill_map!(
            LaylaDamageEnum
            Q1 locale!(zh_cn: "星光弹伤害", en: "Starlight Slug DMG")
        )
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LaylaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use LaylaDamageEnum::*;
        let ratio = match s {
            Normal1 => LAYLA_SKILL.normal_dmg1[s1],
            Normal2 => LAYLA_SKILL.normal_dmg2[s1],
            Normal3 => LAYLA_SKILL.normal_dmg3[s1],
            Charged11 => LAYLA_SKILL.charged_dmg11[s1],
            Charged12 => LAYLA_SKILL.charged_dmg12[s1],
            Plunging1 => LAYLA_SKILL.plunging_dmg1[s1],
            Plunging2 => LAYLA_SKILL.plunging_dmg2[s1],
            Plunging3 => LAYLA_SKILL.plunging_dmg3[s1],
            E1 => LAYLA_SKILL.e_dmg1[s2],
            E2 => LAYLA_SKILL.e_dmg2[s2],
            Q1 => LAYLA_SKILL.q_dmg1[s3],
        };

        let mut builder = D::new();
        if s != Q1 {
            builder.add_atk_ratio("技能倍率", ratio);
        } else {
            builder.add_hp_ratio("技能倍率", ratio);
        }
        if context.character_common_data.has_talent2 && s == E2 {
            builder.add_hp_ratio("天赋「勿扰沉眠」", 0.015);
        }
        if context.character_common_data.constellation >= 6 {
            if s == E2 || s == Q1 {
                builder.add_extra_bonus("C6「曜光灵炬」", 0.4);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}