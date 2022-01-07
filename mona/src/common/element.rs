use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Element {
    Electro,
    Pyro,
    Cryo,
    Dendro,
    Geo,
    Anemo,
    Hydro,
    Physical,
    NoElement,
}