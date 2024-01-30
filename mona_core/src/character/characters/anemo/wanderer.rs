use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct WandererSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],
    pub e_bonus_normal: [f64; 15],
    pub e_bonus_charged: [f64; 15],

    pub q_dmg: [f64; 15],
}

pub const WANDERER_SKILL: WandererSkillType = WandererSkillType {
    normal_dmg1: [0.6871, 0.7431, 0.799, 0.8789, 0.9348, 0.9987, 1.0866, 1.1745, 1.2624, 1.3583, 1.4542, 1.5501, 1.6459, 1.7418, 1.8377],
    normal_dmg2: [0.6502, 0.7031, 0.756, 0.8316, 0.8845, 0.945, 1.0282, 1.1113, 1.1945, 1.2852, 1.3759, 1.4666, 1.5574, 1.6481, 1.7388],
    normal_dmg3: [0.4764, 0.5152, 0.554, 0.6094, 0.6482, 0.6925, 0.7534, 0.8144, 0.8753, 0.9418, 1.0083, 1.0748, 1.1412, 1.2077, 1.2742],
    charged_dmg1: [1.3208, 1.4199, 1.5189, 1.651, 1.7501, 1.8491, 1.9812, 2.1133, 2.2454, 2.3774, 2.5095, 2.6416, 2.8067, 2.9718, 3.1369],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg: [0.952, 1.0234, 1.0948, 1.19, 1.2614, 1.3328, 1.428, 1.5232, 1.6184, 1.7136, 1.8088, 1.904, 2.023, 2.142, 2.261],
    e_bonus_normal: [1.3298, 1.3496, 1.3693, 1.395, 1.4147, 1.4345, 1.4602, 1.4858, 1.5115, 1.5372, 1.5629, 1.5885, 1.6142, 1.6399, 1.6656],
    e_bonus_charged: [1.2639, 1.2797, 1.2955, 1.316, 1.3318, 1.3476, 1.3681, 1.3887, 1.4092, 1.4298, 1.4503, 1.4708, 1.4914, 1.5119, 1.5325],
    q_dmg: [1.472, 1.5824, 1.6928, 1.84, 1.9504, 2.0608, 2.208, 2.3552, 2.5024, 2.6496, 2.7968, 2.944, 3.128, 3.312, 3.496],
};

damage_enum!(
    WandererDamageEnum
    Normal1
    Normal2
    Normal3
    Normal1C6
    Normal2C6
    Normal3C6
    Charged1
    Dash1
    Plunging1
    Plunging2
    Plunging3
    E1
    Q1
);

impl WandererDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use WandererDamageEnum::*;
        match *self {
            E1 | Dash1 => SkillType::ElementalSkill, //TODO: dash1 => e not confirmed
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Charged1 => SkillType::ChargedAttack,
            Normal1 | Normal2 | Normal3 | Normal1C6 | Normal2C6 | Normal3C6 => SkillType::NormalAttack,
            Q1 => SkillType::ElementalBurst,
        }
    }
}

pub struct WandererEffect {
    pub talent1: bool,
    pub e_pyro: bool,
    pub e_cryo: bool,
}

impl<A: Attribute> ChangeAttribute<A> for WandererEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.talent1 {
            if self.e_pyro {
                attribute.add_atk_percentage("天赋「拾玉得花」染火加成", 0.3);
            }
            if self.e_cryo {
                attribute.set_value_by(AttributeName::CriticalBase, "天赋「拾玉得花」染冰加成", 0.2);
            }
        }
    }
}

pub struct Wanderer;

impl CharacterTrait for Wanderer {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Wanderer,
        internal_name: "Wanderer",
        element: Element::Anemo,
        hp: [791, 2053, 2731, 4086, 4568, 5256, 5899, 6593, 7076, 7777, 8259, 8968, 9450, 10164],
        atk: [26, 66, 88, 132, 147, 169, 190, 213, 228, 251, 266, 289, 305, 328],
        def: [47, 123, 163, 244, 273, 314, 352, 394, 423, 465, 493, 536, 564, 607],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·行幡鸣弦",
            en: "Normal Attack: Yuuban Meigen",
        ),
        skill_name2: locale!(
            zh_cn: "羽画·风姿华歌",
            en: "Hanega: Fushi Kakka",
        ),
        skill_name3: locale!(
            zh_cn: "狂言·式乐五番",
            en: "Kyougen: Shikiraku Gobandate",
        ),
        name_locale: locale!(
            zh_cn: "流浪者",
            en: "Wanderer",
        )
    };
    type SkillType = WandererSkillType;
    const SKILL: Self::SkillType = WANDERER_SKILL;
    type DamageEnumType = WandererDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            WandererDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 locale!(zh_cn: "三段伤害/2", en: "2-Hit DMG/2")
            Normal1C6 locale!(zh_cn: "一段六命额外", en: "C6 Additional 1")
            Normal2C6 locale!(zh_cn: "二段六命额外", en: "C6 Additional 2")
            Normal3C6 locale!(zh_cn: "三段六命额外", en: "C6 Additional 3")
            Charged1 charged_dmg!()
            Dash1 locale!(zh_cn: "「梦迹一风」风矢伤害", en: "Gales of Reverie Wind Arrow DMG")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            WandererDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            WandererDamageEnum
            Q1 locale!(zh_cn: "技能伤害/5", en: "Skill DMG/5")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_pyro",
            title: locale!(
                zh_cn: "「拾玉得花」染火",
                en: "Jade-Claimed Flower Pyro",
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "e_cryo",
            title: locale!(
                zh_cn: "「拾玉得花」染冰",
                en: "Jade-Claimed Flower Cryo"
            ),
            config: ItemConfigType::Bool { default: false },
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "e_enabled",
            title: locale!(
                zh_cn: "处于「优风倾姿」状态",
                en: "Under Windfavored State"
            ),
            config: ItemConfigType::Bool { default: true },
        },
        ItemConfig {
            name: "e_hydro",
            title: locale!(
                zh_cn: "「拾玉得花」染水",
                en: "Jade-Claimed Flower Hydro",
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "sdpoints",
            title: locale!(
                zh_cn: "等效「空居力」",
                en: "Kuugoryoku Points"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 120.0, default: 50.0 },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: WandererDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use WandererDamageEnum::*;

        let mut builder = D::new();

        let (e_enabled, e_hydro, sdpoints) = match *config {
            CharacterSkillConfig::Wanderer { e_enabled, e_hydro, sdpoints } => (e_enabled, e_hydro, sdpoints),
            _ => (false, false, 0.0),
        };

        let ratio = match s {
            Normal1 => WANDERER_SKILL.normal_dmg1[s1] * (if e_enabled { WANDERER_SKILL.e_bonus_normal[s2] } else { 1.0 }),
            Normal2 => WANDERER_SKILL.normal_dmg2[s1] * (if e_enabled { WANDERER_SKILL.e_bonus_normal[s2] } else { 1.0 }),
            Normal3 => WANDERER_SKILL.normal_dmg3[s1] * (if e_enabled { WANDERER_SKILL.e_bonus_normal[s2] } else { 1.0 }),
            Normal1C6 => WANDERER_SKILL.normal_dmg1[s1] * (if e_enabled { WANDERER_SKILL.e_bonus_normal[s2] * 0.4 } else { 1.0 }),
            Normal2C6 => WANDERER_SKILL.normal_dmg2[s1] * (if e_enabled { WANDERER_SKILL.e_bonus_normal[s2] * 0.4 } else { 1.0 }),
            Normal3C6 => WANDERER_SKILL.normal_dmg3[s1] * (if e_enabled { WANDERER_SKILL.e_bonus_normal[s2] * 0.4 } else { 1.0 }),
            Charged1 => WANDERER_SKILL.charged_dmg1[s1] * (if e_enabled { WANDERER_SKILL.e_bonus_charged[s2] } else { 1.0 }),
            Plunging1 => WANDERER_SKILL.plunging_dmg1[s1],
            Plunging2 => WANDERER_SKILL.plunging_dmg2[s1],
            Plunging3 => WANDERER_SKILL.plunging_dmg3[s1],
            Dash1 => if context.character_common_data.constellation >= 1 { 0.5 } else { 0.35 },
            E1 => WANDERER_SKILL.e_dmg[s2],
            Q1 => WANDERER_SKILL.q_dmg[s3],
        };
        builder.add_atk_ratio("技能倍率", ratio);

        if e_enabled {
            let max_sdpoints = if e_hydro { 120.0 } else { 100.0 };
            let q_bonus = ((max_sdpoints - sdpoints).max(0.0) * 0.04).min(2.0);
            if context.character_common_data.constellation >= 2 && s == Q1 {
                builder.add_extra_bonus("二命「二番·箙岛廓白浪」伤害加成", q_bonus)
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Anemo,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (e_pyro, e_cryo) = match *config {
            CharacterConfig::Wanderer { e_pyro, e_cryo } => (e_pyro, e_cryo),
            _ => (false, false),
        };
        Some(Box::new(WandererEffect {
            talent1: common_data.has_talent1,
            e_pyro,
            e_cryo,
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}