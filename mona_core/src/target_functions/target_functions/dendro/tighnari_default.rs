use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::dendro::tighnari::Tighnari;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct TighnariDefaultTargetFunction {
    pub spread_rate: f64,
}

impl TargetFunction for TighnariDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfig::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Tighnari as CharacterTrait>::DamageEnumType;
        let dmg_b = Tighnari::damage::<SimpleDamageBuilder>(&&context, S::Charged3, &CharacterSkillConfig::NoConfig, None);

        let normal_rate = 1.0 - self.spread_rate;
        let dmg = dmg_b.normal.expectation * normal_rate + dmg_b.spread.unwrap().expectation * self.spread_rate;

        dmg
    }
}

impl TargetFunctionMetaTrait for TighnariDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::TighnariDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "提纳里-浅蔚轻行",
            en: "Tighnari-Verdant Strider"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "使得提纳里的重击伤害最大",
            en: "Maximize Tighnari Charged Attack"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Tighnari),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "spread_rate",
            title: locale!(
                zh_cn: "蔓激化比例",
                en: "Spread Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let spread_rate = match *config {
            TargetFunctionConfig::TighnariDefault { spread_rate } => spread_rate,
            _ => 0.0
        };
        Box::new(TighnariDefaultTargetFunction { spread_rate: spread_rate.clamp(0.0, 1.0) })
    }
}