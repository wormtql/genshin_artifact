use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::I18nLocale;
use crate::common::item_config_type::ItemConfig;

pub struct ArtifactMetaData {
    pub name: ArtifactSetName,
    // for historical reasons, this is key
    pub name_mona: &'static str,
    pub name_locale: I18nLocale,
    pub flower: Option<I18nLocale>,
    pub feather: Option<I18nLocale>,
    pub sand: Option<I18nLocale>,
    pub goblet: Option<I18nLocale>,
    pub head: Option<I18nLocale>,
    pub star: (usize, usize),
    pub effect1: Option<I18nLocale>,
    pub effect2: Option<I18nLocale>,
    pub effect3: Option<I18nLocale>,
    pub effect4: Option<I18nLocale>,
    pub effect5: Option<I18nLocale>,
    pub internal_id: usize,
}

pub trait ArtifactTrait {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>>;

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData;

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
