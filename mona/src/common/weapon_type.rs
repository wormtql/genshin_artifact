use std::string::ToString;
use strum_macros::Display;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[derive(Display)]
pub enum WeaponType {
    Sword,
    Claymore,
    Catalyst,
    Polearm,
    Bow,
}
