use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterStaticData};
use crate::character::no_effect::NoEffect;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterConstant, CharacterDamage, CharacterEffect, CharacterTrait};
use crate::common::{Element, SkillType, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;

pub struct AratakiIttoSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub charged_dmg3: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],

    pub elemental_burst_atk_bonus: [f64; 15],
}

pub const ARATAKI_ITTO_SKILL: AratakiIttoSkillType = AratakiIttoSkillType {
    normal_dmg1: [0.7923, 0.8568, 0.9213, 1.0134, 1.0779, 1.1516, 1.253, 1.3543, 1.4557, 1.5662, 1.6929, 1.8419, 1.9908, 2.1398, 2.3023],
    normal_dmg2: [0.7637, 0.8258, 0.888, 0.9768, 1.039, 1.11, 1.2077, 1.3054, 1.403, 1.5096, 1.6317, 1.7753, 1.9189, 2.0625, 2.2191],
    normal_dmg3: [0.9164, 0.991, 1.0656, 1.1722, 1.2468, 1.332, 1.4492, 1.5664, 1.6836, 1.8115, 1.958, 2.1303, 2.3027, 2.475, 2.6629],
    normal_dmg4: [1.1722, 1.2677, 1.3631, 1.4994, 1.5948, 1.7039, 1.8538, 2.0037, 2.1537, 2.3172, 2.5047, 2.7251, 2.9455, 3.1659, 3.4063],
    charged_dmg1: [0.9116, 0.9858, 1.06, 1.166, 1.2402, 1.325, 1.4416, 1.5582, 1.6748, 1.802, 1.9478, 2.1192, 2.2906, 2.462, 2.6489],
    charged_dmg2: [1.9092, 2.0646, 2.22, 2.442, 2.5974, 2.775, 3.0192, 3.2634, 3.5076, 3.774, 4.0793, 4.4382, 4.7972, 5.1562, 5.5478],
    charged_dmg3: [0.9047, 0.9784, 1.052, 1.1572, 1.2308, 1.315, 1.4307, 1.5464, 1.6622, 1.7884, 1.9331, 2.1032, 2.2733, 2.4434, 2.6289],
    plunging_dmg1: [0.8183, 0.8849, 0.9516, 1.0467, 1.1133, 1.1894, 1.2941, 1.3988, 1.5035, 1.6176, 1.7318, 1.846, 1.9602, 2.0744, 2.1886],
    plunging_dmg2: [1.6363, 1.7695, 1.9027, 2.093, 2.2262, 2.3784, 2.5877, 2.797, 3.0063, 3.2346, 3.4629, 3.6912, 3.9196, 4.1479, 4.3762],
    plunging_dmg3: [2.0439, 2.2102, 2.3766, 2.6142, 2.7806, 2.9707, 3.2321, 3.4936, 3.755, 4.0402, 4.3254, 4.6106, 4.8957, 5.1809, 5.4661],
    elemental_skill_dmg1: [3.072, 3.3024, 3.5328, 3.84, 4.0704, 4.3008, 4.608, 4.9152, 5.2224, 5.5296, 5.8368, 6.144, 6.528, 6.912, 7.296],
    elemental_burst_atk_bonus: [0.576, 0.6192, 0.6624, 0.72, 0.7632, 0.8064, 0.864, 0.9216, 0.9792, 1.0368, 1.0944, 1.152, 1.224, 1.296, 1.368],
};

const ARATAKI_ITTO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element: Element::Geo,
    hp: [1001, 2579, 3455, 5170, 5779, 6649, 7462, 8341, 8951, 9838, 10448, 11345, 11954, 12858],
    atk: [18, 46, 61, 91, 102, 117, 132, 147, 158, 174, 185, 200, 211, 227],
    def: [75, 194, 258, 386, 431, 496, 557, 622, 668, 734, 779, 846, 892, 959],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Claymore,
    star: 5,
};

pub struct AratakiItto;

impl CharacterConstant for AratakiItto {
    const STATIC_DATA: CharacterStaticData = ARATAKI_ITTO_STATIC_DATA;
    type SkillType = AratakiIttoSkillType;
    const SKILL: Self::SkillType = ARATAKI_ITTO_SKILL;
    type DamageEnumType = AratakiIttoDamageEnum;
}

impl<A: Attribute> CharacterEffect<A> for AratakiItto {
    type EffectType = NoEffect;
    fn new_effect(common_data: &CharacterCommonData, config: &CharacterConfig) -> Self::EffectType {
        NoEffect
    }
}

#[derive(Copy, Clone)]
#[derive(FromPrimitive)]
pub enum AratakiIttoDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    KesagiriCombo,
    KesagiriFinal,
    Saichimonji,
    Plunging1,
    Plunging2,
    Plunging3,
    E1
}

impl Into<usize> for AratakiIttoDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl AratakiIttoDamageEnum {
    pub fn get_element(&self, after_q: bool) -> Element {
        use AratakiIttoDamageEnum::*;
        if after_q {
            Element::Geo
        } else {
            match *self {
                E1 => Element::Geo,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AratakiIttoDamageEnum::*;
        match *self {
            KesagiriCombo | KesagiriFinal | Saichimonji => SkillType::ChargedAttack,
            Plunging1 | Plunging2 | Plunging3 => SkillType::PlungingAttack,
            E1 => SkillType::ElementalSkill,
            _ => SkillType::NormalAttack
        }
    }
}

impl<D: DamageBuilder> CharacterDamage<D> for AratakiItto {
    fn damage_internal(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig) -> D::Result {
        let after_q = match *config {
            CharacterSkillConfig::AratakiItto { after_q } => after_q,
            _ => false
        };
        let s: AratakiIttoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        use AratakiIttoDamageEnum::*;
        let s1 = context.character_common_data.skill1;
        let s2 = context.character_common_data.skill2;
        let s3 = context.character_common_data.skill3;
        let ratio = match s {
            Normal1 => ARATAKI_ITTO_SKILL.normal_dmg1[s1],
            Normal2 => ARATAKI_ITTO_SKILL.normal_dmg2[s1],
            Normal3 => ARATAKI_ITTO_SKILL.normal_dmg3[s1],
            Normal4 => ARATAKI_ITTO_SKILL.normal_dmg4[s1],
            KesagiriCombo => ARATAKI_ITTO_SKILL.charged_dmg1[s1],
            KesagiriFinal => ARATAKI_ITTO_SKILL.charged_dmg2[s1],
            Saichimonji => ARATAKI_ITTO_SKILL.charged_dmg3[s1],
            Plunging1 => ARATAKI_ITTO_SKILL.plunging_dmg1[s1],
            Plunging2 => ARATAKI_ITTO_SKILL.plunging_dmg2[s1],
            Plunging3 => ARATAKI_ITTO_SKILL.plunging_dmg3[s1],
            E1 => ARATAKI_ITTO_SKILL.elemental_skill_dmg1[s2]
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        if after_q {
            let def = context.attribute.get_value(AttributeName::DEF);
            let atk_bonus = ARATAKI_ITTO_SKILL.elemental_burst_atk_bonus[s3] * def;
            builder.add_extra_atk("大招加成", atk_bonus);
        }

        builder.build(
            &context.attribute,
            &context.enemy,
            s.get_element(after_q),
            s.get_skill_type(),
            false,
            context.character_common_data.level
        )
    }
}

impl<A: Attribute, D: DamageBuilder> CharacterTrait<A, D> for AratakiItto {
}
