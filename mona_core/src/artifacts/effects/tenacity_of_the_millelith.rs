use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct TenacityOfTheMillelithEffect {
    pub rate: f64,
}

impl TenacityOfTheMillelithEffect {
    pub fn new(config: &ArtifactEffectConfig) -> TenacityOfTheMillelithEffect {
        TenacityOfTheMillelithEffect {
            rate: config.config_tenacity_of_the_millelith.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for TenacityOfTheMillelithEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_hp_percentage("千岩牢固2", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("千岩牢固4", self.rate * 0.2);
        attribute.set_value_by(AttributeName::ShieldStrength, "千岩牢固4", self.rate * 0.3);
        attribute.set_value_by(AttributeName::ATKBonusForOther, "千岩牢固4", self.rate * 0.2);
    }
}

pub struct TenacityOfTheMillelith;

impl ArtifactTrait for TenacityOfTheMillelith {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(TenacityOfTheMillelithEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::TenacityOfTheMillelith,
        name_mona: "tenacityOfTheMillelith",
        chs: "千岩牢固",
        flower: Some("勋绩之花"),
        feather: Some("昭武翎羽"),
        sand: Some("金铜时晷"),
        goblet: Some("盟誓金爵"),
        head: Some("将帅兜鍪"),
        star: (4, 5),
        effect1: None,
        effect2: Some("生命值提升20%"),
        effect3: None,
        effect4: Some("元素战技命中敌人后，使队伍中附近的所有角色攻击力提升20%，护盾强效提升30%，持续3秒。该效果每0.5秒至多触发一次。装备此圣遗物套装效果的角色处于队伍后台时，依然能触发该效果。"),
        effect5: None,
        internal_id: 15017,
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
