use crate::attribute::{Attribute, AttributeName, AttributeCommon, SimpleAttributeGraph2};
use crate::character::{CharacterName, CharacterStaticData, Character};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Ineffa;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::{StatName, WeaponType, Element, SkillType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::level_coefficient::LEVEL_MULTIPLIER;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::{weapon_common_data::WeaponCommonData, Weapon};
use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::enemies::Enemy;

pub struct IneffaDefaultTargetFunction {
    pub overclocking_rate: f64,
}

impl TargetFunctionMetaTrait for IneffaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::IneffaDefault,
        name_locale: locale!(
            zh_cn: "伊涅芙-月光机师",
            en: "Ineffa-Mechanical Moon"
        ),
        description: locale!(
            zh_cn: "一轮输出：一次反应月感电+一次直伤月感电+一次薇尔琪塔放电伤害",
            en: "One Round: Reaction Moon Electro + Direct Moon Electro + Vilkita Discharge DMG"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Ineffa),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = None;

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(IneffaDefaultTargetFunction {
            overclocking_rate: 0.8 // 保留字段但不使用
        })
    }
}

impl TargetFunction for IneffaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.3,
            elemental_mastery: 1.0, // 提高元素精通权重，因为影响月感电伤害
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_electro: 1.5, // 降低雷伤加成权重，因为直伤月感电不受益
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::ElementalMastery,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::ElectroBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::GildedDreams,
                ArtifactSetName::WanderersTroupe,
                ArtifactSetName::ThunderingFury,
                ArtifactSetName::GladiatorsFinale,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD,
        }
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Ineffa as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::Ineffa;

        // 一次薇尔琪塔放电伤害
        let dmg_vilkita = Ineffa::damage_internal::<SimpleDamageBuilder>(&context, S::VilkitaDischarge as usize, &config, None);
        let electro_damage = dmg_vilkita.normal.expectation;

        // 一次反应月感电伤害（假设基于E技能触发，手动计算反应伤害）
        let character_level = character.common_data.level;
        let em = attribute.get_value(AttributeName::ElementalMastery);
        
        // 使用正确的等级系数和月感电基础倍率
        let level_multiplier = LEVEL_MULTIPLIER[character_level - 1];
        let moonelectro_base_multiplier = 1.8; // 月感电基础倍率
        let em_multiplier = 1.0 + (6.0 * em) / (em + 2000.0);
        
        // 获取月感电相关加成
        let moonelectro_base_enhance = attribute.get_value(AttributeName::EnhanceMoonelectroBase); // 被动天赋基础伤害提升
        let moonelectro_enhance = attribute.get_value(AttributeName::EnhanceMoonelectro); // 一命伤害提升
        let crit_rate = attribute.get_value(AttributeName::CriticalBase).min(1.0);
        let crit_damage = attribute.get_value(AttributeName::CriticalDamageBase);
        let resistance_ratio = enemy.get_resistance_ratio(Element::Electro, 0.0);
        
        // 反应月感电计算：等级系数 × 基础倍率 × (1+基础提升%) × (1+EM加成) × (1+月感电增伤%) × 抗性系数 × 暴击区
        let enhanced_base_multiplier = moonelectro_base_multiplier * (1.0 + moonelectro_base_enhance);
        let moonelectro_base_damage = level_multiplier * enhanced_base_multiplier * (1.0 + moonelectro_enhance);
        let moonelectro_reaction_damage = moonelectro_base_damage * em_multiplier * 
                                         (1.0 + crit_rate * crit_damage) * resistance_ratio;

        // 一次直伤月感电（天赋2：频率超限回路）
        // 公式：直伤月感电 = 3 × 攻击力 × 倍率 × (1+基础提升%) × (1+(6×元素精通)/(元素精通+2000)+月感电增伤%) × 抗性系数 × 暴击区
        let atk = attribute.get_atk(); // 使用完整的攻击力计算，包括所有来源
        let talent_ratio = 0.65; // 天赋2：频率超限回路 65%攻击力
        let multiplier_3x = 3.0; // 直伤月感电特有的3倍系数
        
        // 直伤月感电计算：3 × 攻击力 × 倍率 × (1+基础提升%) × (1+(6×元素精通)/(元素精通+2000)+月感电增伤%) × 抗性系数 × 暴击区
        let direct_em_multiplier = 6.0 * em / (em + 2000.0); // 直伤月感电的元素精通加成公式
        let direct_moonelectro_base = multiplier_3x * atk * talent_ratio * (1.0 + moonelectro_base_enhance);
        let direct_moonelectro_damage = direct_moonelectro_base * (1.0 + direct_em_multiplier + moonelectro_enhance) * 
                                       (1.0 + crit_rate * crit_damage) * resistance_ratio;

        // 总伤害 = 一次反应月感电 + 一次直伤月感电 + 一次薇尔琪塔放电伤害
        let total_damage = electro_damage + moonelectro_reaction_damage + direct_moonelectro_damage;

        total_damage
    }
}
