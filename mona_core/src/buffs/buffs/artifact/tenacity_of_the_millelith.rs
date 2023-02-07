use crate::artifacts::ArtifactSetName;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffTenacityOfTheMillelith4;

impl<A: Attribute> Buff<A> for BuffTenacityOfTheMillelith4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ShieldStrength, "BUFF: 千岩牢固4", 0.3);
        attribute.add_atk_percentage("BUFF: 千岩牢固4", 0.2);
    }
}

impl BuffMeta for BuffTenacityOfTheMillelith4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::TenacityOfTheMillelith4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "千岩牢固4",
            en: "Tenacity of the Millelith 4",
        ),
        image: BuffImage::Artifact(ArtifactSetName::TenacityOfTheMillelith),
        genre: BuffGenre::Artifact,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技命中敌人后，使队伍中附近的所有角色攻击力提升20%，护盾强效提升30%，持续3秒。",
            en: "元素战技命中敌人后，使队伍中附近的所有角色攻击力提升20%，护盾强效提升30%，持续3秒。",
        )),
        from: BuffFrom::Artifact(ArtifactSetName::TenacityOfTheMillelith),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffTenacityOfTheMillelith4)
    }
}