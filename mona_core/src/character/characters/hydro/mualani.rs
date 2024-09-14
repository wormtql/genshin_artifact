use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::navia::NaviaDamageEnum;
use crate::character::characters::navia::NaviaDamageEnum::{Plunging1, Plunging2, Plunging3};
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

pub struct MualaniSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_bonus1: [f64; 15],
    pub e_bonus2: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const MUALANI_SKILL: MualaniSkillType = MualaniSkillType {
    normal_dmg1: [0.514, 0.5525, 0.5911, 0.6425, 0.681, 0.7195, 0.7709, 0.8223, 0.8737, 0.9251, 0.9765, 1.0279, 1.0922, 1.1564, 1.2207],
    normal_dmg2: [0.4463, 0.4797, 0.5132, 0.5578, 0.5913, 0.6248, 0.6694, 0.714, 0.7586, 0.8033, 0.8479, 0.8925, 0.9483, 1.0041, 1.0599],
    normal_dmg3: [0.7003, 0.7529, 0.8054, 0.8754, 0.928, 0.9805, 1.0505, 1.1206, 1.1906, 1.2606, 1.3307, 1.4007, 1.4882, 1.5758, 1.6633],
    charged_dmg: [1.4288, 1.536, 1.6431, 1.786, 1.8932, 2.0003, 2.1432, 2.2861, 2.429, 2.5718, 2.7147, 2.8576, 3.0362, 3.2148, 3.3934],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [0.0868, 0.0933, 0.0998, 0.1085, 0.115, 0.1215, 0.1302, 0.1389, 0.1476, 0.1562, 0.1649, 0.1736, 0.1845, 0.1953, 0.2062],
    e_bonus1: [0.0434, 0.0467, 0.0499, 0.0543, 0.0575, 0.0608, 0.0651, 0.0694, 0.0738, 0.0781, 0.0825, 0.0868, 0.0922, 0.0977, 0.1031],
    e_bonus2: [0.217, 0.2333, 0.2496, 0.2713, 0.2875, 0.3038, 0.3255, 0.3472, 0.3689, 0.3906, 0.4123, 0.434, 0.4611, 0.4883, 0.5154],
    q_dmg1: [0.5844, 0.6282, 0.6721, 0.7305, 0.7743, 0.8181, 0.8766, 0.935, 0.9935, 1.0519, 1.1103, 1.1688, 1.2418, 1.3149, 1.3879],
};

damage_enum!(
    MualaniDamageEnum
    Normal1
    Normal2
    Normal3
    Charged
    Plunging1
    Plunging2
    Plunging3
    Q1
    A_C1
    A_Stack0
    A_Stack1
    A_Stack2
    A_Stack3
    A_Stack4
);

impl MualaniDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use MualaniDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | A_C1 | A_Stack1 | A_Stack2 | A_Stack3 | A_Stack4 | A_Stack0 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Q1 => SkillType::ElementalBurst
        }
    }

    pub fn is_atk_ratio(&self) -> bool {
        use MualaniDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Charged | Plunging1 | Plunging2 | Plunging3 => true,
            _ => false
        }
    }

    pub fn is_after_e(&self) -> bool {
        use MualaniDamageEnum::*;
        match *self {
            A_Stack0 | A_Stack1 | A_Stack2 | A_Stack3 | A_Stack4 | A_C1 => true,
            _ => false
        }
    }
}

pub struct MualaniEffect {
    pub talent2_stack: f64,
}

impl<A: Attribute> ChangeAttribute<A> for MualaniEffect {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::USER1, "Mualani Stack", self.talent2_stack);
    }
}

pub struct Mualani;

impl CharacterTrait for Mualani {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Mualani,
        internal_name: "Mualani",
        name_locale: locale!(
            zh_cn: "玛拉妮",
            en: "Mualani"
        ),
        element: Element::Hydro,
        hp: [1182, 3066, 4080, 6105, 6825, 7852, 8813, 9850, 10571, 11618, 12338, 13397, 14117, 15185],
        atk: [14, 37, 49, 73, 82, 94, 105, 118, 127, 139, 148, 160, 169, 182],
        def: [44, 115, 153, 229, 256, 295, 331, 370, 397, 436, 463, 503, 530, 570],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·降温处理",
            en: "Normal Attack: Cooling Treatment"
        ),
        skill_name2: locale!(
            zh_cn: "踏鲨破浪",
            en: "Surfshark Wavebreaker"
        ),
        skill_name3: locale!(
            zh_cn: "爆瀑飞弹",
            en: "Boomsharka-laka"
        ),
    };
    type SkillType = MualaniSkillType;
    const SKILL: Self::SkillType = MUALANI_SKILL;
    type DamageEnumType = MualaniDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            MualaniDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            MualaniDamageEnum
            A_C1 locale!(zh_cn: "巨浪鲨鲨撕咬伤害-C1", en: "")
            A_Stack0 locale!(zh_cn: "鲨鲨撕咬基础伤害", en: "Sharky's Bite Base DMG")
            A_Stack1 locale!(zh_cn: "鲨鲨撕咬伤害-1", en: "Sharky's Bite DMG-1")
            A_Stack2 locale!(zh_cn: "鲨鲨撕咬伤害-2", en: "Sharky's Bite DMG-2")
            A_Stack3 locale!(zh_cn: "鲨鲨撕咬伤害-3", en: "Sharky's Bite DMG-3")
            A_Stack4 locale!(zh_cn: "巨浪鲨鲨撕咬伤害", en: "Sharky's Surging Bite DMG")
        ),
        skill3: skill_map!(
            MualaniDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_stack",
            title: locale!(
                zh_cn: "逐浪心得层数",
                en: "Wavechaser's Exploits Stack"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        }
    ]);

    // #[cfg(not(target_family = "wasm"))]
    // const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
    //     ItemConfig {
    //         name: "after_e",
    //         title: locale!(
    //             zh_cn: "夜魂加持",
    //             en: "Nightsoul's Blessing"
    //         ),
    //         config: ItemConfigType::Bool { default: false }
    //     }
    // ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: MualaniDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let mut builder = D::new();
        use MualaniDamageEnum::*;
        let ratio = match s {
            Normal1 => MUALANI_SKILL.normal_dmg1[s1],
            Normal2 => MUALANI_SKILL.normal_dmg2[s1],
            Normal3 => MUALANI_SKILL.normal_dmg3[s1],
            Charged => MUALANI_SKILL.charged_dmg[s1],
            Plunging1 => MUALANI_SKILL.plunging_dmg1[s1],
            Plunging2 => MUALANI_SKILL.plunging_dmg2[s1],
            Plunging3 => MUALANI_SKILL.plunging_dmg3[s1],
            Q1 => MUALANI_SKILL.q_dmg1[s3],
            A_Stack0 => MUALANI_SKILL.e_dmg1[s2],
            A_Stack1 => MUALANI_SKILL.e_dmg1[s2] + MUALANI_SKILL.e_bonus1[s2],
            A_Stack2 => MUALANI_SKILL.e_dmg1[s2] + 2.0 * MUALANI_SKILL.e_bonus1[s2],
            A_Stack3 => MUALANI_SKILL.e_dmg1[s2] + 3.0 * MUALANI_SKILL.e_bonus1[s2],
            A_Stack4 | A_C1 => MUALANI_SKILL.e_dmg1[s2] + 3.0 * MUALANI_SKILL.e_bonus1[s2] + MUALANI_SKILL.e_bonus2[s2],
            // A_C1 => 0.0
        };

        // base
        if s.is_atk_ratio() {
            builder.add_atk_ratio("技能倍率", ratio);
        } else {
            builder.add_hp_ratio("技能倍率", ratio);
        }

        // C1 effects
        if s == A_C1 && context.character_common_data.constellation >= 1 {
            builder.add_hp_ratio("C1「悠闲的「梅兹特利」…」", 0.66);
        }

        let is_after_e = s.is_after_e();

        // talent2
        if context.character_common_data.has_talent2 && s == Q1 {
            let talent2_stack = context.attribute.get_value(AttributeName::USER1);
            let bonus = talent2_stack * 0.15;
            if bonus > 0.0 {
                builder.add_hp_ratio("天赋「纳塔最好的向导」", bonus);
            }
        }

        // C4
        if context.character_common_data.constellation >= 4 && s == Q1 {
            builder.add_extra_bonus("C4「鲨鲨主食是豚豚。」", 0.75);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Hydro,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Mualani { talent2_stack } => Some(Box::new(MualaniEffect {
                talent2_stack
            })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
