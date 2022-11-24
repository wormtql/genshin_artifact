use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;

pub struct DefendersWillEffect;

impl<T: Attribute> ArtifactEffect<T> for DefendersWillEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_def_percentage("守护之心2", 0.3);
    }

    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}

pub struct DefendersWill;

impl ArtifactTrait for DefendersWill {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(DefendersWillEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::DefendersWill,
        name_mona: "defenderWill",
        chs: "守护之心",
        flower: Some("守护之花"),
        feather: Some("守护徽印"),
        sand: Some("守护座钟"),
        goblet: Some("守护之皿"),
        head: Some("守护束带"),
        star: (0, 0),
        effect1: None,
        effect2: Some("防御力提高30%。"),
        effect3: None,
        effect4: Some("队伍里每有不同一种元素类型的自己的角色，自身获得30%相应的元素抗性。"),
        effect5: None,
        internal_id: 10003,
    };
}
