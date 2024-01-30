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

pub struct FurinaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    pub a_dmg1: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],
    pub e_dmg4: [f64; 15],
    pub e_heal1: [f64; 15],
    pub e_heal1_fixed: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_bonus1: [f64; 15],
    pub q_bonus2: [f64; 15],
}

pub const FURINA_SKILL: FurinaSkillType = FurinaSkillType {
    normal_dmg1: [0.4839, 0.5232, 0.5626, 0.6189, 0.6583, 0.7033, 0.7652, 0.8271, 0.889, 0.9565, 1.024, 1.0915, 1.159, 1.2265, 1.294],
    normal_dmg2: [0.4373, 0.4729, 0.5085, 0.5593, 0.5949, 0.6356, 0.6915, 0.7475, 0.8034, 0.8644, 0.9254, 0.9865, 1.0475, 1.1085, 1.1695],
    normal_dmg3: [0.5512, 0.5961, 0.6409, 0.705, 0.7499, 0.8012, 0.8717, 0.9422, 1.0127, 1.0896, 1.1665, 1.2434, 1.3203, 1.3972, 1.4741],
    normal_dmg4: [0.733, 0.7926, 0.8523, 0.9375, 0.9972, 1.0654, 1.1591, 1.2529, 1.3466, 1.4489, 1.5512, 1.6535, 1.7557, 1.858, 1.9603],
    charged_dmg: [0.7422, 0.8026, 0.863, 0.9493, 1.0097, 1.0788, 1.1737, 1.2686, 1.3635, 1.4671, 1.5707, 1.6742, 1.7778, 1.8813, 1.9849],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    a_dmg1: [0.0946, 0.1023, 0.11, 0.121, 0.1287, 0.1375, 0.1496, 0.1617, 0.1738, 0.187, 0.2002, 0.2134, 0.2266, 0.2398, 0.253],
    e_dmg1: [0.0786, 0.0845, 0.0904, 0.0983, 0.1042, 0.1101, 0.118, 0.1258, 0.1337, 0.1416, 0.1494, 0.1573, 0.1671, 0.1769, 0.1868],
    e_dmg2: [0.0596, 0.0641, 0.0685, 0.0745, 0.079, 0.0834, 0.0894, 0.0954, 0.1013, 0.1073, 0.1132, 0.1192, 0.1267, 0.1341, 0.1416],
    e_dmg3: [0.0323, 0.0347, 0.0372, 0.0404, 0.0428, 0.0452, 0.0485, 0.0517, 0.0549, 0.0582, 0.0614, 0.0646, 0.0687, 0.0727, 0.0768],
    e_dmg4: [0.0829, 0.0891, 0.0953, 0.1036, 0.1098, 0.116, 0.1243, 0.1326, 0.1409, 0.1492, 0.1575, 0.1658, 0.1761, 0.1865, 0.1968],
    e_heal1: [0.048, 0.0516, 0.0552, 0.06, 0.0636, 0.0672, 0.072, 0.0768, 0.0816, 0.0864, 0.0912, 0.096, 0.102, 0.108, 0.114],
    e_heal1_fixed: [462.23, 508.45, 558.54, 612.47, 670.26, 731.89, 797.39, 866.73, 939.92, 1016.97, 1097.87, 1182.63, 1271.23, 1363.69, 1460.0],
    q_dmg1: [0.1141, 0.1226, 0.1312, 0.1426, 0.1511, 0.1597, 0.1711, 0.1825, 0.1939, 0.2053, 0.2167, 0.2281, 0.2424, 0.2566, 0.2709],
    q_bonus1: [0.0007, 0.0009, 0.0011, 0.0013, 0.0015, 0.0017, 0.0019, 0.0021, 0.0023, 0.0025, 0.0027, 0.0029, 0.0031, 0.0033, 0.0035],
    q_bonus2: [0.0001, 0.0002, 0.0003, 0.0004, 0.0005, 0.0006, 0.0007, 0.0008, 0.0009, 0.001, 0.0011, 0.0012, 0.0013, 0.0014, 0.0015],
};

damage_enum!(
    FurinaDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    A1
    E1
    E2
    E3
    E4
    EHeal1
    Q1
);

impl FurinaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use FurinaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | A1 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4 | EHeal1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self) -> Element {
        use FurinaDamageEnum::*;
        match *self {
            E1 | E2 | E3 | E4 | Q1 => Element::Hydro,
            _ => Element::Physical
        }
    }

    pub fn is_hp_ratio(&self) -> bool {
        use FurinaDamageEnum::*;
        match *self {
            E1 | E2 | E3 | E4 | EHeal1 | Q1 => true,
            _ => false
        }
    }

    pub fn is_heal(&self) -> bool {
        use FurinaDamageEnum::*;
        match *self {
            EHeal1 => true,
            _ => false
        }
    }
}

pub struct FurinaEffect {
    pub c2_overflow: f64,
}

impl<A: Attribute> ChangeAttribute<A> for FurinaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let ratio = (self.c2_overflow * 0.0035).min(1.4);
        attribute.add_hp_percentage("C2「女人皆善变，仿若水中萍。」", ratio);
    }
}

pub struct Furina;

impl CharacterTrait for Furina {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Furina,
        internal_name: "Furina",
        name_locale: locale!(
            zh_cn: "芙宁娜",
            en: "Furina"
        ),
        element: Element::Hydro,
        hp: [1192, 3091, 4113, 6154, 6880, 7916, 8884, 9930, 10656, 11712, 12438, 13505, 14231, 15307],
        atk: [19, 49, 66, 98, 110, 126, 142, 158, 170, 187, 198, 215, 227, 244],
        def: [54, 140, 187, 280, 313, 360, 404, 451, 484, 532, 565, 614, 647, 696],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·独舞之邀",
            en: "Normal Attack: Soloist's Solicitation"
        ),
        skill_name2: locale!(
            zh_cn: "孤心沙龙",
            en: "Salon Solitaire"
        ),
        skill_name3: locale!(
            zh_cn: "万众狂欢",
            en: "Let the People Rejoice"
        )
    };
    type SkillType = FurinaSkillType;
    const SKILL: Self::SkillType = FURINA_SKILL;
    type DamageEnumType = FurinaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp_above50_count",
            title: locale!(
                zh_cn: "生命值高于50%的角色数量",
                en: "HP>=50% Characters Count"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 3 }
        },
        ItemConfig {
            name: "c6_after_e",
            title: locale!(
                zh_cn: "「万众瞩目」",
                en: "\"Center of Attention\""
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "c6_pneuma",
            title: locale!(
                zh_cn: "C6始基力：芒性",
                en: "C6 Arkhe: Pneuma"
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c2_overflow",
            title: locale!(
                zh_cn: "「气氛值」超过上限的部分",
                en: "Fanfare Above the Limit"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 400.0, default: 0.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            FurinaDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
            A1 locale!(zh_cn: "灵息之刺/流涌之刃伤害", en: "Spiritbreath Thorn/Surging Blade DMG")
        ),
        skill2: skill_map!(
            FurinaDamageEnum
            E1 locale!(zh_cn: "荒性泡沫伤害", en: "Ousia Bubble DMG")
            E2 locale!(zh_cn: "乌瑟勋爵伤害", en: "Gentilhomme Usher DMG")
            E3 locale!(zh_cn: "海薇玛夫人伤害", en: "Surintendante Chevalmarin DMG")
            E4 locale!(zh_cn: "谢贝蕾妲小姐伤害", en: "Mademoiselle Crabaletta DMG")
            EHeal1 locale!(zh_cn: "众水的歌者治疗量", en: "Singer of Many Waters Healing")
        ),
        skill3: skill_map!(
            FurinaDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: FurinaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let skill_type = s.get_skill_type();
        use FurinaDamageEnum::*;

        let (hp_above50_count, c6_after_e, c6_pneuma) = match *config {
            CharacterSkillConfig::Furina { hp_above50_count, c6_after_e, c6_pneuma } => (hp_above50_count, c6_after_e, c6_pneuma),
            _ => (0, false, false)
        };

        let mut builder = D::new();
        if s.is_heal() {
            let ratio = FURINA_SKILL.e_heal1[s2];
            let fixed = FURINA_SKILL.e_heal1_fixed[s2];
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => FURINA_SKILL.normal_dmg1[s1],
                Normal2 => FURINA_SKILL.normal_dmg2[s1],
                Normal3 => FURINA_SKILL.normal_dmg3[s1],
                Normal4 => FURINA_SKILL.normal_dmg4[s1],
                Charged => FURINA_SKILL.charged_dmg[s1],
                Plunging1 => FURINA_SKILL.plunging_dmg1[s1],
                Plunging2 => FURINA_SKILL.plunging_dmg2[s1],
                Plunging3 => FURINA_SKILL.plunging_dmg3[s1],
                A1 => FURINA_SKILL.a_dmg1[s1],
                E1 => FURINA_SKILL.e_dmg1[s2],
                E2 => FURINA_SKILL.e_dmg2[s2],
                E3 => FURINA_SKILL.e_dmg3[s2],
                E4 => FURINA_SKILL.e_dmg4[s2],
                Q1 => FURINA_SKILL.q_dmg1[s3],
                _ => 0.0
            };
            if s.is_hp_ratio() {
                builder.add_hp_ratio("技能倍率", ratio);
            } else {
                builder.add_atk_ratio("技能倍率", ratio);
            }

            if s == E2 || s == E3 || s == E4 {
                let bonus = 0.1 * hp_above50_count as f64;
                builder.add_hp_ratio("「沙龙成员」加成", ratio * bonus);
                if context.character_common_data.has_talent2 {
                    let hp = context.attribute.get_hp();
                    let factor = (hp / 1000.0).floor();
                    builder.add_extra_bonus("天赋「无人听的自白」", (factor * 0.007).min(0.28));
                }
            }

            let is_c6_skill = (skill_type == SkillType::NormalAttack && s != A1)
                || skill_type == SkillType::ChargedAttack
                || skill_type.is_plunging();
            let enable_c6 = is_c6_skill && context.character_common_data.constellation >= 6 && c6_after_e;
            if enable_c6 {
                builder.add_hp_ratio("C6「诸君听我颂，共举爱之杯！」", 0.18);
                if c6_pneuma {
                    builder.add_hp_ratio("C6「诸君听我颂，共举爱之杯！」", 0.25);
                }
            }

            let element = if enable_c6 {
                Element::Hydro
            } else {
                s.get_element()
            };

            builder.damage(
                &context.attribute,
                &context.enemy,
                element,
                skill_type,
                context.character_common_data.level,
                fumo
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Furina { c2_overflow } => Some(Box::new(FurinaEffect {
                c2_overflow
            })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
