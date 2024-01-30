use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::XinyanDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct XinyanSkillType {
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
    pub elemental_skill_shield1: [f64; 15],
    pub elemental_skill_shield1_fixed: [f64; 15],
    pub elemental_skill_shield2: [f64; 15],
    pub elemental_skill_shield2_fixed: [f64; 15],
    pub elemental_skill_shield3: [f64; 15],
    pub elemental_skill_shield3_fixed: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const XINYAN_SKILL: XinyanSkillType = XinyanSkillType {
    normal_dmg1: [0.7654, 0.8277, 0.89, 0.979, 1.0413, 1.1125, 1.2104, 1.3083, 1.4062, 1.513, 1.6198, 1.7266, 1.8334, 1.9402, 2.047],
    normal_dmg2: [0.7396, 0.7998, 0.86, 0.946, 1.0062, 1.075, 1.1696, 1.2642, 1.3588, 1.462, 1.5652, 1.6684, 1.7716, 1.8748, 1.978],
    normal_dmg3: [0.9546, 1.0323, 1.11, 1.221, 1.2987, 1.3875, 1.5096, 1.6317, 1.7538, 1.887, 2.0202, 2.1534, 2.2866, 2.4198, 2.553],
    normal_dmg4: [1.1584, 1.2527, 1.347, 1.4817, 1.576, 1.6838, 1.8319, 1.9801, 2.1283, 2.2899, 2.4515, 2.6132, 2.7748, 2.9365, 3.0981],
    charged_dmg1: [0.6255, 0.6764, 0.7273, 0.8, 0.8509, 0.9091, 0.9891, 1.0691, 1.1491, 1.2364, 1.3236, 1.4109, 1.4982, 1.5855, 1.6727],
    charged_dmg2: [1.1309, 1.223, 1.315, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933, 2.5511, 2.7089, 2.8667, 3.0245],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    elemental_skill_dmg1: [1.696, 1.8232, 1.9504, 2.12, 2.2472, 2.3744, 2.544, 2.7136, 2.8832, 3.0528, 3.2224, 3.392, 3.604, 3.816, 4.028],
    elemental_skill_shield1: [1.0404, 1.1184, 1.1965, 1.3005, 1.3785, 1.4566, 1.5606, 1.6646, 1.7687, 1.8727, 1.9768, 2.0808, 2.2109, 2.3409, 2.471],
    elemental_skill_shield1_fixed: [501.0, 551.0, 605.0, 663.0, 726.0, 793.0, 864.0, 939.0, 1018.0, 1101.0, 1189.0, 1281.0, 1377.0, 1477.0, 1581.0],
    elemental_skill_shield2: [1.224, 1.3158, 1.4076, 1.53, 1.6218, 1.7136, 1.836, 1.9584, 2.0808, 2.2032, 2.3256, 2.448, 2.601, 2.754, 2.907],
    elemental_skill_shield2_fixed: [589.0, 648.0, 712.0, 780.0, 854.0, 932.0, 1016.0, 1104.0, 1197.0, 1296.0, 1399.0, 1507.0, 1620.0, 1737.0, 1860.0],
    elemental_skill_shield3: [1.44, 1.548, 1.656, 1.8, 1.908, 2.016, 2.16, 2.304, 2.448, 2.592, 2.736, 2.88, 3.06, 3.24, 3.42],
    elemental_skill_shield3_fixed: [693.0, 762.0, 837.0, 918.0, 1005.0, 1097.0, 1195.0, 1299.0, 1409.0, 1524.0, 1656.0, 1773.0, 1905.0, 2044.0, 2188.0],
    elemental_skill_dmg2: [0.336, 0.3612, 0.3864, 0.42, 0.4452, 0.4704, 0.504, 0.5376, 0.5712, 0.6048, 0.6384, 0.672, 0.714, 0.756, 0.798],
    elemental_burst_dmg1: [3.408, 3.6636, 3.9192, 4.26, 4.5156, 4.7712, 5.112, 5.4528, 5.7936, 6.1344, 6.4752, 6.816, 7.242, 7.668, 8.094],
    elemental_burst_dmg2: [0.4, 0.43, 0.46, 0.5, 0.53, 0.56, 0.6, 0.64, 0.68, 0.72, 0.76, 0.8, 0.85, 0.9, 0.95],
};

pub const XINYAN_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Xinyan,
    internal_name: "Xinyan",
    element: Element::Pyro,
    hp: [939, 2413, 3114, 4665, 5163, 5939, 6604, 7379, 7878, 8653, 9151, 9927, 10425, 11201],
    atk: [21, 54, 69, 103, 115, 132, 147, 164, 175, 192, 203, 220, 231, 249],
    def: [67, 172, 222, 333, 368, 423, 471, 526, 562, 617, 652, 708, 743, 799],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Claymore,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·炎舞",
        en: "Normal Attack: Dance on Fire",
    ),
    skill_name2: locale!(
        zh_cn: "热情拂扫",
        en: "Sweeping Fervor",
    ),
    skill_name3: locale!(
        zh_cn: "叛逆刮弦",
        en: "Riff Revolution",
    ),
    name_locale: locale!(
        zh_cn: "辛焱",
        en: "Xinyan",
    )
};

pub struct Xinyan;

#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, EnumString, EnumCountMacro)]
pub enum XinyanDamageEnum {
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
    Q1,
    Q2,
}

impl XinyanDamageEnum {
    pub fn get_element(&self) -> Element {
        use XinyanDamageEnum::*;
        match *self {
            E1 | E2 | Q2 => Element::Pyro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use XinyanDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for XinyanDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum XinyanRoleEnum {
    Aux
}

impl CharacterTrait for Xinyan {
    const STATIC_DATA: CharacterStaticData = XINYAN_STATIC_DATA;
    type SkillType = XinyanSkillType;
    const SKILL: Self::SkillType = XINYAN_SKILL;
    type DamageEnumType = XinyanDamageEnum;
    type RoleEnum = XinyanRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: XinyanDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: XinyanDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: XinyanDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: XinyanDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: XinyanDamageEnum::Charged1 as usize, text: charged_dmg!("loop1") },
            CharacterSkillMapItem { index: XinyanDamageEnum::Charged2 as usize, text: charged_dmg!("loop2") },
            CharacterSkillMapItem { index: XinyanDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: XinyanDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: XinyanDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: XinyanDamageEnum::E1 as usize, text: locale!(zh_cn: "挥舞伤害", en: "Swing DMG") },
            CharacterSkillMapItem { index: XinyanDamageEnum::E2 as usize, text: locale!(zh_cn: "持续伤害", en: "DoT") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: XinyanDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: XinyanDamageEnum::Q2 as usize, text: locale!(zh_cn: "火元素持续伤害", en: "Pyro DoT") },
        ]),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "shield_rate",
            title: locale!(
                zh_cn: "「热情拂扫」护盾覆盖比例",
                en: "「Sweeping Fervor」Shield Coverage",
            ),
            config: ItemConfig::RATE01_TYPE,
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: XinyanDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use XinyanDamageEnum::*;
        let ratio = match s {
            Normal1 => XINYAN_SKILL.normal_dmg1[s1],
            Normal2 => XINYAN_SKILL.normal_dmg2[s1],
            Normal3 => XINYAN_SKILL.normal_dmg3[s1],
            Normal4 => XINYAN_SKILL.normal_dmg4[s1],
            Charged1 => XINYAN_SKILL.charged_dmg1[s1],
            Charged2 => XINYAN_SKILL.charged_dmg2[s1],
            Plunging1 => XINYAN_SKILL.plunging_dmg1[s1],
            Plunging2 => XINYAN_SKILL.plunging_dmg2[s1],
            Plunging3 => XINYAN_SKILL.plunging_dmg3[s1],
            E1 => XINYAN_SKILL.elemental_skill_dmg1[s2],
            E2 => XINYAN_SKILL.elemental_skill_dmg2[s2],
            Q1 => XINYAN_SKILL.elemental_burst_dmg1[s3],
            Q2 => XINYAN_SKILL.elemental_burst_dmg2[s3],
        };

        let shield_rate = match *config {
            CharacterSkillConfig::Xinyan { shield_rate } => shield_rate,
            _ => 0.0
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let s_element = s.get_element();

        let c4 = context.character_common_data.constellation >= 4;
        let c6 = context.character_common_data.constellation == 6;

        if c4 && s_element == Element::Physical {
            builder.add_extra_res_minus("四命：「节奏的传染」", 0.15 * shield_rate);
        }

        if context.character_common_data.has_talent2 && s_element == Element::Physical {
            builder.add_extra_bonus("天赋：「...这才是摇滚！」", 0.15 * shield_rate);
        }

        if (s == XinyanDamageEnum::Charged1 || s == XinyanDamageEnum::Charged2) && c6 {
            builder.add_extra_atk("六命：「地狱里摇摆」", context.attribute.get_def() * 0.5);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s_element,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: XinyanRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            XinyanRoleEnum::Aux => Box::new(XinyanDefaultTargetFunction {
                recharge_demand: 1.4,
                damage_demand: 0.5,
            })
        }
    }
}
