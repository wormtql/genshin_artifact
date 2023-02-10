use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffThomaTalent1 {
    pub stack: f64
}

impl<A: Attribute> Buff<A> for BuffThomaTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        let v = 0.05 * self.stack;
        attribute.set_value_by(AttributeName::ShieldStrength, "BUFF: 托马天赋「甲衣交叠」", v);
    }
}

impl BuffMeta for BuffThomaTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ThomaTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "托马-「甲衣交叠」",
            en: "Thoma-「Imbricated Armor」",
        ),
        image: BuffImage::Avatar(CharacterName::Thoma),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "托马天赋1：当前场上自己的角色获取或刷新烈烧佑命护盾时，护盾强效提升5%，持续时间6秒。此效果每0.3秒至多触发一次，至多叠加5次。",
            en: "托马天赋1：当前场上自己的角色获取或刷新烈烧佑命护盾时，护盾强效提升5%，持续时间6秒。此效果每0.3秒至多触发一次，至多叠加5次。",
        )),
        from: BuffFrom::Character(CharacterName::Thoma),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: crate::common::i18n::locale!(
                zh_cn: "叠加层数",
                en: "Stack Level",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 2.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let stack = match *b {
            BuffConfig::ThomaTalent1 { stack } => stack,
            _ => 0.0
        };

        Box::new(BuffThomaTalent1 {
            stack
        })
    }
}


pub struct BuffThomaC6;

impl<A: Attribute> Buff<A> for BuffThomaC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "BUFF: 托马六命「炽烧的至心」", 0.15);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "BUFF: 托马六命「炽烧的至心」", 0.15);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "BUFF: 托马六命「炽烧的至心」", 0.15);
    }
}

impl BuffMeta for BuffThomaC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ThomaC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "托马-「炽烧的至心」",
            en: "Thoma-「Burning Heart」",
        ),
        image: BuffImage::Avatar(CharacterName::Thoma),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "托马命座6：获取或刷新烈烧佑命护盾时，队伍中所有角色的普通攻击，重击与下落攻击造成的伤害提升15%，持续6秒。",
            en: "托马命座6：获取或刷新烈烧佑命护盾时，队伍中所有角色的普通攻击，重击与下落攻击造成的伤害提升15%，持续6秒。",
        )),
        from: BuffFrom::Character(CharacterName::Thoma),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffThomaC6)
    }
}
