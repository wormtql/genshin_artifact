use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffKachinaC4 {
    pub character_count: usize,
    pub rate: f64,
}

impl<A: Attribute> Buff<A> for BuffKachinaC4 {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = self.character_count as f64 * 0.04 + 0.04;
        attribute.add_def_percentage("C4「敌人越多，越要小心」", bonus);
    }
}

impl BuffMeta for BuffKachinaC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KachinaC4,
        name_locale: locale!(
            zh_cn: "卡齐娜-「敌人越多，越要小心」",
            en: "Kachina-「More Foes, More Caution」"
        ),
        image: BuffImage::Avatar(CharacterName::Kachina),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "现在，认真时间！的超级钻钻领域中，存在的敌人数量为1/2/3/4名或更多时，领域中的队伍中当前场上角色的防御力提升8%/12%/16%/20%。",
            en: "When there are 1/2/3/4 (or more) opponents within the Turbo Drill Field created by \"Time to Get Serious!\", the active character within the field gains 8%/12%/16%/20% increased DEF."
        )),
        from: BuffFrom::Character(CharacterName::Kachina),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "character_count",
            title: locale!(
                zh_cn: "敌人数量",
                en: "Enemy Number"
            ),
            config: ItemConfigType::Int { min: 1, max: 4, default: 4 }
        },
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "比例",
                en: "Ratio"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (character_count, rate) = match *b {
            BuffConfig::KachinaC4 { character_count, rate } => (character_count, rate),
            _ => (1, 0.0)
        };
        Box::new(BuffKachinaC4 {
            character_count,
            rate
        })
    }
}
