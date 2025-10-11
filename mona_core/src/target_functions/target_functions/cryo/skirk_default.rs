use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::cryo::skirk::Skirk;
use crate::character::prelude::CharacterTrait;
use crate::character::{CharacterConfig, CharacterName};
use crate::character::skill_config::CharacterSkillConfig;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::artifacts::Artifact;
use crate::common::StatName;

pub struct SkirkDefaultTargetFunction;

impl TargetFunctionMetaTrait for SkirkDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::SkirkDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "丝柯克-虚渊暗星",
            en: "Skirk-Void Star"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "一轮完整输出循环伤害",
            en: "Full Rotation DMG"
        ),
        tags: "伤害",
        four: TargetFunctionFor::SomeWho(CharacterName::Skirk),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(SkirkDefaultTargetFunction)
    }
}

impl TargetFunction for SkirkDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.0,
            elemental_mastery: 0.0,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_electro: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 1.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::CriticalRate,
                StatName::CriticalDamage,
            ],
            goblet_main_stats: vec![
                StatName::CryoBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: None,
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD,
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let context = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy: _enemy
        };

        type S = <Skirk as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::NoConfig;

        // 计算一轮完整输出循环的伤害期望
        // 战斗循环：E技能点按 -> Q技能 -> 七相一闪模式A5次x2 -> 重击1次 -> 七相一闪模式A5次x2
        
        // Q技能：极恶技·灭
        let dmg_burst_slash = Skirk::damage_internal::<SimpleDamageBuilder>(&context, S::BurstSlash as usize, &config, None);
        let dmg_burst_final = Skirk::damage_internal::<SimpleDamageBuilder>(&context, S::BurstFinalSlash as usize, &config, None);
        
        // 七相一闪模式攻击5段连击（元素技能）
        let dmg_skill1 = Skirk::damage_internal::<SimpleDamageBuilder>(&context, S::Skill1 as usize, &config, None);
        let dmg_skill2 = Skirk::damage_internal::<SimpleDamageBuilder>(&context, S::Skill2 as usize, &config, None);
        let dmg_skill3 = Skirk::damage_internal::<SimpleDamageBuilder>(&context, S::Skill3 as usize, &config, None);
        let dmg_skill4 = Skirk::damage_internal::<SimpleDamageBuilder>(&context, S::Skill4 as usize, &config, None);
        let dmg_skill5 = Skirk::damage_internal::<SimpleDamageBuilder>(&context, S::Skill5 as usize, &config, None);
        
        // 七相一闪模式重击伤害
        let dmg_skill_charged = Skirk::damage_internal::<SimpleDamageBuilder>(&context, S::SkillCharged as usize, &config, None);
        
        // 计算七相一闪模式下一套5段攻击的总伤害
        let skill_combo_damage = dmg_skill1.normal.expectation + dmg_skill2.normal.expectation + 
                                dmg_skill3.normal.expectation + dmg_skill4.normal.expectation + 
                                dmg_skill5.normal.expectation;
        
        // 一轮循环伤害组成：
        // 1. Q技能伤害（斩击+最终段）
        // 2. 七相一闪模式5段攻击 × 4轮（前2轮 + 后2轮）
        // 3. 七相一闪模式重击 × 1次
        let total_damage = dmg_burst_slash.normal.expectation + dmg_burst_final.normal.expectation +  // Q技能
                          skill_combo_damage * 4.0 +  // 七相一闪5段 × 4轮
                          dmg_skill_charged.normal.expectation;  // 七相一闪重击1次
        
        total_damage
    }
}
