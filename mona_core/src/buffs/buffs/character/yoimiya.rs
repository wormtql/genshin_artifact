use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffYoimiyaTalent2 {
    pub talent1_stack: usize,
}

impl<A: Attribute> Buff<A> for BuffYoimiyaTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let p = (self.talent1_stack + 10) as f64 / 100.0;
        attribute.add_atk_percentage("BUFF: 宵宫天赋「炎昼风物诗」", p);
    }
}

impl BuffMeta for BuffYoimiyaTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YoimiyaTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "宵宫-「炎昼风物诗」",
            en: "Yoimiya-「Summer Night's Dawn」",
        ),
        image: BuffImage::Avatar(CharacterName::Yoimiya),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "宵宫天赋2：释放琉金云间草后的15秒内，附近的队伍中所有其它角色（不包括宵宫自己）攻击力提高10%。此外，依据宵宫自己释放琉金云间草时固有天赋「袖火百景图」的叠加层数，将额外提升上述的攻击力效果，每层提升1%攻击力。",
            en: "宵宫天赋2：释放琉金云间草后的15秒内，附近的队伍中所有其它角色（不包括宵宫自己）攻击力提高10%。此外，依据宵宫自己释放琉金云间草时固有天赋「袖火百景图」的叠加层数，将额外提升上述的攻击力效果，每层提升1%攻击力。",
        )),
        from: BuffFrom::Character(CharacterName::Yoimiya),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_stack",
            title: crate::common::i18n::locale!(
                zh_cn: "「袖火百景图」叠加层数",
                en: "「Tricks of the Trouble-Maker」Stack",
            ),
            config: ItemConfigType::Int { min: 0, max: 10, default: 0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let talent1_stack = match *b {
            BuffConfig::YoimiyaTalent2 { talent1_stack } => talent1_stack,
            _ => 0
        };

        Box::new(BuffYoimiyaTalent2 {
            talent1_stack
        })
    }
}
