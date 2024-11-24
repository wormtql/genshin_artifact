use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::electro::sethos::SethosDamageEnum;
use crate::character::characters::electro::sethos::SethosDamageEnum::{Plunging1, Plunging2, Plunging3};
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct OroronSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const ORORON_SKILL: OroronSkillType = OroronSkillType {
    normal_dmg1: [0.5064, 0.5476, 0.5889, 0.6477, 0.689, 0.7361, 0.8009, 0.8656, 0.9304, 1.0011, 1.0717, 1.1424, 1.2131, 1.2837, 1.3544],
    normal_dmg2: [0.4437, 0.4799, 0.516, 0.5676, 0.6037, 0.645, 0.7017, 0.7585, 0.8152, 0.8771, 0.9391, 1.001, 1.0629, 1.1248, 1.1867],
    normal_dmg3: [0.6982, 0.755, 0.8119, 0.8931, 0.9499, 1.0148, 1.1041, 1.1934, 1.2828, 1.3802, 1.4776, 1.575, 1.6725, 1.7699, 1.8673],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [1.976, 2.1242, 2.2724, 2.47, 2.6182, 2.7664, 2.964, 3.1616, 3.3592, 3.5568, 3.7544, 3.952, 4.199, 4.446, 4.693],
    q_dmg1: [1.7438, 1.8746, 2.0054, 2.1798, 2.3106, 2.4414, 2.6158, 2.7901, 2.9645, 3.1389, 3.3133, 3.4877, 3.7057, 3.9236, 4.1416],
    q_dmg2: [0.332, 0.3569, 0.3818, 0.415, 0.4399, 0.4648, 0.498, 0.5312, 0.5644, 0.5976, 0.6308, 0.664, 0.7055, 0.747, 0.7885],
};

damage_enum!(
    OroronDamageEnum
    Normal1
    Normal2
    Normal3
    Charged1
    Charged2
    Plunging1
    Plunging2
    Plunging3
    E1
    Q1
    Q2
);

impl OroronDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use OroronDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst,
        }
    }

    pub fn get_element(&self) -> Element {
        use OroronDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            Charged2 | E1 | Q1 | Q2 => Element::Electro
        }
    }
}

pub struct Ororon;

impl CharacterTrait for Ororon {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Ororon,
        internal_name: "Ororon",
        name_locale: locale!(
            zh_cn: "欧洛伦",
            en: "Ororon"
        ),
        element: Element::Electro,
        hp: [775, 1991, 2570, 3850, 4261, 4901, 5450, 6090, 6501, 7141, 7552, 8192, 8604, 9244],
        atk: [20, 53, 68, 102, 113, 130, 144, 161, 172, 189, 200, 216, 227, 244],
        def: [49, 126, 163, 244, 271, 311, 346, 387, 413, 453, 480, 520, 546, 587],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Bow,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·宿灵闪箭",
            en: "Normal Attack: Spiritvessel Snapshot"
        ),
        skill_name2: locale!(
            zh_cn: "暝色缒索",
            en: "Night's Sling"
        ),
        skill_name3: locale!(
            zh_cn: "黯声回响",
            en: "Dark Voices Echo"
        ),
    };
    type SkillType = OroronSkillType;
    const SKILL: Self::SkillType = ORORON_SKILL;
    type DamageEnumType = OroronDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            OroronDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged1 locale!(zh_cn: "瞄准射击", en: "Aimed Shot")
            Charged2 locale!(zh_cn: "满蓄力瞄准射击", en: "Fully-Charged Aimed Shot")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            OroronDamageEnum
            E1 locale!(zh_cn: "宿灵球伤害", en: "Spirit Orb DMG")
        ),
        skill3: skill_map!(
            OroronDamageEnum
            Q1 locale!(zh_cn: "秘仪伤害", en: "Ritual DMG")
            Q1 locale!(zh_cn: "音波碰撞伤害", en: "Soundwave Collision DMG")
        ),
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: OroronDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use OroronDamageEnum::*;
        let mut builder = D::new();

        let ratio = match s {
            Normal1 => ORORON_SKILL.normal_dmg1[s1],
            Normal2 => ORORON_SKILL.normal_dmg2[s1],
            Normal3 => ORORON_SKILL.normal_dmg3[s1],
            Charged1 => ORORON_SKILL.charged_dmg1[s1],
            Charged2 => ORORON_SKILL.charged_dmg2[s1],
            Plunging1 => ORORON_SKILL.plunging_dmg1[s1],
            Plunging2 => ORORON_SKILL.plunging_dmg2[s1],
            Plunging3 => ORORON_SKILL.plunging_dmg3[s1],
            E1 => ORORON_SKILL.e_dmg1[s2],
            Q1 => ORORON_SKILL.q_dmg1[s3],
            Q2 => ORORON_SKILL.q_dmg2[s3],
        };

        builder.add_atk_ratio("技能倍率", ratio);

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
