use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeName};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::DamageContext;
use crate::damage::damage_builder::DamageBuilder;
use crate::target_functions::target_functions::HuTaoDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct HuTaoSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg51: [f64; 15],
    pub normal_dmg52: [f64; 15],
    pub normal_dmg6: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_hp_cost: [f64; 15],
    pub elemental_skill_blood_blossom: [f64; 15],
    pub elemental_skill_atk_bonus: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_heal1: [f64; 15],
    pub elemental_burst_heal2: [f64; 15],
}

pub const HU_TAO_SKILL: HuTaoSkillType = HuTaoSkillType {
    normal_dmg1: [0.4689, 0.5008, 0.5328, 0.5754, 0.6074, 0.6447, 0.6926, 0.7406, 0.7885, 0.8365, 0.8844, 0.9324, 0.9804, 1.0283, 1.0763],
    normal_dmg2: [0.4825, 0.5154, 0.5483, 0.5922, 0.6251, 0.6635, 0.7128, 0.7622, 0.8115, 0.8609, 0.9102, 0.9596, 1.0089, 1.0583, 1.1076],
    normal_dmg3: [0.6105, 0.6521, 0.6938, 0.7493, 0.7909, 0.8394, 0.9019, 0.9643, 1.0268, 1.0892, 1.1516, 1.2141, 1.2765, 1.3389, 1.4014],
    normal_dmg4: [0.6564, 0.7012, 0.7459, 0.8056, 0.8503, 0.9026, 0.9697, 1.0368, 1.104, 1.1711, 1.2382, 1.3054, 1.3725, 1.4396, 1.5068],
    normal_dmg51: [0.3327, 0.3554, 0.3781, 0.4084, 0.431, 0.4575, 0.4915, 0.5256, 0.5596, 0.5936, 0.6277, 0.6617, 0.6957, 0.7298, 0.7638],
    normal_dmg52: [0.352, 0.376, 0.4, 0.432, 0.456, 0.484, 0.52, 0.556, 0.592, 0.628, 0.664, 0.7, 0.736, 0.772, 0.808],
    normal_dmg6: [0.8596, 0.9182, 0.9768, 1.0549, 1.1136, 1.1819, 1.2698, 1.3578, 1.4457, 1.5336, 1.6215, 1.7094, 1.7973, 1.8852, 1.9731],
    charged_dmg1: [1.3596, 1.4523, 1.545, 1.6686, 1.7613, 1.8695, 2.0085, 2.1476, 2.2866, 2.4257, 2.5647, 2.7038, 2.8428, 2.9819, 3.1209],
    plunging_dmg1: [0.6542, 0.6988, 0.7434, 0.8029, 0.8475, 0.8995, 0.9664, 1.0333, 1.1002, 1.1671, 1.234, 1.301, 1.3679, 1.4348, 1.5017],
    plunging_dmg2: [1.3081, 1.3973, 1.4865, 1.6054, 1.6946, 1.7986, 1.9324, 2.0662, 2.2, 2.3338, 2.4676, 2.6013, 2.7351, 2.8689, 3.0027],
    plunging_dmg3: [1.6339, 1.7453, 1.8567, 2.0052, 2.1166, 2.2466, 2.4137, 2.5808, 2.7479, 2.915, 3.0821, 3.2492, 3.4163, 3.5834, 3.7505],
    elemental_skill_hp_cost: [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3],
    elemental_skill_blood_blossom: [0.64, 0.688, 0.736, 0.8, 0.848, 0.896, 0.96, 1.024, 1.088, 1.152, 1.216, 1.28, 1.36, 1.44, 1.52],
    elemental_skill_atk_bonus: [0.0384, 0.0407, 0.043, 0.046, 0.0483, 0.0506, 0.0536, 0.0566, 0.0596, 0.0626, 0.0656, 0.0685, 0.0715, 0.0745, 0.0775],
    elemental_burst_dmg1: [3.0327, 3.2143, 3.3959, 3.632, 3.8136, 3.9952, 4.2313, 4.4674, 4.7034, 4.9395, 5.1756, 5.4117, 5.6478, 5.8838, 6.1199],
    elemental_burst_dmg2: [3.7909, 4.0179, 4.2449, 4.54, 4.767, 4.994, 5.2891, 5.5842, 5.8793, 6.1744, 6.4695, 6.7646, 7.0597, 7.3548, 7.6499],
    elemental_burst_heal1: [0.0626, 0.0664, 0.0701, 0.075, 0.0788, 0.0825, 0.0874, 0.0923, 0.0971, 0.102, 0.1069, 0.1118, 0.1166, 0.1215, 0.1264],
    elemental_burst_heal2: [0.0835, 0.0885, 0.0935, 0.1, 0.105, 0.11, 0.1165, 0.123, 0.1295, 0.136, 0.1425, 0.149, 0.1555, 0.162, 0.1685],
};

pub const HU_TAO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::HuTao,
    internal_name: "Hutao",
    element: Element::Pyro,
    hp: [1211, 3141, 4179, 6253, 6990, 8042, 9026, 10089, 10826, 11899, 12637, 13721, 14459, 15552],
    atk: [8, 21, 29, 43, 48, 55, 62, 69, 74, 81, 86, 94, 99, 106],
    def: [68, 177, 235, 352, 394, 453, 508, 568, 610, 670, 712, 773, 815, 876],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Polearm,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·往生秘传枪法",
        en: "Normal Attack: Secret Spear of Wangsheng",
    ),
    skill_name2: locale!(
        zh_cn: "蝶引来生",
        en: "Guide to Afterlife",
    ),
    skill_name3: locale!(
        zh_cn: "安神秘法",
        en: "Spirit Soother",
    ),
    name_locale: locale!(
        zh_cn: "胡桃",
        en: "Hu Tao",
    )
};

pub struct HuTaoEffect {
    pub has_talent2: bool,
    pub le_50: bool,
}

impl HuTaoEffect {
    pub fn new(common_data: &CharacterCommonData, config: &CharacterConfig) -> HuTaoEffect {
        let le_50 = match config {
            CharacterConfig::HuTao { le_50 } => *le_50,
            _ => false,
        };

        HuTaoEffect {
            has_talent2: common_data.has_talent2,
            le_50,
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for HuTaoEffect {
    fn change_attribute(&self, attribute: &mut T) {
        if self.has_talent2 && self.le_50 {
            attribute.set_value_by(AttributeName::BonusPyro, "胡桃天赋2：血之灶火", 0.33);
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
#[derive(EnumString, EnumCountMacro)]
#[derive(Eq, PartialEq)]
pub enum HuTaoDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Normal51,
    Normal52,
    Normal6,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,
    ElementalSkillBloodBlossom,
    ElementalBurst1,
    ElementalBurstLow1,
}

impl Into<usize> for HuTaoDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl HuTaoDamageEnum {
    pub fn get_element(&self, after_e: bool) -> Element {
        use HuTaoDamageEnum::*;

        if after_e {
            Element::Pyro
        } else {
            match *self {
                ElementalSkillBloodBlossom | ElementalBurst1 | ElementalBurstLow1 => Element::Pyro,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use HuTaoDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal51 | Normal52 | Normal6 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            ElementalSkillBloodBlossom => SkillType::ElementalSkill,
            ElementalBurst1 | ElementalBurstLow1 => SkillType::ElementalBurst
        }
    }
}

pub struct HuTao;

#[derive(Copy, Clone, FromPrimitive)]
pub enum HuTaoRoleEnum {
    Main
}

impl CharacterTrait for HuTao {
    const STATIC_DATA: CharacterStaticData = HU_TAO_STATIC_DATA;
    type SkillType = HuTaoSkillType;
    const SKILL: Self::SkillType = HU_TAO_SKILL;
    type DamageEnumType = HuTaoDamageEnum;
    type RoleEnum = HuTaoRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal51 as usize, text: hit_n_dmg!(5, 1) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal52 as usize, text: hit_n_dmg!(5, 2) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal6 as usize, text: hit_n_dmg!(6) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: HuTaoDamageEnum::ElementalSkillBloodBlossom as usize, text: locale!(zh_cn: "血梅香伤害", en: "Blood Blossom DMG") }
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: HuTaoDamageEnum::ElementalBurst1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: HuTaoDamageEnum::ElementalBurstLow1 as usize, text: locale!(zh_cn: "低血量时技能伤害", en: "Low HP Skill DMG") },
        ]),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "le_50",
            title: locale!(
                zh_cn: "生命值低于50%",
                en: "HP Below 50%",
            ),
            config: ItemConfigType::Bool { default: true },
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_e",
            title: locale!(
                zh_cn: "彼岸蝶舞",
                en: "Guide to Afterlife",
            ),
            config: ItemConfigType::Bool { default: true },
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: HuTaoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use HuTaoDamageEnum::*;
        let ratio = match s {
            Normal1 => HU_TAO_SKILL.normal_dmg1[s1],
            Normal2 => HU_TAO_SKILL.normal_dmg2[s1],
            Normal3 => HU_TAO_SKILL.normal_dmg3[s1],
            Normal4 => HU_TAO_SKILL.normal_dmg4[s1],
            Normal51 => HU_TAO_SKILL.normal_dmg51[s1],
            Normal52 => HU_TAO_SKILL.normal_dmg52[s1],
            Normal6 => HU_TAO_SKILL.normal_dmg6[s1],
            Charged => HU_TAO_SKILL.charged_dmg1[s1],
            Plunging1 => HU_TAO_SKILL.plunging_dmg1[s1],
            Plunging2 => HU_TAO_SKILL.plunging_dmg2[s1],
            Plunging3 => HU_TAO_SKILL.plunging_dmg3[s1],
            ElementalSkillBloodBlossom => HU_TAO_SKILL.elemental_skill_blood_blossom[s2],
            ElementalBurst1 => HU_TAO_SKILL.elemental_burst_dmg1[s3],
            ElementalBurstLow1 => HU_TAO_SKILL.elemental_burst_dmg2[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let c2 = context.character_common_data.constellation >= 2;
        let after_e = match *config {
            CharacterSkillConfig::HuTao { after_e } => after_e,
            _ => false
        };
        if after_e {
            let hp = context.attribute.get_value(AttributeName::HP);
            let atk_base = context.attribute.get_value(AttributeName::ATKBase);
            let atk_bonus = (HU_TAO_SKILL.elemental_skill_atk_bonus[s2] * hp).min(4.0 * atk_base);
            builder.add_extra_atk("胡桃：彼岸蝶舞", atk_bonus);
        }
        if c2 && s == HuTaoDamageEnum::ElementalSkillBloodBlossom {
            builder.add_hp_ratio("二命：最不安神晴又复雨", 0.1);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(after_e),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(HuTaoEffect::new(common_data, config)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: HuTaoRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            HuTaoRoleEnum::Main => Box::new(HuTaoDefaultTargetFunction {
                vaporize_rate: 0.5,
                melt_rate: 0.0,
            })
        }
    }
}
