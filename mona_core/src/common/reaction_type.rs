use serde::{Serialize, Deserialize};
use crate::common::Element;

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum TransformativeType {
    SwirlCryo,
    SwirlHydro,
    SwirlElectro,
    SwirlPyro,
    Superconduct,
    Overload,
    Burning,
    ElectroCharged,
    Shatter,

    Bloom,
    // 烈绽放
    Burgeon,
    // 超绽放
    Hyperbloom,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum AmplifyingType {
    Melt(Element),
    Vaporize(Element)
}
