use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::characters::arlecchino::ArlecchinoDamageEnum;
use crate::character::characters::arlecchino::ArlecchinoDamageEnum::{Plunging1, Plunging2, Plunging3};
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

type SKILL = [f64; 15];

pub struct MavuikaSkillType {
    pub normal_dmg1: SKILL,
    pub normal_dmg2: SKILL,
    pub normal_dmg3: SKILL,
    pub normal_dmg4: SKILL,
    pub charged_dmg: SKILL,
    pub plunging_dmg1: SKILL,
    pub plunging_dmg2: SKILL,
    pub plunging_dmg3: SKILL,

    pub e_dmg1: SKILL,
    pub e_dmg2: SKILL,
    pub e_dmg_normal_1: SKILL,
    pub e_dmg_normal_2: SKILL,
    pub e_dmg_normal_3: SKILL,
    pub e_dmg_normal_4: SKILL,
    pub e_dmg_normal_5: SKILL,
    pub e_dmg_normal_6: SKILL,
    pub e_dmg_charged_1: SKILL,
    pub e_dmg_charged_2: SKILL,
    pub e_dmg_plunging: SKILL,

    pub q_dmg1: SKILL,
    pub q_bonus1: SKILL,
    pub q_bonus2: SKILL,
    pub q_bonus3: SKILL,
}

pub const MAVUIKA_SKILL: MavuikaSkillType = MavuikaSkillType {
    normal_dmg1: [0.8004, 0.8655, 0.9306, 1.0237, 1.0888, 1.1633, 1.2657, 1.368, 1.4704, 1.5821, 1.6938, 1.8054, 1.9171, 2.0288, 2.1405],
    normal_dmg2: [0.3648, 0.3945, 0.4242, 0.4666, 0.4963, 0.5302, 0.5769, 0.6236, 0.6702, 0.7211, 0.772, 0.8229, 0.8738, 0.9247, 0.9756],
    normal_dmg3: [0.3322, 0.3593, 0.3863, 0.4249, 0.452, 0.4829, 0.5254, 0.5679, 0.6104, 0.6567, 0.7031, 0.7495, 0.7958, 0.8422, 0.8885],
    normal_dmg4: [1.1619, 1.2565, 1.3511, 1.4862, 1.5808, 1.6889, 1.8375, 1.9861, 2.1347, 2.2968, 2.459, 2.6211, 2.7832, 2.9454, 3.1075],
    charged_dmg: [1.9384, 2.0962, 2.254, 2.4794, 2.6372, 2.8175, 3.0654, 3.3134, 3.5613, 3.8318, 4.1023, 4.3728, 4.6432, 4.9137, 5.1842],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    e_dmg1: [0.744, 0.7998, 0.8556, 0.93, 0.9858, 1.0416, 1.116, 1.1904, 1.2648, 1.3392, 1.4136, 1.488, 1.581, 1.674, 1.767],
    e_dmg2: [1.28, 1.376, 1.472, 1.6, 1.696, 1.792, 1.92, 2.048, 2.176, 2.304, 2.432, 2.56, 2.72, 2.88, 3.04],
    e_dmg_normal_1: [0.5726, 0.6193, 0.6659, 0.7325, 0.7791, 0.8323, 0.9056, 0.9788, 1.0521, 1.132, 1.2119, 1.2918, 1.3717, 1.4516, 1.5315],
    e_dmg_normal_2: [0.5913, 0.6395, 0.6876, 0.7563, 0.8045, 0.8595, 0.9351, 1.0108, 1.0864, 1.1689, 1.2514, 1.3339, 1.4164, 1.4989, 1.5815],
    e_dmg_normal_3: [0.6999, 0.7568, 0.8138, 0.8952, 0.9521, 1.0173, 1.1068, 1.1963, 1.2858, 1.3835, 1.4811, 1.5788, 1.6764, 1.7741, 1.8717],
    e_dmg_normal_4: [0.697, 0.7538, 0.8105, 0.8916, 0.9483, 1.0132, 1.1023, 1.1915, 1.2806, 1.3779, 1.4751, 1.5724, 1.6697, 1.7669, 1.8642],
    e_dmg_normal_5: [0.91, 0.9841, 1.0582, 1.164, 1.2381, 1.3227, 1.4391, 1.5555, 1.6719, 1.7989, 1.9259, 2.0529, 2.1799, 2.3068, 2.4338],
    e_dmg_normal_6: [0.8084, 0.8742, 0.94, 1.034, 1.0998, 1.175, 1.2784, 1.3818, 1.4852, 1.598, 1.7108, 1.8236, 1.9364, 2.0492, 2.162],
    e_dmg_charged_1: [0.989, 1.0695, 1.15, 1.265, 1.3455, 1.4375, 1.564, 1.6905, 1.817, 1.955, 2.093, 2.231, 2.369, 2.507, 2.645],
    e_dmg_charged_2: [1.376, 1.488, 1.6, 1.76, 1.872, 2., 2.176, 2.352, 2.528, 2.72, 2.912, 3.104, 3.296, 3.488, 3.68],
    e_dmg_plunging: [1.5996, 1.7298, 1.86, 2.046, 2.1762, 2.325, 2.5296, 2.7342, 2.9388, 3.162, 3.3852, 3.6084, 3.8316, 4.0548, 4.278],
    q_dmg1: [4.448, 4.7816, 5.1152, 5.56, 5.8936, 6.2272, 6.672, 7.1168, 7.5616, 8.0064, 8.4512, 8.896, 9.452, 10.008, 10.564],
    q_bonus1: [0.016, 0.0172, 0.0184, 0.02, 0.0212, 0.0224, 0.024, 0.0256, 0.0272, 0.0288, 0.0304, 0.032, 0.034, 0.036, 0.038],
    q_bonus2: [0.0026, 0.0028, 0.003, 0.0033, 0.0035, 0.0038, 0.0041, 0.0044, 0.0047, 0.0051, 0.0055, 0.0058, 0.0062, 0.0065, 0.0069],
    q_bonus3: [0.0052, 0.0056, 0.006, 0.0066, 0.007, 0.0075, 0.0082, 0.0088, 0.0095, 0.0102, 0.0109, 0.0116, 0.0124, 0.0131, 0.0138],
};

pub struct MavuikaEffect {
    pub talent1_rate: f64,
    pub talent2_rate: f64,
    pub has_talent1: bool,
    pub has_talent2: bool,
    pub fighting_spirit: f64,

    pub constellation: usize,
    pub c1_rate: f64,
    pub c2_rate: f64,
}

damage_enum!(
    MavuikaDamageEnum
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
    ENormal1
    ENormal2
    ENormal3
    ENormal4
    ENormal5
    ENormal6        // 驰轮车冲刺伤害
    ECharged1
    ECharged2
    EPlunging
    Q1
);

impl MavuikaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use MavuikaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            ENormal1 | ENormal2 | ENormal3 | ENormal4 | ENormal5 | ENormal6 => SkillType::NormalAttack,
            ECharged1 | ECharged2 => SkillType::ChargedAttack,
            EPlunging => SkillType::PlungingAttackOnGround,
            Q1 => SkillType::ElementalBurst
        }
    }

    pub fn get_element(&self) -> Element {
        use MavuikaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Charged | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            E1 | E2 | ENormal1 | ENormal2 | ENormal3 | ENormal4 | ENormal5 | ENormal6 | ECharged1 | ECharged2
                | EPlunging | Q1 => Element::Pyro,
        }
    }

    pub fn is_certain_skill(&self) -> bool {
        use MavuikaDamageEnum::*;
        match *self {
            Q1 |
            ENormal1 | ENormal2 | ENormal3 | ENormal4 | ENormal5 | ENormal6 | ECharged1 | ECharged2 => true,
            _ => false
        }
    }
}

impl<A: Attribute> ChangeAttribute<A> for MavuikaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent1 {
            let ratio = 0.3 * self.talent1_rate;
            attribute.add_atk_percentage("天赋「炎花献礼」", ratio);
        }

        if self.has_talent2 {
            let bonus = (0.002 * self.fighting_spirit).min(0.4);
            attribute.set_value_by(AttributeName::BonusBase, "天赋「基扬戈兹」", bonus * self.talent2_rate);

            if self.constellation >= 4 {
                attribute.set_value_by(AttributeName::BonusBase, "C4「「领袖」的觉悟」", 0.1);
            }
        }

        if self.constellation >= 1 {
            attribute.add_atk_percentage("C1「夜主的授记」", self.c1_rate * 0.4);
        }

        if self.constellation >= 2 {
            attribute.set_value_by(AttributeName::ATKBase, "C2「灰烬的代价」", 200.0 * self.c2_rate);
            attribute.set_value_by(AttributeName::USER1, "C2「灰烬的代价」", self.c2_rate);
        }

        attribute.set_value_by(AttributeName::USER2, "战意", self.fighting_spirit);
    }
}

pub struct Mavuika;

impl CharacterTrait for Mavuika {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Mavuika,
        internal_name: "Mavuika",
        name_locale: locale!(
            zh_cn: "玛薇卡",
            en: "Mavuika"
        ),
        element: Element::Pyro,
        hp: [977, 2535, 3373, 5046, 5642, 6491, 7285, 8143, 8738, 9604, 10199, 11074, 11670, 12552],
        atk: [28, 72, 96, 144, 161, 186, 208, 233, 250, 274, 292, 317, 334, 359],
        def: [62, 160, 213, 318, 356, 409, 459, 514, 551, 606, 643, 698, 736, 792],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Claymore,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·以火织命",
            en: "Normal Attack: Flames Weave Life"
        ),
        skill_name2: locale!(
            zh_cn: "称名之刻",
            en: "The Named Moment"
        ),
        skill_name3: locale!(
            zh_cn: "燔天之时",
            en: "Hour of Burning Skies"
        ),
    };
    type SkillType = MavuikaSkillType;
    const SKILL: Self::SkillType = MAVUIKA_SKILL;
    type DamageEnumType = MavuikaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            MavuikaDamageEnum
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
            MavuikaDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E2 locale!(zh_cn: "焚曜之环伤害", en: "Ring of Searing Radiance DMG")
            ENormal1 locale!(zh_cn: "驰轮车普通攻击一段伤害", en: "Flamestrider Normal Attack 1-Hit DMG")
            ENormal2 locale!(zh_cn: "驰轮车普通攻击二段伤害", en: "Flamestrider Normal Attack 2-Hit DMG")
            ENormal3 locale!(zh_cn: "驰轮车普通攻击三段伤害", en: "Flamestrider Normal Attack 3-Hit DMG")
            ENormal4 locale!(zh_cn: "驰轮车普通攻击四段伤害", en: "Flamestrider Normal Attack 4-Hit DMG")
            ENormal5 locale!(zh_cn: "驰轮车普通攻击五段伤害", en: "Flamestrider Normal Attack 5-Hit DMG")
            ENormal6 locale!(zh_cn: "驰轮车冲刺伤害", en: "Flamestrider Sprint DMG")
            ECharged1 locale!(zh_cn: "驰轮车重击循环伤害", en: "Flamestrider Charged Attack Cyclic DMG")
            ECharged2 locale!(zh_cn: "驰轮车重击终结伤害", en: "Flamestrider Charged Attack Final DMG")
            EPlunging locale!(zh_cn: "驰轮车坠地冲击伤害", en: "Flamestrider Plunge DMG")
        ),
        skill3: skill_map!(
            MavuikaDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_rate",
            title: locale!(
                zh_cn: "天赋1「炎花献礼」比例",
                en: "Talent1 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "talent2_rate",
            title: locale!(
                zh_cn: "天赋2「「基扬戈兹」」比例",
                en: "Talent2 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "fighting_spirit",
            title: locale!(
                zh_cn: "战意",
                en: "Fighting Spirit",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 200.0, default: 100.0 }
        },
        ItemConfig {
            name: "c1_rate",
            title: locale!(
                zh_cn: "C1「夜主的授记」比例",
                en: "C1 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "c2_rate",
            title: locale!(
                zh_cn: "C2「灰烬的代价」比例",
                en: "C2 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_q",
            title: locale!(
                zh_cn: "死生之炉",
                en: "Crucible of Death and Life"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: MavuikaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let mut builder = D::new();

        use MavuikaDamageEnum::*;
        let ratio = match s {
            Normal1 => MAVUIKA_SKILL.normal_dmg1[s1],
            Normal2 => MAVUIKA_SKILL.normal_dmg2[s1] * 2.0,
            Normal3 => MAVUIKA_SKILL.normal_dmg3[s1] * 3.0,
            Normal4 => MAVUIKA_SKILL.normal_dmg4[s1],
            Charged => MAVUIKA_SKILL.charged_dmg[s1],
            Plunging1 => MAVUIKA_SKILL.plunging_dmg1[s1],
            Plunging2 => MAVUIKA_SKILL.plunging_dmg2[s1],
            Plunging3 => MAVUIKA_SKILL.plunging_dmg3[s1],
            E1 => MAVUIKA_SKILL.e_dmg1[s2],
            E2 => MAVUIKA_SKILL.e_dmg2[s2],
            ENormal1 => MAVUIKA_SKILL.e_dmg_normal_1[s2],
            ENormal2 => MAVUIKA_SKILL.e_dmg_normal_2[s2],
            ENormal3 => MAVUIKA_SKILL.e_dmg_normal_3[s2],
            ENormal4 => MAVUIKA_SKILL.e_dmg_normal_4[s2],
            ENormal5 => MAVUIKA_SKILL.e_dmg_normal_5[s2],
            ENormal6 => MAVUIKA_SKILL.e_dmg_normal_6[s2],
            ECharged1 => MAVUIKA_SKILL.e_dmg_charged_1[s2],
            ECharged2 => MAVUIKA_SKILL.e_dmg_charged_2[s2],
            EPlunging => MAVUIKA_SKILL.e_dmg_plunging[s2],
            Q1 => MAVUIKA_SKILL.q_dmg1[s3],
        };

        builder.add_atk_ratio("技能倍率", ratio);

        let is_certain_skill = s.is_certain_skill();
        let skill_type = s.get_skill_type();

        let after_q = match *config {
            CharacterSkillConfig::Mavuika { after_q } => after_q,
            _ => false
        };

        if context.character_common_data.constellation >= 2 && is_certain_skill {
            let c2_rate = context.attribute.get_value(AttributeName::USER1);
            let bonus = match skill_type {
                SkillType::NormalAttack => 0.6,
                SkillType::ChargedAttack => 0.9,
                SkillType::ElementalBurst => 1.2,
                _ => 0.0
            };
            builder.add_atk_ratio("C2「灰烬的代价」", bonus * c2_rate);
        }

        if after_q && is_certain_skill {
            let fighting_spirit = context.attribute.get_value(AttributeName::USER2);

            let bonus = match skill_type {
                SkillType::ElementalBurst => MAVUIKA_SKILL.q_bonus1[s3],
                SkillType::NormalAttack => MAVUIKA_SKILL.q_bonus2[s3],
                SkillType::ChargedAttack => MAVUIKA_SKILL.q_bonus3[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("燔天之时", bonus * fighting_spirit);
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

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Mavuika { c1_rate, c2_rate, talent1_rate, talent2_rate, fighting_spirit } => Some(Box::new(MavuikaEffect {
                talent1_rate, talent2_rate, fighting_spirit, c1_rate, c2_rate,
                has_talent1: common_data.has_talent1,
                has_talent2: common_data.has_talent2,
                constellation: common_data.constellation as usize,
            })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
