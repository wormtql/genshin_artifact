use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct EchoesOfAnOfferingEffect {
    pub rate: f64,
}

const AVG_TRIGGER: f64 = 1.978911232;

impl<A: Attribute> ArtifactEffect<A> for EchoesOfAnOfferingEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_atk_percentage("来歆余响2", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ATKRatioNormalAttack, "来歆余响4", self.rate * 0.7);
    }
}

pub struct EchoesOfAnOffering;

impl ArtifactTrait for EchoesOfAnOffering {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        let rate = config.config_echoes_of_an_offering.rate;
        Box::new(EchoesOfAnOfferingEffect {
            rate
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::EchoesOfAnOffering,
        name_mona: "EchoesOfAnOffering",
        chs: "来歆余响",
        flower: Some("魂香之花"),
        feather: Some("垂玉之叶"),
        sand: Some("祝祀之凭"),
        goblet: Some("涌泉之盏"),
        head: Some("浮溯之珏"),
        star: (4, 5),
        effect1: None,
        effect2: Some("攻击力提高18%。"),
        effect3: None,
        effect4: Some("普通攻击命中敌人时，有36%概率触发「幽谷祝祀」：普通攻击造成的伤害提高，伤害提高值为攻击力的70%，该效果将在普通攻击造成伤害后的0.05秒后清除。普通攻击未触发「幽谷祝祀」时，会使下次触发概率提升20%；0.2秒内至多判定1次触发与否。（注：平均触发比例为50.53%）"),
        effect5: None,
        internal_id: 15024,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "a5",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 / AVG_TRIGGER }
        }
    ]);
}
