use serde::{Serialize, Deserialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Display)]
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
}