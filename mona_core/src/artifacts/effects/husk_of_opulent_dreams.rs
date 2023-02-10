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
        name_locale: crate::common::i18n::locale!(
            zh_cn: "华馆梦醒形骸记",
            en: "Husk of Opulent Dreams",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "荣花之期",
            en: "Bloom Times",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "华馆之羽",
            en: "Plume of Luxury",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "众生之谣",
            en: "Song of Life",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "梦醒之瓢",
            en: "Calabash of Awakening",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "形骸之笠",
            en: "Skeletal Hat",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "防御力提高30%。",
            en: "DEF +30%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "装备此圣遗物套装的角色在以下情况下，将获得「问答」效果：在场上用岩元素攻击命中敌人后获得一层，每0.3秒至多触发一次；在队伍后台中，每3秒获得一层。问答至多叠加4层，每层能提供6%防御力与6%岩元素伤害加成。每6秒，若未获得问答效果，将损失一层。",
            en: "A character equipped with this Artifact set will obtain the Curiosity effect in the following conditions: When on the field, the character gains 1 stack after hitting an opponent with a Geo attack, triggering a maximum of once every 0.3s. When off the field, the character gains 1 stack every 3s. Curiosity can stack up to 4 times, each providing 6% DEF and a 6% Geo DMG Bonus. When 6 seconds pass without gaining a Curiosity stack, 1 stack is lost.",
        )),
        effect5: None,
        internal_id: 15021,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: crate::common::i18n::locale!(
                zh_cn: "「问答」效果等效层数",
                en: "「Curiosity」Equivalent Stack",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 }
        }
    ]);
}
