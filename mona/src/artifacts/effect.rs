use std::collections::HashMap;

use crate::attribute::{AttributeName, AttributeGraph};
use crate::common::Element;
use super::artifact::ArtifactSetName;
use super::effect_config::ArtifactEffectConfig;
use crate::character::{CharacterStaticData, Character};


pub trait ArtifactEffect {
    fn effect1(&self, attribute: &mut AttributeGraph) {}

    fn effect2(&self, attribute: &mut AttributeGraph) {}

    fn effect3(&self, attribute: &mut AttributeGraph) {}

    fn effect4(&self, attribute: &mut AttributeGraph) {}

    fn effect5(&self, attribute: &mut AttributeGraph) {}
}
