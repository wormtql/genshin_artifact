use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::KaedeharaKazuhaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{hit_n_dmg, locale, plunging_dmg};

pub struct KaedeharaKazuhaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],     // x3
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_dmg3: [f64; 15],
}

pub const KAEDEHARA_KAZUHA_SKILL: KaedeharaKazuhaSkillType = KaedeharaKazuhaSkillType {
    normal_dmg1: [0.4498, 0.4864, 0.523, 0.5753, 0.6119, 0.6538, 0.7113, 0.7688, 0.8263, 0.8891, 0.961, 1.0456, 1.1302, 1.2147, 1.307],
    normal_dmg2: [0.4524, 0.4892, 0.526, 0.5786, 0.6154, 0.6575, 0.7154, 0.7732, 0.8311, 0.8942, 0.9665, 1.0516, 1.1366, 1.2217, 1.3145],
    normal_dmg31: [0.258, 0.279, 0.3, 0.33, 0.351, 0.375, 0.408, 0.441, 0.474, 0.51, 0.5513, 0.5998, 0.6483, 0.6968, 0.7497],
    normal_dmg32: [0.3096, 0.3348, 0.36, 0.396, 0.4212, 0.45, 0.4896, 0.5292, 0.5688, 0.612, 0.6615, 0.7197, 0.7779, 0.8361, 0.8996],
    normal_dmg4: [0.6072, 0.6566, 0.706, 0.7766, 0.826, 0.8825, 0.9602, 1.0378, 1.1155, 1.2002, 1.2973, 1.4114, 1.5256, 1.6398, 1.7643],
    normal_dmg5: [0.2537, 0.2744, 0.295, 0.3245, 0.3452, 0.3688, 0.4012, 0.4337, 0.4661, 0.5015, 0.5421, 0.5898, 0.6375, 0.6852, 0.7372],
    charged_dmg11: [0.43, 0.465, 0.5, 0.55, 0.585, 0.625, 0.68, 0.735, 0.79, 0.85, 0.9188, 0.9996, 1.0805, 1.1613, 1.2495],
    charged_dmg12: [0.7465, 0.8072, 0.868, 0.9548, 1.0156, 1.085, 1.1805, 1.276, 1.3714, 1.4756, 1.595, 1.7353, 1.8757, 2.016, 2.1691],
    plunging_dmg1: [0.8183, 0.8849, 0.9516, 1.0467, 1.1133, 1.1894, 1.2941, 1.3988, 1.5035, 1.6176, 1.7318, 1.846, 1.9602, 2.0744, 2.1886],
    plunging_dmg2: [1.6363, 1.7695, 1.9027, 2.093, 2.2262, 2.3784, 2.5877, 2.797, 3.0063, 3.2346, 3.4629, 3.6912, 3.9196, 4.1479, 4.3762],
    plunging_dmg3: [2.0439, 2.2102, 2.3766, 2.6142, 2.7806, 2.9707, 3.2321, 3.4936, 3.755, 4.0402, 4.3254, 4.6106, 4.8957, 5.1809, 5.4661],
    elemental_skill_dmg1: [1.92, 2.064, 2.208, 2.4, 2.544, 2.688, 2.88, 3.072, 3.264, 3.456, 3.648, 3.84, 4.08, 4.32, 4.56],
    elemental_skill_dmg2: [2.608, 2.8036, 2.9992, 3.26, 3.4556, 3.6512, 3.912, 4.1728, 4.4336, 4.6944, 4.9552, 5.216, 5.542, 5.868, 6.194],
    elemental_burst_dmg1: [2.624, 2.8208, 3.0176, 3.28, 3.4768, 3.6736, 3.936, 4.1984, 4.4608, 4.7232, 4.9856, 5.248, 5.576, 5.904, 6.232],
    elemental_burst_dmg2: [1.2, 1.29, 1.38, 1.5, 1.59, 1.68, 1.8, 1.92, 2.04, 2.16, 2.28, 2.4, 2.55, 2.7, 2.85],
    elemental_burst_dmg3: [0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765, 0.81, 0.855],
};

pub const KAEDEHARA_KAZUHA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::KaedeharaKazuha,
    internal_name: "Kazuha",
    element: Element::Anemo,
    hp: [1039, 2695, 3586, 5366, 5999, 6902, 7747, 8659, 9292, 10213, 10849, 11777, 12410, 13348],
    atk: [23, 60, 80, 119, 133, 153, 172, 192, 206, 227, 241, 262, 276, 297],
    def: [63, 163, 217, 324, 363, 417, 468, 523, 562, 617, 656, 712, 750, 807],
    sub_stat: CharacterSubStatFamily::ElementalMastery115,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·我流剑术",
        en: "Normal Attack: Garyuu Bladework",
    ),
    skill_name2: locale!(
        zh_cn: "千早振",
        en: "Chihayaburu",
    ),
    skill_name3: locale!(
        zh_cn: "万叶之一刀",
        en: "Kazuha Slash",
    ),
    name_locale: locale!(
        zh_cn: "枫原万叶",
        en: "Kaedehara Kazuha",
    )
};

pub struct KaedeharaKazuha;

#[derive(Copy, Clone, FromPrimitive)]
#[derive(EnumString, EnumCountMacro)]
pub enum KaedeharaKazuhaDamageEnum {
    Normal1,
    Normal2,
    Normal31,
    Normal32,
    Normal4,
    Normal5,
    Charged11,
    Charged12,
    Plunging1,
    Plunging2,
    Plunging3,
    PlungingE1,
    PlungingE2,
    PlungingE3,
    PlungingPyro,
    PlungingHydro,
    PlungingCryo,
    PlungingElectro,
    E1,
    E2,
    Q1,
    Q2,
    Q3Pyro,
    Q3Hydro,
    Q3Cryo,
    Q3Electro
}

impl KaedeharaKazuhaDamageEnum {
    pub fn get_element(&self, after_e_or_q: bool) -> Element {
        use KaedeharaKazuhaDamageEnum::*;
        match *self {
            E1 | E2 | Q1 | Q2 | PlungingE1 | PlungingE2 | PlungingE3 => Element::Anemo,
            Q3Cryo | PlungingCryo => Element::Cryo,
            Q3Hydro | PlungingHydro => Element::Hydro,
            Q3Pyro | PlungingPyro => Element::Pyro,
            Q3Electro | PlungingElectro => Element::Electro,
            _ => if after_e_or_q { Element::Anemo } else { Element::Physical }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use KaedeharaKazuhaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 | PlungingE1 => SkillType::PlungingAttackInAction,
            Plunging2 | PlungingE2 | Plunging3 | PlungingE3 | PlungingPyro | PlungingHydro | PlungingElectro | PlungingCryo => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 | Q2 | Q3Hydro | Q3Pyro | Q3Cryo | Q3Electro => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for KaedeharaKazuhaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum KaedeharaKazuhaRoleEnum {
    Default
}

impl CharacterTrait for KaedeharaKazuha {
    const STATIC_DATA: CharacterStaticData = KAEDEHARA_KAZUHA_STATIC_DATA;
    type SkillType = KaedeharaKazuhaSkillType;
    const SKILL: Self::SkillType = KAEDEHARA_KAZUHA_SKILL;
    type DamageEnumType = KaedeharaKazuhaDamageEnum;
    type RoleEnum = KaedeharaKazuhaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_e_or_q",
            title: locale!(
                zh_cn: "六命：血赤叶红",
                en: "C6: Crimson Momiji"
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Normal31 as usize, text: hit_n_dmg!(3, 1) },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Normal32 as usize, text: hit_n_dmg!(3, 2) },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Normal5 as usize, text: locale!(zh_cn: "五段伤害/3", en: "5-Hit DMG/3") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Charged11 as usize, text: locale!(zh_cn: "重击伤害-1", en: "Charged DMG-1") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Charged11 as usize, text: locale!(zh_cn: "重击伤害-2", en: "Charged DMG-2") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::PlungingE1 as usize, text: locale!(zh_cn: "乱岚拨止：下坠期间伤害", en: "Midare Ranzan: Plunge DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::PlungingE2 as usize, text: locale!(zh_cn: "乱岚拨止：低空坠地冲击伤害", en: "Midare Ranzan: Low Plunge DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::PlungingE3 as usize, text: locale!(zh_cn: "乱岚拨止：高空坠地冲击伤害", en: "Midare Ranzan: High Plunge DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::PlungingPyro as usize, text: locale!(zh_cn: "乱岚拨止：火元素转化伤害", en: "Midare Ranzan: Pyro DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::PlungingHydro as usize, text: locale!(zh_cn: "乱岚拨止：水元素转化伤害", en: "Midare Ranzan: Hydro DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::PlungingCryo as usize, text: locale!(zh_cn: "乱岚拨止：冰元素转化伤害", en: "Midare Ranzan: Cryo DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::PlungingElectro as usize, text: locale!(zh_cn: "乱岚拨止：雷元素转化伤害", en: "Midare Ranzan: Electro DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::E1 as usize, text: locale!(zh_cn: "点按技能伤害", en: "Press Skill DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::E2 as usize, text: locale!(zh_cn: "长按技能伤害", en: "Hold Skill DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Q1 as usize, text: locale!(zh_cn: "斩击伤害", en: "Slashing DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Q2 as usize, text: locale!(zh_cn: "持续伤害", en: "DoT") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Q3Pyro as usize, text: locale!(zh_cn: "附加火元素伤害", en: "Additional Pyro DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Q3Hydro as usize, text: locale!(zh_cn: "附加水元素伤害", en: "Additional Hydro DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Q3Cryo as usize, text: locale!(zh_cn: "附加冰元素伤害", en: "Additional Cryo DMG") },
            CharacterSkillMapItem { index: KaedeharaKazuhaDamageEnum::Q3Electro as usize, text: locale!(zh_cn: "附加雷元素伤害", en: "Additional Electro DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KaedeharaKazuhaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KaedeharaKazuhaDamageEnum::*;
        let ratio = match s {
            Normal1 => KAEDEHARA_KAZUHA_SKILL.normal_dmg1[s1],
            Normal2 => KAEDEHARA_KAZUHA_SKILL.normal_dmg2[s1],
            Normal31 => KAEDEHARA_KAZUHA_SKILL.normal_dmg31[s1],
            Normal32 => KAEDEHARA_KAZUHA_SKILL.normal_dmg32[s1],
            Normal4 => KAEDEHARA_KAZUHA_SKILL.normal_dmg4[s1],
            Normal5 => KAEDEHARA_KAZUHA_SKILL.normal_dmg5[s1],
            Charged11 => KAEDEHARA_KAZUHA_SKILL.charged_dmg11[s1],
            Charged12 => KAEDEHARA_KAZUHA_SKILL.charged_dmg12[s1],
            Plunging1 | PlungingE1 => KAEDEHARA_KAZUHA_SKILL.plunging_dmg1[s1],
            Plunging2 | PlungingE2 => KAEDEHARA_KAZUHA_SKILL.plunging_dmg2[s1],
            Plunging3 | PlungingE3 => KAEDEHARA_KAZUHA_SKILL.plunging_dmg3[s1],
            PlungingElectro | PlungingCryo | PlungingHydro | PlungingPyro => 2.0,
            E1 => KAEDEHARA_KAZUHA_SKILL.elemental_skill_dmg1[s2],
            E2 => KAEDEHARA_KAZUHA_SKILL.elemental_skill_dmg2[s2],
            Q1 => KAEDEHARA_KAZUHA_SKILL.elemental_burst_dmg1[s3],
            Q2 => KAEDEHARA_KAZUHA_SKILL.elemental_burst_dmg2[s3],
            Q3Electro | Q3Pyro | Q3Cryo | Q3Hydro => KAEDEHARA_KAZUHA_SKILL.elemental_burst_dmg3[s3]
        };

        let after_e_or_q = match *config {
            CharacterSkillConfig::KaedeharaKazuha { after_e_or_q } => after_e_or_q,
            _ => false
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let skill_type = s.get_skill_type();
        if after_e_or_q
            && (skill_type == SkillType::NormalAttack
                || skill_type == SkillType::ChargedAttack
                || skill_type.is_plunging()
            ) {
            // let em = context.attribute.get_value(AttributeName::ElementalMastery);
            let em = context.attribute.get_em_all();
            let bonus = em * 0.002;
            builder.add_extra_bonus("枫原万叶六命：血赤叶红", bonus);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(after_e_or_q),
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: KaedeharaKazuhaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            KaedeharaKazuhaRoleEnum::Default => Box::new(KaedeharaKazuhaDefaultTargetFunction {
                recharge_demand: 1.8
            }),
        }
    }
}
