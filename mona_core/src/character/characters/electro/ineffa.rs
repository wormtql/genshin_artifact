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

pub struct IneffaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    
    pub elemental_skill_dmg: [f64; 15],
    pub vilkita_discharge_dmg: [f64; 15], // 薇尔琪塔放电伤害
    pub shield_absorption: [f64; 15], 
    pub shield_base: [f64; 15], // 护盾基础值
    
    pub elemental_burst_dmg: [f64; 15],
}

pub const INEFFA_SKILL: IneffaSkillType = IneffaSkillType {
    // Normal Attack: Cyclonic Duster
    normal_dmg1: [0.3484, 0.3767, 0.4051, 0.4456, 0.4739, 0.5063, 0.5509, 0.5954, 0.64, 0.6886, 0.7372, 0.7858, 0.8344, 0.883, 0.9316],
    normal_dmg2: [0.3422, 0.3701, 0.3979, 0.4377, 0.4656, 0.4974, 0.5412, 0.5849, 0.6287, 0.6765, 0.7242, 0.772, 0.8197, 0.8675, 0.9152],
    normal_dmg3: [0.4552, 0.4922, 0.5292, 0.5822, 0.6192, 0.6616, 0.7198, 0.778, 0.8362, 0.8996, 0.9632, 1.0266, 1.0902, 1.1536, 1.2172], // 22.76% × 2
    normal_dmg4: [0.5607, 0.6063, 0.652, 0.7171, 0.7628, 0.8149, 0.8867, 0.9584, 1.0301, 1.1083, 1.1865, 1.2648, 1.343, 1.4213, 1.4995],
    charged_dmg: [0.9494, 1.0267, 1.104, 1.2144, 1.2917, 1.38, 1.5014, 1.6229, 1.7443, 1.8768, 2.0093, 2.1418, 2.2742, 2.4067, 2.5392],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0112, 1.0931, 1.175, 1.2638, 1.3526, 1.4414, 1.5302, 1.619, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    
    // Elemental Skill: Cleaning Mode: Carrier Frequency
    elemental_skill_dmg: [0.864, 0.9288, 0.9936, 1.08, 1.1448, 1.2096, 1.296, 1.3824, 1.4688, 1.5552, 1.6416, 1.728, 1.836, 1.944, 2.052],
    
    // Vilkita Discharge Damage
    vilkita_discharge_dmg: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    
    // Shield absorption based on ATK + base value
    shield_absorption: [2.2118, 2.3777, 2.5436, 2.7648, 2.9307, 3.0966, 3.3178, 3.5389, 3.7601, 3.9813, 4.2024, 4.4236, 4.7046, 4.9856, 5.2666],
    shield_base: [1386.68, 1525.36, 1675.61, 1837.41, 2010.77, 2195.68, 2392.16, 2600.19, 2819.77, 3050.9, 3293.59, 3547.83, 3813.62, 4090.96, 4379.85],
    
    // Elemental Burst: Supreme Instruction: Cyclonic Exterminator
    elemental_burst_dmg: [6.768, 7.2756, 7.7832, 8.46, 8.9676, 9.4752, 10.152, 10.8288, 11.5056, 12.1824, 12.8592, 13.536, 14.382, 15.228, 16.074],
};

pub const INEFFA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Ineffa,
    internal_name: "Ineffa",
    element: Element::Electro,
    hp: [982, 2547, 3389, 5071, 5669, 6523, 7320, 8182, 8780, 9650, 10249, 11128, 11727, 12613, 13510],
    atk: [26, 67, 89, 133, 149, 171, 192, 214, 230, 253, 268, 291, 307, 330, 404],  // Converted from 25.7 base
    def: [64, 167, 222, 333, 372, 428, 480, 537, 576, 633, 673, 730, 770, 828, 887],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Polearm,
    star: 5,
    skill_name1: locale!(zh_cn: "普通攻击·除尘旋刃", en: "Normal Attack: Cyclonic Duster"),
    skill_name2: locale!(zh_cn: "涤净模式·稳态载频", en: "Cleaning Mode: Carrier Frequency"),
    skill_name3: locale!(zh_cn: "至高律令·全域扫灭", en: "Supreme Instruction: Cyclonic Exterminator"),
    name_locale: locale!(zh_cn: "伊涅芙", en: "Ineffa")
};

pub struct IneffaEffect {
    pub c1_moonelectro_bonus: f64, // 一命：循环整流引擎 - 月感电伤害提升
    pub passive_moonelectro_base_bonus: f64, // 被动：月兆祝赐·象拟中继 - 月感电基础伤害提升
    pub em_bonus_rate: f64, // 基于攻击力的元素精通加成比例
}

impl<A: Attribute> ChangeAttribute<A> for IneffaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // *** 使用基础攻击力避免循环依赖 ***
        // 天赋3：全相重构协议 - 基于攻击力的6%，提升元素精通
        // 使用 ATKExcludingSecondaryConversion 替代 ATK，只计算基础攻击力和百分比加成，不包括二次转换的攻击力
        // 这样避免了与赤沙之杖等武器的循环依赖：ATK -> EM -> ATKFixed -> ATK
        let em_bonus_rate = self.em_bonus_rate;
        attribute.add_edge1(
            AttributeName::ATKExcludingSecondaryConversion,  // 只包含基础攻击力，不含二次转换部分
            AttributeName::ElementalMasteryExtra,  // 隔离属性：不会被武器/圣遗物转换
            Box::new(move |atk, _| atk * em_bonus_rate),
            Box::new(move |grad, _atk, _em| (grad * em_bonus_rate, 0.0)),
            "伊涅芙天赋：全相重构协议 - 避免循环依赖"
        );
        
        // 一命：循环整流引擎 - 基于攻击力提升月感电反应伤害
        // 每100点攻击力提升2.5%伤害，至多50%
        // 这里使用完整的ATK，因为月感电伤害加成不会造成循环依赖
        if self.c1_moonelectro_bonus > 0.0 {
            attribute.add_edge1(
                AttributeName::ATK,  // 使用完整攻击力，包括武器转换
                AttributeName::EnhanceMoonelectro,
                Box::new(move |atk, _| {
                    let bonus_per_100_atk = 0.025; // 2.5%
                    let max_bonus = 0.50; // 50%
                    (atk / 100.0 * bonus_per_100_atk).min(max_bonus)
                }),
                Box::new(move |grad, atk, _enhance| {
                    let bonus_per_100_atk = 0.025;
                    let max_bonus = 0.50;
                    let current_bonus = (atk / 100.0 * bonus_per_100_atk).min(max_bonus);
                    // 如果达到上限，梯度为0；否则正常计算
                    let atk_grad = if current_bonus >= max_bonus { 0.0 } else { grad * bonus_per_100_atk / 100.0 };
                    (atk_grad, 0.0)
                }),
                "伊涅芙一命：循环整流引擎"
            );
        }
        
        // 被动天赋：月兆祝赐·象拟中继 - 基于攻击力提升月感电反应基础伤害
        // 每100点攻击力提升0.7%基础伤害，至多14%
        // 这里使用完整的ATK，因为月感电基础伤害加成不会造成循环依赖
        if self.passive_moonelectro_base_bonus > 0.0 {
            attribute.add_edge1(
                AttributeName::ATK,  // 使用完整攻击力，包括武器转换
                AttributeName::EnhanceMoonelectroBase,
                Box::new(move |atk, _| {
                    let bonus_per_100_atk = 0.007; // 0.7%
                    let max_bonus = 0.14; // 14%
                    (atk / 100.0 * bonus_per_100_atk).min(max_bonus)
                }),
                Box::new(move |grad, atk, _enhance| {
                    let bonus_per_100_atk = 0.007;
                    let max_bonus = 0.14;
                    let current_bonus = (atk / 100.0 * bonus_per_100_atk).min(max_bonus);
                    // 如果达到上限，梯度为0；否则正常计算
                    let atk_grad = if current_bonus >= max_bonus { 0.0 } else { grad * bonus_per_100_atk / 100.0 };
                    (atk_grad, 0.0)
                }),
                "月兆祝赐·象拟中继"
            );
        }
    }
}

damage_enum!(
    IneffaDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    Skill
    VilkitaDischarge
    Burst
    TalentOverclocking
    C2Explosion
);

impl IneffaDamageEnum {
    pub fn get_element(&self) -> Element {
        use IneffaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | Charged | Plunging1 | Plunging2 | Plunging3 => Element::Physical,
            Skill | VilkitaDischarge | Burst | TalentOverclocking | C2Explosion => Element::Electro,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use IneffaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            Skill | VilkitaDischarge => SkillType::ElementalSkill,
            Burst => SkillType::ElementalBurst,
            TalentOverclocking | C2Explosion => SkillType::ElementalSkill,
        }
    }
}

pub struct Ineffa;

impl CharacterTrait for Ineffa {
    const STATIC_DATA: CharacterStaticData = INEFFA_STATIC_DATA;
    type SkillType = IneffaSkillType;
    const SKILL: Self::SkillType = INEFFA_SKILL;
    type DamageEnumType = IneffaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            IneffaDamageEnum
            Normal1 locale!(zh_cn: "一段伤害", en: "1-Hit DMG")
            Normal2 locale!(zh_cn: "二段伤害", en: "2-Hit DMG")
            Normal3 locale!(zh_cn: "三段伤害", en: "3-Hit DMG")
            Normal4 locale!(zh_cn: "四段伤害", en: "4-Hit DMG")
            Charged locale!(zh_cn: "重击伤害", en: "Charged Attack DMG")
            Plunging1 locale!(zh_cn: "下坠期间伤害", en: "Plunging DMG")
            Plunging2 locale!(zh_cn: "低空坠地冲击伤害", en: "Low Plunging DMG")
            Plunging3 locale!(zh_cn: "高空坠地冲击伤害", en: "High Plunging DMG")
        ),
        skill2: skill_map!(
            IneffaDamageEnum
            Skill locale!(zh_cn: "技能伤害", en: "Skill DMG")
            VilkitaDischarge locale!(zh_cn: "薇尔琪塔放电伤害", en: "Vilkita Discharge DMG")
            TalentOverclocking locale!(zh_cn: "频率超限回路", en: "Overclocking Circuit")
        ),
        skill3: skill_map!(
            IneffaDamageEnum
            Burst locale!(zh_cn: "技能伤害", en: "Skill DMG")
            C2Explosion locale!(zh_cn: "惩戒敕谕", en: "Punishment Edict")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em_bonus_active",
            title: locale!(
                zh_cn: "全相重构协议生效",
                en: "Parameter Permutation Active"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "moonelectro_relay_active",
            title: locale!(
                zh_cn: "月兆祝赐·象拟中继生效",
                en: "Moonelectro Simulation Relay Active"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = None;

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: IneffaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use IneffaDamageEnum::*;
        let ratio = match s {
            Normal1 => INEFFA_SKILL.normal_dmg1[s1],
            Normal2 => INEFFA_SKILL.normal_dmg2[s1],
            Normal3 => INEFFA_SKILL.normal_dmg3[s1],
            Normal4 => INEFFA_SKILL.normal_dmg4[s1],
            Charged => INEFFA_SKILL.charged_dmg[s1],
            Plunging1 => INEFFA_SKILL.plunging_dmg1[s1],
            Plunging2 => INEFFA_SKILL.plunging_dmg2[s1],
            Plunging3 => INEFFA_SKILL.plunging_dmg3[s1],
            Skill => INEFFA_SKILL.elemental_skill_dmg[s2],
            VilkitaDischarge => INEFFA_SKILL.vilkita_discharge_dmg[s2],
            Burst => INEFFA_SKILL.elemental_burst_dmg[s3],
            TalentOverclocking => 0.65, // 天赋2：频率超限回路 65%攻击力
            C2Explosion => 3.0, // 二命：惩戒敕谕 300%攻击力
        };

        let mut builder = D::new();
        
        // 对于直伤月感电技能，使用特殊计算方式
        match s {
            TalentOverclocking => {
                // 天赋2：频率超限回路的直伤月感电
                builder.add_direct_moonelectro_ratio("频率超限回路", ratio);
            },
            C2Explosion => {
                // 二命：惩戒敕谕的直伤月感电
                builder.add_direct_moonelectro_ratio("惩戒敕谕", ratio);
            },
            _ => {
                // 普通技能使用常规计算
                builder.add_atk_ratio("技能倍率", ratio);
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
        let (em_bonus_active, moonelectro_relay_active) = match config {
            CharacterConfig::Ineffa { em_bonus_active, moonelectro_relay_active } => (*em_bonus_active, *moonelectro_relay_active),
            _ => (true, true),
        };

        let em_bonus_rate = if em_bonus_active { 0.06 } else { 0.0 };
        
        // 一命：循环整流引擎 - 月感电反应伤害提升，每100点攻击力提升2.5%，最多50%
        let c1_moonelectro_bonus = if common_data.constellation >= 1 { 1.0 } else { 0.0 };
        
        // 被动天赋：月兆祝赐·象拟中继 - 月感电反应基础伤害提升，每100点攻击力提升0.7%，最多14%
        let passive_moonelectro_base_bonus = if moonelectro_relay_active { 1.0 } else { 0.0 };

        Some(Box::new(IneffaEffect {
            c1_moonelectro_bonus,
            passive_moonelectro_base_bonus,
            em_bonus_rate,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        Box::new(crate::target_functions::target_functions::electro::ineffa_default::IneffaDefaultTargetFunction {
            overclocking_rate: 0.8
        })
    }
}
