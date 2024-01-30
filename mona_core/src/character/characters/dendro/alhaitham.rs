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
use crate::character::traits::{CharacterSkillMapItem, CharacterSkillMap};
use crate::character::macros::{skill_type, skill_map, damage_ratio};
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};

pub struct AlhaithamSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3x: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1x: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1_atk: [f64; 15],
    pub e_dmg1_em: [f64; 15],
    pub e_dmg2_atk: [f64; 15],
    pub e_dmg2_em: [f64; 15],

    pub q_dmg1_atk: [f64; 15],
    pub q_dmg1_em: [f64; 15],
}

pub const ALHAITHAM_SKILL: AlhaithamSkillType = AlhaithamSkillType {
    normal_dmg1: [0.4953, 0.5356, 0.5759, 0.6335, 0.6738, 0.7199, 0.7832, 0.8465, 0.9099, 0.979, 1.0481, 1.1172, 1.1863, 1.2554, 1.3245],
    normal_dmg2: [0.5075, 0.5488, 0.5901, 0.6491, 0.6904, 0.7376, 0.8026, 0.8675, 0.9324, 1.0032, 1.074, 1.1448, 1.2156, 1.2864, 1.3573],
    normal_dmg3x: [0.3418, 0.3696, 0.3974, 0.4372, 0.465, 0.4968, 0.5405, 0.5842, 0.6279, 0.6756, 0.7233, 0.771, 0.8187, 0.8664, 0.9141],
    normal_dmg4: [0.6677, 0.722, 0.7764, 0.854, 0.9084, 0.9705, 1.0559, 1.1413, 1.2267, 1.3198, 1.413, 1.5062, 1.5993, 1.6925, 1.7857],
    normal_dmg5: [0.8385, 0.9068, 0.975, 1.0725, 1.1408, 1.2188, 1.326, 1.4333, 1.5405, 1.6575, 1.7745, 1.8915, 2.0085, 2.1255, 2.2425],
    charged_dmg1x: [0.5526, 0.5975, 0.6425, 0.7067, 0.7517, 0.8031, 0.8738, 0.9445, 1.0151, 1.0922, 1.1694, 1.2464, 1.3235, 1.4007, 1.4777],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1_atk: [1.936, 2.0812, 2.2264, 2.42, 2.5652, 2.7104, 2.904, 3.0976, 3.2912, 3.4848, 3.6784, 3.872, 4.114, 4.356, 4.598],
    e_dmg1_em: [1.5488, 1.665, 1.7811, 1.936, 2.0522, 2.1683, 2.3232, 2.4781, 2.633, 2.7878, 2.9427, 3.0976, 3.2912, 3.4848, 3.6784],
    e_dmg2_atk: [0.672, 0.7224, 0.7728, 0.84, 0.8904, 0.9408, 1.008, 1.0752, 1.1424, 1.2096, 1.2768, 1.344, 1.428, 1.512, 1.596],
    e_dmg2_em: [1.344, 1.4448, 1.5456, 1.68, 1.7808, 1.8816, 2.016, 2.1504, 2.2848, 2.4192, 2.5536, 2.688, 2.856, 3.024, 3.192],
    q_dmg1_atk: [1.216, 1.3072, 1.3984, 1.52, 1.6112, 1.7024, 1.824, 1.9456, 2.0672, 2.1888, 2.3104, 2.432, 2.584, 2.736, 2.888],
    q_dmg1_em: [0.9728, 1.0458, 1.1187, 1.216, 1.289, 1.3619, 1.4592, 1.5565, 1.6538, 1.751, 1.8483, 1.9456, 2.0672, 2.1888, 2.3104],
};

damage_enum!(
    AlhaithamDamageEnum
    Normal1
    Normal2
    Normal31
    Normal32
    Normal4
    Normal5
    Charged11
    Charged12
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    E4
    Q1
);

impl AlhaithamDamageEnum {
    pub fn get_element(&self, under_e: bool) -> Element {
        if under_e {
            Element::Dendro
        } else {
            use AlhaithamDamageEnum::*;
            match *self {
                E1 | E2 | E3 | E4 | Q1 => Element::Dendro,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AlhaithamDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 | Normal5 => SkillType::NormalAttack,
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

pub struct AlhaithamEffect {
    pub c: usize,
    pub c2_stack: f64,
    pub c4_stack: f64,
    pub c6_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for AlhaithamEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.c >= 2 {
            let em = self.c2_stack * 50.0;
            attribute.set_value_by(AttributeName::ElementalMastery, "二命「辩章」", em);
        }
        if self.c >= 4 {
            let value = self.c4_stack * 0.1;
            attribute.set_value_by(AttributeName::BonusDendro, "四命「义贯」", value);
        }
        if self.c >= 6 {
            let crit = self.c6_rate * 0.1;
            let cd = self.c6_rate * 0.7;
            attribute.set_value_by(AttributeName::CriticalBase, "六命「正理」", crit);
            attribute.set_value_by(AttributeName::CriticalDamageBase, "六命「正理」", cd);
        }
    }
}

pub struct Alhaitham;

impl CharacterTrait for Alhaitham {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Alhaitham,
        internal_name: "Alhatham",
        element: Element::Dendro,
        hp: [1039, 2695, 3586, 5366, 5999, 6902, 7747, 8659, 9292, 10213, 10846, 11777, 12410, 13348],
        atk: [24, 63, 84, 126, 141, 162, 182, 203, 218, 240, 255, 276, 291, 313],
        def: [61, 158, 210, 314, 351, 404, 454, 507, 544, 598, 635, 690, 727, 782],
        sub_stat: CharacterSubStatFamily::Bonus288(StatName::DendroBonus),
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击·溯因反绎法",
            en: "Normal Attack: Abductive Reasoning",
        ),
        skill_name2: locale!(
            zh_cn: "共相·理式摹写",
            en: "Universality: An Elaboration on Form",
        ),
        skill_name3: locale!(
            zh_cn: "殊境·显象缚结",
            en: "Particular Field: Fetters of Phenomena",
        ),
        name_locale: locale!(
            zh_cn: "艾尔海森",
            en: "Alhaitham",
        )
    };
    type SkillType = AlhaithamSkillType;
    const SKILL: Self::SkillType = ALHAITHAM_SKILL;
    type DamageEnumType = AlhaithamDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            AlhaithamDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal31 hit_n_dmg!(3, 1)
            Normal32 hit_n_dmg!(3, 2)
            Normal4 hit_n_dmg!(4)
            Normal5 hit_n_dmg!(5)
            Charged11 charged_dmg!(1)
            Charged12 charged_dmg!(2)
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            AlhaithamDamageEnum
            E1 locale!(zh_cn: "突进攻击伤害", en: "Rush Attack DMG")
            E2 locale!(zh_cn: "1枚光幕攻击伤害", en: "1-Mirror Projection Attack DMG")
            E3 locale!(zh_cn: "2枚光幕攻击伤害", en: "2-Mirror Projection Attack DMG")
            E4 locale!(zh_cn: "3枚光幕攻击伤害", en: "3-Mirror Projection Attack DMG")
        ),
        skill3: skill_map!(
            AlhaithamDamageEnum
            Q1 locale!(zh_cn: "单次伤害", en: "Single-Instance DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c2_stack",
            title: locale!(
                zh_cn: "命座2「辩章」层数",
                en: "C2「Debate」Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 },
        },
        ItemConfig {
            name: "c4_stack",
            title: locale!(
                zh_cn: "命座4「义贯」层数",
                en: "C4「Elucidation」Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 },
        },
        ItemConfig {
            name: "c6_rate",
            title: locale!(
                zh_cn: "命座6「正理」比例",
                en: "C6「Structuration」Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "under_e",
            title: locale!(
                zh_cn: "琢光镜",
                en: "Chisel-Light Mirror",
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn damage_internal<D: crate::damage::damage_builder::DamageBuilder>(context: &crate::damage::DamageContext<'_, D::AttributeType>, s: usize, config: &crate::character::skill_config::CharacterSkillConfig, fumo: Option<crate::common::Element>) -> D::Result {
        let s: AlhaithamDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use AlhaithamDamageEnum::*;
        let mut builder = D::new();

        let ratio_atk = match s {
            Normal1 => ALHAITHAM_SKILL.normal_dmg1[s1],
            Normal2 => ALHAITHAM_SKILL.normal_dmg2[s1],
            Normal31 | Normal32 => ALHAITHAM_SKILL.normal_dmg3x[s1],
            Normal4 => ALHAITHAM_SKILL.normal_dmg4[s1],
            Normal5 => ALHAITHAM_SKILL.normal_dmg5[s1],
            Charged11 | Charged12 => ALHAITHAM_SKILL.charged_dmg1x[s1],
            Plunging1 => ALHAITHAM_SKILL.plunging_dmg1[s1],
            Plunging2 => ALHAITHAM_SKILL.plunging_dmg2[s1],
            Plunging3 => ALHAITHAM_SKILL.plunging_dmg3[s1],
            E1 => ALHAITHAM_SKILL.e_dmg1_atk[s2],
            E2 => ALHAITHAM_SKILL.e_dmg2_atk[s2],
            E3 => ALHAITHAM_SKILL.e_dmg2_atk[s2] * 2.0,
            E4 => ALHAITHAM_SKILL.e_dmg2_atk[s2] * 3.0,
            Q1 => ALHAITHAM_SKILL.q_dmg1_atk[s3],
        };
        let ratio_em = match s {
            E1 => ALHAITHAM_SKILL.e_dmg1_em[s2],
            E2 => ALHAITHAM_SKILL.e_dmg2_em[s2],
            E3 => ALHAITHAM_SKILL.e_dmg2_em[s2] * 2.0,
            E4 => ALHAITHAM_SKILL.e_dmg2_em[s2] * 3.0,
            Q1 => ALHAITHAM_SKILL.q_dmg1_em[s3],
            _ => 0.0
        };

        builder.add_atk_ratio("技能倍率", ratio_atk);
        if ratio_em > 0.0 {
            builder.add_em_ratio("技能倍率", ratio_em);
        }

        // talent 2
        if context.character_common_data.has_talent2 && (s == E2 || s == E3 || s == E4 || s == Q1) {
            let em = context.attribute.get_em_all();
            let bonus = 1.0_f64.min(em * 0.001);
            builder.add_extra_bonus("天赋2「谜林道破」", bonus);
        }

        let under_e = match *config {
            CharacterSkillConfig::Alhaitham { under_e } => under_e,
            _ => false
        };

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(under_e),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(common_data: &crate::character::character_common_data::CharacterCommonData, config: &crate::character::CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (c2_stack, c4_stack, c6_rate) = match *config {
            CharacterConfig::Alhaitham { c2_stack, c4_stack, c6_rate } => (c2_stack, c4_stack, c6_rate),
            _ => (0.0, 0.0, 0.0)
        };

        Some(Box::new(AlhaithamEffect {
            c2_stack,
            c4_stack,
            c6_rate,
            c: common_data.constellation as usize,
        }))
    }

    fn get_target_function_by_role(
            role_index: usize,
            team: &crate::team::TeamQuantization,
            c: &crate::character::character_common_data::CharacterCommonData,
            w: &crate::weapon::weapon_common_data::WeaponCommonData
        ) -> Box<dyn crate::target_functions::TargetFunction> {
        unimplemented!()
    }
}