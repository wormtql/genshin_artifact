use crate::attribute::Attribute;
use crate::buffs::{Buff, BuffConfig};

pub use common::*;
pub use resonance::{BuffResonanceGeo2, BuffResonancePyro2, BuffResonanceCryo2};
pub use character::*;
pub use weapon::sword::freedom_sworn::BuffFreedomSworn;
pub use weapon::claymore::song_of_broken_pines::BuffSongOfBrokenPines;
pub use weapon::claymore::wolfs_gravestone::BuffWolfsGravestone;
pub use weapon::catalyst::thrilling_tales_of_dragon_slayers::BuffThrillingTalesOfDragonSlayers;
pub use weapon::bow::elegy_of_the_end::BuffElegyOfTheEnd;
pub use weapon::catalyst::hakushin_ring::BuffHakushinRing;
pub use artifact::instructor::BuffInstructor4;
pub use artifact::noblesse_oblige::BuffNoblesseOblige4;
pub use artifact::archaic_petra::BuffArchaicPetra4;
pub use artifact::viridescent_venerer::BuffViridescentVenerer4;
pub use artifact::tenacity_of_the_millelith::BuffTenacityOfTheMillelith4;

use crate::buffs::buff_name::BuffName;

pub mod common;
pub mod character;
pub mod weapon;
pub mod resonance;
pub mod artifact;

pub fn get_buff<T: Attribute>(name: &BuffName, b: &BuffConfig) -> Box<dyn Buff<T>> {
    name.create(b)
}
