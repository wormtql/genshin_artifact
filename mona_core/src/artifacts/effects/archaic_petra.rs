use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct ArchaicPetraEffect {
    pub element: Element,
    pub rate: f64,
}

impl ArchaicPetraEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ArchaicPetraEffect {
        ArchaicPetraEffect {
            element: config.config_archaic_petra.element,
            rate: config.config_archaic_petra.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for ArchaicPetraEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusGeo, "悠古的磐岩2", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        let attribute_name = AttributeName::bonus_name_by_element(self.element);

        attribute.set_value_by(attribute_name, "悠古的磐岩4", self.rate * 0.35)
    }
}

pub struct ArchaicPetra;

impl ArtifactTrait for ArchaicPetra {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ArchaicPetraEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ArchaicPetra,
        name_mona: "archaicPetra",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "悠古的磐岩",
            en: "Archaic Petra",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "磐陀裂生之花",
            en: "Flower of Creviced Cliff",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "嵯峨群峰之翼",
            en: "Feather of Jagged Peaks",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "星罗圭璧之晷",
            en: "Sundial of Enduring Jade",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "巉岩琢塑之樽",
            en: "Goblet of Chiseled Crag",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "不动玄石之相",
            en: "Mask of Solitude Basalt",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "获得15%岩元素伤害加成。",
            en: "Gain a 15% Geo DMG Bonus.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "获得结晶反应形成的晶片时，队伍中所有角色获得35%对应元素伤害加成，持续10秒。同时只能通过该效果获得一种元素伤害加成。",
            en: "Upon obtaining an Elemental Shard created through a Crystallize Reaction, all party members gain a 35% DMG Bonus for that particular element for 10s. Only one form of Elemental DMG Bonus can be gained in this manner at any one time.",
        )),
        effect5: None,
        internal_id: 15014,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "element",
            title: crate::common::i18n::locale!(
                zh_cn: "元素",
                en: "Element",
            ),
            config: ItemConfigType::Element4 { default: Element::Electro }
        },
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "效果应用比例",
                en: "Effect Apply Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
