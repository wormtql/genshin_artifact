use num_derive::FromPrimitive;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::KujouSaraDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct KujouSaraSkillType {
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
    pub elemental_skill_atk_bonus: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const KUJOU_SARA_SKILL: KujouSaraSkillType = KujouSaraSkillType {
    normal_dmg1: [0.3689, 0.399, 0.429, 0.4719, 0.5019, 0.5363, 0.5834, 0.6306, 0.6778, 0.7293, 0.7808, 0.8323, 0.8837, 0.9352, 0.9867],
    normal_dmg2: [0.387, 0.4185, 0.45, 0.495, 0.5265, 0.5625, 0.612, 0.6615, 0.711, 0.765, 0.819, 0.873, 0.927, 0.981, 1.035],
    normal_dmg3: [0.485, 0.5245, 0.564, 0.6204, 0.6599, 0.705, 0.767, 0.8291, 0.8911, 0.9588, 1.0265, 1.0942, 1.1618, 1.2295, 1.2972],
    normal_dmg4: [0.504, 0.545, 0.586, 0.6446, 0.6856, 0.7325, 0.797, 0.8614, 0.9259, 0.9962, 1.0665, 1.1368, 1.2072, 1.2775, 1.3478],
    normal_dmg5: [0.5805, 0.6278, 0.675, 0.7425, 0.7898, 0.8438, 0.918, 0.9923, 1.0665, 1.1475, 1.2285, 1.3095, 1.3905, 1.4715, 1.5525],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [1.2576, 1.3519, 1.4462, 1.572, 1.6663, 1.7606, 1.8864, 2.0122, 2.1379, 2.2637, 2.3894, 2.5152, 2.6724, 2.8296, 2.9868],
    elemental_skill_atk_bonus: [0.4296, 0.4618, 0.494, 0.537, 0.5692, 0.6014, 0.6444, 0.6874, 0.7303, 0.7733, 0.8162, 0.8592, 0.9129, 0.9666, 1.0203],
    elemental_burst_dmg1: [4.096, 4.4032, 4.7104, 5.12, 5.4272, 5.7344, 6.144, 6.5536, 6.9632, 7.3728, 7.7824, 8.192, 8.704, 9.216, 9.728],
    elemental_burst_dmg2: [0.3412, 0.3668, 0.3924, 0.4265, 0.4521, 0.4777, 0.5118, 0.5459, 0.58, 0.6142, 0.6483, 0.6824, 0.7251, 0.7677, 0.8104],
};

pub const KUJOU_SARA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::KujouSara,
    internal_name: "Sara",
    element: Element::Electro,
    hp: [802, 2061, 2661, 3985, 4411, 5074, 5642, 6305, 6731, 7393, 7818, 8481, 8907, 9570, 10232],
    atk: [16, 42, 54, 81, 90, 104, 115, 129, 137, 151, 160, 173, 182, 195, 245],
    def: [53, 135, 175, 262, 289, 333, 370, 414, 442, 485, 513, 556, 584, 628, 671],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Bow,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击•天狗传弓术",
        en: "Normal Attack: Tengu Bowmanship",
    ),
    skill_name2: locale!(
        zh_cn: "鸦羽天狗霆雷召咒",
        en: "Tengu Stormcall",
    ),
    skill_name3: locale!(
        zh_cn: "煌煌千道镇式",
        en: "Subjugation: Koukou Sendou",
    ),
    name_locale: locale!(
        zh_cn: "九条裟罗",
        en: "Kujou Sara",
    )
};

pub struct KujouSara;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum KujouSaraDamageEnum {
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
    Q1,
    Q2
}

impl KujouSaraDamageEnum {
    pub fn get_element(&self) -> Element {
        use KujouSaraDamageEnum::*;
        match *self {
            Charged2 | E1 | Q1 | Q2 => Element::Electro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use KujouSaraDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum KujouSaraRoleEnum {
    Aux
}

impl Into<usize> for KujouSaraDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl CharacterTrait for KujouSara {
    const STATIC_DATA: CharacterStaticData = KUJOU_SARA_STATIC_DATA;
    type SkillType = KujouSaraSkillType;
    const SKILL: Self::SkillType = KUJOU_SARA_SKILL;
    type DamageEnumType = KujouSaraDamageEnum;
    type RoleEnum = KujouSaraRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Charged2 as usize, text: charged_dmg!("shoot2") },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: KujouSaraDamageEnum::E1 as usize, text: locale!(zh_cn: "天狗咒雷•伏伤害", en: "Tengu Juurai: Ambush DMG") }
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Q1 as usize, text: locale!(zh_cn: "天狗咒雷•金刚坏伤害", en: "Tengu Juurai: Titanbreaker DMG") },
            CharacterSkillMapItem { index: KujouSaraDamageEnum::Q2 as usize, text: locale!(zh_cn: "天狗咒雷•雷砾伤害", en: "Tengu Juurai: Stormcluster DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KujouSaraDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KujouSaraDamageEnum::*;
        let ratio = match s {
            Normal1 => KUJOU_SARA_SKILL.normal_dmg1[s1],
            Normal2 => KUJOU_SARA_SKILL.normal_dmg2[s1],
            Normal3 => KUJOU_SARA_SKILL.normal_dmg3[s1],
            Normal4 => KUJOU_SARA_SKILL.normal_dmg4[s1],
            Normal5 => KUJOU_SARA_SKILL.normal_dmg5[s1],
            Charged1 => KUJOU_SARA_SKILL.charged_dmg1[s1],
            Charged2 => KUJOU_SARA_SKILL.charged_dmg2[s1],
            Plunging1 => KUJOU_SARA_SKILL.plunging_dmg1[s1],
            Plunging2 => KUJOU_SARA_SKILL.plunging_dmg2[s1],
            Plunging3 => KUJOU_SARA_SKILL.plunging_dmg3[s1],
            E1 => KUJOU_SARA_SKILL.elemental_skill_dmg1[s2],
            Q1 => KUJOU_SARA_SKILL.elemental_burst_dmg1[s3],
            Q2 => KUJOU_SARA_SKILL.elemental_burst_dmg2[s3],
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
        let role: KujouSaraRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            KujouSaraRoleEnum::Aux => Box::new(KujouSaraDefaultTargetFunction)
        }
    }
}
