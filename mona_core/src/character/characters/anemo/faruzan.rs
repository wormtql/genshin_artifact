use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait, CharacterSkillMapItem};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::macros::{skill_type, damage_enum, skill_map, damage_ratio};
pub struct FaruzanSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged1_dmg: [f64; 15],
    pub charged2_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],
    pub e_collapse_dmg: [f64; 15],

    pub q_dmg: [f64; 15],
    pub q_bonus: [f64; 15],
}

pub const FARUZAN_SKILL: FaruzanSkillType = FaruzanSkillType {
    normal_dmg1: [0.4473,0.4837,0.5201,0.5721,0.6085,0.6501,0.7074,0.7646,0.8218,0.8842,0.9466,1.009,1.0714,1.1338,1.1963],
    normal_dmg2: [0.4219,0.4562,0.4905,0.5396,0.5739,0.6132,0.6671,0.7211,0.7751,0.8339,0.8928,0.9516,1.0105,1.0694,1.1282],
    normal_dmg3: [0.5316,0.5749,0.6182,0.68,0.7233,0.7727,0.8407,0.9087,0.9767,1.0509,1.1251,1.1993,1.2735,1.3476,1.4218],
    normal_dmg4: [0.7062,0.7637,0.8212,0.9033,0.9608,1.0265,1.1168,1.2071,1.2974,1.396,1.4945,1.5931,1.6916,1.7902,1.8887],
    charged1_dmg: [0.4386,0.4743,0.51,0.561,0.5967,0.6375,0.6936,0.7497,0.8058,0.867,0.9282,0.9894,1.0506,1.1118,1.173],
    charged2_dmg: [1.24,1.333,1.426,1.55,1.643,1.736,1.86,1.984,2.108,2.232,2.356,2.48,2.635,2.79,2.945],
    plunging_dmg1: [0.5683,0.6145,0.6608,0.7269,0.7731,0.826,0.8987,0.9714,1.0441,1.1234,1.2027,1.282,1.3612,1.4405,1.5198],
    plunging_dmg2: [1.1363,1.2288,1.3213,1.4535,1.5459,1.6516,1.797,1.9423,2.0877,2.2462,2.4048,2.5634,2.7219,2.8805,3.039],
    plunging_dmg3: [1.4193,1.5349,1.6504,1.8154,1.931,2.063,2.2445,2.4261,2.6076,2.8057,3.0037,3.2018,3.3998,3.5979,3.7959],
    e_dmg: [1.488,1.5996,1.7112,1.86,1.9716,2.0832,2.232,2.3808,2.5296,2.6784,2.8272,2.976,3.162,3.348,3.534],
    e_collapse_dmg: [0.4175,0.4425,0.4675,0.5,0.525,0.55,0.5825,0.615,0.6475,0.68,0.7125,0.745,0.7775,0.81,0.8425],
    q_dmg: [3.776,4.0592,4.3424,4.72,5.0032,5.2864,5.664,6.0416,6.4192,6.7968,7.1744,7.552,8.024,8.496,8.968],
    q_bonus: [0.18,0.1935,0.207,0.225,0.2385,0.252,0.27,0.288,0.306,0.324,0.342,0.36,0.3825,0.405,0.4275],
};

damage_enum!(
    FaruzanDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged1
    Charged2
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    Q1
);

impl FaruzanDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use FaruzanDamageEnum::*;
        match *self {
            E1 | E2 => SkillType::ElementalSkill, 
            Plunging1 | Plunging2 | Plunging3 => SkillType::PlungingAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Q1 => SkillType::ElementalBurst,
        }
    }

    pub fn get_skill_element(&self) -> Element {
        use FaruzanDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            Charged2 | E1 | E2 | Q1 => Element::Anemo,
        }
    }
}

// pub struct FaruzanEffect {
//     pub talent1: bool,
//     pub e_pyro: bool,
//     pub e_cryo: bool,
// }

// impl<A: Attribute> ChangeAttribute<A> for FaruzanEffect {
//     fn change_attribute(&self, attribute: &mut A) {
//         if self.talent1 {
//             if self.e_pyro {
//                 attribute.set_value_by(AttributeName::ATKPercentage, "天赋「拾玉得花」染火加成", 0.3);
//             }
//             if self.e_cryo {
//                 attribute.set_value_by(AttributeName::CriticalBase, "天赋「拾玉得花」染冰加成", 0.2);
//             }
//         }
//     }
// }

pub struct Faruzan;

impl CharacterTrait for Faruzan {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Faruzan,
        internal_name: "Faruzan", // todo
        chs: "法露珊",
        element: Element::Anemo,
        hp: [802,2061,2661,3985,4411,5074,5642,6305,6731,7393,7819,8481,8907,9570],
        atk: [16,42,55,82,91,104,116,129,138,152,161,174,183,196],
        def: [53,135,175,262,289,333,370,414,442,485,513,556,584,628],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Bow,
        star: 4,
        skill_name1: "普通攻击·迴身箭术",
        skill_name2: "非想风天",
        skill_name3: "抟风秘道",
    };
    type SkillType = FaruzanSkillType;
    const SKILL: Self::SkillType = FARUZAN_SKILL;
    type DamageEnumType = FaruzanDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            FaruzanDamageEnum
            Normal1 "一段伤害"
            Normal2 "二段伤害"
            Normal3 "三段伤害"
            Normal4 "四段伤害"
            Charged1 "重击伤害"
            Charged1 "满蓄力瞄准伤害"
            Plunging1 "下坠期间伤害"
            Plunging2 "低空坠地冲击伤害"
            Plunging3 "高空坠地冲击伤害"
        ),
        skill2: skill_map!(
            FaruzanDamageEnum
            E1 "技能伤害"
            E2 "风压塌陷伤害"
        ),
        skill3: skill_map!(
            FaruzanDamageEnum
            Q1 "技能伤害"
        )
    };

    // #[cfg(not(target_family = "wasm"))]
    // const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    //     ItemConfig {
    //         name: "e_pyro",
    //         title: "c52",
    //         config: ItemConfigType::Bool { default: false },
    //     },
    //     ItemConfig {
    //         name: "e_cryo",
    //         title: "c53",
    //         config: ItemConfigType::Bool { default: false },
    //     },
    // ]);

    // #[cfg(not(target_family = "wasm"))]
    // const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
    //     ItemConfig {
    //         name: "e_enabled",
    //         title: "c50", 
    //         config: ItemConfigType::Bool { default: true }
    //     },
    //     ItemConfig {
    //         name: "e_hydro",
    //         title: "c51",
    //         config: ItemConfigType::Bool { default: false }
    //     },
    //     ItemConfig {
    //         name: "sdpoints",
    //         title: "c54",
    //         config: ItemConfigType::Float { min: 0.0, max: 120.0, default: 50.0 },
    //     },
    // ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: FaruzanDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use FaruzanDamageEnum::*;

        let mut builder = D::new();
        
        let ratio = match s {
            Normal1 => FARUZAN_SKILL.normal_dmg1[s1],
            Normal2 => FARUZAN_SKILL.normal_dmg2[s1],
            Normal3 => FARUZAN_SKILL.normal_dmg3[s1],
            Normal4 => FARUZAN_SKILL.normal_dmg4[s1],
            Charged1 => FARUZAN_SKILL.charged1_dmg[s1],
            Charged2 => FARUZAN_SKILL.charged2_dmg[s1],
            Plunging1 => FARUZAN_SKILL.plunging_dmg1[s1],
            Plunging2 => FARUZAN_SKILL.plunging_dmg2[s1],
            Plunging3 => FARUZAN_SKILL.plunging_dmg3[s1],
            E1 => FARUZAN_SKILL.e_dmg[s2],
            E2 => FARUZAN_SKILL.e_collapse_dmg[s2],
            Q1 => FARUZAN_SKILL.q_dmg[s3],
        };
        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_skill_element(),
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