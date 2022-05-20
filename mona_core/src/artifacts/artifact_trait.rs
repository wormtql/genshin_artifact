use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct ArtifactMetaData {
    pub name: ArtifactSetName,
    pub name_mona: &'static str,
    pub chs: &'static str,
    pub flower: Option<&'static str>,
    pub feather: Option<&'static str>,
    pub sand: Option<&'static str>,
    pub goblet: Option<&'static str>,
    pub head: Option<&'static str>,
    pub star: (usize, usize),
    pub effect1: Option<&'static str>,
    pub effect2: Option<&'static str>,
    pub effect3: Option<&'static str>,
    pub effect4: Option<&'static str>,
    pub effect5: Option<&'static str>,
}

pub trait ArtifactTrait {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>>;

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData;

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
