use crate::{attribute::Attribute, character::macros::damage_enum, common::ChangeAttribute};
use crate::attribute::AttributeName;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait};
use crate::common::{Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, plunging_dmg};
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::traits::{CharacterSkillMapItem};

pub struct KavehSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_bonus: [f64; 15],
}

pub const KAVEH_SKILL: KavehSkillType = KavehSkillType {
    normal_dmg1: [0.7619, 0.8239, 0.8859, 0.9745, 1.0365, 1.1073, 1.2048, 1.3022, 1.3997, 1.506, 1.6123, 1.7186, 1.8249, 1.9312, 2.0375],
    normal_dmg2: [0.6964, 0.7531, 0.8098, 0.8907, 0.9474, 1.0122, 1.1013, 1.1903, 1.2794, 1.3766, 1.4737, 1.5709, 1.6681, 1.7653, 1.8624],
    normal_dmg3: [0.8426, 0.9112, 0.9798, 1.0778, 1.1463, 1.2247, 1.3325, 1.4403, 1.5481, 1.6656, 1.7832, 1.9008, 2.0183, 2.1359, 2.2535],
    normal_dmg4: [1.0269, 1.1105, 1.194, 1.3135, 1.397, 1.4926, 1.6239, 1.7553, 1.8866, 2.0299, 2.1732, 2.3165, 2.4597, 2.603, 2.7463],
    charged_dmg1: [0.5315, 0.5747, 0.618, 0.6798, 0.7231, 0.7725, 0.8405, 0.9085, 0.9764, 1.0506, 1.1248, 1.1989, 1.2731, 1.3472, 1.4214],
    charged_dmg2: [0.9615, 1.0397, 1.118, 1.2298, 1.3081, 1.3975, 1.5205, 1.6435, 1.7664, 1.9006, 2.0348, 2.1689, 2.3031, 2.4372, 2.5714],
    plunging_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    plunging_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    plunging_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],
    e_dmg1: [2.04, 2.193, 2.346, 2.55, 2.703, 2.856, 3.06, 3.264, 3.468, 3.672, 3.876, 4.08, 4.335, 4.59, 4.845],
    q_dmg1: [1.6, 1.72, 1.84, 2., 2.12, 2.24, 2.4, 2.56, 2.72, 2.88, 3.04, 3.2, 3.4, 3.6, 3.8],
    q_bonus: [0.2749, 0.2955, 0.3161, 0.3436, 0.3642, 0.3848, 0.4123, 0.4398, 0.4673, 0.4948, 0.5223, 0.5498, 0.5841, 0.6185, 0.6528],
};

damage_enum!(
    KavehDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged1
    Charged2
    Plunging1
    Plunging2
    Plunging3
    E1
    Q1
);

impl KavehDamageEnum {
    pub fn get_element(&self, after_q: bool) -> Element {
        if after_q {
            Element::Dendro
        } else {
            use KavehDamageEnum::*;
            match *self {
                E1 | Q1 => Element::Dendro,
                _ => Element::Physical
            }
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use KavehDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
        }
    }
}

pub struct KavehEffect {
    pub has_talent2: bool,
    pub talent2_stack: f64,
    pub constellation: usize,
    pub c2_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for KavehEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent2 {
            let em = 25.0 * self.talent2_stack;
            attribute.set_value_by(AttributeName::ElementalMastery, "卡维天赋「工艺家的奇想」", em);
        }
        if self.constellation >= 2 {
            let speed = 0.25 * self.c2_rate;
            attribute.set_value_by(AttributeName::SpeedNormalAttack, "卡维命座2「御驿的径迹」", speed);
        }
    }
}

pub struct Kaveh;

impl CharacterTrait for Kaveh {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Kaveh,
        internal_name: "Kaveh",
        name_locale: locale!(zh_cn: "卡维", en: "Kaveh"),
        element: Element::Dendro,
        hp: [1003, 2577, 3326, 4982, 5514, 6343, 7052, 7881, 8413, 9241, 9773, 10602, 11134, 11962],
        atk: [20, 50, 65, 97, 108, 124, 138, 154, 164, 180, 191, 207, 217, 234],
        def: [63, 162, 209, 313, 346, 398, 443, 495, 528, 580, 613, 665, 699, 751],
        sub_stat: CharacterSubStatFamily::ElementalMastery96,
        weapon_type: WeaponType::Claymore,
        star: 4,
        skill_name1: locale!(zh_cn: "普通攻击·旋规设矩", en: "Normal Attack: Schematic Setup"),
        skill_name2: locale!(zh_cn: "画则巧施", en: "Artistic Ingenuity"),
        skill_name3: locale!(zh_cn: "繁绘隅穹", en: "Painted Dome"),
    };
    type SkillType = KavehSkillType;
    const SKILL: Self::SkillType = KAVEH_SKILL;
    type DamageEnumType = KavehDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            KavehDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged1 charged_dmg!("loop1")
            Charged2 charged_dmg!("loop2")
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            KavehDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            KavehDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent2_stack",
            title: locale!(zh_cn: "天赋「工艺家的奇想」层数", en: "Talent 「A Craftsman's Curious Conceptions」 Stack"),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 },
        },
        ItemConfig {
            name: "c2_rate",
            title: locale!(zh_cn: "命座2比例", en: "C2 Rate"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_q",
            title: locale!(zh_cn: "「繁绘隅穹」状态", en: "Painted Dome"),
            config: ItemConfigType::Bool { default: false },
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KavehDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use KavehDamageEnum::*;
        let mut builder = D::new();

        let ratio = match s {
            Normal1 => KAVEH_SKILL.normal_dmg1[s1],
            Normal2 => KAVEH_SKILL.normal_dmg2[s1],
            Normal3 => KAVEH_SKILL.normal_dmg3[s1],
            Normal4 => KAVEH_SKILL.normal_dmg4[s1],
            Charged1 => KAVEH_SKILL.charged_dmg2[s1],
            Charged2 => KAVEH_SKILL.charged_dmg2[s1],
            Plunging1 => KAVEH_SKILL.plunging_dmg1[s1],
            Plunging2 => KAVEH_SKILL.plunging_dmg2[s1],
            Plunging3 => KAVEH_SKILL.plunging_dmg3[s1],
            E1 => KAVEH_SKILL.e_dmg1[s2],
            Q1 => KAVEH_SKILL.q_dmg1[s3],
        };

        builder.add_atk_ratio("技能倍率", ratio);

        let after_q = match *config {
            CharacterSkillConfig::Kaveh { after_q } => after_q,
            _ => false,
        };

        let element = s.get_element(after_q);
        let skill_type = s.get_skill_type();

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
        let (stack, c2_rate) = match *config {
            CharacterConfig::Kaveh { talent2_stack, c2_rate } => (talent2_stack, c2_rate),
            _ => (0.0, 0.0)
        };

        Some(Box::new(KavehEffect {
            has_talent2: common_data.has_talent2,
            talent2_stack: stack,
            constellation: common_data.constellation as usize,
            c2_rate,
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
