use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct SethosSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg21: [f64; 15],
    pub normal_dmg22: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub charged_dmg31: [f64; 15],
    pub charged_dmg32: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const SETHOS_SKILL: SethosSkillType = SethosSkillType {
    normal_dmg1: [0.5261, 0.569, 0.6118, 0.673, 0.7158, 0.7647, 0.832, 0.8993, 0.9666, 1.04, 1.1135, 1.1869, 1.2603, 1.3337, 1.4071],
    normal_dmg21: [0.238, 0.2573, 0.2767, 0.3044, 0.3237, 0.3459, 0.3763, 0.4067, 0.4372, 0.4704, 0.5036, 0.5368, 0.57, 0.6032, 0.6364],
    normal_dmg22: [0.2661, 0.2877, 0.3094, 0.3403, 0.362, 0.3868, 0.4208, 0.4548, 0.4889, 0.526, 0.5631, 0.6002, 0.6374, 0.6745, 0.7116],
    normal_dmg3: [0.7399, 0.8001, 0.8603, 0.9463, 1.0066, 1.0754, 1.17, 1.2647, 1.3593, 1.4625, 1.5658, 1.669, 1.7722, 1.8755, 1.9787],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    charged_dmg31: [1.4, 1.505, 1.61, 1.75, 1.855, 1.96, 2.1, 2.24, 2.38, 2.52, 2.66, 2.8, 2.975, 3.15, 3.325],
    charged_dmg32: [1.3456, 1.4465, 1.5474, 1.682, 1.7829, 1.8838, 2.0184, 2.153, 2.2875, 2.4221, 2.5566, 2.6912, 2.8594, 3.0276, 3.1958],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [1.156, 1.2427, 1.3294, 1.445, 1.5317, 1.6184, 1.734, 1.8496, 1.9652, 2.0808, 2.1964, 2.312, 2.4565, 2.601, 2.7455],
    q_dmg1: [1.9616, 2.1087, 2.2558, 2.452, 2.5991, 2.7462, 2.9424, 3.1386, 3.3347, 3.5309, 3.727, 3.9232, 4.1684, 4.4136, 4.6588],
};

damage_enum!(
    SethosDamageEnum
    Normal1
    Normal2
    Normal3
    Normal1Q
    Normal2Q
    Normal3Q
    Charged1
    Charged2
    Charged3
    Plunging1
    Plunging2
    Plunging3
    E1
);

impl SethosDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use SethosDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged1 | Charged2 | Charged3 | Normal1Q | Normal2Q | Normal3Q => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
        }
    }

    pub fn get_element(&self) -> Element {
        use SethosDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            _ => Element::Electro,
        }
    }
}

pub struct SethosEffect {
    pub c2_stack: f64,
}

impl<A: Attribute> ChangeAttribute<A> for SethosEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = self.c2_stack * 0.15;
        attribute.set_value_by(AttributeName::BonusElectro, "C2「寂秘纸草经」", bonus);
    }
}

pub struct Sethos;

impl CharacterTrait for Sethos {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Sethos,
        internal_name: "Sethos",
        name_locale: locale!(
            zh_cn: "赛索斯",
            en: "Sethos"
        ),
        element: Element::Electro,
        hp: [821, 2108, 2721, 4076, 4512, 5189, 5770, 6448, 6884, 7561, 7996, 8674, 9110, 9787],
        atk: [19, 49, 63, 95, 105, 121, 134, 150, 160, 176, 186, 201, 212, 227],
        def: [47, 121, 156, 233, 258, 297, 330, 369, 394, 432, 457, 496, 521, 560],
        sub_stat: CharacterSubStatFamily::ElementalMastery96,
        weapon_type: WeaponType::Bow,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·王家苇箭术",
            en: "Normal Attack: Royal Reed Archery"
        ),
        skill_name2: locale!(
            zh_cn: "古仪·鸣砂掣雷",
            en: "Ancient Rite: The Thundering Sands"
        ),
        skill_name3: locale!(
            zh_cn: "秘仪·瞑光贯影",
            en: "Secret Rite: Twilight Shadowpiercer"
        ),
    };
    type SkillType = SethosSkillType;
    const SKILL: Self::SkillType = SETHOS_SKILL;
    type DamageEnumType = SethosDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            SethosDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged1 locale!(zh_cn: "瞄准射击", en: "Aimed Shot")
            Charged2 locale!(zh_cn: "一段蓄力瞄准射击", en: "Aimed Shot Charge Level 1")
            Charged2 locale!(zh_cn: "贯影箭伤害", en: "Shadowpiercing Shot DMG")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            SethosDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            SethosDamageEnum
            Normal1Q locale!(zh_cn: "瞑弦矢-1", en: "Dusk Bolt DMG-1")
            Normal2Q locale!(zh_cn: "瞑弦矢-2", en: "Dusk Bolt DMG-2")
            Normal3Q locale!(zh_cn: "瞑弦矢-3", en: "Dusk Bolt DMG-3")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c2_stack",
            title: locale!(
                zh_cn: "C2「寂秘纸草经」比例",
                en: "C2 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: SethosDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use SethosDamageEnum::*;
        let atk_ratio = match s {
            Normal1 | Normal1Q => SETHOS_SKILL.normal_dmg1[s1],
            Normal2 | Normal2Q => SETHOS_SKILL.normal_dmg21[s1] + SETHOS_SKILL.normal_dmg22[s1],
            Normal3 | Normal3Q => SETHOS_SKILL.normal_dmg3[s1],
            Charged1 => SETHOS_SKILL.charged_dmg1[s1],
            Charged2 => SETHOS_SKILL.charged_dmg2[s1],
            Charged3 => SETHOS_SKILL.charged_dmg31[s1],
            Plunging1 => SETHOS_SKILL.plunging_dmg1[s1],
            Plunging2 => SETHOS_SKILL.plunging_dmg2[s1],
            Plunging3 => SETHOS_SKILL.plunging_dmg3[s1],
            E1 => SETHOS_SKILL.e_dmg1[s2],
        };
        let mut em_ratio = match s {
            Normal1Q | Normal2Q | Normal3Q => SETHOS_SKILL.q_dmg1[s3],
            Charged3 => SETHOS_SKILL.charged_dmg32[s1],
            _ => 0.0
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", atk_ratio);
        builder.add_em_ratio("技能倍率", em_ratio);
        if context.character_common_data.has_talent2 && s == Charged3 {
            builder.add_em_ratio("天赋「砂王的赐礼」", 7.0);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let c2_stack = match *config {
            CharacterConfig::Sethos { c2_stack } => c2_stack,
            _ => 0.0
        };
        if c2_stack > 0.0 && common_data.constellation >= 2 && common_data.has_talent1 {
            Some(Box::new(SethosEffect {
                c2_stack
            }))
        } else {
            None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}