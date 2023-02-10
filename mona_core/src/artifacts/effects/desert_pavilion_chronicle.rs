use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct DesertPavilionChronicleEffect {
    pub rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for DesertPavilionChronicleEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusAnemo, "沙上楼阁史话2", 0.15);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "沙上楼阁史话4", 0.4 * self.rate);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "沙上楼阁史话4", 0.4 * self.rate);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "沙上楼阁史话4", 0.4 * self.rate);
    }
}

pub struct DesertPavilionChronicle;

impl ArtifactTrait for DesertPavilionChronicle {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(DesertPavilionChronicleEffect {
            rate: config.config_desert_pavilion_chronicle.rate,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::DesertPavilionChronicle,
        name_mona: "DesertPavilionChronicle",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "沙上楼阁史话",
            en: "Desert Pavilion Chronicle",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "众王之都的开端",
            en: "The First Days of the City of Kings",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "黄金邦国的结末",
            en: "End of the Golden Realm",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "失落迷途的机芯",
            en: "Timepiece of the Lost Path",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "迷醉长梦的守护",
            en: "Defender of the Enchanting Dream",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "流沙贵嗣的遗宝",
            en: "Legacy of the Desert High-Born",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "获得15%风元素伤害加成。",
            en: "Anemo DMG Bonus +15%",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "重击命中敌人后，该角色的普通攻击速度提升10%，普通攻击、重击与下落攻击造成的伤害提升40%，持续15秒。",
            en: "After Charged Attacks hit opponents, this character's Normal Attack SPD will increase by 10% while Normal, Charged, and Plunging Attack DMG will increase by 30% for 10s.",
        )),
        effect5: None,
        internal_id: 15027,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "效果应用比例",
                en: "Effect Apply Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);
}