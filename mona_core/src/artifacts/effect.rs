use crate::attribute::Attribute;

pub trait ArtifactEffect<T> {
    fn effect1(&self, attribute: &mut T) {}

    fn effect2(&self, attribute: &mut T) {}

    fn effect3(&self, attribute: &mut T) {}

    fn effect4(&self, attribute: &mut T) {}

    fn effect5(&self, attribute: &mut T) {}
}

pub struct ArtifactEffectNone;

impl<A: Attribute> ArtifactEffect<A> for ArtifactEffectNone {

}
