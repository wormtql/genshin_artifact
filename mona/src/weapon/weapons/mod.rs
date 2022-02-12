pub mod swords;
pub mod claymores;
pub mod polearms;
pub mod catalysts;
pub mod bows;

pub use swords::mistsplitter_reforged::MistsplitterReforged;
pub use swords::aquila_favonia::AquilaFavonia;
pub use swords::summit_shaper::SummitShaper;
pub use swords::skyward_blade::SkywardBlade;
pub use swords::freedom_sworn::FreedomSworn;
pub use swords::primordial_jade_cutter::PrimordialJadeCutter;
pub use swords::the_flute::TheFlute;
pub use swords::the_black_sword::TheBlackSword;
pub use swords::the_alley_flash::TheAlleyFlash;
pub use swords::sacrificial_sword::SacrificialSword;
pub use swords::royal_longsword::RoyalLongsword;
pub use swords::prototype_rancour::PrototypeRancour;
pub use swords::amenoma_kageuchi::AmenomaKageuchi;
pub use swords::lions_roar::LionsRoar;
pub use swords::iron_sting::IronSting;
pub use swords::festering_desire::FesteringDesire;
pub use swords::favonius_sword::FavoniusSword;
pub use swords::cinnabar_spindle::CinnabarSpindle;
pub use swords::blackcliff_longsword::BlackcliffLongsword;
pub use swords::sword_of_descension::SwordOfDescension;
pub use claymores::wolfs_gravestone::WolfsGravestone;
pub use claymores::skyward_pride::SkywardPride;
pub use claymores::the_unforged::TheUnforged;
pub use claymores::song_of_broken_pines::SongOfBrokenPines;
pub use claymores::redhorn_stonethresher::RedhornStonethresher;
pub use claymores::akuoumaru::Akuoumaru;
pub use claymores::royal_greatsword::RoyalGreatsword;
pub use claymores::whiteblind::Whiteblind;
pub use claymores::the_bell::TheBell;
pub use claymores::snow_tombed_starsilver::SnowTombedStarsilver;
pub use claymores::favonius_greatsword::FavoniusGreatsword;
pub use claymores::katsuragikiri_nagamasa::KatsuragikiriNagamasa;
pub use claymores::sacrificial_greatsword::SacrificialGreatsword;
pub use claymores::serpent_spine::SerpentSpine;
pub use claymores::blackcliff_slasher::BlackcliffSlasher;
pub use claymores::rainslasher::Rainslasher;
pub use claymores::prototype_archaic::PrototypeArchaic;
pub use claymores::luxurious_sea_lord::LuxuriousSeaLord;
pub use claymores::lithic_blade::LithicBlade;
pub use polearms::engulfing_lightning::EngulfingLightning;
pub use polearms::skyward_spine::SkywardSpine;
pub use polearms::primordial_jade_winged_spear::PrimordialJadeWingedSpear;
pub use polearms::calamity_queller::CalamityQueller;
pub use polearms::staff_of_homa::StaffOfHoma;
pub use polearms::vortex_vanquisher::VortexVanquisher;
pub use polearms::prototype_starglitter::PrototypeStarglitter;
pub use polearms::lithic_spear::LithicSpear;
pub use polearms::kitain_cross_spear::KitainCrossSpear;
pub use polearms::the_catch::TheCatch;
pub use polearms::favonius_lance::FavoniusLance;
pub use polearms::dragonspine_spear::DragonspineSpear;
pub use polearms::dragons_bane::DragonsBane;
pub use polearms::deathmatch::Deathmatch;
pub use polearms::crescent_pike::CrescentPike;
pub use polearms::blackcliff_pole::BlackcliffPole;
pub use polearms::wavebreakers_fin::WavebreakersFin;
pub use polearms::royal_spear::RoyalSpear;
pub use catalysts::lost_prayer_to_the_sacred_winds::LostPrayerToTheSacredWinds;
pub use catalysts::skyward_atlas::SkywardAtlas;
pub use catalysts::everlasting_moonglow::EverlastingMoonglow;
pub use catalysts::memory_of_dust::MemoryOfDust;
pub use catalysts::wind_and_song::WindAndSong;
pub use catalysts::the_widsith::TheWidsith;
pub use catalysts::solar_pearl::SolarPearl;
pub use catalysts::sacrificial_fragments::SacrificialFragments;
pub use catalysts::royal_grimoire::RoyalGrimoire;
pub use catalysts::prototype_amber::PrototypeAmber;
pub use catalysts::mappa_mare::MappaMare;
pub use catalysts::hakushin_ring::HakushinRing;
pub use catalysts::frostbearer::Frostbearer;
pub use catalysts::favonius_codex::FavoniusCodex;
pub use catalysts::eye_of_perception::EyeOfPerception;
pub use catalysts::dodoco_tales::DodocoTales;
pub use catalysts::blackcliff_agate::BlackcliffAgate;
pub use bows::polar_star::PolarStar;
pub use bows::thundering_pulse::ThunderingPulse;
pub use bows::elegy_of_the_end::ElegyOfTheEnd;
pub use bows::skyward_harp::SkywardHarp;
pub use bows::amos_bow::AmosBow;
pub use bows::alley_hunter::AlleyHunter;
pub use bows::the_viridescent_hunt::TheViridescentHunt;
pub use bows::the_stringless::TheStringless;
pub use bows::sacrificial_bow::SacrificialBow;
pub use bows::rust::Rust;
pub use bows::royal_bow::RoyalBow;
pub use bows::predator::Predator;
pub use bows::prototype_crescent::PrototypeCrescent;
pub use bows::mouuns_moon::MouunsMoon;
pub use bows::mitternachts_waltz::MitternachtsWaltz;
pub use bows::hamayumi::Hamayumi;
pub use bows::favonius_warbow::FavoniusWarbow;
pub use bows::compound_bow::CompoundBow;
pub use bows::blackcliff_warbow::BlackcliffWarbow;
pub use bows::windblume_ode::WindblumeOde;

use crate::attribute::Attribute;
use super::weapon_name::WeaponName;
use super::weapon_config::WeaponConfig;
use super::weapon_effect::WeaponEffect;
use crate::character::character_common_data::CharacterCommonData;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;

pub fn get_static_data(name: WeaponName) -> WeaponStaticData {
    name.get_static_data()
}

pub fn get_effect<T: Attribute>(
    name: WeaponName,
    config: &WeaponConfig,
    character_common_data: &CharacterCommonData,
) -> Option<Box<dyn WeaponEffect<T>>> {
    name.get_effect(config, character_common_data)
}
