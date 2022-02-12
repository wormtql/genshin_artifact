use crate::character::CharacterName;
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
    pub chs: &'static str,
    pub description: &'static str,
    pub tags: &'static str, // comma split
    pub four: TargetFunctionFor,
    pub image: TargetFunctionMetaImage,
}
