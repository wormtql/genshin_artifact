use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffTighnariC4 {
    pub after_reaction: bool,
}

impl<A: Attribute> Buff<A> for BuffTighnariC4 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = 60.0 + if self.after_reaction { 60.0 } else { 0.0 };
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 提纳里命座4", value);
    }
}

impl BuffMeta for BuffTighnariC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::TighnariC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "提纳里-「由片叶管窥枯荣」",
            en: "Tighnari-「Withering Glimpsed in the Leaves」",
        ),
        image: BuffImage::Avatar(CharacterName::Tighnari),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "提纳里命座4：施放造生缠藤箭时，队伍中附近的所有角色的元素精通提升60点，持续8秒。若造生缠藤箭触发了燃烧、绽放、原激化或蔓激化反应，元素精通将进一步提升60点，并刷新该状态的持续时间。",
            en: "Tighnari C4: When Fashioner’s Tanglevine Shaft is unleashed, all nearby party members gain 60 Elemental Mastery for 8s. If the Fashioner’s Tanglevine Shaft triggers a Burning, Bloom, Aggravate, or Spread reaction, their Elemental Mastery will be further increased by 60. This latter case will also refresh the buff state’s duration.",
        )),
        from: BuffFrom::Character(CharacterName::Tighnari)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "after_reaction",
            title: crate::common::i18n::locale!(
                zh_cn: "造生缠藤箭触发了燃烧、绽放、原激化或蔓激化反应",
                en: "Triggered Burning, Bloom, Catalyze or Spread",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let after_reaction = match *b {
            BuffConfig::TighnariC4 { after_reaction } => after_reaction,
            _ => false
        };

        Box::new(BuffTighnariC4 {
            after_reaction
        })
    }
}