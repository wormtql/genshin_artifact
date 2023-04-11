use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Kaveh;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffKavehQ {
    pub q_level: usize,
    pub rate: f64,
}

impl<A: Attribute> Buff<A> for BuffKavehQ {
    fn change_attribute(&self, attribute: &mut A) {
        let value = <Kaveh as CharacterTrait>::SKILL.q_bonus[self.q_level - 1];

        attribute.set_value_by(AttributeName::EnhanceBloom, "BUFF: 卡维Q", value * self.rate);
    }
}

impl BuffMeta for BuffKavehQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KavehQ,
        name_locale: locale!(zh_cn: "卡维Q", en: "Kaveh Q"),
        image: BuffImage::Avatar(CharacterName::Kaveh),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "队伍中自己的角色触发绽放反应产生的草原核，在迸发时造成的伤害提升",
            en: "All Dendro Cores created by all your own party members through Bloom reactions will deal additional DMG when they burst",
        )),
        from: BuffFrom::Character(CharacterName::Kaveh)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "q_level",
            title: locale!(zh_cn: "卡维Q技能等级", en: "Kaveh Q Level"),
            config: ItemConfigType::Int { min: 1, max: 15, default: 8 },
        },
        ItemConfig {
            name: "rate",
            title: locale!(zh_cn: "比例", en: "Rate"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (q_level, rate) = match *b {
            BuffConfig::KavehQ { q_level, rate } => (q_level, rate),
            _ => (1, 0.0)
        };

        Box::new(BuffKavehQ {
            q_level, rate
        })
    }
}
