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

pub struct FreminetSkillType {
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
    pub e_dmg_shatter0: [f64; 15],
    pub e_dmg_shatter1_cryo: [f64; 15],
    pub e_dmg_shatter1_physical: [f64; 15],
    pub e_dmg_shatter2_cryo: [f64; 15],
    pub e_dmg_shatter2_physical: [f64; 15],
    pub e_dmg_shatter3_cryo: [f64; 15],
    pub e_dmg_shatter3_physical: [f64; 15],
    pub e_dmg_shatter4: [f64; 15],
    pub e_dmg3: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const FREMINET_SKILL: FreminetSkillType = FreminetSkillType {
    normal_dmg1: [0.8424, 0.9109, 0.9795, 1.0775, 1.146, 1.2244, 1.3321, 1.4399, 1.5476, 1.6652, 1.7827, 1.9002, 2.0178, 2.1353, 2.2529],
    normal_dmg2: [0.8068, 0.8724, 0.9381, 1.0319, 1.0976, 1.1726, 1.2758, 1.379, 1.4822, 1.5948, 1.7073, 1.8199, 1.9325, 2.045, 2.1576],
    normal_dmg3: [1.019, 1.102, 1.1849, 1.3034, 1.3864, 1.4812, 1.6115, 1.7418, 1.8722, 2.0144, 2.1566, 2.2988, 2.441, 2.5831, 2.7253],
    normal_dmg4: [1.238, 1.3388, 1.4396, 1.5835, 1.6843, 1.7995, 1.9578, 2.1162, 2.2746, 2.4473, 2.6201, 2.7928, 2.9656, 3.1383, 3.3111],
    charged_dmg1: [0.6252, 0.6761, 0.727, 0.7997, 0.8506, 0.9087, 0.9887, 1.0687, 1.1487, 1.2359, 1.3231, 1.4104, 1.4976, 1.5849, 1.6721],
    charged_dmg2: [1.1309, 1.2229, 1.315, 1.4465, 1.5386, 1.6437, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933, 2.5511, 2.7089, 2.8667, 3.0245],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    e_dmg1: [0.8304, 0.8927, 0.955, 1.038, 1.1003, 1.1626, 1.2456, 1.3286, 1.4117, 1.4947, 1.5778, 1.6608, 1.7646, 1.8684, 1.9722],
    e_dmg2: [0.0716, 0.077, 0.0823, 0.0895, 0.0949, 0.1002, 0.1074, 0.1146, 0.1217, 0.1289, 0.136, 0.1432, 0.1522, 0.1611, 0.17],
    e_dmg_shatter0: [2.0048, 2.1552, 2.3055, 2.506, 2.6564, 2.8067, 3.0072, 3.2077, 3.4082, 3.6086, 3.8091, 4.0096, 4.2602, 4.5108, 4.7614],
    e_dmg_shatter1_cryo: [1.0024, 1.0776, 1.1528, 1.253, 1.3282, 1.4034, 1.5036, 1.6038, 1.7041, 1.8043, 1.9046, 2.0048, 2.1301, 2.2554, 2.3807],
    e_dmg_shatter1_physical: [0.4869, 0.5234, 0.5599, 0.6086, 0.6451, 0.6816, 0.7303, 0.779, 0.8277, 0.8764, 0.9251, 0.9738, 1.0346, 1.0955, 1.1563],
    e_dmg_shatter2_cryo: [0.7017, 0.7543, 0.8069, 0.8771, 0.9297, 0.9824, 1.0525, 1.1227, 1.1929, 1.263, 1.3332, 1.4034, 1.4911, 1.5788, 1.6665],
    e_dmg_shatter2_physical: [0.852, 0.9159, 0.9798, 1.0651, 1.129, 1.1929, 1.2781, 1.3633, 1.4485, 1.5337, 1.6189, 1.7041, 1.8106, 1.9171, 2.0236],
    e_dmg_shatter3_cryo: [0.401, 0.431, 0.4611, 0.5012, 0.5313, 0.5613, 0.6014, 0.6415, 0.6816, 0.7217, 0.7618, 0.8019, 0.852, 0.9022, 0.9523],
    e_dmg_shatter3_physical: [1.2172, 1.3085, 1.3998, 1.5215, 1.6128, 1.7041, 1.8258, 1.9475, 2.0692, 2.191, 2.3127, 2.4344, 2.5865, 2.7387, 2.8909],
    e_dmg_shatter4: [2.4344, 2.617, 2.7996, 3.043, 3.2256, 3.4082, 3.6516, 3.895, 4.1385, 4.3819, 4.6254, 4.8688, 5.1731, 5.4774, 5.7817],
    e_dmg3: [0.144, 0.1548, 0.1656, 0.18, 0.1908, 0.2016, 0.216, 0.2304, 0.2448, 0.2592, 0.2736, 0.288, 0.306, 0.324, 0.342],
    q_dmg1: [3.184, 3.4228, 3.6616, 3.98, 4.2188, 4.4576, 4.776, 5.0944, 5.4128, 5.7312, 6.0496, 6.368, 6.766, 7.164, 7.562],
};

damage_enum!(
    FreminetDamageEnum
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
    EShatter0
    EShatter1Cryo
    EShatter1Physical
    EShatter2Cryo
    EShatter2Physical
    EShatter3Cryo
    EShatter3Physical
    EShatter4
    E3
    Q1
    E2Mul2
);

impl FreminetDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use FreminetDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | EShatter0 | EShatter1Cryo | EShatter1Physical | EShatter2Cryo | EShatter2Physical
                | EShatter3Cryo | EShatter3Physical | EShatter4 | E2Mul2 | E3 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
        }
    }

    pub fn get_element(&self) -> Element {
        use FreminetDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Charged1 | Charged2 | Plunging1 | Plunging2 | Plunging3
                | EShatter1Physical | EShatter2Physical | EShatter3Physical | EShatter4 => Element::Physical,
            E1 | E2 | EShatter0 | EShatter1Cryo | EShatter2Cryo | EShatter3Cryo | E2Mul2 | Q1 | E3 => Element::Cryo
        }
    }

    pub fn is_shatter(&self) -> bool {
        use FreminetDamageEnum::*;
        match *self {
            EShatter0 | EShatter1Cryo | EShatter1Physical | EShatter2Cryo | EShatter2Physical
            | EShatter3Cryo | EShatter3Physical | EShatter4 => true,
            _ => false,
        }
    }
}

pub struct FreminetEffect {
    pub c4_stack: f64,
    pub c6_stack: f64,
    pub constellation: usize,
}

impl<A: Attribute> ChangeAttribute<A> for FreminetEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.constellation >= 4 {
            let rate = self.c4_stack * 0.09;
            attribute.add_atk_percentage("命座4「雪月与芦笛之舞」", rate);
        }
        if self.constellation >= 6 {
            let rate = self.c6_stack * 0.12;
            attribute.set_value_by(AttributeName::CriticalDamageBase, "命座6「梦晓与决意之刻」", rate);
        }
    }
}

pub struct Freminet;

impl CharacterTrait for Freminet {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Freminet,
        internal_name: "Freminet",
        name_locale: locale!(
            zh_cn: "菲米尼",
            en: "Freminet",
        ),
        element: Element::Cryo,
        hp: [1012, 2600, 3356, 5027, 5564, 6400, 7117, 7953, 8490, 9325, 9862, 10698, 11235, 12071],
        atk: [21, 55, 71, 106, 117, 135, 150, 168, 179, 197, 208, 226, 237, 255],
        def: [59, 153, 197, 295, 327, 376, 418, 467, 498, 547, 579, 628, 659, 708],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Claymore,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·洑流剑",
            en: "Normal Attack: Flowing Eddies"
        ),
        skill_name2: locale!(
            zh_cn: "浮冰增压",
            en: "Pressurized Floe"
        ),
        skill_name3: locale!(
            zh_cn: "猎影潜袭",
            en: "Shadowhunter's Ambush"
        )
    };
    type SkillType = FreminetSkillType;
    const SKILL: Self::SkillType = FREMINET_SKILL;
    type DamageEnumType = FreminetDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            FreminetDamageEnum
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
            FreminetDamageEnum
            E1 locale!(zh_cn: "上挑攻击伤害", en: "Upward Thrust DMG")
            E2 locale!(zh_cn: "霜寒伤害", en: "Frost DMG")
            EShatter0 locale!(zh_cn: "零阶高压粉碎伤害", en: "Level 0 Shattering Pressure DMG")
            EShatter1Cryo locale!(zh_cn: "一阶高压粉碎伤害（冰）", en: "Level 1 Shattering Pressure DMG (Cryo)")
            EShatter1Physical locale!(zh_cn: "一阶高压粉碎伤害（物理）", en: "Level 1 Shattering Pressure DMG (Physical)")
            EShatter2Cryo locale!(zh_cn: "二阶高压粉碎伤害（冰）", en: "Level 2 Shattering Pressure DMG (Cryo)")
            EShatter2Physical locale!(zh_cn: "二阶高压粉碎伤害（物理）", en: "Level 2 Shattering Pressure DMG (Physical)")
            EShatter3Cryo locale!(zh_cn: "三阶高压粉碎伤害（冰）", en: "Level 3 Shattering Pressure DMG (Cryo)")
            EShatter3Physical locale!(zh_cn: "三阶高压粉碎伤害（物理）", en: "Level 3 Shattering Pressure DMG (Physical)")
            EShatter4 locale!(zh_cn: "四阶高压粉碎伤害", en: "Level 4 Shattering Pressure DMG")
            E3 locale!(zh_cn: "灵息之刺伤害", en: "Spiritbreath Thorn DMG")
            E2Mul2 locale!(zh_cn: "霜寒伤害（x2）", en: "Frost DMG (x2)")
        ),
        skill3: skill_map!(
            FreminetDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c4_stack",
            title: locale!(
                zh_cn: "命座4「雪月与芦笛之舞」层数",
                en: "C4「Dance of the Snowy Moon and Flute」Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 }
        },
        ItemConfig {
            name: "c6_stack",
            title: locale!(
                zh_cn: "命座6「梦晓与决意之刻」层数",
                en: "C6「Moment of Waking and Resolve」Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_rate",
            title: locale!(
                zh_cn: "天赋「并流式冷凝机关」比例",
                en: "Talent「Parallel Condensers」Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: FreminetDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use FreminetDamageEnum::*;

        let mut builder = D::new();
        let ratio = match s {
            Normal1 => FREMINET_SKILL.normal_dmg1[s1],
            Normal2 => FREMINET_SKILL.normal_dmg2[s1],
            Normal3 => FREMINET_SKILL.normal_dmg3[s1],
            Normal4 => FREMINET_SKILL.normal_dmg4[s1],
            Charged1 => FREMINET_SKILL.charged_dmg1[s1],
            Charged2 => FREMINET_SKILL.charged_dmg2[s1],
            Plunging1 => FREMINET_SKILL.plunging_dmg1[s1],
            Plunging2 => FREMINET_SKILL.plunging_dmg2[s1],
            Plunging3 => FREMINET_SKILL.plunging_dmg3[s1],
            E1 => FREMINET_SKILL.e_dmg1[s2],
            E2 => FREMINET_SKILL.e_dmg2[s2],
            EShatter0 => FREMINET_SKILL.e_dmg_shatter0[s2],
            EShatter1Cryo => FREMINET_SKILL.e_dmg_shatter1_cryo[s2],
            EShatter1Physical => FREMINET_SKILL.e_dmg_shatter1_physical[s2],
            EShatter2Cryo => FREMINET_SKILL.e_dmg_shatter2_cryo[s2],
            EShatter2Physical => FREMINET_SKILL.e_dmg_shatter2_physical[s2],
            EShatter3Cryo => FREMINET_SKILL.e_dmg_shatter3_cryo[s2],
            EShatter3Physical => FREMINET_SKILL.e_dmg_shatter3_physical[s2],
            EShatter4 => FREMINET_SKILL.e_dmg_shatter4[s2],
            E3 => FREMINET_SKILL.e_dmg3[s2],
            Q1 => FREMINET_SKILL.q_dmg1[s3],
            E2Mul2 => FREMINET_SKILL.e_dmg2[s2] * 2.0
        };

        let is_shatter = s.is_shatter();
        let talent2_rate = match *config {
            CharacterSkillConfig::Freminet { talent2_rate } => talent2_rate,
            _ => 0.0
        };
        if context.character_common_data.has_talent2 && is_shatter {
            builder.add_extra_bonus("天赋「并流式冷凝机关」", talent2_rate * 0.4);
        }

        // c1
        if is_shatter && context.character_common_data.constellation >= 1 {
            builder.add_extra_critical("1命「深水与泡沫之梦」", 0.15);
        }

        builder.add_atk_ratio("技能倍率", ratio);

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
        let (a, b) = match *config {
            CharacterConfig::Freminet { c4_stack, c6_stack } => (c4_stack, c6_stack),
            _ => (0.0, 0.0)
        };
        Some(Box::new(FreminetEffect {
            c4_stack: a,
            c6_stack: b,
            constellation: common_data.constellation as usize,
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}