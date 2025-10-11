use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::macros::{damage_enum, skill_map};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use num;

pub struct SkirkSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    
    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_dmg4: [f64; 15],
    pub elemental_skill_dmg5: [f64; 15],
    pub elemental_skill_charged_dmg: [f64; 15],
    
    pub elemental_burst_slash_dmg: [f64; 15],
    pub elemental_burst_final_slash_dmg: [f64; 15],
    pub elemental_burst_cunning_bonus: [f64; 15], // 蛇之狡谋加成
    pub void_rift_bonus_0: [f64; 15], // 汲取0枚虚境裂隙伤害提升
    pub void_rift_bonus_1: [f64; 15], // 汲取1枚虚境裂隙伤害提升  
    pub void_rift_bonus_2: [f64; 15], // 汲取2枚虚境裂隙伤害提升
    pub void_rift_bonus_3: [f64; 15], // 汲取3枚虚境裂隙伤害提升
}

pub const SKIRK_SKILL: SkirkSkillType = SkirkSkillType {
    // Normal Attack: Havoc: Sunder
    normal_dmg1: [0.5452, 0.5896, 0.634, 0.6974, 0.7418, 0.7925, 0.8622, 0.932, 1.0017, 1.0778, 1.1539, 1.23, 1.306, 1.3821, 1.4582],
    normal_dmg2: [0.4979, 0.5385, 0.579, 0.6369, 0.6774, 0.7238, 0.7874, 0.8511, 0.9148, 0.9843, 1.0538, 1.1233, 1.1927, 1.2622, 1.3317],
    normal_dmg3: [0.6484, 0.7012, 0.754, 0.8294, 0.8822, 0.9426, 1.0254, 1.1084, 1.1914, 1.2818, 1.3722, 1.4628, 1.5532, 1.6438, 1.7342], // 32.42% + 32.42%
    normal_dmg4: [0.608, 0.6575, 0.707, 0.7777, 0.8272, 0.8838, 0.9615, 1.0393, 1.1171, 1.2019, 1.2867, 1.3716, 1.4564, 1.5413, 1.6261],
    normal_dmg5: [0.829, 0.8965, 0.964, 1.0604, 1.1279, 1.205, 1.311, 1.4171, 1.5231, 1.6388, 1.7545, 1.8702, 1.9858, 2.1015, 2.2172],
    charged_dmg: [1.3364, 1.4452, 1.554, 1.7094, 1.8182, 1.9426, 2.1134, 2.2844, 2.4554, 2.6418, 2.8282, 3.0148, 3.2012, 3.3878, 3.5742], // 66.82%×2
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0112, 1.0931, 1.175, 1.2638, 1.3526, 1.4414, 1.5302, 1.619, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    
    // Elemental Skill: Havoc: Warp (Seven-Phase Flash mode attacks)
    elemental_skill_dmg1: [1.3282, 1.4364, 1.5445, 1.6989, 1.807, 1.9306, 2.1005, 2.2704, 2.4403, 2.6256, 2.8109, 2.9963, 3.1816, 3.3669, 3.5523],
    elemental_skill_dmg2: [1.198, 1.2955, 1.393, 1.5323, 1.6298, 1.7413, 1.8945, 2.0477, 2.201, 2.3681, 2.5353, 2.7025, 2.8696, 3.0368, 3.2039],
    elemental_skill_dmg3: [1.5144, 1.6378, 1.761, 1.9372, 2.0604, 2.2012, 2.395, 2.5886, 2.7824, 2.9938, 3.205, 3.4164, 3.6276, 3.839, 4.0504], // 75.72% + 75.72%
    elemental_skill_dmg4: [1.6108, 1.7418, 1.873, 2.0604, 2.1914, 2.3412, 2.5472, 2.7534, 2.9594, 3.1842, 3.4088, 3.6336, 3.8584, 4.0832, 4.308], // 80.54% + 80.54%
    elemental_skill_dmg5: [1.9662, 2.1263, 2.2863, 2.515, 2.675, 2.8579, 3.1094, 3.3609, 3.6124, 3.8868, 4.1611, 4.4355, 4.7098, 4.9842, 5.2586],
    elemental_skill_charged_dmg: [1.3365, 1.4451, 1.554, 1.7094, 1.8183, 1.9425, 2.1135, 2.2845, 2.4552, 2.6418, 2.8284, 3.0147, 3.2013, 3.3876, 3.5742], // 44.55%×3
    
    // Elemental Burst: Havoc: Ruin  
    elemental_burst_slash_dmg: [1.2276, 1.3197, 1.4117, 1.5345, 1.6266, 1.7186, 1.8414, 1.9642, 2.0869, 2.2097, 2.3324, 2.4552, 2.6087, 2.7621, 2.9156],
    elemental_burst_final_slash_dmg: [2.046, 2.1995, 2.3529, 2.5575, 2.711, 2.8644, 3.069, 3.2736, 3.4782, 3.6828, 3.8874, 4.092, 4.3478, 4.6035, 4.8593],
    elemental_burst_cunning_bonus: [0.1932, 0.2077, 0.2222, 0.2415, 0.256, 0.2705, 0.2899, 0.3092, 0.3285, 0.3478, 0.3671, 0.3865, 0.4106, 0.4348, 0.4589], // 每点蛇之狡谋加成
    
    // Void Rift Damage Bonuses (极恶技·尽 - 虚境裂隙伤害提升)
    void_rift_bonus_0: [0.035, 0.04, 0.045, 0.05, 0.055, 0.06, 0.065, 0.07, 0.075, 0.08, 0.085, 0.09, 0.095, 0.10, 0.105], // 0枚
    void_rift_bonus_1: [0.066, 0.072, 0.078, 0.084, 0.09, 0.096, 0.102, 0.108, 0.114, 0.12, 0.126, 0.132, 0.138, 0.144, 0.15], // 1枚
    void_rift_bonus_2: [0.088, 0.096, 0.104, 0.112, 0.12, 0.128, 0.136, 0.144, 0.152, 0.16, 0.168, 0.176, 0.184, 0.192, 0.20], // 2枚
    void_rift_bonus_3: [0.11, 0.12, 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19, 0.20, 0.21, 0.22, 0.23, 0.24, 0.25], // 3枚
};

pub const SKIRK_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Skirk,
    internal_name: "Skirk",
    element: Element::Cryo,
    hp: [967, 2508, 3336, 4992, 5581, 6421, 7206, 8055, 8644, 9501, 10089, 10956, 11544, 12417, 13300],
    atk: [28, 72, 96, 144, 161, 186, 208, 233, 250, 274, 292, 317, 334, 359, 439],
    def: [63, 163, 217, 324, 362, 417, 468, 523, 561, 617, 655, 711, 750, 806, 863],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(zh_cn: "普通攻击·极恶技·断", en: "Normal Attack: Havoc: Sunder"),
    skill_name2: locale!(zh_cn: "极恶技·闪", en: "Havoc: Warp"),
    skill_name3: locale!(zh_cn: "极恶技·灭", en: "Havoc: Ruin"),
    name_locale: locale!(zh_cn: "丝柯克", en: "Skirk")
};

pub struct SkirkEffect {
    pub death_crossing_stacks: usize,
    pub team_has_hydro_cryo: bool,
}

impl<A: Attribute> ChangeAttribute<A> for SkirkEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // Passive 1: Mutual Weapons Mentorship - 队伍中拥有水或冰元素角色时，元素战技等级+1
        if self.team_has_hydro_cryo {
            attribute.set_value_by(AttributeName::BonusElementalSkill, "丝柯克天赋：诸武相授", 0.15);
        }
        
        // Passive 3: Return to Oblivion (万流归寂) - 死河渡断层数提供七相一闪模式和元素爆发的伤害加成
        // 这个效果在damage_internal中根据技能类型单独处理，不在这里直接加属性
    }
}

damage_enum!(
    SkirkDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Normal5
    Charged
    Plunging1
    Plunging2
    Plunging3
    Skill1
    Skill2
    Skill3
    Skill4
    Skill5
    SkillCharged
    BurstSlash
    BurstFinalSlash
);

impl SkirkDamageEnum {
    pub fn get_element(&self) -> Element {
        use SkirkDamageEnum::*;
        match *self {
            Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            _ => Element::Cryo,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use SkirkDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Normal5 => SkillType::NormalAttack,
            // 七相一闪模式下的攻击被视为普通攻击，类似于达达利亚的近战模式
            Skill1 | Skill2 | Skill3 | Skill4 | Skill5 => SkillType::NormalAttack,
            Charged | SkillCharged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            BurstSlash | BurstFinalSlash => SkillType::ElementalBurst,
        }
    }
}

pub struct Skirk;

impl CharacterTrait for Skirk {
    const STATIC_DATA: CharacterStaticData = SKIRK_STATIC_DATA;
    type SkillType = SkirkSkillType;
    const SKILL: Self::SkillType = SKIRK_SKILL;
    type DamageEnumType = SkirkDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            SkirkDamageEnum
            Normal1 locale!(zh_cn: "一段伤害", en: "1-Hit DMG")
            Normal2 locale!(zh_cn: "二段伤害", en: "2-Hit DMG")
            Normal3 locale!(zh_cn: "三段伤害", en: "3-Hit DMG")
            Normal4 locale!(zh_cn: "四段伤害", en: "4-Hit DMG")
            Normal5 locale!(zh_cn: "五段伤害", en: "5-Hit DMG")
            Charged locale!(zh_cn: "重击伤害", en: "Charged Attack DMG")
            Plunging1 locale!(zh_cn: "下坠期间伤害", en: "Plunging DMG")
            Plunging2 locale!(zh_cn: "低空坠地冲击伤害", en: "Low Plunging DMG")
            Plunging3 locale!(zh_cn: "高空坠地冲击伤害", en: "High Plunging DMG")
        ),
        skill2: skill_map!(
            SkirkDamageEnum
            Skill1 locale!(zh_cn: "七相一闪·一段伤害", en: "Seven-Phase Flash 1-Hit DMG")
            Skill2 locale!(zh_cn: "七相一闪·二段伤害", en: "Seven-Phase Flash 2-Hit DMG")
            Skill3 locale!(zh_cn: "七相一闪·三段伤害", en: "Seven-Phase Flash 3-Hit DMG")
            Skill4 locale!(zh_cn: "七相一闪·四段伤害", en: "Seven-Phase Flash 4-Hit DMG")
            Skill5 locale!(zh_cn: "七相一闪·五段伤害", en: "Seven-Phase Flash 5-Hit DMG")
            SkillCharged locale!(zh_cn: "七相一闪·重击伤害", en: "Seven-Phase Flash Charged Attack DMG")
        ),
        skill3: skill_map!(
            SkirkDamageEnum
            BurstSlash locale!(zh_cn: "斩击伤害", en: "Slash DMG")
            BurstFinalSlash locale!(zh_cn: "斩击最终段伤害", en: "Final Slash DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "death_crossing_stacks",
            title: locale!(
                zh_cn: "死河渡断层数",
                en: "Death's Crossing Stacks"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
        ItemConfig {
            name: "team_has_hydro_cryo",
            title: locale!(
                zh_cn: "队伍满足诸武相授条件",
                en: "Team Has Hydro & Cryo"
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "cunning_stacks",
            title: locale!(
                zh_cn: "蛇之狡谋点数(50-100)",
                en: "Cunning Stacks (50-100)"
            ),
            config: ItemConfigType::Int { min: 50, max: 100, default: 100 }
        },
        ItemConfig {
            name: "seven_phase_mode",
            title: locale!(
                zh_cn: "七相一闪模式",
                en: "Seven-Phase Flash Mode"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "death_crossing_stacks",
            title: locale!(
                zh_cn: "死河渡断层数",
                en: "Death's Crossing Stacks"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
        ItemConfig {
            name: "void_rift_count",
            title: locale!(
                zh_cn: "汲取虚境裂隙数量",
                en: "Absorbed Void Rifts"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: SkirkDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (cunning_stacks, seven_phase_mode, death_crossing_stacks, void_rift_count) = match *config {
            CharacterSkillConfig::Skirk { cunning_stacks, seven_phase_mode, death_crossing_stacks, void_rift_count } => (cunning_stacks, seven_phase_mode, death_crossing_stacks, void_rift_count),
            _ => (100, true, 0, 0)
        };

        use SkirkDamageEnum::*;
        let ratio = match s {
            Normal1 => SKIRK_SKILL.normal_dmg1[s1],
            Normal2 => SKIRK_SKILL.normal_dmg2[s1],
            Normal3 => SKIRK_SKILL.normal_dmg3[s1],
            Normal4 => SKIRK_SKILL.normal_dmg4[s1],
            Normal5 => SKIRK_SKILL.normal_dmg5[s1],
            Charged => SKIRK_SKILL.charged_dmg[s1],
            Plunging1 => SKIRK_SKILL.plunging_dmg1[s1],
            Plunging2 => SKIRK_SKILL.plunging_dmg2[s1],
            Plunging3 => SKIRK_SKILL.plunging_dmg3[s1],
            Skill1 => SKIRK_SKILL.elemental_skill_dmg1[s2],
            Skill2 => SKIRK_SKILL.elemental_skill_dmg2[s2],
            Skill3 => SKIRK_SKILL.elemental_skill_dmg3[s2],
            Skill4 => SKIRK_SKILL.elemental_skill_dmg4[s2],
            Skill5 => SKIRK_SKILL.elemental_skill_dmg5[s2],
            SkillCharged => SKIRK_SKILL.elemental_skill_charged_dmg[s2],
            BurstSlash => SKIRK_SKILL.elemental_burst_slash_dmg[s3] * 5.0, // 5 hits
            BurstFinalSlash => SKIRK_SKILL.elemental_burst_final_slash_dmg[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        // 蛇之狡谋加成 (仅对元素爆发)
        if matches!(s, BurstSlash | BurstFinalSlash) && cunning_stacks > 50 {
            let cunning_bonus_stacks = ((cunning_stacks - 50).min(12)) as f64;
            let cunning_bonus_ratio = SKIRK_SKILL.elemental_burst_cunning_bonus[s3] * cunning_bonus_stacks;
            
            // 对于斩击伤害，蛇之狡谋加成也需要×5
            if matches!(s, BurstSlash) {
                builder.add_atk_ratio("蛇之狡谋加成", cunning_bonus_ratio * 5.0);
            } else {
                builder.add_atk_ratio("蛇之狡谋加成", cunning_bonus_ratio);
            }
        }

        // 万流归寂 - 死河渡断效果
        if death_crossing_stacks > 0 {
            match s {
                // 七相一闪模式下的普通攻击伤害提升：1层110%, 2层120%, 3层170%
                Skill1 | Skill2 | Skill3 | Skill4 | Skill5 | SkillCharged if seven_phase_mode => {
                    let damage_multiplier = match death_crossing_stacks {
                        1 => 1.10,
                        2 => 1.20,
                        3 => 1.70,
                        _ => 1.0,
                    };
                    if damage_multiplier > 1.0 {
                        let base_with_cunning = ratio; // 七相一闪模式下没有蛇之狡谋加成
                        builder.add_atk_ratio("天赋「万流归寂」额外倍率", base_with_cunning * (damage_multiplier - 1.0));
                    }
                },
                // 元素爆发伤害提升：1层105%, 2层115%, 3层160%
                BurstSlash | BurstFinalSlash => {
                    let damage_multiplier = match death_crossing_stacks {
                        1 => 1.05,
                        2 => 1.15,
                        3 => 1.60,
                        _ => 1.0,
                    };
                    if damage_multiplier > 1.0 {
                        // 计算基础倍率+蛇之狡谋加成的总和
                        let mut base_with_cunning = ratio;
                        if cunning_stacks > 50 {
                            let cunning_bonus_stacks = ((cunning_stacks - 50).min(12)) as f64;
                            let cunning_bonus_ratio = SKIRK_SKILL.elemental_burst_cunning_bonus[s3] * cunning_bonus_stacks;
                            if matches!(s, BurstSlash) {
                                base_with_cunning += cunning_bonus_ratio * 5.0;
                            } else {
                                base_with_cunning += cunning_bonus_ratio;
                            }
                        }
                        // 万流归寂额外倍率 = (基础+蛇之狡谋) × (伤害加成百分比 - 1)
                        builder.add_atk_ratio("天赋「万流归寂」额外倍率", base_with_cunning * (damage_multiplier - 1.0));
                    }
                },
                _ => {}
            }
        }

        // 虚境裂隙伤害提升 (极恶技·尽期间的普通攻击伤害提升) - 使用元素爆发等级s3
        if seven_phase_mode && void_rift_count > 0 {
            match s {
                // 七相一闪模式下的普通攻击受虚境裂隙影响
                Skill1 | Skill2 | Skill3 | Skill4 | Skill5 | SkillCharged => {
                    let void_bonus = match void_rift_count {
                        1 => SKIRK_SKILL.void_rift_bonus_1[s3],
                        2 => SKIRK_SKILL.void_rift_bonus_2[s3], 
                        3 => SKIRK_SKILL.void_rift_bonus_3[s3],
                        _ => SKIRK_SKILL.void_rift_bonus_0[s3],
                    };
                    if void_bonus > 0.0 {
                        builder.add_extra_bonus("理外之理·虚境裂隙", void_bonus);
                    }
                },
                _ => {}
            }
        }

        let skill_type = s.get_skill_type();
        let element = s.get_element();
        builder.damage(
            &context.attribute,
            &context.enemy,
            element,
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (death_crossing_stacks, team_has_hydro_cryo) = match config {
            CharacterConfig::Skirk { death_crossing_stacks, team_has_hydro_cryo } => (*death_crossing_stacks, *team_has_hydro_cryo),
            _ => (0, false),
        };

        Some(Box::new(SkirkEffect {
            death_crossing_stacks,
            team_has_hydro_cryo,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        Box::new(crate::target_functions::target_functions::cryo::skirk_default::SkirkDefaultTargetFunction)
    }
}
