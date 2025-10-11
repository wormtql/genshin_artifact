use num_derive::FromPrimitive;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::common::{Element, WeaponType, ChangeAttribute, SkillType};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::character_common_data::CharacterCommonData;
use crate::attribute::{Attribute, AttributeName};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use crate::common::i18n::locale;

pub struct EscoffierSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_frosty_parfait_dmg: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_healing_atk_ratio: [f64; 15],
    pub elemental_burst_healing_base: [f64; 15],
}

pub const ESCOFFIER_SKILL: EscoffierSkillType = EscoffierSkillType {
    // Normal Attack: Kitchen Skills
    normal_dmg1: [0.5155, 0.5575, 0.5994, 0.6594, 0.7013, 0.7493, 0.8152, 0.8812, 0.9471, 1.019, 1.091, 1.1629, 1.2348, 1.3068, 1.3787],
    normal_dmg2: [0.4759, 0.5147, 0.5534, 0.6088, 0.6475, 0.6918, 0.7526, 0.8135, 0.8744, 0.9408, 1.0072, 1.0736, 1.14, 1.2064, 1.2728],
    normal_dmg3: [0.7333, 0.7931, 0.8527, 0.9380, 0.9976, 1.0658, 1.1597, 1.2535, 1.3473, 1.4496, 1.552, 1.6542, 1.7566, 1.8589, 1.9612],
    charged_dmg1: [1.1541, 1.2481, 1.342, 1.4762, 1.5701, 1.6775, 1.8251, 1.9727, 2.1204, 2.2814, 2.4424, 2.6035, 2.7645, 2.9256, 3.0866],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.525, 2.7291, 2.9333, 3.1564, 3.3795, 3.6026, 3.8257, 4.0488, 4.2719],

    // Elemental Skill: Low-Temperature Cooking
    elemental_skill_dmg1: [0.504, 0.5418, 0.5796, 0.63, 0.6678, 0.7056, 0.756, 0.8064, 0.8568, 0.9072, 0.9576, 1.008, 1.071, 1.134, 1.197],
    elemental_skill_frosty_parfait_dmg: [1.2, 1.29, 1.38, 1.5, 1.59, 1.68, 1.8, 1.92, 2.04, 2.16, 2.28, 2.4, 2.55, 2.7, 2.85],

    // Elemental Burst: Scoring Cuts
    elemental_burst_dmg1: [5.928, 6.3726, 6.8172, 7.41, 7.8546, 8.2992, 8.892, 9.4848, 10.0776, 10.6704, 11.2632, 11.856, 12.597, 13.338, 14.079],
    elemental_burst_healing_atk_ratio: [1.7203, 1.8493, 1.9784, 2.1504, 2.2794, 2.4084, 2.5805, 2.7525, 2.9245, 3.0966, 3.2686, 3.4406, 3.6563, 3.872, 4.0877],
    elemental_burst_healing_base: [1078.53, 1186.39, 1303.25, 1429.1, 1563.93, 1707.75, 1860.57, 2022.37, 2193.16, 2372.95, 2561.73, 2759.5, 2966.27, 3182.04, 3406.8],
};

pub const ESCOFFIER_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Escoffier,
    internal_name: "Escoffier",
    element: Element::Cryo,
    hp: [1039, 2695, 3586, 5366, 5999, 6902, 7747, 8659, 9292, 10213, 10846, 11777, 12410, 13348, 14297],
    atk: [27, 70, 93, 139, 156, 179, 201, 225, 241, 265, 282, 306, 322, 347, 425],
    def: [57, 148, 197, 294, 329, 378, 425, 475, 509, 560, 595, 646, 680, 732, 784],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Polearm,
    star: 5,
    skill_name1: locale!(
        zh_cn: "普通攻击·后厨手艺",
        en: "Normal Attack: Kitchen Skills",
        ja: "通常攻撃・料理技術",
    ),
    skill_name2: locale!(
        zh_cn: "低温烹饪",
        en: "Low-Temperature Cooking",
        ja: "低温調理",
    ),
    skill_name3: locale!(
        zh_cn: "花刀技法",
        en: "Scoring Cuts",
        ja: "スコアリングカット",
    ),
    name_locale: locale!(
        zh_cn: "爱可菲",
        en: "Escoffier",
        ja: "エスコフィエ",
    ),
};

pub struct EscoffierEffect {
    pub hydro_cryo_count: usize,
    pub after_burst: bool,
}

impl EscoffierEffect {
    pub fn new(common_data: &CharacterCommonData, config: &CharacterConfig) -> Self {
        let (hydro_cryo_count, after_burst) = match config {
            CharacterConfig::Escoffier { hydro_cryo_count, after_burst } => (*hydro_cryo_count, *after_burst),
            _ => (0, false)
        };
        EscoffierEffect {
            hydro_cryo_count,
            after_burst,
        }
    }
}

impl<A: Attribute> ChangeAttribute<A> for EscoffierEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // Passive 2: 美食胜过良药 - 元素爆发后的康复食疗效果
        if self.after_burst {
            // 这个效果是持续治疗，在实际游戏中体现为持续恢复生命值
            // 在伤害计算系统中可以体现为攻击力加成（因为治疗基于攻击力）
            attribute.set_value_by(AttributeName::ATKPercentage, "爱可菲天赋：美食胜过良药", 0.1382);
        }
        
        // Passive 3: 灵感浸入调味 - 根据队伍水/冰角色数量降低敌人抗性
        if self.hydro_cryo_count > 0 {
            let resistance_reduction = match self.hydro_cryo_count {
                1 => 0.05,
                2 => 0.10,
                3 => 0.15,
                4 => 0.55, // 特殊的4人满水/冰队伍加成
                _ => 0.0,
            };
            if resistance_reduction > 0.0 {
                attribute.set_value_by(AttributeName::ResMinusCryo, "爱可菲天赋：灵感浸入调味", resistance_reduction);
                attribute.set_value_by(AttributeName::ResMinusHydro, "爱可菲天赋：灵感浸入调味", resistance_reduction);
            }
        }
    }
}

#[derive(Copy, Clone)]
#[derive(FromPrimitive, EnumString, EnumCountMacro)]
pub enum EscoffierDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Charged1,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    EFrostyParfait,
    Q1,
    QHeal,  // 添加治疗
}

impl Into<usize> for EscoffierDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

impl EscoffierDamageEnum {
    pub fn get_element(&self) -> Element {
        use EscoffierDamageEnum::*;

        match *self {
            E1 | EFrostyParfait | Q1 | QHeal => Element::Cryo,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use EscoffierDamageEnum::*;

        match *self {
            Charged1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | EFrostyParfait => SkillType::ElementalSkill,
            Q1 | QHeal => SkillType::ElementalBurst,
            _ => SkillType::NormalAttack
        }
    }
}

pub struct Escoffier;

impl CharacterTrait for Escoffier {
    const STATIC_DATA: CharacterStaticData = ESCOFFIER_STATIC_DATA;
    type SkillType = EscoffierSkillType;
    const SKILL: Self::SkillType = ESCOFFIER_SKILL;
    type DamageEnumType = EscoffierDamageEnum;
    type RoleEnum = EscoffierRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: EscoffierDamageEnum::Normal1 as usize, text: locale!(zh_cn: "一段伤害", en: "1-Hit DMG") },
            CharacterSkillMapItem { index: EscoffierDamageEnum::Normal2 as usize, text: locale!(zh_cn: "二段伤害", en: "2-Hit DMG") },
            CharacterSkillMapItem { index: EscoffierDamageEnum::Normal3 as usize, text: locale!(zh_cn: "三段伤害", en: "3-Hit DMG") },
            CharacterSkillMapItem { index: EscoffierDamageEnum::Charged1 as usize, text: locale!(zh_cn: "重击伤害", en: "Charged Attack DMG") },
            CharacterSkillMapItem { index: EscoffierDamageEnum::Plunging1 as usize, text: locale!(zh_cn: "下坠期间伤害", en: "Plunging DMG") },
            CharacterSkillMapItem { index: EscoffierDamageEnum::Plunging2 as usize, text: locale!(zh_cn: "低空坠地冲击伤害", en: "Low Plunging DMG") },
            CharacterSkillMapItem { index: EscoffierDamageEnum::Plunging3 as usize, text: locale!(zh_cn: "高空坠地冲击伤害", en: "High Plunging DMG") },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: EscoffierDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: EscoffierDamageEnum::EFrostyParfait as usize, text: locale!(zh_cn: "冻霜芭菲伤害", en: "Frosty Parfait DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: EscoffierDamageEnum::Q1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: EscoffierDamageEnum::QHeal as usize, text: locale!(zh_cn: "治疗量", en: "Healing") },
        ]),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [crate::common::item_config_type::ItemConfig]> = Some(&[
        crate::common::item_config_type::ItemConfig {
            name: "hydro_cryo_count",
            title: crate::common::i18n::locale!(
                zh_cn: "队伍中水/冰角色数量",
                en: "Hydro/Cryo Characters Count",
                ja: "チーム内の水/氷キャラクター数",
            ),
            config: crate::common::item_config_type::ItemConfigType::Int { min: 0, max: 4, default: 2 }
        },
        crate::common::item_config_type::ItemConfig {
            name: "after_burst",
            title: crate::common::i18n::locale!(
                zh_cn: "元素爆发后（康复食疗）",
                en: "After Elemental Burst (Recovery Effect)",
                ja: "元素爆発後（康復食療）",
            ),
            config: crate::common::item_config_type::ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(
        context: &DamageContext<'_, D::AttributeType>,
        s: usize,
        config: &CharacterSkillConfig,
        fumo: Option<Element>,
    ) -> D::Result {
        let s: EscoffierDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use EscoffierDamageEnum::*;
        let mut builder = D::new();

        match s {
            Normal1 => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.normal_dmg1[s1]);
            }
            Normal2 => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.normal_dmg2[s1]);
            }
            Normal3 => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.normal_dmg3[s1]);
            }
            Charged1 => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.charged_dmg1[s1]);
            }
            Plunging1 => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.plunging_dmg1[s1]);
            }
            Plunging2 => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.plunging_dmg2[s1]);
            }
            Plunging3 => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.plunging_dmg3[s1]);
            }
            E1 => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.elemental_skill_dmg1[s2]);
            }
            EFrostyParfait => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.elemental_skill_frosty_parfait_dmg[s2]);
            }
            Q1 => {
                builder.add_atk_ratio("技能倍率", ESCOFFIER_SKILL.elemental_burst_dmg1[s3]);
            }
            QHeal => {
                builder.add_atk_ratio("治疗量攻击力加成", ESCOFFIER_SKILL.elemental_burst_healing_atk_ratio[s3]);
                builder.add_extra_damage("治疗量基础值", ESCOFFIER_SKILL.elemental_burst_healing_base[s3]);
                return builder.heal(&context.attribute);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(EscoffierEffect::new(common_data, config)))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: EscoffierRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            EscoffierRoleEnum::Default => Box::new(crate::target_functions::target_functions::cryo::EscoffierDefaultTargetFunction)
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum EscoffierRoleEnum {
    Default,
}
