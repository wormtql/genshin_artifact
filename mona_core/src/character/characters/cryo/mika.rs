use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeName};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct MikaSkillType {
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
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],

    pub elemental_burst_heal11: [f64; 15],
    pub elemental_burst_heal12: [f64; 15],
    pub elemental_burst_heal21: [f64; 15],
    pub elemental_burst_heal22: [f64; 15],
}

pub const MIKA_SKILL: MikaSkillType = MikaSkillType {
    normal_dmg1: [
        0.4326, 0.4678, 0.5031, 0.5534, 0.5886, 0.6288, 0.6842, 0.7395, 0.7948, 0.8552, 0.9156,
        0.9759, 1.0363, 1.0967, 1.157,
    ],
    normal_dmg2: [
        0.415, 0.4488, 0.4826, 0.5308, 0.5646, 0.6032, 0.6563, 0.7094, 0.7625, 0.8204, 0.8783,
        0.9362, 0.9941, 1.052, 1.1099,
    ],
    normal_dmg3: [
        0.545, 0.5894, 0.6338, 0.6971, 0.7415, 0.7922, 0.8619, 0.9316, 1.0013, 1.0774, 1.1534,
        1.2295, 1.3055, 1.3816, 1.4576,
    ],
    normal_dmg4: [
        0.2761, 0.2986, 0.3211, 0.3532, 0.3757, 0.4014, 0.4367, 0.472, 0.5073, 0.5459, 0.5844,
        0.6229, 0.6615, 0.7, 0.7385,
    ], // /2
    normal_dmg5: [
        0.7087, 0.7664, 0.8241, 0.9065, 0.9642, 1.0302, 1.1208, 1.2115, 1.3021, 1.401, 1.4999,
        1.5988, 1.6977, 1.7966, 1.8955,
    ],
    charged_dmg1: [
        1.1275, 1.2192, 1.311, 1.4421, 1.5339, 1.6387, 1.783, 1.9272, 2.0714, 2.2287, 2.386,
        2.5433, 2.7007, 2.858, 3.0153,
    ],
    plunging_dmg1: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    plunging_dmg2: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    plunging_dmg3: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.602, 3.8248, 4.0476, 4.2704,
    ],
    elemental_skill_dmg1: [
        0.672, 0.7224, 0.7728, 0.84, 0.8904, 0.9408, 1.008, 1.0752, 1.1424, 1.2096, 1.2768, 1.344,
        1.428, 1.512, 1.596,
    ],
    elemental_skill_dmg2: [
        0.84, 0.903, 0.966, 1.05, 1.113, 1.176, 1.26, 1.344, 1.428, 1.512, 1.596, 1.68, 1.785,
        1.89, 1.995,
    ],
    elemental_skill_dmg3: [
        0.252, 0.2709, 0.2898, 0.315, 0.3339, 0.3528, 0.378, 0.4032, 0.4284, 0.4536, 0.4788, 0.504,
        0.5355, 0.567, 0.5985,
    ],
    elemental_burst_heal11: [
        0.1217, 0.1308, 0.1399, 0.1521, 0.1612, 0.1704, 0.1825, 0.1947, 0.2069, 0.219, 0.2312,
        0.2434, 0.2586, 0.2738, 0.289,
    ],
    elemental_burst_heal12: [
        1172.04, 1289.26, 1416.24, 1553.0, 1699.52, 1855.82, 2021.88, 2197.71, 2383.31, 2578.67, 2783.81, 2998.71,
        3223.38, 3457.82, 3702.03,
    ],
    elemental_burst_heal21: [
        0.0243, 0.0261, 0.028, 0.0304, 0.0322, 0.034, 0.0365, 0.0389, 0.0413, 0.0438, 0.0462,
        0.0486, 0.0517, 0.0547, 0.0578,
    ],
    elemental_burst_heal22: [
        233.95, 257.35, 282.7, 310.0, 339.25, 370.45, 403.59, 438.69, 475.74, 514.74, 555.69, 598.58,
        643.43, 690.23, 738.98,
    ],
};

pub const MIKA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Mika,
    internal_name: "Mika",
    element: Element::Cryo,
    hp: [1049, 2694, 3477, 5208, 5765, 6631, 7373, 8239, 8796, 9661, 10217, 11083, 11640, 12506],
    atk: [19, 48, 62, 93, 103, 118, 131, 147, 157, 172, 182, 198, 208, 223],
    def: [60, 154, 198, 297, 329, 378, 420, 470, 502, 551, 583, 632, 664, 713],
    sub_stat: CharacterSubStatFamily::HP240,
    weapon_type: WeaponType::Polearm,
    star: 4,
    skill_name1: locale!(
        zh_cn: "普通攻击·西风枪术·镝传",
        en: "Normal Attack: Spear of Favonius – Arrow’s Passage",
    ),
    skill_name2: locale!(
        zh_cn: "星霜的流旋",
        en: "Starfrost Swirl",
    ),
    skill_name3: locale!(
        zh_cn: "苍翎的颂愿",
        en: "Skyfeather Song",
    ),
    name_locale: locale!(
        zh_cn: "米卡",
        en: "Mika",
    ),
};

// pub struct MikaEffect {
//     pub has_talent1: bool,
//     pub e_from_behind: bool,
// }

// impl MikaEffect {
//     pub fn new(common_data: &CharacterCommonData, config: &CharacterConfig) -> MikaEffect {
//         let e_from_behind = match config {
//             CharacterConfig::Mika { e_from_behind } => *e_from_behind,
//             _ => false,
//         };

//         MikaEffect {
//             has_talent1: common_data.has_talent1,
//             e_from_behind,
//         }
//     }
// }

// impl<T: Attribute> ChangeAttribute<T> for MikaEffect {
//     fn change_attribute(&self, attribute: &mut T) {
//         if self.has_talent1 && self.e_from_behind {
//             attribute.set_value_by(AttributeName::CriticalBase, "罗莎莉亚天赋：聆听忏悔的幽影", 0.12);
//         }
//     }
// }

pub struct Mika;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum MikaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal5,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    QHeal1,
    QHeal2,
}

impl MikaDamageEnum {
    pub fn is_heal(&self) -> bool {
        *self == MikaDamageEnum::QHeal1 || *self == MikaDamageEnum::QHeal2
    }

    pub fn get_element(&self) -> Element {
        use MikaDamageEnum::*;
        match *self {
            E1 | E2 | E3 => Element::Cryo,
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use MikaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            QHeal1 | QHeal2 => SkillType::ElementalBurst,
        }
    }
}

impl Into<usize> for MikaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum MikaRoleEnum {
    Physical
}

impl CharacterTrait for Mika {
    const STATIC_DATA: CharacterStaticData = MIKA_STATIC_DATA;
    type SkillType = MikaSkillType;
    const SKILL: Self::SkillType = MIKA_SKILL;
    type DamageEnumType = MikaDamageEnum;
    type RoleEnum = MikaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem {
                index: MikaDamageEnum::Normal1 as usize,
                text: hit_n_dmg!(1),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::Normal2 as usize,
                text: hit_n_dmg!(2),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::Normal3 as usize,
                text: hit_n_dmg!(3),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::Normal4 as usize,
                text: locale!(zh_cn: "四段伤害/2", en: "4-Hit DMG/2"),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::Normal5 as usize,
                text: hit_n_dmg!(5),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::Charged as usize,
                text: charged_dmg!(),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::Plunging1 as usize,
                text: plunging_dmg!(1),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::Plunging2 as usize,
                text: plunging_dmg!(2),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::Plunging3 as usize,
                text: plunging_dmg!(3),
            },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem {
                index: MikaDamageEnum::E1 as usize,
                text: locale!(zh_cn: "霜流矢伤害", en: "Flowfrost Arrow DMG"),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::E2 as usize,
                text: locale!(zh_cn: "冰星信标伤害", en: "Rimestar Flare"),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::E3 as usize,
                text: locale!(zh_cn: "冰星破片伤害", en: "Rimestar Shard DMG"),
            },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem {
                index: MikaDamageEnum::QHeal1 as usize,
                text: locale!(zh_cn: "施放回复量", en: "Regeneration Upon Use"),
            },
            CharacterSkillMapItem {
                index: MikaDamageEnum::QHeal2 as usize,
                text: locale!(zh_cn: "鹰翎回复量", en: "Eagleplume Regeneration"),
            },
        ]),
    };

    // #[cfg(not(target_family = "wasm"))]
    // const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    //     ItemConfig {
    //         name: "e_from_behind",
    //         title: locale!(
    //             zh_cn: "应用天赋「聆听忏悔的幽影」效果",
    //             en: "Use Talent「Regina Probationum」"
    //         ),
    //         config: ItemConfigType::Bool { default: true },
    //     }
    // ]);

    fn damage_internal<D: DamageBuilder>(
        context: &DamageContext<'_, D::AttributeType>,
        s: usize,
        config: &CharacterSkillConfig,
        fumo: Option<Element>,
    ) -> D::Result {
        let s: MikaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use MikaDamageEnum::*;
        if s.is_heal() {
            let ratio = match s {
                QHeal1 => MIKA_SKILL.elemental_burst_heal11[s3],
                QHeal2 => MIKA_SKILL.elemental_burst_heal21[s3],
                _ => 0.0
            };
            let fixed = match s {
                QHeal1 => MIKA_SKILL.elemental_burst_heal12[s3],
                QHeal2 => MIKA_SKILL.elemental_burst_heal22[s3],
                _ => 0.0
            };
            let mut builder = D::new();
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => MIKA_SKILL.normal_dmg1[s1],
                Normal2 => MIKA_SKILL.normal_dmg2[s1],
                Normal3 => MIKA_SKILL.normal_dmg3[s1],
                Normal4 => MIKA_SKILL.normal_dmg4[s1],
                Normal5 => MIKA_SKILL.normal_dmg5[s1],
                Charged => MIKA_SKILL.charged_dmg1[s1],
                Plunging1 => MIKA_SKILL.plunging_dmg1[s1],
                Plunging2 => MIKA_SKILL.plunging_dmg2[s1],
                Plunging3 => MIKA_SKILL.plunging_dmg3[s1],
                E1 => MIKA_SKILL.elemental_skill_dmg1[s2],
                E2 => MIKA_SKILL.elemental_skill_dmg2[s2],
                E3 => MIKA_SKILL.elemental_skill_dmg3[s2],
                QHeal1 | QHeal2 => 0.0,
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
    }

    fn new_effect<A: Attribute>(
        common_data: &CharacterCommonData,
        config: &CharacterConfig,
    ) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(
        role_index: usize,
        _team: &TeamQuantization,
        _c: &CharacterCommonData,
        _w: &WeaponCommonData,
    ) -> Box<dyn TargetFunction> {
        // let role: MikaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        // match role {
        //     MikaRoleEnum::Freezing => Box::new(MikaDefaultTargetFunction::default())
        // }
        unimplemented!()
    }
}
