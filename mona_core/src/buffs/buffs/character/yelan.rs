use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffYelanTalent2 {
    pub secs: usize,
}

impl<A: Attribute> Buff<A> for BuffYelanTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = self.secs as f64 * 0.035 + 0.01;
        attribute.set_value_by(AttributeName::BonusBase, "BUFF: 夜兰天赋「妙转随心」", value);
    }
}

impl BuffMeta for BuffYelanTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YelanTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "夜兰-「妙转随心」",
            en: "Yelan-「Adapt With Ease」",
        ),
        image: BuffImage::Avatar(CharacterName::Yelan),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "夜兰天赋2：「玄掷玲珑」存在期间，能使队伍中自己的当前场上角色造成的伤害提高1%，并且每1秒进一步提高3.5%，至多使角色造成的伤害提高50%。效果存在期间重新施放渊图玲珑骰，将移除原有的上述效果。",
            en: "夜兰天赋2：「玄掷玲珑」存在期间，能使队伍中自己的当前场上角色造成的伤害提高1%，并且每1秒进一步提高3.5%，至多使角色造成的伤害提高50%。效果存在期间重新施放渊图玲珑骰，将移除原有的上述效果。",
        )),
        from: BuffFrom::Character(CharacterName::Yelan)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "secs",
            title: crate::common::i18n::locale!(
                zh_cn: "经过的秒数",
                en: "Seconds Passed",
            ),
            config: ItemConfigType::Int { min: 0, max: 14, default: 14 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let secs = match *b {
            BuffConfig::YelanTalent2 { secs } => secs,
            _ => 0
        };

        Box::new(BuffYelanTalent2 {
            secs: secs.clamp(0, 14)
        })
    }
}

pub struct BuffYelanC4 {
    pub count: usize
}

impl<A: Attribute> Buff<A> for BuffYelanC4 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = self.count as f64 * 0.1;
        attribute.add_hp_percentage("BUFF: 夜兰四命「诓惑者，接树移花」", value);
    }
}

impl BuffMeta for BuffYelanC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YelanC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "夜兰-「诓惑者，接树移花」",
            en: "Yelan-「Bait-and-Switch」",
        ),
        image: BuffImage::Avatar(CharacterName::Yelan),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "夜兰4命：依照「络命丝」标记敌人的数量，每次标记将在爆发时使队伍中所有角色的生命值上限提升10%，持续25秒。通过这种方式，生命值上限至多获得40%提升。",
            en: "夜兰4命：依照「络命丝」标记敌人的数量，每次标记将在爆发时使队伍中所有角色的生命值上限提升10%，持续25秒。通过这种方式，生命值上限至多获得40%提升。",
        )),
        from: BuffFrom::Character(CharacterName::Yelan)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "count",
            title: crate::common::i18n::locale!(
                zh_cn: "标记数量",
                en: "Opponents Marked",
            ),
            config: ItemConfigType::Int { min: 1, max: 4, default: 4 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let count = match *b {
            BuffConfig::YelanC4 { count } => count,
            _ => 4
        };

        Box::new(BuffYelanC4 {
            count
        })
    }
}
