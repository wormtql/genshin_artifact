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
    Shatter
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum AmplifyingType {
    Melt(Element),
    Vaporize(Element)
}
