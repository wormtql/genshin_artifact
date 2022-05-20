use crate::common::Element;

pub enum DamageType {
    Melt(Element),
    Vaporize(Element),
    Physical,
    Pryo,
    Cryo,
    Electro,
    Dendro,
    Geo,
    Anemo,
    Hydro,

    Superconduct,
    Overload,
    Electrocharged,
    Swirl(Element)
}