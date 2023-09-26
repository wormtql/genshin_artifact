use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Wriothesley;
use crate::character::prelude::CharacterTrait;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::i18n::locale;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct WriothesleyDefaultTargetFunction;

impl TargetFunction for WriothesleyDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };

        type S = <Wriothesley as CharacterTrait>::DamageEnumType;
        let dmg_charged2 = Wriothesley::damage::<SimpleDamageBuilder>(
            &context,
            S::ChargedTalent1,
            &CharacterSkillConfig::NoConfig,
            None
        );

        dmg_charged2.normal.expectation
    }
}

impl TargetFunctionMetaTrait for WriothesleyDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::WriothesleyDefault,
        name_locale: locale!(
            zh_cn: "莱欧斯利-寂罪的密使",
            en: "Wriothesley-Emissary of Solitary Iniquity"
        ),
        description: locale!(
            zh_cn: "最大化惩戒·凌跃拳伤害",
            en: "Maximize Rebuke: Vaulting Fist DMG"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Wriothesley),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(WriothesleyDefaultTargetFunction)
    }
}
