extern crate mona_derive;
use mona_derive::WeaponMetaData;

#[derive(WeaponMetaData)]
pub enum Color {
    Red,
    Green
}

#[test]
fn test_1() {
    println!("{}", x());
}