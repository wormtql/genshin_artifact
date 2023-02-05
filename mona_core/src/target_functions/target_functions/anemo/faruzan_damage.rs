use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{Character, CharacterName};
use crate::character::characters::Faruzan;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct FaruzanDamageTargetFunction {
    pub recharge_demand: f64,
}

impl TargetFunctionMetaTrait for FaruzanDamageTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::FaruzanDamage,
        name_locale: locale!(zh_cn: "珐露珊-机逐封秘", en: "Faruzan-Enigmatic Machinist"),
        description: locale!(zh_cn: "使得珐露珊大招伤害最大", en: "Maximize Faruzan's Q damage"),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Faruzan),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 2.0 }
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let recharge_demand = match *config {
            TargetFunctionConfig::FaruzanDamage { recharge_demand } => recharge_demand,
            _ => 1.0
        };
        Box::new(FaruzanDamageTargetFunction {
            recharge_demand
        })
    }
}

impl TargetFunction for FaruzanDamageTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type T = <Faruzan as CharacterTrait>::DamageEnumType;
        let dmg_q = Faruzan::damage::<SimpleDamageBuilder>(
            &context,
            T::Q1,
            &CharacterSkillConfig::Faruzan { talent2_ratio: 1.0 },
            None
        ).normal.expectation;
        let r = self.recharge_demand.min(attribute.get_value(AttributeName::Recharge));

        r * dmg_q
    }
}
