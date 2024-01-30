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

pub struct CharlotteSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],
    pub e_dmg4: [f64; 15],

    pub q_heal1: [f64; 15],
    pub q_heal1_fixed: [f64; 15],
    pub q_dmg1: [f64; 15],
    pub q_heal2: [f64; 15],
    pub q_heal2_fixed: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const CHARLOTTE_SKILL: CharlotteSkillType = CharlotteSkillType {
    normal_dmg1: [0.4985, 0.5358, 0.5732, 0.6231, 0.6605, 0.6978, 0.7477, 0.7975, 0.8474, 0.8972, 0.9471, 0.9969, 1.0592, 1.1215, 1.1838],
    normal_dmg2: [0.4338, 0.4663, 0.4988, 0.5422, 0.5747, 0.6073, 0.6506, 0.694, 0.7374, 0.7808, 0.8241, 0.8675, 0.9217, 0.9759, 1.0302],
    normal_dmg3: [0.646, 0.6945, 0.7429, 0.8075, 0.856, 0.9044, 0.969, 1.0336, 1.0982, 1.1628, 1.2274, 1.292, 1.3728, 1.4535, 1.5343],
    charged_dmg1: [1.0051, 1.0805, 1.1559, 1.2564, 1.3318, 1.4072, 1.5077, 1.6082, 1.7087, 1.8092, 1.9097, 2.0102, 2.1359, 2.2615, 2.3872],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    // [0.1117, 0.1201, 0.1284, 0.1396, 0.148, 0.1564, 0.1675, 0.1787, 0.1899, 0.201, 0.2122, 0.2234, 0.2373, 0.2513, 0.2652],
    e_dmg1: [0.672, 0.7224, 0.7728, 0.84, 0.8904, 0.9408, 1.008, 1.0752, 1.1424, 1.2096, 1.2768, 1.344, 1.428, 1.512, 1.596],
    e_dmg2: [1.392, 1.4964, 1.6008, 1.74, 1.8444, 1.9488, 2.088, 2.2272, 2.3664, 2.5056, 2.6448, 2.784, 2.958, 3.132, 3.306],
    e_dmg3: [0.392, 0.4214, 0.4508, 0.49, 0.5194, 0.5488, 0.588, 0.6272, 0.6664, 0.7056, 0.7448, 0.784, 0.833, 0.882, 0.931],
    e_dmg4: [0.406, 0.4365, 0.4669, 0.5075, 0.5379, 0.5684, 0.609, 0.6496, 0.6902, 0.7308, 0.7714, 0.812, 0.8627, 0.9135, 0.9643],
    q_heal1: [2.5657, 2.7582, 2.9506, 3.2072, 3.3996, 3.592, 3.8486, 4.1052, 4.3617, 4.6183, 4.8749, 5.1315, 5.4522, 5.7729, 6.0936],
    q_heal1_fixed: [1608.49, 1769.36, 1943.63, 2131.32, 2332.41, 2546.9, 2774.8, 3016.11, 3270.82, 3538.94, 3820.46, 4115.39, 4423.73, 4745.47, 5080.62],
    q_dmg1: [0.7762, 0.8344, 0.8926, 0.9702, 1.0284, 1.0866, 1.1642, 1.2419, 1.3195, 1.3971, 1.4747, 1.5523, 1.6493, 1.7464, 1.8434],
    q_heal2: [0.0922, 0.0991, 0.106, 0.1152, 0.1221, 0.129, 0.1382, 0.1475, 0.1567, 0.1659, 0.1751, 0.1843, 0.1958, 0.2074, 0.2189],
    q_heal2_fixed: [57.45, 63.19, 69.42, 76.12, 83.3, 90.96, 99.1, 107.72, 116.82, 126.39, 136.45, 146.98, 157.99, 169.48, 181.45],
    q_dmg2: [0.0647, 0.0695, 0.0744, 0.0808, 0.0857, 0.0906, 0.097, 0.1035, 0.11, 0.1164, 0.1229, 0.1294, 0.1374, 0.1455, 0.1536],
};

damage_enum!(
    CharlotteDamageEnum
    Normal1
    Normal2
    Normal3
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    E4
    Q1
    QHeal1
    Q2
    QHeal2
);

impl CharlotteDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use CharlotteDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4 => SkillType::ElementalSkill,
            QHeal1 | QHeal2 | Q1 | Q2 => SkillType::ElementalBurst
        }
    }

    pub fn is_heal(&self) -> bool {
        use CharlotteDamageEnum::*;
        match *self {
            QHeal1 | QHeal2 => true,
            _ => false
        }
    }
}

pub struct CharlotteEffect {
    pub talent2_fontaine_count: usize,
    pub talent2_non_fontaine_count: usize,
    pub c2_count: usize,
    pub c2_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for CharlotteEffect {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::HealingBonus, "天赋「多样性调查」", 0.05 * (self.talent2_fontaine_count as f64));
        attribute.set_value_by(AttributeName::BonusCryo, "天赋「多样性调查」", 0.05 * (self.talent2_non_fontaine_count as f64));
        attribute.add_atk_percentage("C2「以求真为职守」", (0.1 * self.c2_count as f64) * self.c2_rate);
    }
}

pub struct Charlotte;

impl CharacterTrait for Charlotte {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Charlotte,
        internal_name: "Charlotte",
        name_locale: locale!(
            zh_cn: "夏洛蒂",
            en: "Charlotte"
        ),
        element: Element::Cryo,
        hp: [903, 2319, 2993, 4484, 4963, 5708, 6347, 7093, 7572, 8317, 8796, 9541, 10021, 10766],
        atk: [15, 37, 48, 72, 80, 92, 102, 114, 122, 134, 141, 153, 161, 173],
        def: [46, 118, 152, 227, 252, 290, 322, 360, 384, 422, 446, 484, 508, 546],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Catalyst,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·冷色摄影律",
            en: "Normal Attack: Cool-Color Capture",
        ),
        skill_name2: locale!(
            zh_cn: "取景·冰点构图法",
            en: "Framing: Freezing Point Composition"
        ),
        skill_name3: locale!(
            zh_cn: "定格·全方位确证",
            en: "Still Photo: Comprehensive Confirmation"
        )
    };
    type SkillType = CharlotteSkillType;
    const SKILL: Self::SkillType = CHARLOTTE_SKILL;
    type DamageEnumType = CharlotteDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            CharlotteDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            CharlotteDamageEnum
            E1 locale!(zh_cn: "点按拍照伤害", en: "Photo DMG (Press)")
            E2 locale!(zh_cn: "长按拍照伤害", en: "Photo DMG (Hold)")
            E3 locale!(zh_cn: "「瞬时剪影」印记伤害", en: "\"Snappy Silhouette\" Mark DMG")
            E4 locale!(zh_cn: "「聚焦印象」印记伤害", en: "\"Focused Impression\" Mark DMG")
        ),
        skill3: skill_map!(
            CharlotteDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "相机伤害", en: "Kamera DMG")
            QHeal1 locale!(zh_cn: "施放治疗量", en: "Cast Healing")
            QHeal2 locale!(zh_cn: "相机持续治疗量", en: "Kamera Continuous Regeneration")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_fontaine_count",
            title: locale!(
                zh_cn: "枫丹角色数量",
                en: "Fontaine Character Count"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 1 },
        },
        ItemConfig {
            name: "talent2_non_fontaine_count",
            title: locale!(
                zh_cn: "非枫丹角色数量",
                en: "Non-Fontaine Character Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 1 }
        },
        ItemConfig {
            name: "c2_count",
            title: locale!(
                zh_cn: "「温亨廷先生」命中数量",
                en: "Monsieur Verite Hit Count"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
        ItemConfig {
            name: "c2_rate",
            title: locale!(
                zh_cn: "C2比例",
                en: "C2 Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: CharlotteDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use CharlotteDamageEnum::*;
        let mut builder = D::new();

        if s.is_heal() {
            let ratio = match s {
                QHeal1 => CHARLOTTE_SKILL.q_heal1[s3],
                QHeal2 => CHARLOTTE_SKILL.q_heal2[s3],
                _ => 0.0
            };
            let fixed = match s {
                QHeal1 => CHARLOTTE_SKILL.q_heal1_fixed[s3],
                QHeal2 => CHARLOTTE_SKILL.q_heal2_fixed[s3],
                _ => 0.0
            };
            builder.add_atk_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => CHARLOTTE_SKILL.normal_dmg1[s1],
                Normal2 => CHARLOTTE_SKILL.normal_dmg2[s1],
                Normal3 => CHARLOTTE_SKILL.normal_dmg3[s1],
                Charged => CHARLOTTE_SKILL.charged_dmg1[s1],
                Plunging1 => CHARLOTTE_SKILL.plunging_dmg1[s1],
                Plunging2 => CHARLOTTE_SKILL.plunging_dmg2[s1],
                Plunging3 => CHARLOTTE_SKILL.plunging_dmg3[s1],
                E1 => CHARLOTTE_SKILL.e_dmg1[s2],
                E2 => CHARLOTTE_SKILL.e_dmg2[s2],
                E3 => CHARLOTTE_SKILL.e_dmg3[s2],
                E4 => CHARLOTTE_SKILL.e_dmg4[s2],
                Q1 => CHARLOTTE_SKILL.q_dmg1[s3],
                Q2 => CHARLOTTE_SKILL.q_dmg2[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);
            builder.damage(
                &context.attribute,
                &context.enemy,
                Element::Cryo,
                s.get_skill_type(),
                context.character_common_data.level,
                fumo
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        match *config {
            CharacterConfig::Charlotte { talent2_fontaine_count, talent2_non_fontaine_count, c2_count, c2_rate } => {
                Some(Box::new(CharlotteEffect {
                    talent2_fontaine_count,
                    talent2_non_fontaine_count,
                    c2_count,
                    c2_rate
                }))
            },
            _ => None
        }
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
