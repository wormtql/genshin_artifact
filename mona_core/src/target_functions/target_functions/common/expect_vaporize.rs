use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, SimpleAttributeGraph2, AttributeCommon, AttributeName};
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::{Element, SkillType, StatName};
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::SimpleDamageBuilder;
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct ExpectVaporizeTargetFunction {
    pub t: usize,
    pub skill: SkillType,
}

impl TargetFunctionMetaTrait for ExpectVaporizeTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::ExpectVaporize,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "期望蒸发伤害",
            en: "Expect Vaporize DMG"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "使得蒸发反应的期望伤害最高。<br><b>注意：</b>仅考虑最简单的情况，特殊机制不考虑（例如某些技能的属性转化等）",
            en: "Maximize vaporize DMG<br><b>Attention:</b>This function only calculates the simplest case, some attribute conversions are not considered, you may not use this unless you know what you're doing"
        ),
        tags: "输出",
        four: TargetFunctionFor::Common,
        image: TargetFunctionMetaImage::Custom("misc/sword")
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "t",
            title: locale!(
                zh_cn: "触发元素",
                en: "Trigger Element",
            ),
            config: ItemConfigType::Option {
                options: "火,水",
                default: 0
            }
        },
        ItemConfig {
            name: "skill",
            title: locale!(
                zh_cn: "技能",
                en: "Skill"
            ),
            config: ItemConfigType::Skill4 { default: SkillType::NormalAttack }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (t, skill) = match *config {
            TargetFunctionConfig::ExpectVaporize { t, skill } => (t, skill),
            _ => (0, SkillType::NormalAttack)
        };

        Box::new(ExpectVaporizeTargetFunction {
            t,
            skill
        })
    }
}

impl TargetFunction for ExpectVaporizeTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        // let mut goblets = Vec::new();
        // goblets.push(StatName::ATKPercentage);
        // goblets.push(StatName::ElementalMastery);
        // if self.t == 0 {
        //     goblets.push(StatName::PyroBonus);
        // } else {
        //     goblets.push(StatName::HydroBonus);
        // }
        //
        // TargetFunctionOptConfig {
        //     atk_fixed: 1.0,
        //     atk_percentage: 0.0,
        //     hp_fixed: 0.5,
        //     hp_percentage: 1.0,
        //     def_fixed: 0.0,
        //     def_percentage: 0.0,
        //     recharge: 0.0,
        //     elemental_mastery: 1.0,
        //     critical: 1.0,
        //     critical_damage: 1.0,
        //     healing_bonus: 0.0,
        //     bonus_electro: 0.0,
        //     bonus_pyro: if self.t == 0 { 2.0 } else { 0.0 },
        //     bonus_hydro: if self.t == 0 { 0.0 } else { 2.0 },
        //     bonus_anemo: 0.0,
        //     bonus_cryo: 0.0,
        //     bonus_geo: 0.0,
        //     bonus_dendro: 0.0,
        //     bonus_physical: 0.0,
        //     sand_main_stats: vec![
        //         StatName::ATKPercentage,
        //         StatName::ElementalMastery,
        //     ],
        //     goblet_main_stats: goblets,
        //     head_main_stats: vec![
        //         StatName::ATKPercentage,
        //         StatName::CriticalRate,
        //         StatName::CriticalDamage,
        //         StatName::ElementalMastery,
        //     ],
        //     set_names: Some(vec![
        //         ArtifactSetName::CrimsonWitchOfFlames,
        //     ]),
        //     very_critical_set_names: None,
        //     normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
        //     critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
        //     very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        // }
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, _character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let element = if self.t == 0 {
            Element::Pyro
        } else {
            Element::Hydro
        };

        let builder = SimpleDamageBuilder::new(3.0, 0.0, 0.0);
        let result = builder.damage(&attribute, &enemy, element, self.skill, 90, None);

        result.vaporize.unwrap().expectation
    }
}
