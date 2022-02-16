use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct TheExileEffect;

impl<T: Attribute> ArtifactEffect<T> for TheExileEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "流放者2", 0.2);
    }
}

pub struct TheExile;

impl ArtifactTrait for TheExile {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(TheExileEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::TheExile,
        name_mona: "exile",
        chs: "流放者",
        flower: Some("流放者之花"),
        feather: Some("流放者之羽"),
        sand: Some("流放者怀表"),
        goblet: Some("流放者之杯"),
        head: Some("流放者头冠"),
        star: (3, 4),
        effect1: None,
        effect2: Some("元素充能效率提高20%"),
        effect3: None,
        effect4: Some("施放元素爆发后，每2秒为队伍中所有角色（不包括自己）恢复2点元素能量。该效果持续6秒，无法叠加。"),
        effect5: None
    };
}
