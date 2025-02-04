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

pub struct YumemizukiMizukiSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_bonus: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
    pub q_heal: [f64; 15],
    pub q_heal_fixed: [f64; 15]
}

pub const YUMEMIZUKI_MIZUKI_SKILL: YumemizukiMizukiSkillType = YumemizukiMizukiSkillType {
    normal_dmg1: [0.5228, 0.562, 0.6012, 0.6535, 0.6927, 0.7319, 0.7842, 0.8364, 0.8887, 0.941, 0.9933, 1.0455, 1.1109, 1.1762, 1.2416],
    normal_dmg2: [0.4691, 0.5043, 0.5395, 0.5864, 0.6216, 0.6568, 0.7037, 0.7506, 0.7975, 0.8445, 0.8914, 0.9383, 0.9969, 1.0556, 1.1142],
    normal_dmg3: [0.7137, 0.7672, 0.8207, 0.8921, 0.9456, 0.9992, 1.0705, 1.1419, 1.2133, 1.2846, 1.356, 1.4274, 1.5166, 1.6058, 1.695],
    charged_dmg: [1.3, 1.3975, 1.495, 1.625, 1.7225, 1.82, 1.95, 2.08, 2.21, 2.34, 2.47, 2.6, 2.7625, 2.925, 3.0875],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [0.5774, 0.6207, 0.6641, 0.7218, 0.7651, 0.8084, 0.8662, 0.9239, 0.9816, 1.0394, 1.0971, 1.1549, 1.2271, 1.2992, 1.3714],
    e_dmg2: [0.4491, 0.4828, 0.5165, 0.5614, 0.5951, 0.6288, 0.6737, 0.7186, 0.7635, 0.8084, 0.8533, 0.8982, 0.9544, 1.0105, 1.0667],
    e_bonus: [0.0018, 0.0021, 0.0024, 0.0027, 0.003, 0.0033, 0.0036, 0.0039, 0.0042, 0.0045, 0.0048, 0.0051, 0.0054, 0.0057, 0.006],
    q_dmg1: [0.9408, 1.0114, 1.0819, 1.176, 1.2466, 1.3171, 1.4112, 1.5053, 1.5994, 1.6934, 1.7875, 1.8816, 1.9992, 2.1168, 2.2344],
    q_dmg2: [0.7056, 0.7585, 0.8114, 0.882, 0.9349, 0.9878, 1.0584, 1.129, 1.1995, 1.2701, 1.3406, 1.4112, 1.4994, 1.5876, 1.6758],
    q_heal: [1.3056, 1.4035, 1.5014, 1.632, 1.7299, 1.8278, 1.9584, 2.089, 2.2195, 2.3501, 2.4806, 2.6112, 2.7744, 2.9376, 3.1008],
    q_heal_fixed: [314.57, 346.03, 380.11, 416.82, 456.15, 498.09, 542.66, 589.86, 639.67, 692.11, 747.16, 804.84, 865.14, 928.07, 993.61]
};

struct YumemizukiMizukiEffect {
    pub talent2_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for YumemizukiMizukiEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let em_bonus = self.talent2_rate * 100.0;
        attribute.set_value_by(AttributeName::ElementalMastery, "天赋「昼想夜梦」", em_bonus);
    }
}

damage_enum!(
    YumemizukiMizukiDamageEnum
    Normal1
    Normal2
    Normal3
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    Q1
    Q2
    QHeal
);

impl YumemizukiMizukiDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use YumemizukiMizukiDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 | Q2 | QHeal => SkillType::ElementalBurst
        }
    }
}

pub struct YumemizukiMizuki;

impl CharacterTrait for YumemizukiMizuki {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::YumemizukiMizuki,
        internal_name: "YumemizukiMizuki",
        name_locale: locale!(
            zh_cn: "梦见月瑞希",
            en: "Yumemizuki Mizuki"
        ),
        element: Element::Anemo,
        hp: [991, 2572, 3422, 5120, 5724, 6586, 7391, 8262, 8866, 9744, 10348, 11236, 11840, 12736],
        atk: [17, 43, 58, 87, 97, 111, 125, 140, 150, 165, 175, 190, 200, 215],
        def: [59, 153, 203, 304, 340, 391, 439, 491, 527, 579, 615, 668, 704, 757],
        sub_stat: CharacterSubStatFamily::ElementalMastery115,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·梦我梦心",
            en: "Normal Attack: Pure Heart, Pure Dreams"
        ),
        skill_name2: locale!(
            zh_cn: "秋沙歌枕巡礼",
            en: "Aisa Utamakura Pilgrimage"
        ),
        skill_name3: locale!(
            zh_cn: "安乐秘汤疗法",
            en: "Anraku Secret Spring Therapy"
        ),
    };
    type SkillType = YumemizukiMizukiSkillType;
    const SKILL: Self::SkillType = YUMEMIZUKI_MIZUKI_SKILL;
    type DamageEnumType = YumemizukiMizukiDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            YumemizukiMizukiDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            YumemizukiMizukiDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E2 locale!(zh_cn: "持续攻击伤害", en: "Continuous Attack DMG")
        ),
        skill3: skill_map!(
            YumemizukiMizukiDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q1 locale!(zh_cn: "梦念冲击波伤害", en: "Munen Shockwave DMG")
            QHeal locale!(zh_cn: "拾取点心回复生命值", en: "Snack Pick-Up HP Regeneration")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_rate",
            title: locale!(
                zh_cn: "天赋「昼想夜梦」比例",
                en: "Talent 2 Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: YumemizukiMizukiDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use YumemizukiMizukiDamageEnum::*;
        let mut builder = D::new();

        if s == QHeal {
            let rate = YUMEMIZUKI_MIZUKI_SKILL.q_heal[s3];
            let fixed = YUMEMIZUKI_MIZUKI_SKILL.q_heal_fixed[s3];

            builder.add_em_ratio("技能倍率", rate);
            builder.add_extra_damage("技能倍率", fixed);

            return builder.heal(&context.attribute);
        }

        let rate = match s {
            Normal1 => YUMEMIZUKI_MIZUKI_SKILL.normal_dmg1[s1],
            Normal2 => YUMEMIZUKI_MIZUKI_SKILL.normal_dmg2[s1],
            Normal3 => YUMEMIZUKI_MIZUKI_SKILL.normal_dmg3[s1],
            Charged => YUMEMIZUKI_MIZUKI_SKILL.charged_dmg[s1],
            Plunging1 => YUMEMIZUKI_MIZUKI_SKILL.plunging_dmg1[s1],
            Plunging2 => YUMEMIZUKI_MIZUKI_SKILL.plunging_dmg2[s1],
            Plunging3 => YUMEMIZUKI_MIZUKI_SKILL.plunging_dmg3[s1],
            E1 => YUMEMIZUKI_MIZUKI_SKILL.e_dmg1[s2],
            E2 => YUMEMIZUKI_MIZUKI_SKILL.e_dmg2[s2],
            Q1 => YUMEMIZUKI_MIZUKI_SKILL.q_dmg1[s3],
            Q2 => YUMEMIZUKI_MIZUKI_SKILL.q_dmg2[s3],
            _ => 0.0
        };

        builder.add_atk_ratio("技能倍率", rate);
        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Anemo,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::YumemizukiMizuki { talent2_rate } => Some(Box::new(YumemizukiMizukiEffect {
                talent2_rate
            })),
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
