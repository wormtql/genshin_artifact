use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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
use crate::target_functions::target_functions::KamisatoAyatoDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct KamisatoAyatoSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_bonus: [f64; 15],
    pub elemental_skill_dmg4: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_bonus: [f64; 15]
}

pub const KAMISATO_AYATO_SKILL: KamisatoAyatoSkillType = KamisatoAyatoSkillType {
    normal_dmg1: [0.4496, 0.4862, 0.5228, 0.5751, 0.6117, 0.6535, 0.711, 0.7685, 0.826, 0.8888, 0.9515, 1.0143, 1.077, 1.1397, 1.2025],
    normal_dmg2: [0.4716, 0.51, 0.5483, 0.6032, 0.6416, 0.6854, 0.7457, 0.8061, 0.8664, 0.9322, 0.998, 1.0638, 1.1296, 1.1954, 1.2612],
    normal_dmg3: [0.5861, 0.6338, 0.6815, 0.7497, 0.7974, 0.8519, 0.9269, 1.0019, 1.0768, 1.1586, 1.2404, 1.3222, 1.404, 1.4858, 1.5675],
    normal_dmg4: [0.2945, 0.3185, 0.3424, 0.3767, 0.4006, 0.428, 0.4657, 0.5034, 0.541, 0.5821, 0.6232, 0.6643, 0.7054, 0.7465, 0.7876],
    normal_dmg5: [0.756, 0.8176, 0.8791, 0.967, 1.0286, 1.0989, 1.1956, 1.2923, 1.389, 1.4945, 1.6, 1.7055, 1.811, 1.9165, 2.022],
    charged_dmg: [1.2953, 1.4007, 1.5062, 1.6568, 1.7622, 1.8827, 2.0484, 2.2141, 2.3797, 2.5605, 2.7412, 2.9219, 3.1027, 3.2834, 3.4642],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [0.5289, 0.572, 0.615, 0.6765, 0.7196, 0.7688, 0.8364, 0.9041, 0.9717, 1.0455, 1.1193, 1.1931, 1.2669, 1.3407, 1.4145],
    elemental_skill_dmg2: [0.5891, 0.6371, 0.685, 0.7535, 0.8015, 0.8563, 0.9316, 1.007, 1.0823, 1.1645, 1.2467, 1.3289, 1.4111, 1.4933, 1.5755],
    elemental_skill_dmg3: [0.6493, 0.7022, 0.755, 0.8305, 0.8834, 0.9438, 1.0268, 1.1099, 1.1929, 1.2835, 1.3741, 1.4647, 1.5553, 1.6459, 1.7365],
    elemental_skill_bonus: [0.0056, 0.0061, 0.0065, 0.0072, 0.0076, 0.0082, 0.0089, 0.0096, 0.0103, 0.0111, 0.0119, 0.0127, 0.0134, 0.0142, 0.015],
    elemental_skill_dmg4: [1.0148, 1.0974, 1.18, 1.298, 1.3806, 1.475, 1.6048, 1.7346, 1.8644, 2.006, 2.1476, 2.2892, 2.4308, 2.5724, 2.714],
    elemental_burst_dmg1: [0.6646, 0.7144, 0.7642, 0.8307, 0.8805, 0.9304, 0.9968, 1.0633, 1.1298, 1.1962, 1.2627, 1.3291, 1.4122, 1.4953, 1.5783],
    elemental_burst_bonus: [0.11, 0.12, 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2]
};

// pub struct KamisatoAyatoEffect {
//     pub use_c2: bool
// }
//
// impl<A: Attribute> ChangeAttribute<A> for KamisatoAyatoEffect {
//     fn change_attribute(&self, attribute: &mut A) {
//         if self.use_c2 {
//             attribute.add_edge1(
//                 AttributeName::HPPercentage,
//                 AttributeName::HP,
//                 Box::new(|x, _| x * 0.5),
//                 Box::new(|grad, x1, x2| (grad * 0.5, 0.0)),
//                 "神里绫人二命「世有源泉」"
//             );
//             attribute.add_edge1(
//                 AttributeName::HPFixed,
//                 AttributeName::HP,
//                 Box::new(|x, _| x * 0.5),
//                 Box::new(|grad, x1, x2| (grad * 0.5, 0.0)),
//                 "神里绫人二命「世有源泉」"
//             );
//             attribute.add_edge1(
//                 AttributeName::HPBase,
//                 AttributeName::HP,
//                 Box::new(|x, _| x * 0.5),
//                 Box::new(|grad, x1, x2| (grad * 0.5, 0.0)),
//                 "神里绫人二命「世有源泉」"
//             );
//         }
//     }
// }

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum KamisatoAyatoDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4Div2,
    Normal5,
    Charged1,
    Plunging1,
    Plunging2,
    Plunging3,
    ENormal1,
    ENormal2,
    ENormal3,
    E4,
    Q1
}

impl Into<usize> for KamisatoAyatoDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl KamisatoAyatoDamageEnum {
    fn get_element(&self) -> Element {
        use KamisatoAyatoDamageEnum::*;
        match *self {
            ENormal1 | ENormal2 | ENormal3 | E4 | Q1 => Element::Hydro,
            _ => Element::Physical
        }
    }

    fn is_e_normal(&self) -> bool {
        use KamisatoAyatoDamageEnum::*;
        match *self {
            ENormal1 | ENormal2 | ENormal3 => true,
            _ => false
        }
    }

    fn get_skill_type(&self) -> SkillType {
        use KamisatoAyatoDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4Div2 | Normal5 | ENormal1 | ENormal2 | ENormal3 => SkillType::NormalAttack,
            Charged1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E4 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum KamisatoAyatoRoleEnum {
    Main
}

pub struct KamisatoAyato;

impl CharacterTrait for KamisatoAyato {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::KamisatoAyato,
        internal_name: "Ayato",
        element: Element::Hydro,
        hp: [1068, 2770, 3685, 5514, 6165, 7092, 7960, 8897, 9548, 10494, 11144, 12101, 12751, 13715],
        atk: [23, 60, 80, 120, 134, 155, 174, 194, 208, 229, 243, 264, 278, 299],
        def: [60, 155, 206, 309, 345, 397, 446, 499, 535, 588, 624, 678, 715, 769],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·神里流·转",
            en: "Normal Attack: Kamisato Art - Marobashi",
        ),
        skill_name2: locale!(
            zh_cn: "神里流·镜花",
            en: "Kamisato Art: Kyouka",
        ),
        skill_name3: locale!(
            zh_cn: "神里流·水囿",
            en: "Kamisato Art: Suiyuu",
        ),
        name_locale: locale!(
            zh_cn: "神里绫人",
            en: "Kamisato Ayato",
        )
    };

    type SkillType = KamisatoAyatoSkillType;
    const SKILL: Self::SkillType = KAMISATO_AYATO_SKILL;
    type DamageEnumType = KamisatoAyatoDamageEnum;
    type RoleEnum = KamisatoAyatoRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Normal4Div2 as usize, text: locale!(zh_cn: "四段伤害", en: "4-Hit DMG/2") },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Charged1 as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::ENormal1 as usize, text: locale!(zh_cn: "一段瞬水剑伤害", en: "Shunsuiken 1-Hit DMG") },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::ENormal2 as usize, text: locale!(zh_cn: "二段瞬水剑伤害", en: "Shunsuiken 2-Hit DMG") },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::ENormal3 as usize, text: locale!(zh_cn: "三段瞬水剑伤害", en: "Shunsuiken 3-Hit DMG") },
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::E4 as usize, text: locale!(zh_cn: "水影伤害", en: "Water Illusion DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: KamisatoAyatoDamageEnum::Q1 as usize, text: locale!(zh_cn: "水花剑伤害", en: "Bloomwater Blade DMG") },
        ])
    };

    // #[cfg(not(target_family = "wasm"))]
    // const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    //     ItemConfig {
    //         name: "use_c2",
    //         title: "二命效果",
    //         config: ItemConfigType::Bool { default: false }
    //     }
    // ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_stack",
            title: locale!(
                zh_cn: "「浪闪」层数",
                en: "「Namisen」Stack",
            ),
            config: ItemConfigType::Int { min: 0, max: 5, default: 4 }
        },
        ItemConfig {
            name: "in_q",
            title: locale!(
                zh_cn: "处于「水囿」",
                en: "Under「Suiyuu」"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KamisatoAyatoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KamisatoAyatoDamageEnum::*;
        let ratio = match s {
            Normal1 => KAMISATO_AYATO_SKILL.normal_dmg1[s1],
            Normal2 => KAMISATO_AYATO_SKILL.normal_dmg2[s1],
            Normal3 => KAMISATO_AYATO_SKILL.normal_dmg3[s1],
            Normal4Div2 => KAMISATO_AYATO_SKILL.normal_dmg4[s1],
            Normal5 => KAMISATO_AYATO_SKILL.normal_dmg5[s1],
            Charged1 => KAMISATO_AYATO_SKILL.charged_dmg[s1],
            Plunging1 => KAMISATO_AYATO_SKILL.plunging_dmg1[s1],
            Plunging2 => KAMISATO_AYATO_SKILL.plunging_dmg2[s1],
            Plunging3 => KAMISATO_AYATO_SKILL.plunging_dmg3[s1],
            ENormal1 => KAMISATO_AYATO_SKILL.elemental_skill_dmg1[s2],
            ENormal2 => KAMISATO_AYATO_SKILL.elemental_skill_dmg2[s2],
            ENormal3 => KAMISATO_AYATO_SKILL.elemental_skill_dmg3[s2],
            E4 => KAMISATO_AYATO_SKILL.elemental_skill_dmg4[s2],
            Q1 => KAMISATO_AYATO_SKILL.elemental_burst_dmg1[s3],
        };

        let (e_stack, in_q) = match *config {
            CharacterSkillConfig::KamisatoAyato { e_stack, in_q } => (e_stack, in_q),
            _ => (0, false)
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        if s.is_e_normal() {
            let hp = context.attribute.get_hp();
            let extra = hp * (e_stack as f64 * KAMISATO_AYATO_SKILL.elemental_skill_bonus[s2]);
            builder.add_extra_damage("「浪闪」伤害提高", extra);
        }

        let skill_type = s.get_skill_type();
        if skill_type == SkillType::NormalAttack && in_q {
            builder.add_extra_bonus("Q技能伤害提高", KAMISATO_AYATO_SKILL.elemental_burst_bonus[s3]);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        // let use_c2 = match *config {
        //     CharacterConfig::KamisatoAyato { use_c2 } => use_c2,
        //     _ => false
        // };
        //
        // Some(Box::new(KamisatoAyatoEffect {
        //     use_c2
        // }))
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: KamisatoAyatoRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            KamisatoAyatoRoleEnum::Main => Box::new(KamisatoAyatoDefaultTargetFunction)
        }
    }
}
