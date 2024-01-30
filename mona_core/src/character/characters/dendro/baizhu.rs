use crate::character::{CharacterName, CharacterConfig};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{StatName, WeaponType, Element, SkillType};
use crate::{character::macros::damage_enum, common::ChangeAttribute, attribute::Attribute};
use crate::attribute::AttributeName;
use crate::character::character_static_data::CharacterStaticData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::attribute::{AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::traits::{CharacterSkillMapItem, CharacterSkillMap};
use crate::character::macros::{skill_type, skill_map, damage_ratio};
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct BaizhuSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3x: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_heal1: [f64; 15],
    pub e_heal1_fixed: [f64; 15],

    pub q_shield1: [f64; 15],
    pub q_shield1_fixed: [f64; 15],
    pub q_heal1: [f64; 15],
    pub q_heal1_fixed: [f64; 15],
    pub q_dmg1: [f64; 15],
}

pub const BAIZHU_SKILL: BaizhuSkillType = BaizhuSkillType {
    normal_dmg1: [0.3737, 0.4017, 0.4298, 0.4671, 0.4952, 0.5232, 0.5606, 0.5979, 0.6353, 0.6727, 0.71, 0.7474, 0.7941, 0.8408, 0.8875],
    normal_dmg2: [0.3642, 0.3916, 0.4189, 0.4553, 0.4826, 0.5099, 0.5464, 0.5828, 0.6192, 0.6556, 0.6921, 0.7285, 0.774, 0.8196, 0.8651],
    normal_dmg3x: [0.2254, 0.2423, 0.2592, 0.2818, 0.2987, 0.3156, 0.3381, 0.3607, 0.3832, 0.4057, 0.4283, 0.4508, 0.479, 0.5072, 0.5354],
    normal_dmg4: [0.5414, 0.582, 0.6226, 0.6767, 0.7173, 0.7579, 0.8121, 0.8662, 0.9203, 0.9745, 1.0286, 1.0828, 1.1504, 1.2181, 1.2858],
    charged_dmg: [1.2104, 1.3012, 1.392, 1.513, 1.6038, 1.6946, 1.8156, 1.9366, 2.0577, 2.1787, 2.2998, 2.4208, 2.5721, 2.7234, 2.8747],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e_dmg1: [0.792, 0.8514, 0.9108, 0.99, 1.0494, 1.1088, 1.188, 1.2672, 1.3464, 1.4256, 1.5048, 1.584, 1.683, 1.782, 1.881],
    e_heal1: [0.08, 0.086, 0.092, 0.1, 0.106, 0.112, 0.12, 0.128, 0.136, 0.144, 0.152, 0.16, 0.17, 0.18, 0.19],
    e_heal1_fixed: [770.38, 847.42, 930.89, 1020.78, 1117.09, 1219.82, 1328.98, 1444.55, 1566.54, 1694.95, 1829.79, 1971.04, 2118.72, 2272.82, 2433.33],
    q_shield1: [0.008, 0.0086, 0.0092, 0.01, 0.0106, 0.0112, 0.012, 0.0128, 0.0136, 0.0144, 0.0152, 0.016, 0.017, 0.018, 0.019],
    q_shield1_fixed: [77.04, 84.74, 93.09, 102.08, 111.71, 121.98, 132.9, 144.45, 156.65, 169.5, 182.98, 197.1, 211.87, 227.28, 243.33],
    q_heal1: [0.052, 0.0559, 0.0598, 0.065, 0.0689, 0.0728, 0.078, 0.0832, 0.0884, 0.0936, 0.0988, 0.104, 0.1105, 0.117, 0.1235],
    q_heal1_fixed: [500.74, 550.83, 605.08, 663.51, 726.11, 792.89, 863.83, 938.96, 1018.25, 1101.72, 1189.36, 1281.18, 1377.17, 1477.33, 1581.67],
    q_dmg1: [0.9706, 1.0434, 1.1162, 1.2133, 1.2861, 1.3589, 1.456, 1.553, 1.6501, 1.7472, 1.8442, 1.9413, 2.0626, 2.1839, 2.3053],
};

damage_enum!(
    BaizhuDamageEnum
    Normal1
    Normal2
    Normal31
    Normal32
    Normal4
    Charged1
    Plunging1
    Plunging2
    Plunging3
    E1
    EHeal1
    QHeal1
    Q1
);

impl BaizhuDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use BaizhuDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 => SkillType::NormalAttack,
            Charged1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | EHeal1 => SkillType::ElementalSkill,
            Q1 | QHeal1 => SkillType::ElementalBurst,
        }
    }
}

pub struct BaizhuEffect {
    pub hp_below_50: bool,
    pub has_talent1: bool,
}

impl<A: Attribute> ChangeAttribute<A> for BaizhuEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent1 {
            if self.hp_below_50 {
                attribute.set_value_by(AttributeName::HealingBonus, "白术天赋1「五运终天」", 0.2);
            } else {
                attribute.set_value_by(AttributeName::BonusDendro, "白术天赋1「五运终天」", 0.25);
            }
        }
    }
}

pub struct Baizhu;

impl CharacterTrait for Baizhu {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Baizhu,
        internal_name: "Baizhuer",
        name_locale: locale!(zh_cn: "白术", en: "Baizhu"),
        element: Element::Dendro,
        hp: [1039, 2695, 3586, 5366, 5999, 6902, 7747, 8659, 9292, 10213, 10846, 11777, 12410, 13348],
        atk: [15, 39, 52, 77, 87, 100, 112, 125, 134, 147, 156, 170, 179, 193],
        def: [39, 101, 134, 201, 225, 258, 290, 324, 348, 382, 406, 441, 464, 500],
        sub_stat: CharacterSubStatFamily::HP288,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(zh_cn: "普通攻击·金匮针解", en: "Normal Attack: The Classics of Acupuncture"),
        skill_name2: locale!(zh_cn: "太素诊要", en: "Universal Diagnosis"),
        skill_name3: locale!(zh_cn: "愈气全形论", en: "Holistic Revivification"),
    };
    type SkillType = BaizhuSkillType;
    const SKILL: Self::SkillType = BAIZHU_SKILL;
    type DamageEnumType = BaizhuDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            BaizhuDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal31 hit_n_dmg!(3, 1)
            Normal32 hit_n_dmg!(3, 2)
            Normal4 hit_n_dmg!(4)
            Charged1 charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            BaizhuDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            EHeal1 locale!(zh_cn: "治疗量", en: "Healing")
        ),
        skill3: skill_map!(
            BaizhuDamageEnum
            Q1 locale!(zh_cn: "灵气脉技能伤害", en: "Spiritvein DMG")
            QHeal1 locale!(zh_cn: "无郤气护盾治疗量", en: "Seamless Shield Healing")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp_below_50",
            title: locale!(zh_cn: "场上角色生命值低于50%", en: "Active Character's HP < 50%"),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: BaizhuDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use BaizhuDamageEnum::*;

        let mut builder = D::new();
        if s == EHeal1 || s == QHeal1 {
            let ratio = match s {
                EHeal1 => BAIZHU_SKILL.e_heal1[s2],
                QHeal1 => BAIZHU_SKILL.q_heal1[s3],
                _ => 0.0
            };
            let fixed = match s {
                EHeal1 => BAIZHU_SKILL.e_heal1_fixed[s2],
                QHeal1 => BAIZHU_SKILL.q_heal1_fixed[s3],
                _ => 0.0
            };
            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);
            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => BAIZHU_SKILL.normal_dmg1[s1],
                Normal2 => BAIZHU_SKILL.normal_dmg2[s1],
                Normal31 => BAIZHU_SKILL.normal_dmg3x[s1],
                Normal32 => BAIZHU_SKILL.normal_dmg3x[s1],
                Normal4 => BAIZHU_SKILL.normal_dmg4[s1],
                Charged1 => BAIZHU_SKILL.charged_dmg[s1],
                Plunging1 => BAIZHU_SKILL.plunging_dmg1[s1],
                Plunging2 => BAIZHU_SKILL.plunging_dmg2[s1],
                Plunging3 => BAIZHU_SKILL.plunging_dmg3[s1],
                E1 => BAIZHU_SKILL.e_dmg1[s2],
                Q1 => BAIZHU_SKILL.q_dmg1[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);

            // c6
            if context.character_common_data.constellation == 6 && s == Q1 {
                builder.add_hp_ratio("白术命座6「真邪合离」", 0.08);
            }

            builder.damage(
                &context.attribute,
                &context.enemy,
                Element::Dendro,
                s.get_skill_type(),
                context.character_common_data.level,
                fumo
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let hp_below_50 = match *config {
            CharacterConfig::Baizhu { hp_below_50 } => hp_below_50,
            _ => false
        };
        Some(Box::new(BaizhuEffect {
            hp_below_50,
            has_talent1: common_data.has_talent1
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
