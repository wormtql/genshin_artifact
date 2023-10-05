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
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct WriothesleyDefaultTargetFunction {
    pub punch_ratio: f64,
    pub melt_rate: f64,
}

impl WriothesleyDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (
            punch_ratio,
            melt_rate,
        ) = match *config {
            TargetFunctionConfig::WriothesleyDefault {
                punch_ratio,
                melt_rate,
            } =>
                (
                    punch_ratio,
                    melt_rate,
                ),
            _ => (0.0, 0.0)
        };

        Self {
            punch_ratio,
            melt_rate,
        }
    }
}

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
        let dmg_normal = Wriothesley::damage::<SimpleDamageBuilder>(
            &context,
            S::Normal1,
            &CharacterSkillConfig::Wriothesley {under_chilling_penalty: true},
            None
        );
        let dmg_charged2 = Wriothesley::damage::<SimpleDamageBuilder>(
            &context,
            S::ChargedTalent1,
            &CharacterSkillConfig::Wriothesley {under_chilling_penalty: true},
            None
        );

        let dmg_normal_mean = self.melt_rate * dmg_normal.melt.unwrap().expectation + (1.0-self.melt_rate) * dmg_normal.normal.expectation;
        let dmg_charged2_mean = self.melt_rate * dmg_charged2.melt.unwrap().expectation + (1.0-self.melt_rate)*dmg_charged2.normal.expectation;
        
        dmg_normal_mean * 6.3886 + self.punch_ratio*dmg_charged2_mean
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
            zh_cn: "最大化强化普+重混合伤害",
            en: "Maximize normal+charged combo DMG"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Wriothesley),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "punch_ratio",
            title: locale!(
                zh_cn: "一套普攻打几个重击",
                en: "Punch per Normal Attack Combo"
            ),
            config: ItemConfigType::Float { default: 0.5, min: 0.0, max: 5.0 }
        },
        ItemConfig {
            name: "melt_rate",
            title: locale!(
                zh_cn: "融化占比",
                en: "Melt Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        },
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(WriothesleyDefaultTargetFunction::new(config))
    }
}
