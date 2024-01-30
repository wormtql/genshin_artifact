// use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::KamisatoAyatoDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
// use strum::EnumCount;
// use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::character::macros::{skill_type, damage_enum, skill_map, damage_ratio};
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};

pub struct NilouSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg21: [f64; 15],
    pub elemental_skill_dmg22: [f64; 15],
    pub elemental_skill_dmg31: [f64; 15],
    pub elemental_skill_dmg32: [f64; 15],
    pub elemental_skill_dmg41: [f64; 15],
    pub elemental_skill_dmg42: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
}

pub const NILOU_SKILL: NilouSkillType = NilouSkillType {
    normal_dmg1: [0.5031, 0.544, 0.585, 0.6435, 0.6844, 0.7312, 0.7956, 0.8599, 0.9243, 0.9944, 1.0646, 1.1348, 1.205, 1.2752, 1.3454],
    normal_dmg2: [0.4544, 0.4914, 0.5284, 0.5812, 0.6182, 0.6604, 0.7186, 0.7767, 0.8348, 0.8982, 0.9616, 1.025, 1.0884, 1.1518, 1.2152],
    normal_dmg3: [0.7035, 0.7608, 0.8181, 0.8999, 0.9571, 1.0226, 1.1126, 1.2026, 1.2926, 1.3907, 1.4889, 1.5871, 1.6852, 1.7834, 1.8816],
    charged_dmg11: [0.5022, 0.5431, 0.584, 0.6424, 0.6833, 0.73, 0.7942, 0.8585, 0.9227, 0.9928, 1.0629, 1.133, 1.203, 1.2731, 1.3432],
    charged_dmg12: [0.5444, 0.5887, 0.633, 0.6963, 0.7406, 0.7912, 0.8609, 0.9305, 1.0001, 1.0761, 1.1521, 1.228, 1.304, 1.3799, 1.4559],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [0.0334, 0.0359, 0.0384, 0.0417, 0.0442, 0.0467, 0.0501, 0.0534, 0.0568, 0.0601, 0.0634, 0.0668, 0.071, 0.0751, 0.0793],
    elemental_skill_dmg21: [0.0455, 0.0489, 0.0524, 0.0569, 0.0603, 0.0637, 0.0683, 0.0728, 0.0774, 0.0819, 0.0865, 0.091, 0.0967, 0.1024, 0.1081],
    elemental_skill_dmg22: [0.0326, 0.0351, 0.0375, 0.0408, 0.0432, 0.0457, 0.0489, 0.0522, 0.0555, 0.0587, 0.062, 0.0652, 0.0693, 0.0734, 0.0775],
    elemental_skill_dmg31: [0.0514, 0.0553, 0.0592, 0.0643, 0.0682, 0.072, 0.0772, 0.0823, 0.0875, 0.0926, 0.0977, 0.1029, 0.1093, 0.1158, 0.1222],
    elemental_skill_dmg32: [0.0396, 0.0426, 0.0455, 0.0495, 0.0525, 0.0554, 0.0594, 0.0634, 0.0673, 0.0713, 0.0752, 0.0792, 0.0842, 0.0891, 0.0941],
    elemental_skill_dmg41: [0.0717, 0.0771, 0.0824, 0.0896, 0.095, 0.1004, 0.1075, 0.1147, 0.1219, 0.129, 0.1362, 0.1434, 0.1523, 0.1613, 0.1703],
    elemental_skill_dmg42: [0.0506, 0.0544, 0.0582, 0.0633, 0.0671, 0.0709, 0.0759, 0.081, 0.086, 0.0911, 0.0962, 0.1012, 0.1076, 0.1139, 0.1202],
    elemental_burst_dmg1: [0.1843, 0.1981, 0.212, 0.2304, 0.2442, 0.258, 0.2765, 0.2949, 0.3133, 0.3318, 0.3502, 0.3686, 0.3917, 0.4147, 0.4378],
    elemental_burst_dmg2: [0.2253, 0.2422, 0.2591, 0.2816, 0.2985, 0.3154, 0.3379, 0.3604, 0.383, 0.4055, 0.428, 0.4506, 0.4787, 0.5069, 0.535],
};

pub struct NilouEffect {
    pub c6: bool,
    pub has_talent1: bool,
    pub has_talent2: bool,
    pub golden_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for NilouEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.c6 {
            attribute.add_edge1(
                AttributeName::HP, AttributeName::CriticalBase,
                Box::new(|hp, _| ((hp / 1000.0).floor() * 0.006).min(0.3)),
                Box::new(|hp, _, grad| (0.0, 0.0)), // todo
                "六命：「断霜的弦歌」"
            );
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::CriticalDamageBase,
                Box::new(|hp, _| ((hp / 1000.0).floor() * 0.012).min(0.6)),
                Box::new(|hp, _, grad| (0.0, 0.0)), // todo
                "六命：「断霜的弦歌」"
            );
        }

        if self.has_talent1 {
            attribute.set_value_by(AttributeName::ElementalMastery, "天赋「折旋落英之庭」", self.golden_rate * 100.0);
        }

        if self.has_talent2 {
            let rate = self.golden_rate;
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::EnhanceBloom,
                Box::new(move |hp, _| {
                    if hp >= 31000.0 {
                        let value = ((hp - 30000.0) / 1000.0).floor() * 0.09;
                        let value = value.min(4.0);
                        value * rate
                    } else {
                        0.0
                    }
                }),
                Box::new(move |hp, _, grad| (0.0, 0.0)),
                "天赋「翩舞永世之梦」"
            );
        }
    }
}

damage_enum!(
    NilouDamageEnum
    Normal1
    Normal2
    Normal3
    Charged11
    Charged12
    Plunging1
    Plunging2
    Plunging3
    E1
    E21
    E22
    E31
    E32
    E41
    E42
    Q1
    Q2
);

impl NilouDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use NilouDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Q1 | Q2 => SkillType::ElementalBurst,
            _ => SkillType:: ElementalSkill,
        }
    }

    pub fn get_element(&self) -> Element {
        use NilouDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Charged11 | Charged12 | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            _ => Element::Hydro
        }
    }
}

pub struct Nilou;

impl CharacterTrait for Nilou {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Nilou,
        internal_name: "Nilou",
        element: Element::Hydro,
        hp: [1182, 3066, 4080, 6105, 6825, 7852, 8813, 9850, 10571, 11618, 12338, 13397, 14117, 15185],
        atk: [18, 46, 62, 92, 103, 119, 133, 149, 160, 176, 187, 203, 213, 230],
        def: [57, 147, 196, 293, 327, 377, 423, 473, 507, 557, 592, 643, 677, 729],
        sub_stat: CharacterSubStatFamily::HP288,
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·弦月舞步",
            en: "Normal Attack: Dance of Samser",
        ),
        skill_name2: locale!(
            zh_cn: "七域舞步",
            en: "Dance of Haftkarsvar",
        ),
        skill_name3: locale!(
            zh_cn: "浮莲舞步·远梦聆泉",
            en: "Dance of Abzendegi: Distant Dreams, Listening Spring",
        ),
        name_locale: locale!(
            zh_cn: "妮露",
            en: "Nilou",
        )
    };
    type SkillType = NilouSkillType;
    const SKILL: Self::SkillType = NILOU_SKILL;
    type DamageEnumType = NilouDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "golden_rate",
            title: locale!(
                zh_cn: "「金杯的丰馈」比例",
                en: "「Golden Chalice's Bounty」Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            NilouDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged11 charged_dmg!(1)
            Charged12 charged_dmg!(2)
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            NilouDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E21 locale!(zh_cn: "剑舞步一段伤害", en: "Sword Dance 1-Hit DMG")
            E22 locale!(zh_cn: "旋舞步一段伤害", en: "Whirling Steps 1-Hit DMG")
            E31 locale!(zh_cn: "剑舞步二段伤害", en: "Sword Dance 2-Hit DMG")
            E32 locale!(zh_cn: "旋舞步二段伤害", en: "Whirling Steps 2-Hit DMG")
            E41 locale!(zh_cn: "水月伤害", en: "Luminous Illusion DMG")
            E42 locale!(zh_cn: "水轮伤害", en: "Water Wheel DMG")
        ),
        skill3: skill_map!(
            NilouDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "永世流沔伤害", en: "Lingering Aeon DMG")
        )
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: NilouDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use NilouDamageEnum::*;
        let ratio = damage_ratio!(
            NILOU_SKILL
            s
            Normal1 normal_dmg1 s1
            Normal2 normal_dmg2 s1
            Normal3 normal_dmg3 s1
            Charged11 charged_dmg11 s1
            Charged12 charged_dmg12 s1
            Plunging1 plunging_dmg1 s1
            Plunging2 plunging_dmg2 s1
            Plunging3 plunging_dmg3 s1
            E1 elemental_skill_dmg1 s2
            E21 elemental_skill_dmg21 s2
            E22 elemental_skill_dmg22 s2
            E31 elemental_skill_dmg31 s2
            E32 elemental_skill_dmg32 s2
            E41 elemental_skill_dmg41 s2
            E42 elemental_skill_dmg42 s2
            Q1 elemental_burst_dmg1 s3
            Q2 elemental_burst_dmg2 s3
        );

        let skill_type = s.get_skill_type();
        let element = s.get_element();

        let mut builder = D::new();
        if skill_type == SkillType::NormalAttack || skill_type == SkillType::ChargedAttack || skill_type.is_plunging() {
            builder.add_atk_ratio("技能倍率", ratio);
        } else {
            builder.add_hp_ratio("技能倍率", ratio);
        }

        if context.character_common_data.constellation >= 1 && s == NilouDamageEnum::E41 {
            builder.add_extra_bonus("一命「却月的轻舞」", 0.65);
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
        let golden_rate = match *config {
            CharacterConfig::Nilou { golden_rate } => golden_rate,
            _ => 0.0
        };
        Some(Box::new(NilouEffect {
            c6: common_data.constellation >= 6,
            has_talent1: common_data.has_talent1,
            has_talent2: common_data.has_talent2,
            golden_rate,
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}

