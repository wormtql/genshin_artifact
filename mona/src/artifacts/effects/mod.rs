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
pub use viridescent_venerer::ViridescentVenerer;
pub use wanderers_troupe::WanderersTroupe;

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

pub fn get_effect<T: Attribute>(name: ArtifactSetName, config: &ArtifactEffectConfig, character: &Character<T>) -> Box<dyn ArtifactEffect<T>> {
    name.create_effect(config, &character.common_data)
    // match name {
    //     ArtifactSetName::Adventurer => Box::new(AdventurerEffect::new()),
    //     ArtifactSetName::ArchaicPetra => Box::new(ArchaicPetraEffect::new(config)),
    //     ArtifactSetName::Berserker => Box::new(BerserkerEffect::new(config)),
    //     ArtifactSetName::BlizzardStrayer => Box::new(BlizzardStrayerEffect::new(config)),
    //     ArtifactSetName::BloodstainedChivalry => Box::new(BloodstainedChivalryEffect::new(config)),
    //     ArtifactSetName::BraveHeart => Box::new(BraveHeartEffect::new(config)),
    //     ArtifactSetName::CrimsonWitchOfFlames => Box::new(CrimsonWitchOfFlamesEffect::new(config)),
    //     ArtifactSetName::DefendersWill => Box::new(DefendersWillEffect::new()),
    //     ArtifactSetName::EmblemOfSeveredFate => Box::new(EmblemOfSeveredFateEffect::new()),
    //     ArtifactSetName::Gambler => Box::new(GamblerEffect::new()),
    //     ArtifactSetName::GladiatorsFinale => Box::new(GladiatorsFinaleEffect::new(character)),
    //     ArtifactSetName::HeartOfDepth => Box::new(HeartOfDepthEffect::new(config)),
    //     ArtifactSetName::HuskOfOpulentDreams => Box::new(HuskOfOpulentDreamsEffect::new(config)),
    //     ArtifactSetName::Instructor => Box::new(InstructorEffect::new(config)),
    //     ArtifactSetName::Lavawalker => Box::new(LavawalkerEffect::new(config)),
    //     ArtifactSetName::LuckyDog => Box::new(LuckyDogEffect::new()),
    //     ArtifactSetName::MaidenBeloved => Box::new(MaidenBelovedEffect::new()),
    //     ArtifactSetName::MartialArtist => Box::new(MartialArtistEffect::new(config)),
    //     ArtifactSetName::NoblesseOblige => Box::new(NoblesseObligeEffect::new(config)),
    //     ArtifactSetName::OceanHuedClam => Box::new(OceanHuedClamEffect::new()),
    //     ArtifactSetName::PaleFlame => Box::new(PaleFlameEffect::new(config)),
    //     ArtifactSetName::ResolutionOfSojourner => Box::new(ResolutionOfSojournerEffect::new()),
    //     ArtifactSetName::RetracingBolide => Box::new(RetracingBolideEffect::new(config)),
    //     ArtifactSetName::Scholar => Box::new(ScholarEffect::new()),
    //     ArtifactSetName::ShimenawasReminiscence => Box::new(ShimenawasReminiscenceEffect::new(config)),
    //     ArtifactSetName::TenacityOfTheMillelith => Box::new(TenacityOfTheMillelithEffect::new(config)),
    //     ArtifactSetName::TheExile => Box::new(TheExileEffect::new()),
    //     ArtifactSetName::ThunderingFury => Box::new(ThunderingFuryEffect::new()),
    //     ArtifactSetName::Thundersoother => Box::new(ThundersootherEffect::new(config)),
    //     ArtifactSetName::ViridescentVenerer => Box::new(ViridescentVenererEffect::new()),
    //     ArtifactSetName::WanderersTroupe => Box::new(WanderersTroupeEffect::new(character.common_data.static_data.weapon_type)),
    //     _ => Box::new(NoEffect {}),
    // }
}
