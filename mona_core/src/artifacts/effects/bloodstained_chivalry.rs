use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct BloodstainedChivalryEffect {
    pub rate: f64,
}

impl BloodstainedChivalryEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BloodstainedChivalryEffect {
        BloodstainedChivalryEffect {
            rate: config.config_bloodstained_chivalry.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BloodstainedChivalryEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPhysical, "染血的骑士道2", 0.25);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusChargedAttack, "染血的骑士道4", self.rate * 0.5);
    }
}

pub struct BloodstainedChivalry;

impl ArtifactTrait for BloodstainedChivalry {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BloodstainedChivalryEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::BloodstainedChivalry,
        name_mona: "bloodstainedChivalry",
        chs: "染血的骑士道",
        flower: Some("染血的铁之心"),
        feather: Some("染血的黑之羽"),
        sand: Some("骑士染血之时"),
        goblet: Some("染血骑士之杯"),
        head: Some("染血的铁假面"),
        star: (4, 5),
        effect1: None,
        effect2: Some("造成的物理伤害提高25%。"),
        effect3: None,
        effect4: Some("击败敌人后的10秒内，施放重击时不消耗体力，且造成的伤害提升50%。"),
        effect5: None,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "a2",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
