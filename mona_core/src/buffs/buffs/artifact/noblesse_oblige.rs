use crate::artifacts::ArtifactSetName;
use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffNoblesseOblige4;

impl<A: Attribute> Buff<A> for BuffNoblesseOblige4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: 昔日宗室之仪4", 0.2);
    }
}

impl BuffMeta for BuffNoblesseOblige4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NoblesseOblige4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "昔日宗室之仪4",
            en: "Noblesse Oblige 4",
        ),
        image: BuffImage::Artifact(ArtifactSetName::NoblesseOblige),
        genre: BuffGenre::Artifact,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素爆发后，队伍中所有角色攻击力提升20％，持续12秒。该效果不可叠加。",
            en: "施放元素爆发后，队伍中所有角色攻击力提升20％，持续12秒。该效果不可叠加。",
        )),
        from: BuffFrom::Artifact(ArtifactSetName::NoblesseOblige),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffNoblesseOblige4)
    }
}
