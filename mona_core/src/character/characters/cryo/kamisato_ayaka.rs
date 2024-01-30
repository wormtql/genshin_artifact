use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::cryo::kamisato_ayaka_default::KamisatoAyakaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{hit_n_dmg, locale, plunging_dmg};

pub struct KamisatoAyakaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const KAMISATO_AYAKA_SKILL: KamisatoAyakaSkillType = KamisatoAyakaSkillType {
    normal_dmg1: [0.4573, 0.4945, 0.5317, 0.5849, 0.6221, 0.6646, 0.7231, 0.7816, 0.8401, 0.9039, 0.9677, 1.0315, 1.0953, 1.1591, 1.2229],
    normal_dmg2: [0.4868, 0.5265, 0.5661, 0.6227, 0.6623, 0.7076, 0.7699, 0.8322, 0.8944, 0.9624, 1.0303, 1.0982, 1.1662, 1.2341, 1.302],
    normal_dmg3: [0.6262, 0.6772, 0.7282, 0.801, 0.8519, 0.9102, 0.9903, 1.0704, 1.1505, 1.2379, 1.3253, 1.4126, 1.5, 1.5874, 1.6748],
    normal_dmg4: [0.2265, 0.2449, 0.2633, 0.2897, 0.3081, 0.3292, 0.3581, 0.3871, 0.4161, 0.4477, 0.4793, 0.5109, 0.5425, 0.5741, 0.6057],
    normal_dmg5: [0.7818, 0.8455, 0.9091, 1., 1.0636, 1.1364, 1.2364, 1.3364, 1.4364, 1.5455, 1.6545, 1.7636, 1.8727, 1.9818, 2.0909],
    charged_dmg1: [0.5513, 0.5961, 0.641, 0.7051, 0.75, 0.8013, 0.8718, 0.9423, 1.0128, 1.0897, 1.1666, 1.2435, 1.3205, 1.3974, 1.4743],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [2.392, 2.5714, 2.7508, 2.99, 3.1694, 3.3488, 3.588, 3.8272, 4.0664, 4.3056, 4.5448, 4.784, 5.083, 5.382, 5.681],
    elemental_burst_dmg1: [1.123, 1.2072, 1.2915, 1.4038, 1.488, 1.5722, 1.6845, 1.7968, 1.9091, 2.0214, 2.1337, 2.246, 2.3864, 2.5268, 2.6671],
    elemental_burst_dmg2: [1.6845, 1.8108, 1.9372, 2.1056, 2.232, 2.3583, 2.5268, 2.6952, 2.8636, 3.0321, 3.2005, 3.369, 3.5796, 3.7901, 4.0007],
};

pub const KAMISATO_AYAKA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::KamisatoAyaka,
    internal_name: "Ayaka",
    element: Element::Cryo,
    hp: [1011, 2597, 3455, 5170, 5779, 6649, 7462, 8341, 8951, 9838, 10448, 11345, 11954, 12858],
    atk: [27, 69, 92, 138, 154, 177, 198, 222, 238, 262, 278, 302, 318, 342],
    def: [61, 158, 211, 315, 352, 405, 455, 509, 546, 600, 637, 692, 729, 784],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·神里流·倾",
        en: "Normal Attack: Kamisato Art - Kabuki",
    ),
    skill_name2: locale!(
        zh_cn: "神里流·冰华",
        en: "Kamisato Art: Hyouka",
    ),
    skill_name3: locale!(
        zh_cn: "神里流·霜灭",
        en: "Kamisato Art: Soumetsu",
    ),
    name_locale: locale!(
        zh_cn: "神里绫华",
        en: "Kamisato Ayaka",
    )
};

pub struct KamisatoAyakaEffect {
    pub has_talent1: bool,
    pub has_talent2: bool,
    pub rate1: f64,
    pub rate2: f64
}

impl KamisatoAyakaEffect {
    pub fn new(common_data: &CharacterCommonData, config: &CharacterConfig) -> Self {
        let (r1, r2) = match *config {
            CharacterConfig::KamisatoAyaka { talent1_rate, talent2_rate } => (talent1_rate, talent2_rate),
            _ => (0.0, 0.0)
        };
        KamisatoAyakaEffect {
            has_talent1: common_data.has_talent1,
            has_talent2: common_data.has_talent2,
            rate1: r1,
            rate2: r2
        }
    }
}

impl<A: Attribute> ChangeAttribute<A> for KamisatoAyakaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent1 {
            attribute.set_value_by(AttributeName::BonusNormalAttack, "神里绫华天赋：天罪国罪镇词", 0.3 * self.rate1);
            attribute.set_value_by(AttributeName::BonusChargedAttack, "神里绫华天赋：天罪国罪镇词", 0.3 * self.rate1);
        }
        if self.has_talent2 {
            attribute.set_value_by(AttributeName::BonusCryo, "神里绫华天赋：寒天宣命祝词", 0.18 * self.rate2);
        }
    }
}

pub struct KamisatoAyaka;

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum KamisatoAyakaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal4Times3,
    Normal5,
    Charged,
    ChargedTimes3,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    Q1,
    Q2
}

impl Into<usize> for KamisatoAyakaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl KamisatoAyakaDamageEnum {
    pub fn get_element(&self, after_dash: bool) -> Element {
        if after_dash {
            Element::Cryo
        } else {
            use KamisatoAyakaDamageEnum::*;
            match *self {
                E1 | Q1 | Q2 => Element::Cryo,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use KamisatoAyakaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal4Times3 | Normal5 => SkillType::NormalAttack,
            Charged | ChargedTimes3 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum KamisatoAyakaRoleEnum {
    Main
}

impl CharacterTrait for KamisatoAyaka {
    const STATIC_DATA: CharacterStaticData = KAMISATO_AYAKA_STATIC_DATA;
    type SkillType = KamisatoAyakaSkillType;
    const SKILL: Self::SkillType = KAMISATO_AYAKA_SKILL;
    type DamageEnumType = KamisatoAyakaDamageEnum;
    type RoleEnum = KamisatoAyakaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Normal4 as usize, text: locale!(zh_cn: "四段伤害/3", en: "4-Hit DMG/3") },
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Charged as usize, text: locale!(zh_cn: "重击伤害/3", en: "Charged Attack DMG/3") },
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Q1 as usize, text: locale!(zh_cn: "切割伤害", en: "Cutting DMG") },
            CharacterSkillMapItem { index: KamisatoAyakaDamageEnum::Q2 as usize, text: locale!(zh_cn: "绽放伤害", en: "Bloom DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_rate",
            title: locale!(
                zh_cn: "天赋「天罪国罪镇词」应用比例",
                en: "Talent「Amatsumi Kunitsumi Sanctification」Apply Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "talent2_rate",
            title: locale!(
                zh_cn: "天赋「寒天宣命祝词」应用比例",
                en: "Talent「Kanten Senmyou Blessing」Apply Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_dash",
            title: locale!(
                zh_cn: "神里流·霰步",
                en: "Kamisato Art: Senho",
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "use_c6",
            title: locale!(
                zh_cn: "六命效果",
                en: "C6 Effect",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KamisatoAyakaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KamisatoAyakaDamageEnum::*;
        let ratio = match s {
            Normal1 => KAMISATO_AYAKA_SKILL.normal_dmg1[s1],
            Normal2 => KAMISATO_AYAKA_SKILL.normal_dmg2[s1],
            Normal3 => KAMISATO_AYAKA_SKILL.normal_dmg3[s1],
            Normal4 => KAMISATO_AYAKA_SKILL.normal_dmg4[s1],
            Normal4Times3 => KAMISATO_AYAKA_SKILL.normal_dmg4[s1] * 3.0,
            Normal5 => KAMISATO_AYAKA_SKILL.normal_dmg5[s1],
            Charged => KAMISATO_AYAKA_SKILL.charged_dmg1[s1],
            ChargedTimes3 => KAMISATO_AYAKA_SKILL.charged_dmg1[s1] * 3.0,
            Plunging1 => KAMISATO_AYAKA_SKILL.plunging_dmg1[s1],
            Plunging2 => KAMISATO_AYAKA_SKILL.plunging_dmg2[s1],
            Plunging3 => KAMISATO_AYAKA_SKILL.plunging_dmg3[s1],
            E1 => KAMISATO_AYAKA_SKILL.elemental_skill_dmg1[s2],
            Q1 => KAMISATO_AYAKA_SKILL.elemental_burst_dmg1[s3],
            Q2 => KAMISATO_AYAKA_SKILL.elemental_burst_dmg2[s3]
        };

        let (after_dash, use_c6) = match *config {
            CharacterSkillConfig::KamisatoAyaka { after_dash, use_c6 } => (after_dash, use_c6),
            _ => (false, false)
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let skill_type = s.get_skill_type();
        if skill_type == SkillType::ChargedAttack && use_c6 {
            builder.add_extra_bonus("绫华六命：间水月", 2.98);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(after_dash),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(KamisatoAyakaEffect::new(common_data, config)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: KamisatoAyakaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            KamisatoAyakaRoleEnum::Main => Box::new(KamisatoAyakaDefaultTargetFunction {
                recharge_demand: 1.0
            })
        }
    }
}
