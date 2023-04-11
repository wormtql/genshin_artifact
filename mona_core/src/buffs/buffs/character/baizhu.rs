use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffBaizhuTalent2 {
    pub hp: f64,
    pub rate: f64,
}

impl<A: Attribute> Buff<A> for BuffBaizhuTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let hp = self.hp.min(50000.0);
        let temp = (hp / 1000.0).floor();

        attribute.set_value_by(AttributeName::EnhanceBurning, "BUFF: 白术天赋2「在地为化」", temp * 0.002 * self.rate);
        attribute.set_value_by(AttributeName::EnhanceBloom, "BUFF: 白术天赋2「在地为化」", temp * 0.002 * self.rate);
        attribute.set_value_by(AttributeName::EnhanceBurgeon, "BUFF: 白术天赋2「在地为化」", temp * 0.002 * self.rate);
        attribute.set_value_by(AttributeName::EnhanceHyperbloom, "BUFF: 白术天赋2「在地为化」", temp * 0.002 * self.rate);
        attribute.set_value_by(AttributeName::EnhanceAggravate, "BUFF: 白术天赋2「在地为化」", temp * 0.008 * self.rate);
        attribute.set_value_by(AttributeName::EnhanceSpread, "BUFF: 白术天赋2「在地为化」", temp * 0.008 * self.rate);
    }
}

impl BuffMeta for BuffBaizhuTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BaizhuTalent2,
        name_locale: locale!(zh_cn: "白术-「在地为化」", en: "Baizhu-「All Things Are of the Earth」"),
        image: BuffImage::Avatar(CharacterName::Baizhu),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "受到无郤气护盾治疗的角色，将获得「木运之岁」效果：基于白术生命值上限不超过50000点的部分，每1000点将使该角色触发的燃烧、绽放、超绽放、烈绽放反应造成的伤害提升2%，超激化、蔓激化反应带来的伤害提升0.8%，持续6秒。",
            en: "Characters who are healed by Seamless Shields will gain the Year of Verdant Favor effect: Each 1,000 Max HP that Baizhu possesses that does not exceed 50,000 will increase the Burning, Bloom, Hyperbloom, and Burgeon reaction DMG dealt by these characters by 2%, while the Aggravate and Spread reaction DMG dealt by these characters will be increased by 0.8%. This effect lasts 6s."
        )),
        from: BuffFrom::Character(CharacterName::Baizhu)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp",
            title: locale!(zh_cn: "白术的生命值", en: "Baizhu HP"),
            config: ItemConfigType::Float { min: 0.0, max: 50000.0, default: 50000.0 },
        },
        ItemConfig {
            name: "rate",
            title: locale!(zh_cn: "比例", en: "Rate"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (hp, rate) = match *b {
            BuffConfig::BaizhuTalent2 { hp, rate } => (hp, rate),
            _ => (0.0, 0.0)
        };
        Box::new(BuffBaizhuTalent2 {
            hp, rate
        })
    }
}

pub struct BuffBaizhuC4 {
    pub rate: f64,
}

impl<A: Attribute> Buff<A> for BuffBaizhuC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 白术C4「法古观冥」", 80.0 * self.rate);
    }
}

impl BuffMeta for BuffBaizhuC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BaizhuC4,
        name_locale: locale!(zh_cn: "白术-「法古观冥」", en: "Baizhu-「Ancient Art of Perception」"),
        image: BuffImage::Avatar(CharacterName::Baizhu),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "施放愈气全形论之后的15秒内，队伍中附近的所有角色元素精通提升80点。",
            en: "For 15s after Holistic Revivification is used, Baizhu will increase all nearby party members' Elemental Mastery by 80."
        )),
        from: BuffFrom::Character(CharacterName::Baizhu)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: locale!(zh_cn: "比例", en: "Rate"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let rate = match *b {
            BuffConfig::BaizhuC4 { rate } => rate,
            _ => 0.0
        };
        Box::new(BuffBaizhuC4 {
            rate
        })
    }
}
