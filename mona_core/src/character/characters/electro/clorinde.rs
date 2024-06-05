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

pub struct ClorindeSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg11: [f64; 15],
    pub e_dmg12: [f64; 15],
    pub e_dmg21: [f64; 15],
    pub e_dmg22: [f64; 15],
    pub e_dmg23: [f64; 15],
    pub e_dmg3: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_life: [f64; 15],
}

pub const CLORINDE_SKILL: ClorindeSkillType = ClorindeSkillType {
    normal_dmg1: [0.5406, 0.5846, 0.6286, 0.6915, 0.7355, 0.7858, 0.8549, 0.924, 0.9932, 1.0686, 1.1441, 1.2195, 1.2949, 1.3703, 1.4458],
    normal_dmg2: [0.5163, 0.5583, 0.6003, 0.6604, 0.7024, 0.7504, 0.8164, 0.8825, 0.9485, 1.0206, 1.0926, 1.1646, 1.2367, 1.3087, 1.3808],
    normal_dmg3: [0.3419, 0.3697, 0.3975, 0.4373, 0.4651, 0.4969, 0.5406, 0.5843, 0.6281, 0.6758, 0.7235, 0.7712, 0.8189, 0.8666, 0.9143],
    normal_dmg4: [0.2313, 0.2502, 0.269, 0.2959, 0.3147, 0.3363, 0.3658, 0.3954, 0.425, 0.4573, 0.4896, 0.5219, 0.5541, 0.5864, 0.6187],
    normal_dmg5: [0.9001, 0.9734, 1.0466, 1.1513, 1.2246, 1.3083, 1.4234, 1.5385, 1.6537, 1.7793, 1.9049, 2.0305, 2.1561, 2.2817, 2.4072],
    charged_dmg: [1.2814, 1.3857, 1.49, 1.639, 1.7433, 1.8625, 2.0264, 2.1903, 2.3542, 2.533, 2.7118, 2.8906, 3.0694, 3.2482, 3.427],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg11: [0.2676, 0.2894, 0.3112, 0.3423, 0.3641, 0.389, 0.4232, 0.4575, 0.4917, 0.529, 0.5664, 0.6037, 0.6411, 0.6784, 0.7158],
    e_dmg12: [0.3879, 0.4194, 0.451, 0.4961, 0.5277, 0.5638, 0.6134, 0.663, 0.7126, 0.7667, 0.8208, 0.8749, 0.9291, 0.9832, 1.0373],
    e_dmg21: [0.3297, 0.3566, 0.3834, 0.4217, 0.4486, 0.4793, 0.5214, 0.5636, 0.6058, 0.6518, 0.6978, 0.7438, 0.7898, 0.8358, 0.8818],
    e_dmg22: [0.4396, 0.4754, 0.5112, 0.5623, 0.5981, 0.639, 0.6952, 0.7515, 0.8077, 0.869, 0.9304, 0.9917, 1.0531, 1.1144, 1.1758],
    e_dmg23: [0.2511, 0.2716, 0.292, 0.3212, 0.3416, 0.365, 0.3971, 0.4292, 0.4614, 0.4964, 0.5314, 0.5665, 0.6015, 0.6366, 0.6716],
    e_dmg3: [0.432, 0.4644, 0.4968, 0.54, 0.5724, 0.6048, 0.648, 0.6912, 0.7344, 0.7776, 0.8208, 0.864, 0.918, 0.972, 1.026],
    q_dmg1: [1.2688, 1.364, 1.4591, 1.586, 1.6812, 1.7763, 1.9032, 2.0301, 2.157, 2.2838, 2.4107, 2.5376, 2.6962, 2.8548, 3.0134],
    q_life: [0.66, 0.72, 0.78, 0.84, 0.9, 0.96, 1.02, 1.08, 1.14, 1.2, 1.26, 1.32, 1.38, 1.44, 1.5],
};

damage_enum!(
    ClorindeDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Normal5
    Charged
    Plunging1
    Plunging2
    Plunging3
    E11
    E12
    E21
    E22
    E23
    E3
    Q1
);

impl ClorindeDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use ClorindeDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 | E11 | E12 | E21 | E22 | E23 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E3 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
        }
    }

    pub fn get_element(&self) -> Element {
        use ClorindeDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 | Charged | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            E11 | E12 | E21 | E22 | E23 | E3 | Q1 => Element::Electro,
        }
    }
}

pub struct ClorindeEffect {
    pub talent1_stack: f64,
    pub talent2_stack: f64,
    pub c6_rate: f64,
    pub constellation: usize,
    pub has_talent2: bool,
    pub has_talent1: bool,
}

impl<A: Attribute> ChangeAttribute<A> for ClorindeEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent1 && self.talent1_stack > 0.0 {
            let ratio = if self.constellation >= 2 { 0.3 } else { 0.2 };
            let max = if self.constellation >= 2 { 2700.0 } else { 1800.0 };
            let talent1_stack = self.talent1_stack;
            attribute.add_edge1(
                AttributeName::ATK,
                AttributeName::ExtraDmgNormalAttack,
                Box::new(move |atk, _| (atk * ratio * talent1_stack).min(max)),
                Box::new(|_x, _y, _grad| (0.0, 0.0)),
                "天赋1「破夜的明焰」",
            );
            attribute.add_edge1(
                AttributeName::ATK,
                AttributeName::ExtraDmgElementalBurst,
                Box::new(move |atk, _| (atk * ratio * talent1_stack).min(max)),
                Box::new(|_x, _y, _grad| (0.0, 0.0)),
                "天赋1「破夜的明焰」",
            )
        }
        if self.has_talent2 && self.talent2_stack > 0.0 {
            let bonus = 0.1 * self.talent2_stack;
            attribute.set_value_by(AttributeName::CriticalBase, "天赋「契令的酬偿」", bonus);
        }
        if self.constellation >= 6 && self.c6_rate > 0.0 {
            attribute.set_value_by(AttributeName::CriticalBase, "C6「为此，勿将希望弃扬」", 0.1 * self.c6_rate);
            attribute.set_value_by(AttributeName::CriticalDamageBase, "C6「为此，勿将希望弃扬」", 0.7 * self.c6_rate);
        }
    }
}

pub struct Clorinde;

impl CharacterTrait for Clorinde {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Clorinde,
        internal_name: "Clorinde",
        name_locale: locale!(
            zh_cn: "克洛琳德",
            en: "Clorinde",
        ),
        element: Element::Electro,
        hp: [1009, 2616, 3481, 5209, 5823, 6700, 7519, 8405, 9019, 9913, 10527, 11431, 12045, 12956],
        atk: [26, 68, 91, 136, 152, 174, 196, 219, 235, 258, 274, 298, 314, 337],
        def: [61, 158, 211, 315, 352, 405, 455, 509, 546, 600, 637, 692, 729, 784],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·逐影之誓",
            en: "Normal Attack: Oath of Hunting Shadows",
        ),
        skill_name2: locale!(
            zh_cn: "狩夜之巡",
            en: "Hunter's Vigil",
        ),
        skill_name3: locale!(
            zh_cn: "残光将终",
            en: "Last Lightfall",
        ),
    };
    type SkillType = ClorindeSkillType;
    const SKILL: Self::SkillType = CLORINDE_SKILL;
    type DamageEnumType = ClorindeDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ClorindeDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 locale!(zh_cn: "三段伤害/2", en: "3-Hit DMG/2")
            Normal4 locale!(zh_cn: "四段伤害/3", en: "4-Hit DMG/3")
            Normal5 hit_n_dmg!(5)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ClorindeDamageEnum
            E11 locale!(zh_cn: "驰猎伤害-1", en: "Swift Hunt DMG-1")
            E12 locale!(zh_cn: "驰猎伤害-2", en: "Swift Hunt DMG-2")
            E21 locale!(zh_cn: "贯夜伤害-1", en: "Impale the Night DMG-1")
            E22 locale!(zh_cn: "贯夜伤害-2", en: "Impale the Night DMG-2")
            E23 locale!(zh_cn: "贯夜伤害-3", en: "Impale the Night DMG-3")
            E3 locale!(zh_cn: "流涌之刃伤害", en: "Surging Blade DMG")
        ),
        skill3: skill_map!(
            ClorindeDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_stack",
            title: locale!(
                zh_cn: "天赋1「破夜的明焰」层数",
                en: "Talent1 Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        },
        ItemConfig {
            name: "talent2_stack",
            title: locale!(
                zh_cn: "天赋2「契令的酬偿」层数",
                en: "Talent2 Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 }
        },
        ItemConfig {
            name: "c6_rate",
            title: locale!(
                zh_cn: "C6「为此，勿将希望弃扬」比例",
                en: "C6 Rate",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "bond_of_life",
            title: locale!(
                zh_cn: "生命之契百分比",
                en: "Bond of Life percentage"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 0.0 },
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ClorindeDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use ClorindeDamageEnum::*;
        let ratio = match s {
            Normal1 => CLORINDE_SKILL.normal_dmg1[s1],
            Normal2 => CLORINDE_SKILL.normal_dmg2[s1],
            Normal3 => CLORINDE_SKILL.normal_dmg3[s1],
            Normal4 => CLORINDE_SKILL.normal_dmg4[s1],
            Normal5 => CLORINDE_SKILL.normal_dmg5[s1],
            Charged => CLORINDE_SKILL.charged_dmg[s1],
            Plunging1 => CLORINDE_SKILL.plunging_dmg1[s1],
            Plunging2 => CLORINDE_SKILL.plunging_dmg2[s1],
            Plunging3 => CLORINDE_SKILL.plunging_dmg3[s1],
            E11 => CLORINDE_SKILL.e_dmg11[s2],
            E12 => CLORINDE_SKILL.e_dmg12[s2],
            E21 => CLORINDE_SKILL.e_dmg21[s2],
            E22 => CLORINDE_SKILL.e_dmg22[s2],
            E23 => CLORINDE_SKILL.e_dmg23[s2],
            E3 => CLORINDE_SKILL.e_dmg3[s2],
            Q1 => CLORINDE_SKILL.q_dmg1[s3],
        };
        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let bond_of_life = match *config {
            CharacterSkillConfig::Clorinde { bond_of_life } => bond_of_life,
            _ => 0.0
        };

        if s == Q1 && context.character_common_data.constellation >= 4 {
            let bonus = (bond_of_life * 100.0).floor() * 0.02;
            let bonus = bonus.min(2.0);
            builder.add_extra_bonus("C4「铭记泪，生命与仁爱」", bonus);
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
        match *config {
            CharacterConfig::Clorinde { talent1_stack, talent2_stack, c6_rate } => {
                Some(Box::new(ClorindeEffect {
                    talent1_stack,
                    talent2_stack,
                    c6_rate,
                    has_talent1: common_data.has_talent1,
                    has_talent2: common_data.has_talent2,
                    constellation: common_data.constellation as usize,
                }))
            },
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
