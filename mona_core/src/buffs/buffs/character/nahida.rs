use std::cmp::max;
use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffNahidaTalent1 {
    pub max_em: f64,
}

impl<A: Attribute> Buff<A> for BuffNahidaTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.max_em * 0.25).min(250.0);
        attribute.set_value_by(AttributeName::ElementalMasteryExtra, "BUFF: 纳西妲天赋1", value);
    }
}

impl BuffMeta for BuffNahidaTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NahidaTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "纳西妲-「净善摄受明论」",
            en: "Nahida-「Compassion Illuminated」",
        ),
        image: BuffImage::Avatar(CharacterName::Nahida),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "施放心景幻成时，摩耶之殿将获得以下效果：<br>依据队伍中元素精通最高的角色的元素精通数值的25%，提高领域内当前场上角色的元素精通。通过这种方式，至多提升250点元素精通。",
            en: "When unleashing Illusory Heart, the Shrine of Maya will gain the following effects:<br>The Elemental Mastery of the active character within the field will be increased by 25% of the Elemental Mastery of the party member with the highest Elemental Mastery. You can gain a maximum of 250 Elemental Mastery in this manner."
        )),
        from: BuffFrom::Character(CharacterName::Nahida)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "max_em",
            title: crate::common::i18n::locale!(
                zh_cn: "队伍最大元素精通",
                en: "Max EM in Team",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3000.0, default: 1000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let max_em = match *b {
            BuffConfig::NahidaTalent1 { max_em } => max_em,
            _ => 0.0
        };
        Box::new(BuffNahidaTalent1 {
            max_em
        })
    }
}
