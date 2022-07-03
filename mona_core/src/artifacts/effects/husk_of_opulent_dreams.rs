use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct HuskOfOpulentDreamsEffect {
    pub level: f64,
}

impl HuskOfOpulentDreamsEffect {
    pub fn new(config: &ArtifactEffectConfig) -> HuskOfOpulentDreamsEffect {
        HuskOfOpulentDreamsEffect {
            level: config.config_husk_of_opulent_dreams.level,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for HuskOfOpulentDreamsEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_def_percentage("华馆梦醒形骸记2", 0.3);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_def_percentage("华馆梦醒形骸记4", 0.06 * self.level);
        attribute.set_value_by(AttributeName::BonusGeo, "华馆梦醒形骸记4", self.level * 0.06);
    }
}

pub struct HuskOfOpulentDreams;

impl ArtifactTrait for HuskOfOpulentDreams {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(HuskOfOpulentDreamsEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::HuskOfOpulentDreams,
        name_mona: "huskOfOpulentDreams",
        chs: "华馆梦醒形骸记",
        flower: Some("荣花之期"),
        feather: Some("华馆之羽"),
        sand: Some("众生之谣"),
        goblet: Some("梦醒之瓢"),
        head: Some("形骸之笠"),
        star: (4, 5),
        effect1: None,
        effect2: Some("防御力提高30%。"),
        effect3: None,
        effect4: Some("装备此圣遗物套装的角色在以下情况下，将获得「问答」效果：在场上用岩元素攻击命中敌人后获得一层，每0.3秒至多触发一次；在队伍后台中，每3秒获得一层。问答至多叠加4层，每层能提供6%防御力与6%岩元素伤害加成。每6秒，若未获得问答效果，将损失一层。"),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: "a6",
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 }
        }
    ]);
}
