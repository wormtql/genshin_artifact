use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffNilouTalent1;

impl<A: Attribute> Buff<A> for BuffNilouTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 「金杯的丰馈」", 100.0);
    }
}

impl BuffMeta for BuffNilouTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NilouTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "妮露-「折旋落英之庭」",
            en: "Nilou-「Court of Dancing Petals」",
        ),
        image: BuffImage::Avatar(CharacterName::Nilou),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "处于金杯的丰馈状态下的角色受到草元素攻击会使附近的所有角色元素精通提升100点，持续10秒",
            en: "Characters under the effect of Golden Chalice’s Bounty will increase the Elemental Mastery of all nearby characters by 100 for 10s whenever they are hit by Dendro attacks",
        )),
        from: BuffFrom::Character(CharacterName::Nilou)
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffNilouTalent1)
    }
}

pub struct BuffNilouTalent2 {
    pub hp: f64,
}

impl<A: Attribute> Buff<A> for BuffNilouTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.hp >= 31000.0 {
            let value = (((self.hp - 30000.0) / 1000.0).floor() * 0.09).min(4.0);
            attribute.set_value_by(AttributeName::EnhanceBloom, "BUFF: 妮露天赋「翩舞永世之梦」", value);
        }
    }
}

impl BuffMeta for BuffNilouTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NilouTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "妮露-「翩舞永世之梦」",
            en: "Nilou-「Dreamy Dance of Aeons」",
        ),
        image: BuffImage::Avatar(CharacterName::Nilou),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "基于妮露生命值上限超过30000的部分，每1000点生命值上限将使处于「金杯的丰馈」状态下的角色触发的丰穰之核造成的伤害提升9%。<br>通过这种方式至多使丰穰之核造成的伤害提升400%。",
            en: "Each 1,000 points of Nilou’s Max HP above 30,000 will cause the DMG dealt by Bountiful Cores created by characters affected by Golden Chalice’s Bounty to increase by 7%.<br>The maximum increase in Bountiful Core DMG that can be achieved this way is 300%.",
        )),
        from: BuffFrom::Character(CharacterName::Nilou)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp",
            title: crate::common::i18n::locale!(
                zh_cn: "妮露的生命值",
                en: "Nilou's HP",
            ),
            config: ItemConfigType::FloatInput { default: 60000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let hp = match *b {
            BuffConfig::NilouTalent2 { hp } => hp,
            _ => 0.0
        };
        Box::new(BuffNilouTalent2 { hp })
    }
}
