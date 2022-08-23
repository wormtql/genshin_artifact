use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct CrimsonWitchOfFlamesEffect {
    pub level: f64,
}

impl CrimsonWitchOfFlamesEffect {
    pub fn new(config: &ArtifactEffectConfig) -> CrimsonWitchOfFlamesEffect {
        CrimsonWitchOfFlamesEffect {
            level: config.config_crimson_witch_of_flames.level.min(3.0),
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for CrimsonWitchOfFlamesEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPyro, "炽烈的炎之魔女2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        let key = "炽烈的炎之魔女4";
        attribute.set_value_by(AttributeName::EnhanceOverload, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceBurning, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceBurgeon, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceVaporize, key, 0.15);
        attribute.set_value_by(AttributeName::EnhanceMelt, key, 0.15);
        attribute.set_value_by(AttributeName::BonusPyro, key, self.level * 0.075);
    }
}

pub struct CrimsonWitchOfFlames;

impl ArtifactTrait for CrimsonWitchOfFlames {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(CrimsonWitchOfFlamesEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::CrimsonWitchOfFlames,
        name_mona: "crimsonWitch",
        chs: "炽烈的炎之魔女",
        flower: Some("魔女的炎之花"),
        feather: Some("魔女常燃之羽"),
        sand: Some("魔女破灭之时"),
        goblet: Some("魔女的心之火"),
        head: Some("焦灼的魔女帽"),
        star: (4, 5),
        effect1: None,
        effect2: Some("获得15%火元素伤害加成。"),
        effect3: None,
        effect4: Some("超载、燃烧反应造成的伤害提升40%，蒸发、融化反应的加成系数提高15%。施放元素战技后的10秒内，二件套的效果提高50%，该效果最多叠加3次。"),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: "a4",
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);
}
