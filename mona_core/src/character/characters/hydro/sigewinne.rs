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

pub struct SigewinneSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub charged_dmg3: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_heal1: [f64; 15],
    pub e_heal1_fixed: [f64; 15],
    pub e_heal2: f64,
    pub e_dmg2: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const SIGEWINNE_SKILL: SigewinneSkillType = SigewinneSkillType {
    normal_dmg1: [0.5261, 0.569, 0.6118, 0.673, 0.7158, 0.7647, 0.832, 0.8993, 0.9666, 1.04, 1.1135, 1.1869, 1.2603, 1.3337, 1.4071],
    normal_dmg2: [0.5107, 0.5523, 0.5939, 0.6532, 0.6948, 0.7423, 0.8076, 0.873, 0.9383, 1.0095, 1.0808, 1.1521, 1.2233, 1.2946, 1.3659],
    normal_dmg3: [0.7829, 0.8466, 0.9104, 1.0014, 1.0651, 1.138, 1.2381, 1.3382, 1.4384, 1.5476, 1.6569, 1.7661, 1.8753, 1.9846, 2.0938],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.1408, 1.2264, 1.426, 1.55, 1.5116, 1.5971, 1.7112, 1.8253, 1.9394, 2.0534, 2.1675, 2.2816, 2.4242, 2.5668, 2.7094],
    charged_dmg3: [0.2282, 0.2453, 0.2852, 0.31, 0.3023, 0.3194, 0.3422, 0.3651, 0.3879, 0.4107, 0.4335, 0.4563, 0.4848, 0.5134, 0.5419],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [0.0228, 0.0245, 0.0262, 0.0285, 0.0302, 0.0319, 0.0342, 0.0365, 0.0388, 0.041, 0.0433, 0.0456, 0.0485, 0.0513, 0.0542],
    e_heal1: [0.028, 0.0301, 0.0322, 0.035, 0.0371, 0.0392, 0.042, 0.0448, 0.0476, 0.0504, 0.0532, 0.056, 0.0595, 0.063, 0.0665],
    e_heal1_fixed: [269.63, 296.6, 325.81, 357.27, 390.98, 426.94, 465.14, 505.59, 548.29, 593.23, 640.43, 689.87, 741.55, 795.49, 851.67],
    e_heal2: 0.5,
    e_dmg2: [0.0068, 0.0074, 0.0079, 0.0086, 0.0091, 0.0096, 0.0103, 0.0109, 0.0116, 0.0123, 0.013, 0.0137, 0.0145, 0.0154, 0.0162],
    q_dmg1: [0.1177, 0.1265, 0.1354, 0.1471, 0.156, 0.1648, 0.1766, 0.1883, 0.2001, 0.2119, 0.2236, 0.2354, 0.2501, 0.2648, 0.2796],
};

damage_enum!(
    SigewinneDamageEnum
    Normal1
    Normal2
    Normal3
    Charged1
    Charged2
    Charged3
    Plunging1
    Plunging2
    Plunging3
    E1
    E12
    E13
    EHeal1
    EHeal12
    EHeal13
    EHeal2
    E2
    Q1
);

impl SigewinneDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use SigewinneDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged1 | Charged2 | Charged3 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E12 | E13 | EHeal1 | EHeal12 | EHeal13 | EHeal2 | E2 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self) -> Element {
        use SigewinneDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Charged1 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            Charged2 | Charged3 | E1 | E12 | E13 | EHeal1 | EHeal12 | EHeal13 | EHeal2 | E2 | Q1 => Element::Hydro
        }
    }

    pub fn is_heal(&self) -> bool {
        use SigewinneDamageEnum::*;
        match *self {
            EHeal1 | EHeal12 | EHeal13 | EHeal2 => true,
            _ => false
        }
    }
}

pub struct SigewinneEffect {
    pub c6_rate: f64,
    pub constellation: usize,
}

impl<A: Attribute> ChangeAttribute<A> for SigewinneEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let r = self.c6_rate;
        if self.constellation >= 6 {
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::CriticalElementalBurst,
                Box::new(move |hp, _| {
                    ((hp / 1000.0).floor() * 0.004).min(0.2)
                }),
                Box::new(|_x, _y, _grad| (0.0, 0.0)),
                "C6「最光辉的精灵，可否为我祷告」"
            );
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::CriticalDamageElementalBurst,
                Box::new(move |hp, _| {
                    ((hp / 1000.0).floor() * 0.022).min(1.1)
                }),
                Box::new(|_x, _y, _grad| (0.0, 0.0)),
                "C6「最光辉的精灵，可否为我祷告」"
            );
        }
    }
}

pub struct Sigewinne;

impl CharacterTrait for Sigewinne {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Sigewinne,
        internal_name: "Sigewinne",
        name_locale: locale!(
            zh_cn: "希格雯",
            en: "Sigewinne"
        ),
        element: Element::Hydro,
        hp: [1039, 2695, 3586, 5366, 5999, 6902, 7747, 8659, 9292, 10213, 10846, 11777, 12410, 13348],
        atk: [15, 39, 52, 77, 87, 100, 112, 125, 134, 147, 156, 170, 179, 193],
        def: [39, 101, 134, 201, 225, 258, 290, 324, 348, 382, 406, 441, 464, 500],
        sub_stat: CharacterSubStatFamily::HP288,
        weapon_type: WeaponType::Bow,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·靶向治疗",
            en: "Normal Attack: Targeted Treatment"
        ),
        skill_name2: locale!(
            zh_cn: "弹跳水疗法",
            en: "Rebound Hydrotherapy"
        ),
        skill_name3: locale!(
            zh_cn: "过饱和心意注射",
            en: "Super Saturated Syringing"
        ),
    };
    type SkillType = SigewinneSkillType;
    const SKILL: Self::SkillType = SIGEWINNE_SKILL;
    type DamageEnumType = SigewinneDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            SigewinneDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged1 locale!(zh_cn: "瞄准射击", en: "Aimed Shot")
            Charged2 locale!(zh_cn: "满蓄力瞄准射击", en: "Fully-Charged Aimed Shot")
            Charged3 locale!(zh_cn: "小小关心气泡伤害", en: "Mini-Stration Bubble DMG")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            SigewinneDamageEnum
            E1 locale!(zh_cn: "激愈水球伤害", en: "Bolstering Bubblebalm DMG")
            E12 locale!(zh_cn: "激愈水球伤害-一级", en: "Bolstering Bubblebalm DMG-tier1")
            E13 locale!(zh_cn: "激愈水球伤害-二级", en: "Bolstering Bubblebalm DMG-tier2")
            EHeal1 locale!(zh_cn: "激愈水球治疗量", en: "Bolstering Bubblebalm Healing")
            EHeal12 locale!(zh_cn: "激愈水球治疗量-一级", en: "Bolstering Bubblebalm Healing-tier1")
            EHeal13 locale!(zh_cn: "激愈水球治疗量-二级", en: "Bolstering Bubblebalm Healing-tier2")
            EHeal2 locale!(zh_cn: "弹跳结束治疗量", en: "Final Bounce Healing")
            E2 locale!(zh_cn: "流涌之刃伤害", en: "Surging Blade DMG")
        ),
        skill3: skill_map!(
            SigewinneDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c6_rate",
            title: locale!(
                zh_cn: "C6「最光辉的精灵，可否为我祷告」比例",
                en: "C6 Rate",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: SigewinneDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use SigewinneDamageEnum::*;
        let is_heal = s.is_heal();
        let mut builder = D::new();
        let skill_type = s.get_skill_type();
        if is_heal {
            let ratio = match s {
                EHeal1 => SIGEWINNE_SKILL.e_heal1[s2],
                SigewinneDamageEnum::EHeal12 => SIGEWINNE_SKILL.e_heal1[s2] * 1.05,
                EHeal13 => SIGEWINNE_SKILL.e_heal1[s2] * 1.1,
                EHeal2 => SIGEWINNE_SKILL.e_heal2,
                _ => 0.0
            };
            let fixed = match s {
                EHeal1 => SIGEWINNE_SKILL.e_heal1_fixed[s2],
                SigewinneDamageEnum::EHeal12 => SIGEWINNE_SKILL.e_heal1_fixed[s2] * 1.05,
                EHeal13 => SIGEWINNE_SKILL.e_heal1_fixed[s2] * 1.1,
                _ => 0.0
            };
            builder.add_hp_ratio("技能倍率", ratio);
            if fixed > 0.0 {
                builder.add_extra_damage("技能倍率", fixed);
            }
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => SIGEWINNE_SKILL.normal_dmg1[s1],
                Normal2 => SIGEWINNE_SKILL.normal_dmg2[s1],
                Normal3 => SIGEWINNE_SKILL.normal_dmg3[s1],
                Charged1 => SIGEWINNE_SKILL.charged_dmg1[s1],
                Charged2 => SIGEWINNE_SKILL.charged_dmg2[s1],
                Charged3 => SIGEWINNE_SKILL.charged_dmg3[s1],
                Plunging1 => SIGEWINNE_SKILL.plunging_dmg1[s1],
                Plunging2 => SIGEWINNE_SKILL.plunging_dmg2[s1],
                Plunging3 => SIGEWINNE_SKILL.plunging_dmg3[s1],
                E1 => SIGEWINNE_SKILL.e_dmg1[s2],
                E12 => SIGEWINNE_SKILL.e_dmg1[s2] * 1.05,
                E13 => SIGEWINNE_SKILL.e_dmg1[s2] * 1.1,
                E2 => SIGEWINNE_SKILL.e_dmg2[s2],
                Q1 => SIGEWINNE_SKILL.q_dmg1[s3],
                _ => 0.0
            };
            if skill_type == SkillType::NormalAttack {
                builder.add_atk_ratio("技能倍率", ratio);
            } else {
                builder.add_hp_ratio("技能倍率", ratio);
            }
            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                skill_type,
                context.character_common_data.level,
                fumo
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Sigewinne { c6_rate } => {
                Some(Box::new(SigewinneEffect {
                    c6_rate,
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
