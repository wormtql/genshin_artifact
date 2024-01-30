use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct KukiShinobuSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_heal1: [f64; 15],
    pub elemental_skill_heal1_fixed: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],
    pub elemental_burst_dmg3: [f64; 15],
}

pub const KUKI_SHINOBU_SKILL: KukiShinobuSkillType = KukiShinobuSkillType {
    normal_dmg1: [0.4876, 0.5273, 0.567, 0.6237, 0.6634, 0.7088, 0.7711, 0.8335, 0.8959, 0.9639, 1.0319, 1.1, 1.168, 1.2361, 1.3041],
    normal_dmg2: [0.4455, 0.4817, 0.518, 0.5698, 0.6061, 0.6475, 0.7045, 0.7615, 0.8184, 0.8806, 0.9428, 1.0049, 1.0671, 1.1292, 1.1914],
    normal_dmg3: [0.5934, 0.6417, 0.69, 0.759, 0.8073, 0.8625, 0.9384, 1.0143, 1.0902, 1.173, 1.2558, 1.3386, 1.4214, 1.5042, 1.587],
    normal_dmg4: [0.7611, 0.8231, 0.885, 0.9735, 1.0355, 1.1063, 1.2036, 1.301, 1.3983, 1.5045, 1.6107, 1.7169, 1.8231, 1.9293, 2.0355],
    charged_dmg11: [0.5563, 0.6016, 0.6469, 0.7116, 0.7569, 0.8086, 0.8798, 0.9509, 1.0221, 1.0997, 1.1774, 1.255, 1.3326, 1.4102, 1.4879],
    charged_dmg12: [0.6677, 0.722, 0.7763, 0.854, 0.9083, 0.9704, 1.0558, 1.1412, 1.2266, 1.3198, 1.4129, 1.5061, 1.5993, 1.6924, 1.7856],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [0.7571, 0.8139, 0.8707, 0.9464, 1.0032, 1.06, 1.1357, 1.2114, 1.2871, 1.3628, 1.4385, 1.5142, 1.6089, 1.7035, 1.7982],
    elemental_skill_heal1: [0.03, 0.0323, 0.0345, 0.0375, 0.0398, 0.042, 0.045, 0.048, 0.051, 0.054, 0.057, 0.06, 0.0638, 0.0675, 0.0712],
    elemental_skill_heal1_fixed: [289.0, 318.0, 349.0, 383.0, 419.0, 457.0, 498.0, 542.0, 587.0, 636.0, 686.0, 739.0, 795.0, 852.0, 913.0],
    elemental_skill_dmg2: [0.2524, 0.2713, 0.2903, 0.3155, 0.3344, 0.3534, 0.3786, 0.4038, 0.4291, 0.4543, 0.4796, 0.5048, 0.5364, 0.5679, 0.5995],
    elemental_burst_dmg1: [0.036, 0.0388, 0.0415, 0.0451, 0.0478, 0.0505, 0.0541, 0.0577, 0.0613, 0.0649, 0.0685, 0.0721, 0.0766, 0.0811, 0.0856],
    elemental_burst_dmg2: [0.2523, 0.2713, 0.2902, 0.3154, 0.3343, 0.3533, 0.3785, 0.4037, 0.429, 0.4542, 0.4794, 0.5047, 0.5362, 0.5678, 0.5993],
    elemental_burst_dmg3: [0.4326, 0.465, 0.4975, 0.5407, 0.5732, 0.6056, 0.6489, 0.6921, 0.7354, 0.7786, 0.8219, 0.8652, 0.9192, 0.9733, 1.0274],
};

pub struct KukiShinobuEffect {
    pub hp_le_50: bool,
    pub use_c6: bool,
    pub has_talent1: bool,
    pub has_c6: bool,
}

impl<A: Attribute> ChangeAttribute<A> for KukiShinobuEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent1 && self.hp_le_50 {
            attribute.set_value_by(AttributeName::HealingBonus, "天赋：破笼之志", 0.15);
        }
        if self.has_c6 && self.use_c6 {
            attribute.set_value_by(AttributeName::ElementalMastery, "六命：割舍软弱之心", 150.0);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum KukiShinobuDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged11,
    Charged12,
    /// charged11 + charged12
    Charged1,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    EHeal1,
    E2,
    Q1,
    Q2,
    Q3,
}

impl Into<usize> for KukiShinobuDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl KukiShinobuDamageEnum {
    pub fn is_hp_ratio(&self) -> bool {
        use KukiShinobuDamageEnum::*;
        match *self {
            Q1 | Q2 | Q3 => true,
            _ => false
        }
    }

    pub fn get_element(&self) -> Element {
        use KukiShinobuDamageEnum::*;
        match *self {
            E1 | E2 | Q1 | Q2 | Q3 => Element::Electro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use KukiShinobuDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | EHeal1 => SkillType::ElementalSkill,
            Q1 | Q2 | Q3 => SkillType::ElementalBurst,
        }
    }
}

pub enum KukiShinobuRoleEnum {
    Sub
}

pub struct KukiShinobu;

impl CharacterTrait for KukiShinobu {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::KukiShinobu,
        internal_name: "Shinobu",
        element: Element::Electro,
        hp: [1030, 2647, 3417, 5118, 5665, 6516, 7245, 8096, 8643, 9493, 10040, 10891, 11438, 12289],
        atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212],
        def: [63, 162, 209, 313, 346, 398, 443, 495, 528, 580, 613, 665, 699, 751],
        sub_stat: CharacterSubStatFamily::HP240,
        weapon_type: WeaponType::Sword,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·忍流飞刃斩",
            en: "Normal Attack: Shinobu's Shadowsword",
        ),
        skill_name2: locale!(
            zh_cn: "越祓雷草之轮",
            en: "Sanctifying Ring",
        ),
        skill_name3: locale!(
            zh_cn: "御咏鸣神刈山祭",
            en: "Gyoei Narukami Kariyama Rite",
        ),
        name_locale: locale!(
            zh_cn: "久岐忍",
            en: "Kuki Shinobu",
        )
    };
    type SkillType = KukiShinobuSkillType;
    const SKILL: Self::SkillType = KUKI_SHINOBU_SKILL;
    type DamageEnumType = KukiShinobuDamageEnum;
    type RoleEnum = KukiShinobuRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Charged11 as usize, text: charged_dmg!(1) },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Charged12 as usize, text: charged_dmg!(2) },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Charged1 as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::EHeal1 as usize, text: locale!(zh_cn: "越祓草轮治疗量", en: "Grass Ring of Sanctification Healing") },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::E2 as usize, text: locale!(zh_cn: "越祓草轮伤害", en: "Grass Ring of Sanctification DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Q1 as usize, text: locale!(zh_cn: "单次伤害", en: "Single Instance DMG") },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Q2 as usize, text: locale!(zh_cn: "总伤害", en: "Total DMG") },
            CharacterSkillMapItem { index: KukiShinobuDamageEnum::Q3 as usize, text: locale!(zh_cn: "总伤害-低血量", en: "Total DMG-Low HP") },
        ]),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp_le_50",
            title: locale!(
                zh_cn: "生命值不高于50%（天赋1：治疗加成+15%）",
                en: "HP <= 50%(Talent1: Healing Bonus + 15%)",
            ),
            config: ItemConfigType::Bool { default: true },
        },
        ItemConfig {
            name: "use_c6",
            title: locale!(
                zh_cn: "启用六命",
                en: "Use C6",
            ),
            config: ItemConfigType::Bool { default: false },
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KukiShinobuDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KukiShinobuDamageEnum::*;

        let mut builder = D::new();
        if s != EHeal1 {
            let ratio = match s {
                Normal1 => KUKI_SHINOBU_SKILL.normal_dmg1[s1],
                Normal2 => KUKI_SHINOBU_SKILL.normal_dmg2[s1],
                Normal3 => KUKI_SHINOBU_SKILL.normal_dmg3[s1],
                Normal4 => KUKI_SHINOBU_SKILL.normal_dmg4[s1],
                Charged11 => KUKI_SHINOBU_SKILL.charged_dmg11[s1],
                Charged12 => KUKI_SHINOBU_SKILL.charged_dmg12[s1],
                Charged1 => KUKI_SHINOBU_SKILL.charged_dmg11[s1] + KUKI_SHINOBU_SKILL.charged_dmg12[s1],
                Plunging1 => KUKI_SHINOBU_SKILL.plunging_dmg1[s1],
                Plunging2 => KUKI_SHINOBU_SKILL.plunging_dmg2[s1],
                Plunging3 => KUKI_SHINOBU_SKILL.plunging_dmg3[s1],
                E1 => KUKI_SHINOBU_SKILL.elemental_skill_dmg1[s2],
                E2 => KUKI_SHINOBU_SKILL.elemental_skill_dmg2[s2],
                Q1 => KUKI_SHINOBU_SKILL.elemental_burst_dmg1[s3],
                Q2 => KUKI_SHINOBU_SKILL.elemental_burst_dmg2[s3],
                Q3 => KUKI_SHINOBU_SKILL.elemental_burst_dmg3[s3],
                _ => unreachable!()
            };

            if s.is_hp_ratio() {
                builder.add_hp_ratio("技能倍率", ratio);
            } else {
                builder.add_atk_ratio("技能倍率", ratio);
            }

            if s == E2 {
                // let em = context.attribute.get_value(AttributeName::ElementalMastery);
                let em = context.attribute.get_em_all();
                builder.add_extra_damage("天赋：安心之所", em * 0.25);
            }

            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        } else {
            let ratio = KUKI_SHINOBU_SKILL.elemental_skill_heal1[s2];
            let fixed = KUKI_SHINOBU_SKILL.elemental_skill_heal1_fixed[s2];

            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            if s == EHeal1 {
                // let em = context.attribute.get_value(AttributeName::ElementalMastery);
                let em = context.attribute.get_em_all();
                builder.add_extra_damage("天赋：安心之所", em * 0.75);
            }

            builder.heal(&context.attribute)
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (hp_le_50, use_c6) = match *config {
            CharacterConfig::KukiShinobu { hp_le_50, use_c6 } => (hp_le_50, use_c6),
            _ => (true, false)
        };

        Some(Box::new(KukiShinobuEffect {
            hp_le_50,
            has_talent1: common_data.has_talent1,
            use_c6,
            has_c6: common_data.constellation == 6,
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}