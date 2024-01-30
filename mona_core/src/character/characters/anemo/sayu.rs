use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::SayuDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{hit_n_dmg, locale, plunging_dmg};

pub struct SayuSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_dmg4: [f64; 15],
    pub elemental_skill_dmg5: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_heal1: [f64; 15],
    pub elemental_burst_heal1_fixed: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_heal2: [f64; 15],
    pub elemental_burst_heal2_fixed: [f64; 15],
}

pub const SAYU_SKILL: SayuSkillType = SayuSkillType {
    normal_dmg1: [0.7224, 0.7812, 0.84, 0.924, 0.9828, 1.05, 1.1424, 1.2348, 1.3272, 1.428, 1.5435, 1.6793, 1.8152, 1.951, 2.0992],
    normal_dmg2: [0.7138, 0.7719, 0.83, 0.913, 0.9711, 1.0375, 1.1288, 1.2201, 1.3114, 1.411, 1.5251, 1.6593, 1.7935, 1.9278, 2.0742],
    normal_dmg31: [0.4343, 0.4697, 0.505, 0.5555, 0.5909, 0.6313, 0.6868, 0.7423, 0.7979, 0.8585, 0.9279, 1.0096, 1.0913, 1.1729, 1.262],
    normal_dmg32: [0.4343, 0.4697, 0.505, 0.5555, 0.5909, 0.6313, 0.6868, 0.7423, 0.7979, 0.8585, 0.9279, 1.0096, 1.0913, 1.1729, 1.262],
    normal_dmg4: [0.9813, 1.0611, 1.141, 1.2551, 1.335, 1.4263, 1.5518, 1.6773, 1.8028, 1.9397, 2.0966, 2.2811, 2.4656, 2.6501, 2.8514],
    charged_dmg1: [0.6255, 0.6764, 0.7273, 0.8, 0.8509, 0.9091, 0.9891, 1.0691, 1.1491, 1.2364, 1.3364, 1.454, 1.5716, 1.6892, 1.8175],
    charged_dmg2: [1.1309, 1.223, 1.315, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.4163, 2.6289, 2.8416, 3.0542, 3.2862],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    elemental_skill_dmg1: [0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765, 0.81, 0.855],
    elemental_skill_dmg2: [1.584, 1.7028, 1.8216, 1.98, 2.0988, 2.2176, 2.376, 2.5344, 2.6928, 2.8512, 3.0096, 3.168, 3.366, 3.564, 3.762],
    elemental_skill_dmg3: [2.176, 2.3392, 2.5024, 2.72, 2.8832, 3.0464, 3.264, 3.4816, 3.6992, 3.9168, 4.1344, 4.352, 4.624, 4.896, 5.168],
    elemental_skill_dmg4: [0.168, 0.1806, 0.1932, 0.21, 0.2226, 0.2352, 0.252, 0.2688, 0.2856, 0.3024, 0.3192, 0.336, 0.357, 0.378, 0.399],
    elemental_skill_dmg5: [0.7616, 0.8187, 0.8758, 0.952, 1.0091, 1.0662, 1.1424, 1.2186, 1.2947, 1.3709, 1.447, 1.5232, 1.6184, 1.7136, 1.8088],
    elemental_burst_dmg1: [1.168, 1.2556, 1.3432, 1.46, 1.5476, 1.6352, 1.752, 1.8688, 1.9856, 2.1024, 2.2192, 2.336, 2.482, 2.628, 2.774],
    elemental_burst_heal1: [0.9216, 0.9907, 1.0598, 1.152, 1.2211, 1.2902, 1.3824, 1.4746, 1.5667, 1.6589, 1.751, 1.8432, 1.9584, 2.0736, 2.1888],
    elemental_burst_heal1_fixed: [577.0, 635.0, 698.0, 765.0, 837.0, 914.0, 996.0, 1083.0, 1174.0, 1270.0, 1371.0, 1477.0, 1588.0, 1703.0, 1824.0],
    elemental_burst_dmg2: [0.52, 0.559, 0.598, 0.65, 0.689, 0.728, 0.78, 0.832, 0.884, 0.936, 0.988, 1.04, 1.105, 1.17, 1.235],
    elemental_burst_heal2: [0.7987, 0.8586, 0.9185, 0.9984, 1.0583, 1.1182, 1.1981, 1.278, 1.3578, 1.4377, 1.5176, 1.5974, 1.6973, 1.7971, 1.897],
    elemental_burst_heal2_fixed: [500.0, 550.0, 605.0, 663.0, 726.0, 792.0, 863.0, 938.0, 1017.0, 1101.0, 1188.0, 1280.0, 1376.0, 1476.0, 1580.0],
};

pub const SAYU_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Sayu,
    internal_name: "Sayu",
    element: Element::Anemo,
    hp: [994, 2553, 3296, 4937, 5464, 6285, 6988, 7809, 8337, 9157, 9684, 10505, 11033, 11854],
    atk: [20, 53, 68, 102, 113, 130, 144, 161, 172, 189, 200, 216, 227, 244],
    def: [62, 160, 207, 310, 343, 395, 439, 491, 524, 575, 608, 660, 693, 745],
    sub_stat: CharacterSubStatFamily::ElementalMastery96,
    weapon_type: WeaponType::Claymore,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·忍刀·终末番",
        en: "Normal Attack: Shuumatsuban Ninja Blade",
    ),
    skill_name2: locale!(
        zh_cn: "呜呼流·风隐急进",
        en: "Yoohoo Art: Fuuin Dash",
    ),
    skill_name3: locale!(
        zh_cn: "呜呼流·影貉缭乱",
        en: "Yoohoo Art: Mujina Flurry",
    ),
    name_locale: locale!(
        zh_cn: "早柚",
        en: "Sayu",
    )
};

pub struct Sayu;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum SayuDamageEnum {
    Normal1,
    Normal2,
    Normal31,
    Normal32,
    Normal4,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    E4Pyro,
    E4Electro,
    E4Cryo,
    E4Hydro,
    E5Pyro,
    E5Electro,
    E5Cryo,
    E5Hydro,
    Q1,
    QHeal1,
    Q2,
    QHeal2
}

impl SayuDamageEnum {
    pub fn is_heal(&self) -> bool {
        use SayuDamageEnum::*;
        match *self {
            QHeal1 | QHeal2 => true,
            _ => false
        }
    }

    pub fn is_q(&self) -> bool {
        use SayuDamageEnum::*;
        match *self {
            Q1 | Q2 | QHeal1 | QHeal2 => true,
            _ => false
        }
    }

    pub fn get_element(&self) -> Element {
        use SayuDamageEnum::*;
        match *self {
            E1 | E2 | E3 | Q1 | Q2 => Element::Anemo,
            E4Pyro | E5Pyro => Element::Pyro,
            E4Electro | E5Electro => Element::Electro,
            E4Cryo | E5Cryo => Element::Cryo,
            E4Hydro | E5Hydro => Element::Hydro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use SayuDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Q1 | QHeal1 | Q2 | QHeal2 => SkillType::ElementalBurst,
            _ => SkillType::ElementalSkill
        }
    }
}

impl Into<usize> for SayuDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum SayuRoleEnum {
    Normal
}

impl CharacterTrait for Sayu {
    const STATIC_DATA: CharacterStaticData = SAYU_STATIC_DATA;
    type SkillType = SayuSkillType;
    const SKILL: Self::SkillType = SAYU_SKILL;
    type DamageEnumType = SayuDamageEnum;
    type RoleEnum = SayuRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: SayuDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: SayuDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: SayuDamageEnum::Normal31 as usize, text: hit_n_dmg!(3, 1) },
            CharacterSkillMapItem { index: SayuDamageEnum::Normal32 as usize, text: hit_n_dmg!(3, 2) },
            CharacterSkillMapItem { index: SayuDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: SayuDamageEnum::Charged1 as usize, text: locale!(zh_cn: "重击循环伤害", en: "Charged Attack Spinning DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::Charged2 as usize, text: locale!(zh_cn: "重击终结伤害", en: "Charged Attack Final DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: SayuDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: SayuDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: SayuDamageEnum::E1 as usize, text: locale!(zh_cn: "风风轮伤害", en: "Fuufuu Windwheel DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E2 as usize, text: locale!(zh_cn: "风风轮舞踢点按伤害", en: "Fuufuu Whirlwind Kick DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E3 as usize, text: locale!(zh_cn: "风风轮舞踢长按伤害", en: "Fuufuu Whirlwind Kick Hold DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E4Pyro as usize, text: locale!(zh_cn: "风风轮附带火元素伤害", en: "Fuufuu Windwheel Pyro DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E4Hydro as usize, text: locale!(zh_cn: "风风轮附带水元素伤害", en: "Fuufuu Windwheel Hydro DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E4Cryo as usize, text: locale!(zh_cn: "风风轮附带冰元素伤害", en: "Fuufuu Windwheel Cryo DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E4Electro as usize, text: locale!(zh_cn: "风风轮附带雷元素伤害", en: "Fuufuu Windwheel Electro DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E5Pyro as usize, text: locale!(zh_cn: "风风轮舞踢长按附带火元素伤害", en: "Fuufuu Whirlwind Kick Pyro DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E5Hydro as usize, text: locale!(zh_cn: "风风轮舞踢长按附带水元素伤害", en: "Fuufuu Whirlwind Kick Hydro DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E5Cryo as usize, text: locale!(zh_cn: "风风轮舞踢长按附带冰元素伤害", en: "Fuufuu Whirlwind Kick Cryo DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::E5Electro as usize, text: locale!(zh_cn: "风风轮舞踢长按附带雷元素伤害", en: "Fuufuu Whirlwind Kick Electro DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: SayuDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能发动伤害", en: "Skill Activation DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::QHeal1 as usize, text: locale!(zh_cn: "技能发动治疗量", en: "Skill Activation Healing") },
            CharacterSkillMapItem { index: SayuDamageEnum::Q2 as usize, text: locale!(zh_cn: "不倒貉貉伤害", en: "Muji-Muji Daruma DMG") },
            CharacterSkillMapItem { index: SayuDamageEnum::QHeal2 as usize, text: locale!(zh_cn: "不倒貉貉治疗量", en: "Muji-Muji Daruma Healing") },
        ])
    };


    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: SayuDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use SayuDamageEnum::*;
        if s.is_heal() {
            let ratio = match s {
                QHeal1 => SAYU_SKILL.elemental_burst_heal1[s3],
                QHeal2 => SAYU_SKILL.elemental_burst_heal2[s3],
                _ => 0.0
            };
            let fixed = match s {
                QHeal1 => SAYU_SKILL.elemental_burst_heal1_fixed[s3],
                QHeal2 => SAYU_SKILL.elemental_burst_heal2_fixed[s3],
                _ => 0.0
            };
            let mut builder = D::new();
            builder.add_atk_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            if s == QHeal2 && context.character_common_data.constellation >= 6 {
                // let em = context.attribute.get_value(AttributeName::ElementalMastery);
                let em = context.attribute.get_em_all();
                let extra = (em * 3.0).min(6000.0);
                builder.add_extra_damage("6命：呼呼大睡时间", extra);
            }
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => SAYU_SKILL.normal_dmg1[s1],
                Normal2 => SAYU_SKILL.normal_dmg2[s1],
                Normal31 => SAYU_SKILL.normal_dmg31[s1],
                Normal32 => SAYU_SKILL.normal_dmg32[s1],
                Normal4 => SAYU_SKILL.normal_dmg4[s1],
                Charged1 => SAYU_SKILL.charged_dmg1[s1],
                Charged2 => SAYU_SKILL.charged_dmg2[s1],
                Plunging1 => SAYU_SKILL.plunging_dmg1[s1],
                Plunging2 => SAYU_SKILL.plunging_dmg2[s1],
                Plunging3 => SAYU_SKILL.plunging_dmg3[s1],
                E1 => SAYU_SKILL.elemental_skill_dmg1[s2],
                E2 => SAYU_SKILL.elemental_skill_dmg2[s2],
                E3 => SAYU_SKILL.elemental_skill_dmg3[s2],
                E4Cryo | E4Hydro | E4Electro | E4Pyro => SAYU_SKILL.elemental_skill_dmg4[s2],
                E5Cryo | E5Hydro | E5Electro | E5Pyro => SAYU_SKILL.elemental_skill_dmg5[s2],
                Q1 => SAYU_SKILL.elemental_burst_dmg1[s3],
                Q2 => SAYU_SKILL.elemental_burst_dmg2[s3],
                _ => 0.0
            };
            let mut builder = D::new();
            builder.add_atk_ratio("技能倍率", ratio);
            if s == Q2 && context.character_common_data.constellation >= 6 {
                // let em = context.attribute.get_value(AttributeName::ElementalMastery);
                let em = context.attribute.get_em_all();
                let bonus = (0.002 * em).min(4.0);
                builder.add_atk_ratio("6命：呼呼大睡时间", bonus);
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
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: SayuRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            SayuRoleEnum::Normal => Box::new(SayuDefaultTargetFunction {
                c6: c.constellation >= 6,
                recharge_demand: 1.4
            })
        }
    }
}
