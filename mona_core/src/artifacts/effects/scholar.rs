use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct ScholarEffect;

impl<T: Attribute> ArtifactEffect<T> for ScholarEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "学士2", 0.2);
    }
}

pub struct Scholar;

impl ArtifactTrait for Scholar {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ScholarEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Scholar,
        name_mona: "scholar",
        chs: "学士",
        flower: Some("学士的书签"),
        feather: Some("学士的羽笔"),
        sand: Some("学士的时钟"),
        goblet: Some("学士的墨杯"),
        head: Some("学士的镜片"),
        star: (3, 4),
        effect1: None,
        effect2: Some("元素充能效率提高20%"),
        effect3: None,
        effect4: Some("获得元素微粒或元素晶球时，队伍中所有弓箭和法器角色额外恢复3点元素能量。该效果每3秒只能触发一次。"),
        effect5: None,
        internal_id: 10012,
    };
}
