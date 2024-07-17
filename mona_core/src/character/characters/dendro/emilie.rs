use crate::{attribute::Attribute, character::macros::damage_enum, common::ChangeAttribute, attribute::AttributeCommon};
use crate::attribute::AttributeName;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::kirara::KiraraDamageEnum;
use crate::character::characters::kirara::KiraraDamageEnum::{Plunging1, Plunging2, Plunging3};
use crate::character::macros::{damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait};
use crate::common::{Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, plunging_dmg};
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::traits::{CharacterSkillMapItem};

pub struct EmilieSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],
    pub e_dmg4: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const EMILIE_SKILL: EmilieSkillType = EmilieSkillType {
    normal_dmg1: [0.4856, 0.5251, 0.5647, 0.6211, 0.6607, 0.7058, 0.7679, 0.8301, 0.8922, 0.9599, 1.0277, 1.0954, 1.1632, 1.231, 1.2987],
    normal_dmg2: [0.449, 0.4855, 0.522, 0.5742, 0.6108, 0.6526, 0.71, 0.7674, 0.8248, 0.8875, 0.9501, 1.0128, 1.0754, 1.138, 1.2007],
    normal_dmg3: [0.593, 0.6413, 0.6895, 0.7585, 0.8068, 0.8619, 0.9378, 1.0136, 1.0895, 1.1722, 1.255, 1.3377, 1.4205, 1.5032, 1.5859],
    normal_dmg4: [0.751, 0.8122, 0.8733, 0.9606, 1.0217, 1.0916, 1.1877, 1.2837, 1.3798, 1.4846, 1.5894, 1.6942, 1.799, 1.9038, 2.0086],
    charged_dmg: [0.9133, 0.9877, 1.062, 1.1682, 1.2425, 1.3275, 1.4443, 1.5611, 1.678, 1.8054, 1.9328, 2.0603, 2.1877, 2.3152, 2.4426],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [0.4708, 0.5061, 0.5414, 0.5885, 0.6238, 0.6591, 0.7062, 0.7533, 0.8004, 0.8474, 0.8945, 0.9416, 1.0005, 1.0593, 1.1182],
    e_dmg2: [0.396, 0.4257, 0.4554, 0.495, 0.5247, 0.5544, 0.594, 0.6336, 0.6732, 0.7128, 0.7524, 0.792, 0.8415, 0.891, 0.9405],
    e_dmg3: [0.84, 0.903, 0.966, 1.05, 1.113, 1.176, 1.26, 1.344, 1.428, 1.512, 1.596, 1.68, 1.785, 1.89, 1.995],
    e_dmg4: [0.3852, 0.4141, 0.443, 0.4815, 0.5104, 0.5393, 0.5778, 0.6163, 0.6548, 0.6934, 0.7319, 0.7704, 0.8186, 0.8667, 0.9149],
    q_dmg1: [2.172, 2.3349, 2.4978, 2.715, 2.8779, 3.0408, 3.258, 3.4752, 3.6924, 3.9096, 4.1268, 4.344, 4.6155, 4.887, 5.1585],
};

damage_enum!(
    EmilieDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    E4
    Q1
    Talent1
);

impl EmilieDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use EmilieDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
            Talent1 => SkillType::NoneType
        }
    }

    pub fn get_element(&self, use_c6: bool) -> Element {
        use EmilieDamageEnum::*;
        if use_c6 {
            match *self {
                Plunging1 | Plunging2 | Plunging3 => Element::Physical,
                _ => Element::Dendro,
            }
        } else {
            match *self {
                Normal1 | Normal2 | Normal3 | Normal4 | Charged | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
                _ => Element::Dendro,
            }
        }
    }
}

pub struct Emilie;

impl CharacterTrait for Emilie {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Emilie,
        internal_name: "Emilie",
        name_locale: locale!(
            zh_cn: "艾梅莉埃",
            en: "Emilie"
        ),
        element: Element::Dendro,
        hp: [1056, 2740, 3646, 5455, 6099, 7016, 7874, 8802, 9445, 10381, 11025, 11971, 12615, 13568],
        atk: [26, 68, 90, 135, 151, 173, 194, 217, 233, 256, 272, 295, 311, 335],
        def: [57, 147, 196, 294, 328, 378, 424, 474, 508, 559, 593, 644, 679, 730],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Polearm,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·逐影枪术·改",
            en: "Normal Attack: Shadow-Hunting Spear (Custom)"
        ),
        skill_name2: locale!(
            zh_cn: "撷萃调香",
            en: "Fragrance Extraction"
        ),
        skill_name3: locale!(
            zh_cn: "香氛演绎",
            en: "Aromatic Explication"
        ),
    };
    type SkillType = EmilieSkillType;
    const SKILL: Self::SkillType = EMILIE_SKILL;
    type DamageEnumType = EmilieDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            EmilieDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            EmilieDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E2 locale!(zh_cn: "柔灯之匣·一阶攻击伤害", en: "Level 1 Lumidouce Case Attack DMG")
            E3 locale!(zh_cn: "柔灯之匣·二阶攻击伤害", en: "Level 2 Lumidouce Case Attack DMG")
            E4 locale!(zh_cn: "灵息之刺伤害", en: "Spiritbreath Thorn DMG")
            Talent1 locale!(zh_cn: "「清露香氛」", en: "Cleardew Cologne")
        ),
        skill3: skill_map!(
            EmilieDamageEnum
            Q1 locale!(zh_cn: "柔灯之匣·三阶攻击伤害", en: "Level 3 Lumidouce Case Attack DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "enemy_burn",
            title: locale!(
                zh_cn: "敌人处于燃烧状态",
                en: "Enemy is Burnt"
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "use_c6",
            title: locale!(
                zh_cn: "C6「茉洁香迹」",
                en: "C6: Marcotte Sillage"
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: EmilieDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let mut builder = D::new();
        use EmilieDamageEnum::*;

        let (enemy_burn, use_c6) = match *config {
            CharacterSkillConfig::Emilie { enemy_burn, use_c6 } => (enemy_burn, use_c6),
            _ => (false, false)
        };

        let ratio = match s {
            Normal1 => EMILIE_SKILL.normal_dmg1[s1],
            Normal2 => EMILIE_SKILL.normal_dmg2[s1],
            Normal3 => EMILIE_SKILL.normal_dmg3[s1],
            Normal4 => EMILIE_SKILL.normal_dmg4[s1],
            Charged => EMILIE_SKILL.charged_dmg[s1],
            Plunging1 => EMILIE_SKILL.plunging_dmg1[s1],
            Plunging2 => EMILIE_SKILL.plunging_dmg2[s1],
            Plunging3 => EMILIE_SKILL.plunging_dmg3[s1],
            E1 => EMILIE_SKILL.e_dmg1[s2],
            E2 => EMILIE_SKILL.e_dmg2[s2],
            E3 => EMILIE_SKILL.e_dmg3[s2],
            E4 => EMILIE_SKILL.e_dmg4[s2],
            Q1 => EMILIE_SKILL.q_dmg1[s3],
            Talent1 => 6.0
        };
        builder.add_atk_ratio("技能倍率", ratio);
        if context.character_common_data.constellation >= 1 && s == Talent1 {
            builder.add_extra_bonus("C1「淡香浸析」", 0.2);
        }
        let use_c6 = use_c6 && context.character_common_data.constellation >= 6;

        let skill_type = s.get_skill_type();
        let element = s.get_element(use_c6);

        if use_c6 && (skill_type == SkillType::NormalAttack || skill_type == SkillType::ChargedAttack) {
            builder.add_atk_ratio("C6「」茉洁香迹", 3.0);
        }

        if enemy_burn && context.character_common_data.has_talent2 {
            let atk = context.attribute.get_value(AttributeName::ATK);
            let bonus = ((atk / 1000.0).floor() * 0.15).min(0.36);
            builder.add_extra_bonus("天赋「精馏」", bonus);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            element,
            skill_type,
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
