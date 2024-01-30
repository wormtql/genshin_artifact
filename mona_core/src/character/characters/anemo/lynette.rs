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

pub struct LynetteSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg31: [f64; 15],
    pub normal_dmg32: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg11: [f64; 15],
    pub charged_dmg12: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_heal: f64,
    pub e_hp_consumption: f64,

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
    pub q_dmg3: [f64; 15],
}

pub const LYNETTE_SKILL: LynetteSkillType = LynetteSkillType {
    normal_dmg1: [0.4308, 0.4659, 0.5009, 0.551, 0.5861, 0.6262, 0.6813, 0.7364, 0.7915, 0.8516, 0.9117, 0.9718, 1.032, 1.0921, 1.1522],
    normal_dmg2: [0.3761, 0.4067, 0.4374, 0.4811, 0.5117, 0.5467, 0.5948, 0.6429, 0.691, 0.7435, 0.796, 0.8485, 0.9009, 0.9534, 1.0059],
    normal_dmg31: [0.2786, 0.3013, 0.324, 0.3564, 0.3791, 0.405, 0.4406, 0.4763, 0.5119, 0.5508, 0.5897, 0.6286, 0.6674, 0.7063, 0.7452],
    normal_dmg32: [0.2159, 0.2335, 0.2511, 0.2762, 0.2938, 0.3138, 0.3415, 0.3691, 0.3967, 0.4268, 0.457, 0.4871, 0.5172, 0.5474, 0.5775],
    normal_dmg4: [0.6315, 0.6829, 0.7344, 0.8078, 0.8592, 0.9179, 0.9987, 1.0795, 1.1603, 1.2484, 1.3365, 1.4246, 1.5128, 1.6009, 1.689],
    charged_dmg11: [0.442, 0.478, 0.514, 0.5654, 0.6014, 0.6425, 0.699, 0.7556, 0.8121, 0.8738, 0.9355, 0.9972, 1.0588, 1.1205, 1.1822],
    charged_dmg12: [0.614, 0.664, 0.714, 0.7854, 0.8354, 0.8925, 0.971, 1.0496, 1.1281, 1.2138, 1.2995, 1.3852, 1.4708, 1.5565, 1.6422],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [2.68, 2.881, 3.082, 3.35, 3.551, 3.752, 4.02, 4.288, 4.556, 4.824, 5.092, 5.36, 5.695, 6.03, 6.365],
    e_dmg2: [0.312, 0.3354, 0.3588, 0.39, 0.4134, 0.4368, 0.468, 0.4992, 0.5304, 0.5616, 0.5928, 0.624, 0.663, 0.702, 0.741],
    e_heal: 0.25,
    e_hp_consumption: 0.06,
    q_dmg1: [0.832, 0.8944, 0.9568, 1.04, 1.1024, 1.1648, 1.248, 1.3312, 1.4144, 1.4976, 1.5808, 1.664, 1.768, 1.872, 1.976],
    q_dmg2: [0.512, 0.5504, 0.5888, 0.64, 0.6784, 0.7168, 0.768, 0.8192, 0.8704, 0.9216, 0.9728, 1.024, 1.088, 1.152, 1.216],
    q_dmg3: [0.456, 0.4902, 0.5244, 0.57, 0.6042, 0.6384, 0.684, 0.7296, 0.7752, 0.8208, 0.8664, 0.912, 0.969, 1.026, 1.083],
};

damage_enum!(
    LynetteDamageEnum
    Normal1
    Normal2
    Normal31
    Normal32
    Normal4
    Charged11
    Charged12
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    EHeal
    Q1
    Q2
    Q3
    Q3Hydro
    Q3Pyro
    Q3Electro
    Q3Cryo
);

impl LynetteDamageEnum {
    pub fn get_element(&self) -> Element {
        use LynetteDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 => Element::Physical,
            Charged11 | Charged12 => Element::Physical,
            Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            E1 | E2 | EHeal => Element::Anemo,
            Q1 | Q2 | Q3 => Element::Anemo,
            Q3Hydro => Element::Hydro,
            Q3Pyro => Element::Pyro,
            Q3Electro => Element::Electro,
            Q3Cryo => Element::Cryo,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use LynetteDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal31 | Normal32 | Normal4 => SkillType::NormalAttack,
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | EHeal => SkillType::ElementalSkill,
            Q1 | Q2 | Q3 | Q3Hydro | Q3Pyro | Q3Electro | Q3Cryo => SkillType::ElementalBurst
        }
    }
}

pub struct LynetteEffect {
    pub has_talent1: bool,
    pub has_talent2: bool,
    pub talent1_rate: f64,
    pub talent1_count: usize,
    pub talent2_rate: f64,
}

impl<A: Attribute> ChangeAttribute<A> for LynetteEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_talent1 {
            let rate_base: f64 = 0.04 * self.talent1_count as f64 + 0.04;
            attribute.add_atk_percentage("天赋1「巧施协同」", rate_base * self.talent1_rate);
        }

        if self.has_talent2 {
            attribute.set_value_by(AttributeName::BonusElementalBurst, "天赋2「道具完备」", self.talent2_rate * 0.15);
        }
    }
}

pub struct Lynette;

impl CharacterTrait for Lynette {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Lynette,
        internal_name: "Lynette",
        name_locale: locale!(
            zh_cn: "琳妮特",
            en: "Lynette",
        ),
        element: Element::Anemo,
        hp: [1039, 2670, 3447, 5163, 5715, 6573, 7309, 8168, 8719, 9577, 10129, 10987, 11539, 12397],
        atk: [19, 50, 64, 96, 107, 123, 136, 153, 163, 179, 189, 205, 215, 232],
        def: [60, 153, 198, 296, 328, 377, 420, 469, 501, 550, 582, 631, 663, 712],
        sub_stat: CharacterSubStatFamily::Bonus240(StatName::AnemoBonus),
        weapon_type: WeaponType::Sword,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·迅捷礼刺剑",
            en: "Normal Attack: Rapid Ritesword",
        ),
        skill_name2: locale!(
            zh_cn: "谜影障身法",
            en: "Enigmatic Feint",
        ),
        skill_name3: locale!(
            zh_cn: "魔术·运变惊奇",
            en: "Magic Trick: Astonishing Shift",
        )
    };
    type SkillType = LynetteSkillType;
    const SKILL: Self::SkillType = LYNETTE_SKILL;
    type DamageEnumType = LynetteDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LynetteDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal31 hit_n_dmg!(3, 1)
            Normal32 hit_n_dmg!(3, 2)
            Normal4 hit_n_dmg!(4)
            Charged11 charged_dmg!(1)
            Charged12 charged_dmg!(2)
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            LynetteDamageEnum
            E1 locale!(zh_cn: "谜影突刺伤害", en: "Enigma Thrust DMG")
            E2 locale!(zh_cn: "流涌之刃伤害", en: "Surging Blade DMG")
            EHeal locale!(zh_cn: "命中治疗量", en: "HP Regeneration")
        ),
        skill3: skill_map!(
            LynetteDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "惊奇猫猫盒伤害", en: "Bogglecat Box DMG")
            Q3 locale!(zh_cn: "彩练术弹伤害", en: "Vivid Shot DMG")
            Q3Hydro locale!(zh_cn: "彩练术弹伤害（水）", en: "Vivid Shot DMG(Hydro)")
            Q3Pyro locale!(zh_cn: "彩练术弹伤害（火）", en: "Vivid Shot DMG(Pyro)")
            Q3Electro locale!(zh_cn: "彩练术弹伤害（雷）", en: "Vivid Shot DMG(Electro)")
            Q3Cryo locale!(zh_cn: "彩练术弹伤害（冰）", en: "Vivid Shot DMG(Cryo)")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_rate",
            title: locale!(
                zh_cn: "天赋「巧施协同」比例",
                en: "Talent 「Sophisticated Synergy」 Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 }
        },
        ItemConfig {
            name: "talent1_count",
            title: locale!(
                zh_cn: "天赋「巧施协同」不同元素类型",
                en: "Talent 「Sophisticated Synergy」 Element Counts"
            ),
            config: ItemConfigType::Int { min: 1, max: 4, default: 4 }
        },
        ItemConfig {
            name: "talent2_rate",
            title: locale!(
                zh_cn: "天赋「道具完备」比例",
                en: "Talent 「Props Positively Prepped」 Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.5 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LynetteDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use LynetteDamageEnum::*;

        let mut builder = D::new();

        if s == EHeal {
            let ratio = LYNETTE_SKILL.e_heal;
            builder.add_hp_ratio("技能倍率", ratio);

            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                Normal1 => LYNETTE_SKILL.normal_dmg1[s1],
                Normal2 => LYNETTE_SKILL.normal_dmg2[s1],
                Normal31 => LYNETTE_SKILL.normal_dmg31[s1],
                Normal32 => LYNETTE_SKILL.normal_dmg32[s1],
                Normal4 => LYNETTE_SKILL.normal_dmg4[s1],
                Charged11 => LYNETTE_SKILL.charged_dmg11[s1],
                Charged12 => LYNETTE_SKILL.charged_dmg12[s1],
                Plunging1 => LYNETTE_SKILL.plunging_dmg1[s1],
                Plunging2 => LYNETTE_SKILL.plunging_dmg2[s1],
                Plunging3 => LYNETTE_SKILL.plunging_dmg3[s1],
                E1 => LYNETTE_SKILL.e_dmg1[s2],
                E2 => LYNETTE_SKILL.e_dmg2[s2],
                Q1 => LYNETTE_SKILL.q_dmg1[s3],
                Q2 => LYNETTE_SKILL.q_dmg2[s3],
                Q3 | Q3Hydro | Q3Electro | Q3Pyro | Q3Cryo => LYNETTE_SKILL.q_dmg3[s3],
                _ => 0.0,
            };

            builder.add_atk_ratio("技能倍率", ratio);
            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (a, b, c) = match *config {
            CharacterConfig::Lynette { talent1_rate, talent1_count, talent2_rate } => (talent1_rate, talent1_count, talent2_rate),
            _ => (0.0, 1, 0.0)
        };
        Some(Box::new(LynetteEffect {
            talent1_rate: a,
            talent1_count: b,
            talent2_rate: c,
            has_talent1: common_data.has_talent1,
            has_talent2: common_data.has_talent2,
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}