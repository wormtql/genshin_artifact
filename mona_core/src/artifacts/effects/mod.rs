use super::artifact::{ArtifactSetName};
use super::effect::ArtifactEffect;
use super::effect_config::ArtifactEffectConfig;
use crate::character::Character;
use crate::attribute::Attribute;

pub use empty::Empty;
pub use adventurer::Adventurer;
pub use archaic_petra::ArchaicPetra;
pub use berserker::Berserker;
pub use blizzard_strayer::BlizzardStrayer;
pub use bloodstained_chivalry::BloodstainedChivalry;
pub use brave_heart::BraveHeart;
pub use crimson_witch_of_flames::CrimsonWitchOfFlames;
pub use defenders_will::DefendersWill;
pub use echoes_of_an_offering::EchoesOfAnOffering;
pub use emblem_of_severed_fate::EmblemOfSeveredFate;
pub use gambler::Gambler;
pub use gladiators_finale::GladiatorsFinale;
pub use heart_of_depth::HeartOfDepth;
pub use husk_of_opulent_dreams::HuskOfOpulentDreams;
pub use instructor::Instructor;
pub use lavawalker::Lavawalker;
pub use lucky_dog::LuckyDog;
pub use maiden_beloved::MaidenBeloved;
pub use martial_artist::MartialArtist;
pub use noblesse_oblige::NoblesseOblige;
pub use ocean_hued_clam::OceanHuedClam;
pub use pale_flame::PaleFlame;
pub use prayers_for_destiny::PrayersForDestiny;
pub use prayers_for_illumination::PrayersForIllumination;
pub use prayers_for_wisdom::PrayersForWisdom;
pub use prayers_to_springtime::PrayersToSpringtime;
pub use resolution_of_sojourner::ResolutionOfSojourner;
pub use retracing_bolide::RetracingBolide;
pub use scholar::Scholar;
pub use shimenawas_reminiscence::ShimenawasReminiscence;
pub use tenacity_of_the_millelith::TenacityOfTheMillelith;
pub use the_exile::TheExile;
pub use thundering_fury::ThunderingFury;
pub use thundersoother::Thundersoother;
pub use tiny_miracle::TinyMiracle;
pub use traveling_doctor::TravelingDoctor;
pub use vermillion_hereafter::VermillionHereafter;
pub use viridescent_venerer::ViridescentVenerer;
pub use wanderers_troupe::WanderersTroupe;
pub use deepwood_memories::DeepwoodMemories;
pub use gilded_dreams::GildedDreams;

pub mod empty;
pub mod adventurer;
pub mod archaic_petra;
pub mod berserker;
pub mod blizzard_strayer;
pub mod bloodstained_chivalry;
pub mod brave_heart;
pub mod crimson_witch_of_flames;
pub mod defenders_will;
pub mod emblem_of_severed_fate;
pub mod gambler;
pub mod gladiators_finale;
pub mod heart_of_depth;
pub mod husk_of_opulent_dreams;
pub mod instructor;
pub mod lavawalker;
pub mod lucky_dog;
pub mod maiden_beloved;
pub mod martial_artist;
pub mod noblesse_oblige;
pub mod ocean_hued_clam;
pub mod pale_flame;
pub mod prayers_for_destiny;
pub mod prayers_for_illumination;
pub mod prayers_for_wisdom;
pub mod prayers_to_springtime;
pub mod resolution_of_sojourner;
pub mod retracing_bolide;
pub mod scholar;
pub mod shimenawas_reminiscence;
pub mod tenacity_of_the_millelith;
pub mod the_exile;
pub mod thundering_fury;
pub mod thundersoother;
pub mod tiny_miracle;
pub mod traveling_doctor;
pub mod viridescent_venerer;
pub mod wanderers_troupe;
pub mod vermillion_hereafter;
pub mod echoes_of_an_offering;
pub mod deepwood_memories;
pub mod gilded_dreams;

pub fn get_effect<T: Attribute>(name: ArtifactSetName, config: &ArtifactEffectConfig, character: &Character<T>) -> Box<dyn ArtifactEffect<T>> {
    name.create_effect(config, &character.common_data)
}
