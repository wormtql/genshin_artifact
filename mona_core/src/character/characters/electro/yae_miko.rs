use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::YaeMikoDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

pub struct YaeMikoSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
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

pub const YAE_MIKO_SKILL: YaeMikoSkillType = YaeMikoSkillType {
    normal_dmg1: [0.3966, 0.4263, 0.4561, 0.4957, 0.5255, 0.5552, 0.5949, 0.6345, 0.6742, 0.7139, 0.7535, 0.7932, 0.8427, 0.8923, 0.9419],
    normal_dmg2: [0.3852, 0.4141, 0.443, 0.4815, 0.5104, 0.5393, 0.5778, 0.6163, 0.6548, 0.6933, 0.7319, 0.7704, 0.8185, 0.8667, 0.9148],
    normal_dmg3: [0.5689, 0.6116, 0.6542, 0.7111, 0.7538, 0.7964, 0.8533, 0.9102, 0.9671, 1.024, 1.0809, 1.1378, 1.2089, 1.28, 1.3511],
    charged_dmg1: [1.4289, 1.5361, 1.6433, 1.7862, 1.8934, 2.0005, 2.1434, 2.2863, 2.4292, 2.5721, 2.715, 2.8579, 3.0365, 3.2151, 3.3938],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.6067, 0.6522, 0.6977, 0.7584, 0.8039, 0.8494, 0.9101, 0.9708, 1.0314, 1.0921, 1.1528, 1.2134, 1.2893, 1.3651, 1.441],
    elemental_skill_dmg2: [0.7584, 0.8153, 0.8722, 0.948, 1.0049, 1.0618, 1.1376, 1.2134, 1.2893, 1.3651, 1.441, 1.5168, 1.6116, 1.7064, 1.8012],
    elemental_skill_dmg3: [0.948, 1.0191, 1.0902, 1.185, 1.2561, 1.3272, 1.422, 1.5168, 1.6116, 1.7064, 1.8012, 1.896, 2.0145, 2.133, 2.2515],
    elemental_skill_dmg4: [1.185, 1.2739, 1.3628, 1.4813, 1.5701, 1.659, 1.7775, 1.896, 2.0145, 2.133, 2.2515, 2.37, 2.5181, 2.6663, 2.8144],
    elemental_burst_dmg1: [2.6, 2.795, 2.99, 3.25, 3.445, 3.64, 3.9, 4.16, 4.42, 4.68, 4.94, 5.2, 5.525, 5.85, 6.175],
    elemental_burst_dmg2: [3.3382, 3.5885, 3.8389, 4.1727, 4.4231, 4.6734, 5.0072, 5.3411, 5.6749, 6.0087, 6.3425, 6.6763, 7.0936, 7.5109, 7.9281]
};

pub struct YaeMiko;

#[derive(FromPrimitive, Copy, Clone, EnumString, EnumCountMacro)]
pub enum YaeMikoDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    E4,
    Q1,
    Q2
}

impl YaeMikoDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use YaeMikoDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for YaeMikoDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(FromPrimitive, Copy, Clone)]
pub enum YaeMikoRoleEnum {
    Default
}

impl CharacterTrait for YaeMiko {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::YaeMiko,
        internal_name: "Yae",
        element: Element::Electro,
        hp: [807, 2095, 2787, 4170, 4662, 5364, 6020, 6729, 7220, 7936, 8428, 9151, 9643, 10372],
        atk: [26, 69, 91, 137, 153, 176, 197, 220, 236, 260, 276, 300, 316, 340],
        def: [44, 115, 153, 229, 256, 294, 330, 369, 396, 435, 462, 502, 529, 569],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·狐灵食罪式",
            en: "Normal Attack: Spiritfox Sin-Eater",
        ),
        skill_name2: locale!(
            zh_cn: "野干役咒·杀生樱",
            en: "Yakan Evocation: Sesshou Sakura",
        ),
        skill_name3: locale!(
            zh_cn: "大密法·天狐显真",
            en: "Great Secret Art: Tenko Kenshin",
        ),
        name_locale: locale!(
            zh_cn: "八重神子",
            en: "Yae Miko",
        )
    };
    type SkillType = YaeMikoSkillType;
    const SKILL: Self::SkillType = YAE_MIKO_SKILL;
    type DamageEnumType = YaeMikoDamageEnum;
    type RoleEnum = YaeMikoRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: YaeMikoDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: YaeMikoDamageEnum::E1 as usize, text: locale!(zh_cn: "杀生樱伤害·壹阶", en: "Sesshou Sakura DMG: Level 1") },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::E2 as usize, text: locale!(zh_cn: "杀生樱伤害·贰阶", en: "Sesshou Sakura DMG: Level 2") },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::E3 as usize, text: locale!(zh_cn: "杀生樱伤害·叁阶", en: "Sesshou Sakura DMG: Level 3") },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::E4 as usize, text: locale!(zh_cn: "杀生樱伤害·肆阶", en: "Sesshou Sakura DMG: Level 4") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: YaeMikoDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: YaeMikoDamageEnum::Q2 as usize, text: locale!(zh_cn: "天狐霆雷伤害", en: "Tenko Thunderbolt DMG") },
        ])
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: YaeMikoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use YaeMikoDamageEnum::*;
        let ratio = match s {
            Normal1 => YAE_MIKO_SKILL.normal_dmg1[s1],
            Normal2 => YAE_MIKO_SKILL.normal_dmg2[s1],
            Normal3 => YAE_MIKO_SKILL.normal_dmg3[s1],
            Charged => YAE_MIKO_SKILL.charged_dmg1[s1],
            Plunging1 => YAE_MIKO_SKILL.plunging_dmg1[s1],
            Plunging2 => YAE_MIKO_SKILL.plunging_dmg2[s1],
            Plunging3 => YAE_MIKO_SKILL.plunging_dmg3[s1],
            E1 => YAE_MIKO_SKILL.elemental_skill_dmg1[s2],
            E2 => YAE_MIKO_SKILL.elemental_skill_dmg2[s2],
            E3 => YAE_MIKO_SKILL.elemental_skill_dmg3[s2],
            E4 => YAE_MIKO_SKILL.elemental_skill_dmg4[s2],
            Q1 => YAE_MIKO_SKILL.elemental_burst_dmg1[s3],
            Q2 => YAE_MIKO_SKILL.elemental_burst_dmg2[s3],
        };

        let skill_type = s.get_skill_type();

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        if context.character_common_data.has_talent2 && skill_type == SkillType::ElementalSkill {
            // let em = context.attribute.get_value(AttributeName::ElementalMastery);
            let em = context.attribute.get_em_all();
            builder.add_extra_bonus("八重神子天赋「启蜇之祝词」", em * 0.0015);
        }

        if context.character_common_data.constellation >= 6 && skill_type == SkillType::ElementalSkill {
            builder.add_extra_def_penetration("八重神子六命「大杀生咒禁」", 0.6);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Electro,
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
