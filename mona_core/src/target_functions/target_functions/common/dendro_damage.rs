use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{SimpleAttributeGraph2, AttributeCommon, AttributeName, Attribute};
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct DendroDamageTargetFunction {
    pub t: usize,
}

impl TargetFunctionMetaTrait for DendroDamageTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::DendroDamage,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "草伤",
            en: "Dendro DMG"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "草元素伤害最大化或最大化期望",
            en: "Maximize Crit or Avg Dendro Damage"
        ),
        tags: "",
        four: TargetFunctionFor::Common,
        image: TargetFunctionMetaImage::Custom("misc/dendro")
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "t",
            title: locale!(
                zh_cn: "类型",
                en: "Type"
            ),
            config: ItemConfigType::Option {
                options: "期望,最大值",
                default: 0
            }
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let t = match *config {
            TargetFunctionConfig::DendroDamage { t } => t,
            _ => 0
        };
        Box::new(DendroDamageTargetFunction { t })
    }
}

impl TargetFunction for DendroDamageTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let atk = attribute.get_atk();
        let crit = attribute.get_value(AttributeName::CriticalBase) + attribute.get_value(AttributeName::CriticalDendro)
            + attribute.get_value(AttributeName::CriticalAttacking);
        let crit = crit.clamp(0.0, 1.0);

        let critical_damage = attribute.get_value(AttributeName::CriticalDamageBase) + attribute.get_value(AttributeName::CriticalDamageDendro);

        let bonus = attribute.get_value(AttributeName::BonusBase) + attribute.get_value(AttributeName::BonusDendro);

        if self.t == 0 {
            atk * (1.0 + crit * critical_damage) * (1.0 + bonus)
        } else {
            atk * (1.0 + critical_damage) * (1.0 + bonus)
        }
    }
}
