use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
// use strum_macros::{EnumCount as EnumCountMacro, EnumString};
// use num_derive::FromPrimitive;
// use strum::EnumCount;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::macros::{skill_type, damage_enum};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};

// pub struct TighnariSkillType {
//     pub normal_dmg1: [f64; 15],
//     pub normal_dmg2: [f64; 15],
//     pub normal_dmg3: [f64; 15],
//     pub normal_dmg4: [f64; 15],
//     pub charged_dmg1: [f64; 15],
//     pub charged_dmg2: [f64; 15],
//     pub charged_dmg3: [f64; 15],
//     pub charged_dmg4: [f64; 15],
//     pub plunging_dmg1: [f64; 15],
//     pub plunging_dmg2: [f64; 15],
//     pub plunging_dmg3: [f64; 15],
//
//     pub e1: [f64; 15],
//
//     pub q1: [f64; 15],
//     pub q2: [f64; 15],
// }


//
skill_type!(TighnariSkillType normal_dmg1 normal_dmg2 normal_dmg3 normal_dmg4 charged_dmg1 charged_dmg2 charged_dmg3 charged_dmg4 plunging_dmg1 plunging_dmg2 plunging_dmg3 e1 q1 q2);

pub const TIGHNARI_SKILL: TighnariSkillType = TighnariSkillType {
    normal_dmg1: [0.4463, 0.4827, 0.519, 0.5709, 0.6072, 0.6488, 0.7058, 0.7629, 0.82, 0.8823, 0.9446, 1.0069, 1.0691, 1.1314, 1.1937],
    normal_dmg2: [0.4197, 0.4538, 0.488, 0.5368, 0.571, 0.61, 0.6637, 0.7174, 0.771, 0.8296, 0.8882, 0.9467, 1.0053, 1.0638, 1.1224],
    normal_dmg3: [0.2645, 0.286, 0.3075, 0.3383, 0.3598, 0.3844, 0.4182, 0.452, 0.4859, 0.5228, 0.5597, 0.5965, 0.6334, 0.6704, 0.7072],
    normal_dmg4: [0.6863, 0.7421, 0.798, 0.8778, 0.9337, 0.9975, 1.0853, 1.1731, 1.2608, 1.3566, 1.4524, 1.5481, 1.6439, 1.7396, 1.8354],
    charged_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    charged_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    charged_dmg3: [0.872, 0.9374, 1.0028, 1.09, 1.1554, 1.2208, 1.308, 1.3952, 1.4824, 1.5696, 1.6568, 1.744, 1.853, 1.962, 2.071],
    charged_dmg4: [0.386, 0.415, 0.4439, 0.4825, 0.5114, 0.5404, 0.579, 0.6176, 0.6562, 0.6948, 0.7334, 0.772, 0.8202, 0.8685, 0.9168],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    e1: [1.496, 1.6082, 1.7204, 1.87, 1.9822, 2.0944, 2.244, 2.3936, 2.5432, 2.6928, 2.8424, 2.992, 3.179, 3.366, 3.553],
    q1: [0.5562, 0.5979, 0.6396, 0.6952, 0.737, 0.7787, 0.8343, 0.8899, 0.9455, 1.0012, 1.0568, 1.1124, 1.1819, 1.2514, 1.321],
    q2: [0.6798, 0.7308, 0.7818, 0.8497, 0.9007, 0.9517, 1.0197, 1.0877, 1.1557, 1.2236, 1.2916, 1.3596, 1.4446, 1.5295, 1.6145],
};

pub struct TighnariEffect {
    pub c1: bool,
    pub has_talent1: bool,
    pub talent1_ratio: f64,
    pub c2: bool,
    pub c2_ratio: f64,
}

impl<A: Attribute> ChangeAttribute<A> for TighnariEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.c1 {
            attribute.set_value_by(AttributeName::CriticalChargedAttack, "提纳里1命", 0.15);
        }

        if self.has_talent1 {
            attribute.set_value_by(AttributeName::ElementalMastery, "提纳里天赋", 50.0 * self.talent1_ratio);
        }

        if self.c2 {
            attribute.set_value_by(AttributeName::BonusDendro, "提纳里2命", 0.2 * self.c2_ratio);
        }
    }
}



damage_enum!(
    TighnariDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged1
    Charged2
    Charged3
    Charged4
    ChargedC6
    Plunging1
    Plunging2
    Plunging3
    E1
    Q1
    Q2
);

impl TighnariDamageEnum {
    pub fn get_element(&self) -> Element {
        use TighnariDamageEnum::*;
        match *self {
            Charged2 | Charged3 | Charged4 | ChargedC6 | E1 | Q1 | Q2 => Element::Dendro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use TighnariDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 | Charged3 | Charged4 | ChargedC6 => SkillType::ChargedAttack,
            E1 => SkillType::ElementalSkill,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

pub struct Tighnari;

impl CharacterTrait for Tighnari {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Tighnari,
        internal_name: "Tighnari",
        element: Element::Dendro,
        hp: [845, 2191, 2915, 4362, 4877, 5611, 6297, 7038, 7553, 8301, 8816, 9573, 10087, 10850],
        atk: [21, 54, 72, 108, 120, 139, 155, 174, 186, 205, 218, 236, 249, 268],
        def: [49, 127, 169, 253, 283, 326, 366, 409, 439, 482, 512, 556, 586, 630],
        sub_stat: CharacterSubStatFamily::Bonus288(StatName::DendroBonus),
        weapon_type: WeaponType::Bow,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·藏蕴破障",
            en: "Normal Attack: Khanda Barrier-Buster",
        ),
        skill_name2: locale!(
            zh_cn: "识果种雷",
            en: "Vijnana-Phala Mine",
        ),
        skill_name3: locale!(
            zh_cn: "造生缠藤箭",
            en: "Fashioner’s Tanglevine Shaft",
        ),
        name_locale: locale!(
            zh_cn: "提纳里",
            en: "Tighnari",
        )
    };
    type SkillType = TighnariSkillType;
    const SKILL: Self::SkillType = TIGHNARI_SKILL;
    type DamageEnumType = TighnariDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: TighnariDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: TighnariDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: TighnariDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: TighnariDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: TighnariDamageEnum::Charged1 as usize, text: charged_dmg!("shoot1") },
            CharacterSkillMapItem { index: TighnariDamageEnum::Charged2 as usize, text: locale!(zh_cn: "一段蓄力瞄准射击", en: "Level 1 Aimed Shot") },
            CharacterSkillMapItem { index: TighnariDamageEnum::Charged3 as usize, text: locale!(zh_cn: "花筥箭伤害", en: "Wreath Arrow DMG") },
            CharacterSkillMapItem { index: TighnariDamageEnum::Charged4 as usize, text: locale!(zh_cn: "藏蕴花矢伤害", en: "Clusterbloom Arrow DMG") },
            CharacterSkillMapItem { index: TighnariDamageEnum::ChargedC6 as usize, text: locale!(zh_cn: "六命藏蕴花矢伤害", en: "C6 Clusterbloom Arrow DMG") },
            CharacterSkillMapItem { index: TighnariDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: TighnariDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: TighnariDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: TighnariDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") }
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: TighnariDamageEnum::Q1 as usize, text: locale!(zh_cn: "缠藤箭伤害", en: "Tanglevine Shaft DMG") },
            CharacterSkillMapItem { index: TighnariDamageEnum::Q2 as usize, text: locale!(zh_cn: "次级缠藤箭伤害", en: "Secondary Tanglevine Shaft DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_ratio",
            title: locale!(
                zh_cn: "天赋「眼识殊明」应用比例",
                en: "Talent「Keen Sight」Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "c2_ratio",
            title: locale!(
                zh_cn: "命座2「由茎干剖析来缘」应用比例",
                en: "C2「Origins Known From the Stem」Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: TighnariDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use TighnariDamageEnum::*;
        let ratio = match s {
            Normal1 => TIGHNARI_SKILL.normal_dmg1[s1],
            Normal2 => TIGHNARI_SKILL.normal_dmg2[s1],
            Normal3 => TIGHNARI_SKILL.normal_dmg3[s1],
            Normal4 => TIGHNARI_SKILL.normal_dmg4[s1],
            Charged1 => TIGHNARI_SKILL.charged_dmg1[s1],
            Charged2 => TIGHNARI_SKILL.charged_dmg2[s1],
            Charged3 => TIGHNARI_SKILL.charged_dmg3[s1],
            Charged4 => TIGHNARI_SKILL.charged_dmg4[s1],
            ChargedC6 => 1.5,
            Plunging1 => TIGHNARI_SKILL.plunging_dmg1[s1],
            Plunging2 => TIGHNARI_SKILL.plunging_dmg2[s1],
            Plunging3 => TIGHNARI_SKILL.plunging_dmg3[s1],
            E1 => TIGHNARI_SKILL.e1[s2],
            Q1 => TIGHNARI_SKILL.q1[s3],
            Q2 => TIGHNARI_SKILL.q2[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let skill_type = s.get_skill_type();
        if context.character_common_data.has_talent2 && (skill_type == SkillType::ChargedAttack || skill_type == SkillType::ElementalBurst) {
            // let em = context.attribute.get_value(AttributeName::ElementalMastery);
            let em = context.attribute.get_em_all();
            let bonus = 0.6_f64.min(0.0006 * em);
            builder.add_extra_bonus("提纳里天赋「诸叶辨通」", bonus);
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

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (talent1_ratio, c2_ratio) = match *config {
            CharacterConfig::Tighnari { talent1_ratio, c2_ratio } => (talent1_ratio, c2_ratio),
            _ => (0.0, 0.0)
        };

        Some(Box::new(TighnariEffect {
            c1: common_data.constellation >= 1,
            has_talent1: common_data.has_talent1,
            talent1_ratio,
            c2: common_data.constellation >= 2,
            c2_ratio
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}