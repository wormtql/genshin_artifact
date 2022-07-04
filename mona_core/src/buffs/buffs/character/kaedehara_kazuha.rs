use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffKaedeharaKazuhaTalent2 {
    pub em: f64,
    pub element: Element
}

impl<A: Attribute> Buff<A> for BuffKaedeharaKazuhaTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let name = AttributeName::bonus_name_by_element(self.element);
        let value = 0.0004 * self.em;
        attribute.set_value_by(name, "BUFF: 枫原万叶天赋「风物之诗咏」", value);
    }
}

impl BuffMeta for BuffKaedeharaKazuhaTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KaedeharaKazuhaTalent2,
        chs: "枫原万叶-「风物之诗咏」",
        image: BuffImage::Avatar(CharacterName::KaedeharaKazuha),
        genre: BuffGenre::Character,
        description: Some("枫原万叶天赋2：枫原万叶触发扩散反应后，枫原万叶的每点元素精通，会为队伍中所有角色提供0.04%对应元素伤害加成，持续8秒。"),
        from: BuffFrom::Character(CharacterName::KaedeharaKazuha),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "element",
            title: "b6",
            config: ItemConfigType::Element4 { default: Element::Electro }
        },
        ItemConfig {
            name: "em",
            title: "b7",
            config: ItemConfigType::FloatInput { default: 800.0 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (element, em) = match *b {
            BuffConfig::KaedeharaKazuhaTalent2 { element, em } => (element, em),
            _ => (Element::Electro, 0.0)
        };

        Box::new(BuffKaedeharaKazuhaTalent2 {
            element, em
        })
    }
}

pub struct BuffKaedeharaKazuhaC2;

impl<A: Attribute> Buff<A> for BuffKaedeharaKazuhaC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 枫原万叶二命「山岚残芯」", 200.0);
    }
}

impl BuffMeta for BuffKaedeharaKazuhaC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KaedeharaKazuhaC2,
        chs: "枫原万叶-「山岚残芯」",
        image: BuffImage::Avatar(CharacterName::KaedeharaKazuha),
        genre: BuffGenre::Character,
        description: Some("枫原万叶命座2：场上角色的元素精通提升200点。"),
        from: BuffFrom::Character(CharacterName::KaedeharaKazuha),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffKaedeharaKazuhaC2)
    }
}