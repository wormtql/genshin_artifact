use crate::character::CharacterName;
use crate::common::i18n::I18nLocale;
use crate::target_functions::TargetFunctionName;

pub enum TargetFunctionFor {
    SomeWho(CharacterName),
    Common
}

pub enum TargetFunctionMetaImage {
    Avatar,
    Custom(&'static str)
}

pub struct TargetFunctionMeta {
    pub name: TargetFunctionName,
    pub name_locale: I18nLocale,
    pub description: I18nLocale,
    pub tags: &'static str, // comma split
    pub four: TargetFunctionFor,
    pub image: TargetFunctionMetaImage,
}
