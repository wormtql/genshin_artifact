use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::geo::albedo::AlbedoRoleEnum;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use strum::EnumCount;
use std::str::FromStr;
use crate::common::i18n::{hit_n_dmg, locale, plunging_dmg};

pub struct AetherAnemoSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_dmg4: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

const AETHER_ANEMO_SKILL: AetherAnemoSkillType = AetherAnemoSkillType {
    normal_dmg1: [0.4446, 0.4808, 0.517, 0.5687, 0.6049, 0.6463, 0.7031, 0.76, 0.8169, 0.8789, 0.9409, 1.003, 1.065, 1.1271, 1.1891],
    normal_dmg2: [0.4343, 0.4697, 0.505, 0.5555, 0.5909, 0.6313, 0.6868, 0.7423, 0.7979, 0.8585, 0.9191, 0.9797, 1.0403, 1.1009, 1.1615],
    normal_dmg3: [0.5298, 0.5729, 0.616, 0.6776, 0.7207, 0.77, 0.8378, 0.9055, 0.9733, 1.0472, 1.1211, 1.195, 1.269, 1.3429, 1.4168],
    normal_dmg4: [0.5831, 0.6305, 0.678, 0.7458, 0.7933, 0.8475, 0.9221, 0.9967, 1.0712, 1.1526, 1.234, 1.3153, 1.3967, 1.478, 1.5594],
    normal_dmg5: [0.7078, 0.7654, 0.823, 0.9053, 0.9629, 1.0288, 1.1193, 1.2098, 1.3003, 1.3991, 1.4979, 1.5966, 1.6954, 1.7941, 1.8929],
    charged_dmg11: [0.559, 0.6045, 0.65, 0.715, 0.7605, 0.8125, 0.884, 0.9555, 1.027, 1.105, 1.183, 1.261, 1.339, 1.417, 1.495],
    charged_dmg12: [0.6072, 0.6566, 0.706, 0.7766, 0.826, 0.8825, 0.9602, 1.0378, 1.1155, 1.2002, 1.2849, 1.3696, 1.4544, 1.5391, 1.6238],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [0.12, 0.129, 0.138, 0.15, 0.159, 0.168, 0.18, 0.192, 0.204, 0.216, 0.228, 0.24, 0.255, 0.27, 0.285],
    elemental_skill_dmg2: [0.168, 0.1806, 0.1932, 0.21, 0.2226, 0.2352, 0.252, 0.2688, 0.2856, 0.3024, 0.3192, 0.336, 0.357, 0.378, 0.399],
    elemental_skill_dmg3: [1.76, 1.892, 2.024, 2.2, 2.332, 2.464, 2.64, 2.816, 2.992, 3.168, 3.344, 3.52, 3.74, 3.96, 4.18],
    elemental_skill_dmg4: [1.92, 2.064, 2.208, 2.4, 2.544, 2.688, 2.88, 3.072, 3.264, 3.456, 3.648, 3.84, 4.08, 4.32, 4.56],
    elemental_burst_dmg1: [0.808, 0.8686, 0.9292, 1.01, 1.0706, 1.1312, 1.212, 1.2928, 1.3736, 1.4544, 1.5352, 1.616, 1.717, 1.818, 1.919],
    elemental_burst_dmg2: [0.248, 0.2666, 0.2852, 0.31, 0.3286, 0.3472, 0.372, 0.3968, 0.4216, 0.4464, 0.4712, 0.496, 0.527, 0.558, 0.589]
};

pub struct AetherAnemoEffect {
    pub c2: bool
}

impl<A: Attribute> ChangeAttribute<A> for AetherAnemoEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.c2 {
            attribute.set_value_by(AttributeName::Recharge, "命座2革新的旋风", 0.16);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(FromPrimitive, EnumCountMacro, EnumString)]
pub enum AetherAnemoDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Charged11,
    Charged12,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    E4,
    Q1,
    Q2Pyro,
    Q2Cryo,
    Q2Electro,
    Q2Hydro
}

impl Into<usize> for AetherAnemoDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl AetherAnemoDamageEnum {
    pub fn get_element(&self) -> Element {
        use AetherAnemoDamageEnum::*;

        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 | Charged11 | Charged12 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            E1 | E2 | E3 | E4 | Q1 => Element::Anemo,
            Q2Pyro => Element::Pyro,
            Q2Electro => Element::Electro,
            Q2Cryo => Element::Cryo,
            Q2Hydro => Element::Hydro
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AetherAnemoDamageEnum::*;

        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4 => SkillType::ElementalSkill,
            Q1 | Q2Hydro | Q2Cryo | Q2Electro | Q2Pyro => SkillType::ElementalBurst
        }
    }
}

pub enum AetherAnemoRoleEnum {
    Sub
}

pub struct AetherAnemo;

impl CharacterTrait for AetherAnemo {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::AetherAnemo,
        internal_name: "PlayerBoy",
        element: Element::Anemo,
        hp: [912, 2342, 3024, 4529, 5031, 5766, 6411, 7164, 7648, 8401, 8885, 9638, 10122, 10875],
        atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212],
        def: [57, 147, 190, 284, 315, 362, 402, 450, 480, 527, 558, 605, 635, 683],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·异邦铁风",
            en: "Normal Attack: Foreign Ironwind",
        ),
        skill_name2: locale!(
            zh_cn: "风涡剑",
            en: "Palm Vortex",
        ),
        skill_name3: locale!(
            zh_cn: "风息激荡",
            en: "Gust Surge",
        ),
        name_locale: locale!(
            zh_cn: "空-风",
            en: "Aether(Anemo)",
        )
    };
    type SkillType = AetherAnemoSkillType;
    const SKILL: Self::SkillType = AETHER_ANEMO_SKILL;
    type DamageEnumType = AetherAnemoDamageEnum;
    type RoleEnum = AlbedoRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Charged11 as usize, text: locale!(zh_cn: "重击伤害-1", en: "Charged Attack-1") },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Charged12 as usize, text: locale!(zh_cn: "重击伤害-2", en: "Charged Attack-2") },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::E1 as usize, text: locale!(zh_cn: "初始切割伤害", en: "Initial Cutting DMG") },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::E2 as usize, text: locale!(zh_cn: "最大切割伤害", en: "Max Cutting DMG") },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::E3 as usize, text: locale!(zh_cn: "初始爆风伤害", en: "Initial Storm DMG") },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::E4 as usize, text: locale!(zh_cn: "最大爆风伤害", en: "Max Storm DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Q1 as usize, text: locale!(zh_cn: "龙卷风伤害", en: "Tornado DMG") },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Q2Pyro as usize, text: locale!(zh_cn: "附加火元素伤害", en: "Additional Pyro DMG") },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Q2Hydro as usize, text: locale!(zh_cn: "附加水元素伤害", en: "Additional Hydro DMG") },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Q2Electro as usize, text: locale!(zh_cn: "附加雷元素伤害", en: "Additional Electro DMG") },
            CharacterSkillMapItem { index: AetherAnemoDamageEnum::Q2Cryo as usize, text: locale!(zh_cn: "附加冰元素伤害", en: "Additional Cryo DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: AetherAnemoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use AetherAnemoDamageEnum::*;
        let ratio = match s {
            Normal1 => AETHER_ANEMO_SKILL.normal_dmg1[s1],
            Normal2 => AETHER_ANEMO_SKILL.normal_dmg2[s1],
            Normal3 => AETHER_ANEMO_SKILL.normal_dmg3[s1],
            Normal4 => AETHER_ANEMO_SKILL.normal_dmg4[s1],
            Normal5 => AETHER_ANEMO_SKILL.normal_dmg5[s1],
            Charged11 => AETHER_ANEMO_SKILL.charged_dmg11[s1],
            Charged12 => AETHER_ANEMO_SKILL.charged_dmg12[s1],
            Plunging1 => AETHER_ANEMO_SKILL.plunging_dmg1[s1],
            Plunging2 => AETHER_ANEMO_SKILL.plunging_dmg2[s1],
            Plunging3 => AETHER_ANEMO_SKILL.plunging_dmg3[s1],
            E1 => AETHER_ANEMO_SKILL.elemental_skill_dmg1[s2],
            E2 => AETHER_ANEMO_SKILL.elemental_skill_dmg2[s2],
            E3 => AETHER_ANEMO_SKILL.elemental_skill_dmg3[s2],
            E4 => AETHER_ANEMO_SKILL.elemental_skill_dmg4[s2],
            Q1 => AETHER_ANEMO_SKILL.elemental_burst_dmg1[s3],
            Q2Cryo | Q2Electro | Q2Pyro | Q2Hydro => AETHER_ANEMO_SKILL.elemental_burst_dmg2[s3]
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

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(AetherAnemoEffect {
            c2: common_data.constellation >= 2
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
