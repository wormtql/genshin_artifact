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
use crate::target_functions::target_functions::VentiDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct VentiSkillType {
    pub normal_dmg11: [f64; 15],
    pub normal_dmg12: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg41: [f64; 15],
    pub normal_dmg42: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub normal_dmg6: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const VENTI_SKILL: VentiSkillType = VentiSkillType {
    normal_dmg11: [0.2038, 0.2204, 0.237, 0.2607, 0.2773, 0.2963, 0.3223, 0.3484, 0.3745, 0.4029, 0.4355, 0.4738, 0.5121, 0.5505, 0.5923],
    normal_dmg12: [0.2038, 0.2204, 0.237, 0.2607, 0.2773, 0.2963, 0.3223, 0.3484, 0.3745, 0.4029, 0.4355, 0.4738, 0.5121, 0.5505, 0.5923],
    normal_dmg2: [0.4438, 0.4799, 0.516, 0.5676, 0.6037, 0.645, 0.7018, 0.7585, 0.8153, 0.8772, 0.9482, 1.0316, 1.115, 1.1985, 1.2895],
    normal_dmg3: [0.5237, 0.5664, 0.609, 0.6699, 0.7125, 0.7613, 0.8282, 0.8952, 0.9622, 1.0353, 1.119, 1.2175, 1.316, 1.4145, 1.5219],
    normal_dmg41: [0.2606, 0.2818, 0.303, 0.3333, 0.3545, 0.3787, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568, 0.6058, 0.6548, 0.7037, 0.7572],
    normal_dmg42: [0.2606, 0.2818, 0.303, 0.3333, 0.3545, 0.3787, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568, 0.6058, 0.6548, 0.7037, 0.7572],
    normal_dmg5: [0.5065, 0.5478, 0.589, 0.6479, 0.6891, 0.7363, 0.801, 0.8658, 0.9306, 1.0013, 1.0823, 1.1775, 1.2728, 1.368, 1.4719],
    normal_dmg6: [0.7095, 0.7673, 0.825, 0.9075, 0.9653, 1.0313, 1.122, 1.2128, 1.3035, 1.4025, 1.5159, 1.6493, 1.7827, 1.9161, 2.0617],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9371, 1.0196, 1.1021, 1.1845, 1.2745],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.361, 2.5296, 2.6982, 2.8669, 3.0355],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [2.76, 2.967, 3.174, 3.45, 3.657, 3.864, 4.14, 4.416, 4.692, 4.968, 5.244, 5.52, 5.865, 6.21, 6.555],
    elemental_skill_dmg2: [3.8, 4.085, 4.37, 4.75, 5.035, 5.32, 5.7, 6.08, 6.46, 6.84, 7.22, 7.6, 8.075, 8.55, 9.025],
    elemental_burst_dmg1: [0.376, 0.4042, 0.4324, 0.47, 0.4982, 0.5264, 0.564, 0.6016, 0.6392, 0.6768, 0.7144, 0.752, 0.799, 0.846, 0.893],
    elemental_burst_dmg2: [0.188, 0.2021, 0.2162, 0.235, 0.2491, 0.2632, 0.282, 0.3008, 0.3196, 0.3384, 0.3572, 0.376, 0.3995, 0.423, 0.4465]
};

pub const VENTI_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Venti,
    internal_name: "Venti",
    element: Element::Anemo,
    hp: [820, 2127, 2830, 4234, 4734, 5446, 6112, 6832, 7331, 8058, 8557, 9292, 9791, 10531],
    atk: [20, 53, 71, 106, 118, 136, 153, 171, 183, 201, 214, 232, 245, 263],
    def: [52, 135, 180, 269, 301, 346, 388, 434, 465, 512, 543, 590, 622, 669],
    sub_stat: CharacterSubStatFamily::Recharge320,
    weapon_type: WeaponType::Bow,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·神代射术",
        en: "Normal Attack: Divine Marksmanship",
    ),
    skill_name2: locale!(
        zh_cn: "高天之歌",
        en: "Skyward Sonnet",
    ),
    skill_name3: locale!(
        zh_cn: "风神之诗",
        en: "Wind's Grand Ode",
    ),
    name_locale: locale!(
        zh_cn: "温迪",
        en: "Venti",
    )
};

pub struct Venti;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum VentiDamageEnum {
    Normal11,
    Normal12,
    Normal1,
    Normal2,
    Normal3,
    Normal41,
    Normal42,
    Normal4,
    Normal5,
    Normal6,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    Q1,
    Q2Pyro,
    Q2Electro,
    Q2Hydro,
    Q2Cryo
}

impl VentiDamageEnum {
    pub fn get_element(&self) -> Element {
        use VentiDamageEnum::*;
        match *self {
            E1 | E2 | Q1 | Charged2 => Element::Anemo,
            Q2Pyro => Element::Pyro,
            Q2Electro => Element::Electro,
            Q2Hydro => Element::Hydro,
            Q2Cryo => Element::Cryo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use VentiDamageEnum::*;
        match *self {
            Normal1 | Normal11 | Normal12 | Normal2 | Normal3 | Normal41 | Normal42 | Normal4 | Normal5 | Normal6 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 | Q2Pyro | Q2Electro | Q2Hydro | Q2Cryo => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for VentiDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum VentiRoleEnum {
    Sub,
}

impl CharacterTrait for Venti {
    const STATIC_DATA: CharacterStaticData = VENTI_STATIC_DATA;
    type SkillType = VentiSkillType;
    const SKILL: Self::SkillType = VENTI_SKILL;
    type DamageEnumType = VentiDamageEnum;
    type RoleEnum = VentiRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: VentiDamageEnum::Normal11 as usize, text: hit_n_dmg!(1, 1) },
            CharacterSkillMapItem { index: VentiDamageEnum::Normal12 as usize, text: hit_n_dmg!(1, 2) },
            CharacterSkillMapItem { index: VentiDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: VentiDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: VentiDamageEnum::Normal41 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: VentiDamageEnum::Normal42 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: VentiDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: VentiDamageEnum::Normal6 as usize, text: hit_n_dmg!(6) },
            CharacterSkillMapItem { index: VentiDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: VentiDamageEnum::Charged2 as usize, text: charged_dmg!("shoot2") },
            CharacterSkillMapItem { index: VentiDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: VentiDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: VentiDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: VentiDamageEnum::E1 as usize, text: locale!(zh_cn: "点按伤害", en: "Tapping DMG") },
            CharacterSkillMapItem { index: VentiDamageEnum::E2 as usize, text: locale!(zh_cn: "长按伤害", en: "Hold DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: VentiDamageEnum::Q1 as usize, text: locale!(zh_cn: "持续伤害", en: "DoT") },
            CharacterSkillMapItem { index: VentiDamageEnum::Q2Pyro as usize, text: locale!(zh_cn: "附加火元素伤害", en: "Additional Pyro DMG") },
            CharacterSkillMapItem { index: VentiDamageEnum::Q2Hydro as usize, text: locale!(zh_cn: "附加水元素伤害", en: "Additional Hydro DMG") },
            CharacterSkillMapItem { index: VentiDamageEnum::Q2Cryo as usize, text: locale!(zh_cn: "附加冰元素伤害", en: "Additional Cryo DMG") },
            CharacterSkillMapItem { index: VentiDamageEnum::Q2Electro as usize, text: locale!(zh_cn: "附加雷元素伤害", en: "Additional Electro DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: VentiDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use VentiDamageEnum::*;
        let ratio = match s {
            Normal11 => VENTI_SKILL.normal_dmg11[s1],
            Normal12 => VENTI_SKILL.normal_dmg12[s1],
            Normal1 => VENTI_SKILL.normal_dmg11[s1] + VENTI_SKILL.normal_dmg12[s1],
            Normal2 => VENTI_SKILL.normal_dmg2[s1],
            Normal3 => VENTI_SKILL.normal_dmg3[s1],
            Normal41 => VENTI_SKILL.normal_dmg41[s1],
            Normal42 => VENTI_SKILL.normal_dmg42[s1],
            Normal4 => VENTI_SKILL.normal_dmg41[s1] + VENTI_SKILL.normal_dmg42[s1],
            Normal5 => VENTI_SKILL.normal_dmg5[s1],
            Normal6 => VENTI_SKILL.normal_dmg6[s1],
            Charged1 => VENTI_SKILL.charged_dmg1[s1],
            Charged2 => VENTI_SKILL.charged_dmg2[s1],
            Plunging1 => VENTI_SKILL.plunging_dmg1[s1],
            Plunging2 => VENTI_SKILL.plunging_dmg2[s1],
            Plunging3 => VENTI_SKILL.plunging_dmg3[s1],
            E1 => VENTI_SKILL.elemental_skill_dmg1[s2],
            E2 => VENTI_SKILL.elemental_skill_dmg2[s2],
            Q1 => VENTI_SKILL.elemental_burst_dmg1[s3],
            Q2Pyro | Q2Electro | Q2Hydro | Q2Cryo => VENTI_SKILL.elemental_burst_dmg2[s3]
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
        let role: VentiRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            VentiRoleEnum::Sub => Box::new(VentiDefaultTargetFunction {
                swirl_rate: 0.7
            })
        }
    }
}
