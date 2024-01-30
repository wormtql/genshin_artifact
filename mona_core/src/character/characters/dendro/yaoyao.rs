use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{skill_type, damage_enum, skill_map, damage_ratio};
use crate::character::prelude::CharacterTrait;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterSkillMap;
use crate::common::{ChangeAttribute, Element, StatName, WeaponType, SkillType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::traits::{CharacterSkillMapItem};
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};


pub struct YaoyaoSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_heal1: [f64; 15],
    pub e_heal1_fixed: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
    pub q_heal1: [f64; 15],
    pub q_heal1_fixed: [f64; 15],
}

pub const YAOYAO_SKILL: YaoyaoSkillType = YaoyaoSkillType {
    normal_dmg1: [0.51, 0.5515, 0.593, 0.6523, 0.6939, 0.7413, 0.8065, 0.8718, 0.937, 1.0082, 1.0793, 1.1505, 1.2217, 1.2928, 1.364],
    normal_dmg2: [0.4744, 0.513, 0.5517, 0.6068, 0.6454, 0.6896, 0.7503, 0.8109, 0.8716, 0.9378, 1.004, 1.0702, 1.1364, 1.2026, 1.2688],
    normal_dmg31: [0.3138, 0.3393, 0.3649, 0.4013, 0.4269, 0.4561, 0.4962, 0.5363, 0.5765, 0.6202, 0.664, 0.7078, 0.7516, 0.7954, 0.8392],
    normal_dmg32: [0.3295, 0.3563, 0.3831, 0.4214, 0.4482, 0.4789, 0.521, 0.5631, 0.6053, 0.6513, 0.6972, 0.7432, 0.7892, 0.8351, 0.8811],
    normal_dmg4: [0.7793, 0.8427, 0.9062, 0.9968, 1.0602, 1.1327, 1.2324, 1.3321, 1.4318, 1.5405, 1.6492, 1.758, 1.8667, 1.9755, 2.0842],
    charged_dmg1: [1.1266, 1.2183, 1.31, 1.441, 1.5327, 1.6375, 1.7816, 1.9257, 2.0698, 2.227, 2.3842, 2.5414, 2.6986, 2.8558, 3.013],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [0.2992, 0.3216, 0.3441, 0.374, 0.3964, 0.4189, 0.4488, 0.4787, 0.5086, 0.5386, 0.5685, 0.5984, 0.6358, 0.6732, 0.7106],
    e_heal1: [0.0171, 0.0184, 0.0197, 0.0214, 0.0227, 0.024, 0.0257, 0.0274, 0.0291, 0.0309, 0.0326, 0.0343, 0.0364, 0.0386, 0.0407],
    e_heal1_fixed: [165.08, 181.59, 199.48, 218.74, 239.38, 261.39, 284.78, 309.54, 335.69, 363.2, 392.1, 422.37, 454.01, 487.03, 521.43],
    q_dmg1: [1.1456, 1.2315, 1.3174, 1.432, 1.5179, 1.6038, 1.7184, 1.833, 1.9475, 2.0621, 2.1766, 2.2912, 2.4344, 2.5776, 2.7208],
    q_dmg2: [0.7216, 0.7757, 0.8298, 0.902, 0.9561, 1.0102, 1.0824, 1.1546, 1.2267, 1.2989, 1.371, 1.4432, 1.5334, 1.6236, 1.7138],
    q_heal1: [0.0202, 0.0217, 0.0232, 0.0252, 0.0267, 0.0282, 0.0303, 0.0323, 0.0343, 0.0363, 0.0383, 0.0403, 0.0429, 0.0454, 0.0479],
    q_heal1_fixed: [194.21, 213.64, 234.68, 257.34, 281.62, 307.52, 335.04, 364.17, 394.93, 427.3, 461.29, 496.9, 534.13, 572.98, 613.45],
};

damage_enum!(
    YaoyaoDamageEnum
    Normal1
    Normal2
    Normal31
    Normal32
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    EHeal1
    Q1
    Q2
    QHeal1
);

impl YaoyaoDamageEnum {
    pub fn get_element(&self) -> Element {
        use YaoyaoDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 | Charged | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            _ => Element::Dendro
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use YaoyaoDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | EHeal1 => SkillType::ElementalSkill,
            Q1 | Q2 | QHeal1 => SkillType::ElementalBurst
        }
    }
}

pub struct YaoyaoEffect {
    pub is_c4: bool,
    pub c4_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for YaoyaoEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.is_c4 {
            let rate = self.c4_rate;
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::ElementalMastery,
                Box::new(move |hp, _| 120.0_f64.min(hp * 0.003) * rate),
                Box::new(|_, _, _| (0.0, 0.0)),
                "瑶瑶四命「爰爰可亲」",
            );
        }
    }
}

pub struct Yaoyao;

impl CharacterTrait for Yaoyao {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Yaoyao,
        internal_name: "Yaoyao",
        element: Element::Dendro,
        hp: [1030, 2647, 3417, 5118, 5665, 6515, 7245, 8096, 8643, 9493, 10040, 10891, 11438, 12289],
        atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212],
        def: [63, 162, 209, 313, 346, 398, 443, 495, 528, 580, 613, 665, 699, 751],
        sub_stat: CharacterSubStatFamily::Bonus240(StatName::HPPercentage),
        weapon_type: WeaponType::Polearm,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·颠扑连环枪",
            en: "Normal Attack: Toss ‘N’ Turn Spear",
        ),
        skill_name2: locale!(
            zh_cn: "云台团团降芦菔",
            en: "Raphanus Sky Cluster",
        ),
        skill_name3: locale!(
            zh_cn: "玉颗珊珊月中落",
            en: "Moonjade Descent",
        ),
        name_locale: locale!(
            zh_cn: "瑶瑶",
            en: "Yaoyao",
        )
    };
    type SkillType = YaoyaoSkillType;
    const SKILL: Self::SkillType = YAOYAO_SKILL;
    type DamageEnumType = YaoyaoDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            YaoyaoDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal31 hit_n_dmg!(3, 1)
            Normal32 hit_n_dmg!(3, 2)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            YaoyaoDamageEnum
            E1 locale!(zh_cn: "白玉萝卜伤害", en: "White Jade Radish DMG")
            EHeal1 locale!(zh_cn: "白玉萝卜治疗量", en: "White Jade Radish Healing")
        ),
        skill3: skill_map!(
            YaoyaoDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "桂子仙机白玉萝卜伤害", en: "Adeptal Legacy White Jade Radish DMG")
            QHeal1 locale!(zh_cn: "桂子仙机白玉萝卜治疗量", en: "Adeptal Legacy White Jade Radish Healing")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c4_rate",
            title: locale!(
                zh_cn: "命座4「爰爰可亲」比例",
                en: "C4「Winsome」Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: YaoyaoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use YaoyaoDamageEnum::*;
        let mut builder = D::new();

        if s == EHeal1 || s == QHeal1 {
            let ratio = if s == EHeal1 { YAOYAO_SKILL.e_heal1[s2] } else { YAOYAO_SKILL.q_heal1[s3] };
            let fixed = if s == EHeal1 { YAOYAO_SKILL.e_heal1_fixed[s2] } else { YAOYAO_SKILL.q_heal1_fixed[s3] };
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);

            builder.heal(
                &context.attribute
            )
        } else {
            let ratio = match s {
                Normal1 => YAOYAO_SKILL.normal_dmg1[s1],
                Normal2 => YAOYAO_SKILL.normal_dmg2[s1],
                Normal31 => YAOYAO_SKILL.normal_dmg31[s1],
                Normal32 => YAOYAO_SKILL.normal_dmg32[s1],
                Normal4 => YAOYAO_SKILL.normal_dmg4[s1],
                Charged => YAOYAO_SKILL.charged_dmg1[s1],
                Plunging1 => YAOYAO_SKILL.plunging_dmg1[s1],
                Plunging2 => YAOYAO_SKILL.plunging_dmg2[s1],
                Plunging3 => YAOYAO_SKILL.plunging_dmg3[s1],
                E1 => YAOYAO_SKILL.e_dmg1[s2],
                Q1 => YAOYAO_SKILL.q_dmg1[s3],
                Q2 => YAOYAO_SKILL.q_dmg2[s3],
                _ => 0.0
            };
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
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let c4_rate = match *config {
            CharacterConfig::Yaoyao { c4_rate } => c4_rate,
            _ => 0.0
        };
        Some(Box::new(YaoyaoEffect {
            c4_rate,
            is_c4: common_data.constellation >= 4
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}