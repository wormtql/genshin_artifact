use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;

pub struct ResolutionOfSojournerEffect;

impl<T: Attribute> ArtifactEffect<T> for ResolutionOfSojournerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("行者之心2", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalChargedAttack, "行者之心4", 0.3);
    }
}

pub struct ResolutionOfSojourner;

impl ArtifactTrait for ResolutionOfSojourner {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ResolutionOfSojournerEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ResolutionOfSojourner,
        name_mona: "resolutionOfSojourner",
        chs: "行者之心",
        flower: Some("故人之心"),
        feather: Some("归乡之羽"),
        sand: Some("逐光之石"),
        goblet: Some("异国之盏"),
        head: Some("感别之冠"),
        star: (3, 4),
        effect1: None,
        effect2: Some("攻击力提高18%。"),
        effect3: None,
        effect4: Some("重击的暴击率提高30%。"),
        effect5: None,
        internal_id: 10001,
    };
}
