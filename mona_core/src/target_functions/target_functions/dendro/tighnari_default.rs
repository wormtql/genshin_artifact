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
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct TighnariDefaultTargetFunction;

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
            attribute,
            enemy,
        };

        type S = <Tighnari as CharacterTrait>::DamageEnumType;
        let dmg_c3 = Tighnari::damage::<SimpleDamageBuilder>(&&context, S::Charged3, &CharacterSkillConfig::NoConfig, None);
        let dmg_c4 = Tighnari::damage::<SimpleDamageBuilder>(&&context, S::Charged4, &CharacterSkillConfig::NoConfig, None);
        let dmg_cc6: f64 = if character.common_data.constellation == 6 { Tighnari::damage::<SimpleDamageBuilder>(&&context, S::ChargedC6, &CharacterSkillConfig::NoConfig, None).spread.unwrap().expectation } else { 0.0 };
        let dmg_q1 = Tighnari::damage::<SimpleDamageBuilder>(&&context, S::Q1, &CharacterSkillConfig::NoConfig, None);
        let dmg_q2 = Tighnari::damage::<SimpleDamageBuilder>(&&context, S::Q2, &CharacterSkillConfig::NoConfig, None);
        let dmg_e = Tighnari::damage::<SimpleDamageBuilder>(&&context, S::E1, &CharacterSkillConfig::NoConfig, None);
        let dmg =
            (dmg_c3.spread.unwrap().expectation +
                dmg_c4.spread.unwrap().expectation +
                dmg_c4.normal.expectation * 3.0 + dmg_cc6) * 3.0 +
                (dmg_q1.spread.unwrap().expectation + dmg_q2.spread.unwrap().expectation) * 2.0 +
                (dmg_q1.normal.expectation + dmg_q2.normal.expectation) * 4.0 +
                dmg_e.spread.unwrap().expectation;

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
            zh_cn: "最大化激化ezzzq一套的总伤害,如果非六命用猎人之径则把覆盖率调到0.8，六命猎人之径覆盖率调到0.7",
            en: "Maximize ecccq Combo DMG with Spread, when Hunter's Path is equiped, if you are C6, set coverage to 0.7, else set it to 0.8"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Tighnari),
        image: TargetFunctionMetaImage::Avatar,
    };


    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(TighnariDefaultTargetFunction)
    }
}
