use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffSucroseTalent1;

impl<A: Attribute> Buff<A> for BuffSucroseTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 砂糖天赋「触媒置换术」", 50.0);
    }
}

impl BuffMeta for BuffSucroseTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "砂糖-「触媒置换术」",
            en: "Sucrose-「Catalyst Conversion」",
        ),
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "砂糖天赋1：砂糖触发扩散反应时，使队伍中所有对应元素类型的角色（不包括砂糖自己）元素精通提升50，持续8秒。",
            en: "砂糖天赋1：砂糖触发扩散反应时，使队伍中所有对应元素类型的角色（不包括砂糖自己）元素精通提升50，持续8秒。",
        )),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffSucroseTalent1)
    }
}


pub struct BuffSucroseTalent2 {
    pub em: f64
}

impl<A: Attribute> Buff<A> for BuffSucroseTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.em * 0.2;
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 砂糖天赋「小小的慧风」", v);
    }
}

impl BuffMeta for BuffSucroseTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "砂糖-「小小的慧风」",
            en: "Sucrose-「Mollis Favonius」",
        ),
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "砂糖天赋2：风灵作成·陆叁零捌或禁·风灵作成·染伍同构贰型命中敌人时，基于砂糖元素精通的20%,为队伍中所有角色（不包括砂糖自己）提供元素精通加成，持续8秒。",
            en: "砂糖天赋2：风灵作成·陆叁零捌或禁·风灵作成·染伍同构贰型命中敌人时，基于砂糖元素精通的20%,为队伍中所有角色（不包括砂糖自己）提供元素精通加成，持续8秒。",
        )),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: crate::common::i18n::locale!(
                zh_cn: "砂糖的元素精通",
                en: "Sucrose's EM",
            ),
            config: ItemConfigType::FloatInput { default: 200.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::SucroseTalent2 { em } => em,
            _ => 0.0
        };

        Box::new(BuffSucroseTalent2 {
            em
        })
    }
}


pub struct BuffSucroseC6 {
    pub element: Element
}

impl<A: Attribute> Buff<A> for BuffSucroseC6 {
    fn change_attribute(&self, attribute: &mut A) {
        let name = AttributeName::bonus_name_by_element(self.element);
        attribute.set_value_by(name, "BUFF: 砂糖六命「混元熵增论」", 0.2);
    }
}

impl BuffMeta for BuffSucroseC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "砂糖-「混元熵增论」",
            en: "Sucrose-「Chaotic Entropy」",
        ),
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "砂糖命座6：禁·风灵作成·柒伍同构贰型如果发生了元素转化，则使队伍中所有角色在技能持续时间内获得20%的对应元素伤害加成。",
            en: "砂糖命座6：禁·风灵作成·柒伍同构贰型如果发生了元素转化，则使队伍中所有角色在技能持续时间内获得20%的对应元素伤害加成。",
        )),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "element",
            title: crate::common::i18n::locale!(
                zh_cn: "扩散类型",
                en: "Swirl Type",
            ),
            config: ItemConfigType::Element4 { default: Element::Electro }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let element = match *b {
            BuffConfig::SucroseC6 { element } => element,
            _ => Element::Electro
        };

        Box::new(BuffSucroseC6 {
            element
        })
    }
}
