use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::Attribute;
use crate::attribute::attribute_name::AttributeName;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::DehyaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct DehyaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3_atk: [f64; 15],
    pub elemental_skill_dmg3_hp: [f64; 15],

    pub elemental_burst_dmg1_atk: [f64; 15],
    pub elemental_burst_dmg1_hp: [f64; 15],
    pub elemental_burst_dmg2_atk: [f64; 15],
    pub elemental_burst_dmg2_hp: [f64; 15],
}

pub const DEHYA_SKILL: DehyaSkillType = DehyaSkillType {
    normal_dmg1: [0.6212, 0.6717, 0.7223, 0.7945, 0.8451, 0.9029, 0.9823, 1.0618, 1.1412, 1.2279, 1.3146, 1.4013, 1.4879, 1.5746, 1.6613],
    normal_dmg2: [0.6171, 0.6673, 0.7176, 0.7893, 0.8395, 0.897, 0.9759, 1.0548, 1.1337, 1.2199, 1.306, 1.3921, 1.4782, 1.5643, 1.6504],
    normal_dmg3: [0.7663, 0.8287, 0.8911, 0.9802, 1.0425, 1.1138, 1.2118, 1.3099, 1.4079, 1.5148, 1.6217, 1.7287, 1.8356, 1.9425, 2.0494],
    normal_dmg4: [0.9529, 1.0305, 1.108, 1.2188, 1.2964, 1.3851, 1.5069, 1.6288, 1.7507, 1.8837, 2.0166, 2.1496, 2.2826, 2.4155, 2.5485],
    charged_dmg1: [0.5633, 0.6091, 0.655, 0.7205, 0.7663, 0.8188, 0.8908, 0.9628, 1.0349, 1.1135, 1.1921, 1.2707, 1.3493, 1.4279, 1.5065],
    charged_dmg2: [1.0182, 1.1011, 1.184, 1.3024, 1.3853, 1.48, 1.6102, 1.7405, 1.8707, 2.0128, 2.1549, 2.297, 2.439, 2.5811, 2.7232],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    elemental_skill_dmg1: [1.1288, 1.2135, 1.2981, 1.411, 1.4957, 1.5803, 1.6932, 1.8061, 1.919, 2.0318, 2.1447, 2.2576, 2.3987, 2.5398, 2.6809],
    elemental_skill_dmg2: [1.328, 1.4276, 1.5272, 1.66, 1.7596, 1.8592, 1.992, 2.1248, 2.2576, 2.3904, 2.5232, 2.656, 2.822, 2.988, 3.154],
    elemental_skill_dmg3_atk: [0.602, 0.6471, 0.6923, 0.7525, 0.7976, 0.8428, 0.903, 0.9632, 1.0234, 1.0836, 1.1438, 1.204, 1.2793, 1.3545, 1.4297],
    elemental_skill_dmg3_hp: [0.0103, 0.0111, 0.0119, 0.0129, 0.0137, 0.0144, 0.0155, 0.0165, 0.0175, 0.0186, 0.0196, 0.0206, 0.0219, 0.0232, 0.0245],
    elemental_burst_dmg1_atk: [0.987, 1.061, 1.1351, 1.2337, 1.3078, 1.3818, 1.4805, 1.5792, 1.6779, 1.7766, 1.8753, 1.974, 2.0974, 2.2208, 2.3441],
    elemental_burst_dmg1_hp: [0.0169, 0.0182, 0.0195, 0.0212, 0.0224, 0.0237, 0.0254, 0.0271, 0.0288, 0.0305, 0.0321, 0.0338, 0.036, 0.0381, 0.0402],
    elemental_burst_dmg2_atk: [1.393, 1.4975, 1.602, 1.7413, 1.8457, 1.9502, 2.0895, 2.2288, 2.3681, 2.5074, 2.6467, 2.786, 2.9601, 3.1342, 3.3084],
    elemental_burst_dmg2_hp: [0.0239, 0.0257, 0.0275, 0.0299, 0.0316, 0.0334, 0.0358, 0.0382, 0.0406, 0.043, 0.0454, 0.0478, 0.0507, 0.0537, 0.0567],
};

const DEHYA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Dehya,
    internal_name: "Dehya",
    element: Element::Pyro,
    hp: [1220, 3165, 4212, 6302, 7045, 8106, 9097, 10168, 10912, 11993, 12736, 13829, 14573, 15675],
    atk: [21, 54, 71, 107, 119, 137, 154, 172, 185, 203, 216, 234, 247, 265],
    def: [49, 127, 169, 252, 282, 325, 364, 407, 437, 480, 510, 554, 584, 628],
    sub_stat: CharacterSubStatFamily::HP288,
    weapon_type: WeaponType::Claymore,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·拂金剑斗术",
        en: "Normal Attack: Sandstorm Assault",
    ),
    skill_name2: locale!(
        zh_cn: "熔铁流狱",
        en: "Molten Inferno",
    ),
    skill_name3: locale!(
        zh_cn: "炎啸狮子咬",
        en: "Leonine Bite",
    ),
    name_locale: locale!(
        zh_cn: "迪希雅",
        en: "Dehya",
    ),
};

pub struct DehyaEffect {
    pub c1: bool,
}

impl<A: Attribute> ChangeAttribute<A> for DehyaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.c1 {
            attribute.set_value_by(AttributeName::HPPercentage, "迪希雅一命：皎洁之火铓辉灿漫", 0.2);
        }
    }
}

pub struct Dehya;

#[derive(Copy, Clone)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum DehyaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3,
    Q1,
    Q2,
}

impl DehyaDamageEnum {
    pub fn get_element(&self) -> Element {
        use DehyaDamageEnum::*;
        match *self {
            E1 | E2 | E3 | Q1 | Q2 => Element::Pyro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use DehyaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

impl Into<usize> for DehyaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum DehyaRoleEnum {
    Main
}

impl CharacterTrait for Dehya {
    const STATIC_DATA: CharacterStaticData = DEHYA_STATIC_DATA;
    type SkillType = DehyaSkillType;
    const SKILL: Self::SkillType = DEHYA_SKILL;
    type DamageEnumType = DehyaDamageEnum;
    type RoleEnum = DehyaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: DehyaDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: DehyaDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: DehyaDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: DehyaDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: DehyaDamageEnum::Charged1 as usize, text: charged_dmg!("loop1") },
            CharacterSkillMapItem { index: DehyaDamageEnum::Charged2 as usize, text: charged_dmg!("loop2") },
            CharacterSkillMapItem { index: DehyaDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: DehyaDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: DehyaDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: DehyaDamageEnum::E1 as usize, text: locale!(zh_cn: "净焰昂藏伤害", en: "Indomitable Flame DMG") },
            CharacterSkillMapItem { index: DehyaDamageEnum::E2 as usize, text: locale!(zh_cn: "剑域炽焰伤害", en: "Ranging Flame DMG") },
            CharacterSkillMapItem { index: DehyaDamageEnum::E3 as usize, text: locale!(zh_cn: "领域伤害", en: "Field DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: DehyaDamageEnum::Q1 as usize, text: locale!(zh_cn: "炽鬃拳伤害", en: "Flame-Mane’s Fist DMG") },
            CharacterSkillMapItem { index: DehyaDamageEnum::Q2 as usize, text: locale!(zh_cn: "焚落踢伤害", en: "Incineration Drive DMG") },
        ]),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c2_rate",
            title: locale!(
                zh_cn: "二命等效增伤覆盖率",
                en: "C2 Equivalent DMG Bonus",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "c6_stack",
            title: locale!(
                zh_cn: "六命爆伤等效层数",
                en: "C6 Equivalent CritDMG Bonus",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 3.5 },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: DehyaDamageEnum = num::FromPrimitive::from_usize(s).expect("wrong skill index");
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use DehyaDamageEnum::*;
        let ratio_atk = match s {
            Normal1 => DEHYA_SKILL.normal_dmg1[s1],
            Normal2 => DEHYA_SKILL.normal_dmg2[s1],
            Normal3 => DEHYA_SKILL.normal_dmg3[s1],
            Normal4 => DEHYA_SKILL.normal_dmg4[s1],
            Charged1 => DEHYA_SKILL.charged_dmg1[s1],
            Charged2 => DEHYA_SKILL.charged_dmg2[s1],
            Plunging1 => DEHYA_SKILL.plunging_dmg1[s1],
            Plunging2 => DEHYA_SKILL.plunging_dmg2[s1],
            Plunging3 => DEHYA_SKILL.plunging_dmg3[s1],
            E1 => DEHYA_SKILL.elemental_skill_dmg1[s2],
            E2 => DEHYA_SKILL.elemental_skill_dmg2[s2],
            E3 => DEHYA_SKILL.elemental_skill_dmg3_atk[s2],
            Q1 => DEHYA_SKILL.elemental_burst_dmg1_atk[s3],
            Q2 => DEHYA_SKILL.elemental_burst_dmg2_atk[s3],
        };
        let c1 = context.character_common_data.constellation >= 1;
        let c2 = context.character_common_data.constellation >= 2;
        let c6 = context.character_common_data.constellation >= 6;

        let ratio_hp_base = match s {
            E3 => DEHYA_SKILL.elemental_skill_dmg3_hp[s2],
            Q1 => DEHYA_SKILL.elemental_burst_dmg1_hp[s3],
            Q2 => DEHYA_SKILL.elemental_burst_dmg2_hp[s3],
            _ => 0.0,
        };

        let mut builder = D::new();
        builder.add_atk_ratio("攻击倍率", ratio_atk);
        if ratio_hp_base != 0.0 {
            builder.add_hp_ratio("生命倍率", ratio_hp_base);
        }

        if c1 {
            let ratio_hp_c1 = match s {
                E1 | E2 | E3 => 0.036,
                Q1 | Q2 => 0.06,
                _ => 0.0,
            };
            if ratio_hp_c1 != 0.0 {
                builder.add_hp_ratio("一命：皎洁之火铓辉灿漫", ratio_hp_c1);
            }
        }

        let (c2_rate, c6_stack) = match *config {
            CharacterSkillConfig::Dehya { c2_rate, c6_stack } => (c2_rate, c6_stack),
            _ => (0.0, 0.0),
        };

        // C2 effect
        if c2 {
            let c2_dmgb = match s {
                E3 => 0.5,
                _ => 0.0,
            };
            builder.add_extra_bonus("二命：净沙利刃明映万乘", c2_dmgb * c2_rate);
        }

        let skill_type = s.get_skill_type();
        if c6 {
            let c6_cdmg = match s {
                Q1 | Q2 => 0.15 * c6_stack,
                _ => 0.0,
            };
            builder.add_extra_critical_damage("六命：燎燃利爪裂帛斫金", c6_cdmg);
            if skill_type == SkillType::ElementalBurst {
                builder.add_extra_critical("六命：燎燃利爪裂帛斫金", 0.1);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(DehyaEffect {
            c1: common_data.constellation >= 1,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
