use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffXinyanC4;

impl<A: Attribute> Buff<A> for BuffXinyanC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusPhysical, "BUFF: 辛焱四命「节奏的传染」", 0.15);
    }
}

impl BuffMeta for BuffXinyanC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XinyanC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "辛焱-「节奏的传染」",
            en: "Xinyan-「Wildfire Rhythm」",
        ),
        image: BuffImage::Avatar(CharacterName::Xinyan),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "辛焱命座4：热情拂扫的伤害，会使敌人的物理抗性降低15%，持续12秒。",
            en: "辛焱命座4：热情拂扫的伤害，会使敌人的物理抗性降低15%，持续12秒。",
        )),
        from: BuffFrom::Character(CharacterName::Xinyan),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXinyanC4)
    }
}


pub struct BuffXinyanTalent2;

impl<A: Attribute> Buff<A> for BuffXinyanTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPhysical, "BUFF: 辛焱天赋「这才是摇滚!」", 0.15);
    }
}

impl BuffMeta for BuffXinyanTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XinyanTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "辛焱-「这才是摇滚!」",
            en: "Xinyan-「Now That's Rock 'N' Roll!」",
        ),
        image: BuffImage::Avatar(CharacterName::Xinyan),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "辛焱天赋2：处于热情拂扫的护盾保护下的角色造成的物理伤害提高15%。",
            en: "辛焱天赋2：处于热情拂扫的护盾保护下的角色造成的物理伤害提高15%。",
        )),
        from: BuffFrom::Character(CharacterName::Xinyan),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXinyanTalent2)
    }
}
