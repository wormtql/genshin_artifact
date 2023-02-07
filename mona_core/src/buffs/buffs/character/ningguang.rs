use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffNingguangTalent2;

impl<A: Attribute> Buff<A> for BuffNingguangTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusGeo, "BUFF: 凝光天赋「储之千日，用之一刻」", 0.12);
    }
}

impl BuffMeta for BuffNingguangTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NingguangTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "凝光-「储之千日，用之一刻」",
            en: "Ningguang-「Strategic Reserve」",
        ),
        image: BuffImage::Avatar(CharacterName::Ningguang),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "凝光天赋2：穿过璇玑屏的角色会获得12%岩元素伤害加成，持续10秒。",
            en: "凝光天赋2：穿过璇玑屏的角色会获得12%岩元素伤害加成，持续10秒。",
        )),
        from: BuffFrom::Character(CharacterName::Ningguang),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffNingguangTalent2)
    }
}