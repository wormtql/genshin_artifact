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
        name_locale: crate::common::i18n::locale!(
            zh_cn: "行者之心",
            en: "Resolution of Sojourner",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "故人之心",
            en: "Heart of Comradeship",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "归乡之羽",
            en: "Feather of Homecoming",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "逐光之石",
            en: "Sundial of the Sojourner",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "异国之盏",
            en: "Goblet of the Sojourner",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "感别之冠",
            en: "Crown of Parting",
        )),
        star: (3, 4),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力提高18%。",
            en: "ATK +18%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "重击的暴击率提升30%。",
            en: "Increases Charged Attack CRIT Rate by 30%.",
        )),
        effect5: None,
        internal_id: 10001,
    };
}
