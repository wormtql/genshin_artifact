use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffCandaceQ;

impl<A: Attribute> Buff<A> for BuffCandaceQ {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAndElemental, "BUFF: 坎蒂丝Q", 0.2);
    }
}

impl BuffMeta for BuffCandaceQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CandaceQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "坎蒂丝-「赤冕祝祷」",
            en: "Candace-「Prayer of the Crimson Crown」",
        ),
        image: BuffImage::Avatar(CharacterName::Candace),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "角色的普通攻击对敌人造成元素伤害时，提高造成的伤害（20%）",
            en: "Characters deal increased Elemental DMG with their Normal Attacks(20%)",
        )),
        from: BuffFrom::Character(CharacterName::Candace)
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffCandaceQ)
    }
}

pub struct BuffCandaceTalent2 {
    pub hp: f64,
}

impl<A: Attribute> Buff<A> for BuffCandaceTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAndElemental, "BUFF: 坎蒂丝天赋2", (self.hp / 1000.0).floor() * 0.005);
    }
}

impl BuffMeta for BuffCandaceTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CandaceTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "坎蒂丝-「漫沙陨穹」",
            en: "Candace-「Celestial Dome of Sand」",
        ),
        image: BuffImage::Avatar(CharacterName::Candace),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "处于圣仪·灰鸰衒潮的赤冕祝祷状态下的角色，普通攻击对敌人造成元素伤害时，坎蒂丝每1000点生命值上限会使这次伤害提高0.5%。",
            en: "Characters affected by the Prayer of the Crimson Crown caused by Sacred Rite: Wagtail’s Tide will deal 0.5% increased DMG to opponents for every 1,000 points of Candace’s Max HP when they deal Elemental DMG with their Normal Attacks.",
        )),
        from: BuffFrom::Character(CharacterName::Candace)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp",
            title: crate::common::i18n::locale!(
                zh_cn: "坎蒂丝的生命值",
                en: "Candace's HP",
            ),
            config: ItemConfigType::FloatInput { default: 30000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let hp = match *b {
            BuffConfig::CandaceTalent2 { hp } => hp,
            _ => 0.0
        };

        Box::new(BuffCandaceTalent2 { hp })
    }
}
