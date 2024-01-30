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

pub struct NaviaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const NAVIA_SKILL: NaviaSkillType = NaviaSkillType {
    normal_dmg1: [0.9352, 1.0113, 1.0874, 1.1962, 1.2723, 1.3593, 1.4789, 1.5985, 1.7181, 1.8486, 1.9791, 2.1096, 2.2401, 2.3706, 2.5011],
    normal_dmg2: [0.8651, 0.9355, 1.0059, 1.1065, 1.1769, 1.2574, 1.368, 1.4787, 1.5893, 1.71, 1.8307, 1.9514, 2.0721, 2.1928, 2.3135],
    normal_dmg3: [0.3489, 0.3773, 0.4056, 0.4462, 0.4746, 0.5071, 0.5517, 0.5963, 0.6409, 0.6896, 0.7383, 0.787, 0.8356, 0.8843, 0.933],
    normal_dmg4: [1.3343, 1.4429, 1.5515, 1.7067, 1.8153, 1.9394, 2.1101, 2.2807, 2.4514, 2.6376, 2.8238, 3.01, 3.1962, 3.3823, 3.5685],
    charged_dmg1: [0.6252, 0.6761, 0.727, 0.7997, 0.8506, 0.9087, 0.9887, 1.0687, 1.1487, 1.2359, 1.3231, 1.4104, 1.4976, 1.5849, 1.6721],
    charged_dmg2: [1.1309, 1.2229, 1.315, 1.4465, 1.5386, 1.6437, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933, 2.5511, 2.7089, 2.8667, 3.0245],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    e_dmg1: [3.948, 4.2441, 4.5402, 4.935, 5.2311, 5.5272, 5.922, 6.3168, 6.7116, 7.1064, 7.5012, 7.896, 8.3895, 8.883, 9.3765],
    e_dmg2: [0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765, 0.81, 0.855],
    q_dmg1: [0.752, 0.8084, 0.8648, 0.94, 0.9964, 1.0528, 1.128, 1.2032, 1.2784, 1.3536, 1.4288, 1.504, 1.598, 1.692, 1.786],
    q_dmg2: [0.4315, 0.4639, 0.4962, 0.5394, 0.5717, 0.6041, 0.6472, 0.6904, 0.7336, 0.7767, 0.8199, 0.863, 0.9169, 0.9709, 1.0248],
};

damage_enum!(
    NaviaDamageEnum
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
    E1Total
    E2
    Q1
    Q2
);

impl NaviaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use NaviaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E1Total | E2 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self, fumo: bool) -> Element {
        use NaviaDamageEnum::*;
        if fumo {
            return Element::Geo;
        }
        match *self {
            E1 | E1Total | E2 | Q1 | Q2 => Element::Geo,
            _ => Element::Physical
        }
    }
}

pub struct NaviaEffect {
    pub talent2_character_count: usize,
    pub has_talent2: bool,
}

impl<A: Attribute> ChangeAttribute<A> for NaviaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent2 {
            attribute.add_atk_percentage("天赋2「互助关系网」", self.talent2_character_count as f64 * 0.2);
        }
    }
}

pub struct Navia;

impl CharacterTrait for Navia {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Navia,
        internal_name: "Navia",
        name_locale: locale!(
            zh_cn: "娜维娅",
            en: "Navia"
        ),
        element: Element::Geo,
        hp: [985, 2555, 3399, 5086, 5686, 6542, 7341, 8206, 8806, 9679, 10278, 11161, 11761, 12650],
        atk: [27, 71, 94, 141, 158, 182, 204, 228, 245, 269, 286, 310, 327, 352],
        def: [62, 160, 213, 319, 356, 410, 460, 515, 552, 607, 644, 700, 737, 793],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Claymore,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·直率的辞绝",
            en: "Normal Attack: Blunt Refusal"
        ),
        skill_name2: locale!(
            zh_cn: "典仪式晶火",
            en: "Ceremonial Crystalshot"
        ),
        skill_name3: locale!(
            zh_cn: "如霰澄天的鸣礼",
            en: "As the Sunlit Sky's Singing Salute"
        )
    };
    type SkillType = NaviaSkillType;
    const SKILL: Self::SkillType = NAVIA_SKILL;
    type DamageEnumType = NaviaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            NaviaDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged1 charged_dmg!("loop1")
            Charged2 charged_dmg!("loop2")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            NaviaDamageEnum
            E1 locale!(zh_cn: "玫瑰晶弹基础伤害", en: "Rosula Shardshot Base DMG")
            E1Total locale!(zh_cn: "玫瑰晶弹总伤", en: "Rosula Shardshot Total DMG")
            E2 locale!(zh_cn: "流涌之刃伤害", en: "Surging Blade DMG")
        ),
        skill3: skill_map!(
            NaviaDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "支援炮击伤害", en: "Cannon Fire Support DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_character_count",
            title: locale!(zh_cn: "火/雷/冰/水角色数量", en: "Pyro/Electro/Cryo/Hydro Character Count"),
            config: ItemConfigType::Int { min: 0, max: 3, default: 2 },
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "shard_count",
            title: locale!(zh_cn: "「裂晶弹片」消耗量", en: "Rosula Shardshot Consumption"),
            config: ItemConfigType::Int { min: 0, max: 6, default: 3 },
        },
        ItemConfig {
            name: "strike11",
            title: locale!(zh_cn: "全部命中", en: "All 11 Strike"),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "after_e",
            title: locale!(zh_cn: "岩附魔", en: "After E"),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: NaviaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use NaviaDamageEnum::*;
        let mut ratio = match s {
            Normal1 => NAVIA_SKILL.normal_dmg1[s1],
            Normal2 => NAVIA_SKILL.normal_dmg2[s1],
            Normal3 => NAVIA_SKILL.normal_dmg3[s1],
            Normal4 => NAVIA_SKILL.normal_dmg4[s1],
            Charged1 => NAVIA_SKILL.charged_dmg1[s1],
            Charged2 => NAVIA_SKILL.charged_dmg1[s1],
            Plunging1 => NAVIA_SKILL.plunging_dmg1[s1],
            Plunging2 => NAVIA_SKILL.plunging_dmg2[s1],
            Plunging3 => NAVIA_SKILL.plunging_dmg3[s1],
            E1 => NAVIA_SKILL.e_dmg1[s2],
            E1Total => NAVIA_SKILL.e_dmg1[s2],
            E2 => NAVIA_SKILL.e_dmg2[s2],
            Q1 => NAVIA_SKILL.q_dmg1[s3],
            Q2 => NAVIA_SKILL.q_dmg2[s3],
        };

        let (shard_count, strike11, after_e) = match *config {
            CharacterSkillConfig::Navia { shard_count, strike11, after_e } => (shard_count, strike11, after_e),
            _ => (0, false, false)
        };

        let mut builder = D::new();

        if s == E1Total {
            ratio = ratio * 2.0;
            // let count = match shard_count {
            //     0 => 5,
            //     1 => 7,
            //     2 => 9,
            //     _ => 11
            // };
            // ratio = ratio * count as f64;
            // let mut bonus = 0.15 * (shard_count - 3).max(0) as f64 * count as f64;
            // if strike11 && count == 11 {
            //     ratio *= 2.0;
            //     bonus *= 2.0;
            // }
            // if shard_count > 3 {
            //     builder.add_atk_ratio("「裂晶弹片」加成", bonus);
            // }
        }
        builder.add_atk_ratio("技能倍率", ratio);

        if (s == E1 || s == E1Total) && shard_count > 3 {
            builder.add_extra_bonus("「裂晶弹片」加成", 0.15 * (shard_count - 3) as f64);
        }
        if context.character_common_data.constellation >= 2 && (s == E1 || s == E1Total) {
            let crit_bonus = 0.36_f64.min(shard_count as f64 * 0.12);
            builder.add_extra_critical("C2「总指挥的乘胜追击」", crit_bonus);
        }
        if context.character_common_data.constellation >= 6 && (s == E1 || s == E1Total) && shard_count > 3 {
            let crit_damage_bonus = 0.45 * (shard_count - 3) as f64;
            builder.add_extra_critical_damage("C6「刺玫会长的灵活手腕」", crit_damage_bonus);
        }

        let skill_type = s.get_skill_type();
        let element = s.get_element(after_e);
        if skill_type == SkillType::NormalAttack || skill_type == SkillType::ChargedAttack || skill_type.is_plunging() {
            if after_e {
                builder.add_extra_bonus("天赋「不明流通渠道」", 0.4);
            }
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
        let count = match *config {
            CharacterConfig::Navia { talent2_character_count } => talent2_character_count,
            _ => 0
        };
        Some(Box::new(NaviaEffect {
            talent2_character_count: count,
            has_talent2: common_data.has_talent2
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
