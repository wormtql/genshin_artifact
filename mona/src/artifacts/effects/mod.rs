use super::artifact::{ArtifactSetName};
use super::effect::ArtifactEffect;
use super::effect_config::ArtifactEffectConfig;
use crate::character::Character;

pub use adventurer::AdventurerEffect;
use crate::attribute::Attribute;
pub use archaic_petra::ArchaicPetraEffect;
pub use berserker::BerserkerEffect;
pub use blizzard_strayer::BlizzardStrayerEffect;
pub use bloodstained_chivalry::BloodstainedChivalryEffect;
pub use brave_heart::BraveHeartEffect;
pub use crimson_witch_of_flames::CrimsonWitchOfFlamesEffect;
pub use defenders_will::DefendersWillEffect;
pub use emblem_of_severed_fate::EmblemOfSeveredFataEffect;
pub use gambler::GamblerEffect;
pub use gladiators_finale::GladiatorsFinaleEffect;
pub use heart_of_depth::HeartOfDepthEffect;
pub use husk_of_opulent_dreams::HuskOfOpulentDreamsEffect;
pub use instructor::InstructorEffect;
pub use lavawalker::LavawalkerEffect;
pub use lucky_dog::LuckyDogEffect;
pub use maiden_beloved::MaidenBelovedEffect;
pub use martial_artist::MartialArtistEffect;
pub use noblesse_oblige::NoblesseObligeEffect;
pub use ocean_hued_clam::OceanHuedClamEffect;
pub use pale_flame::PaleFlameEffect;
pub use resolution_of_sojourner::ResolutionOfSojournerEffect;
pub use retracing_bolide::RetracingBolideEffect;
pub use scholar::ScholarEffect;
pub use shimenawas_reminiscence::ShimenawasReminiscenceEffect;
pub use tenacity_of_the_millelith::TenacityOfTheMillelithEffect;
pub use the_exile::TheExileEffect;
pub use thundering_fury::ThunderingFuryEffect;
pub use thundersoother::ThundersootherEffect;
pub use viridescent_venerer::ViridescentVenererEffect;
pub use wanderers_troupe::WanderersTroupeEffect;

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
pub mod resolution_of_sojourner;
pub mod retracing_bolide;
pub mod scholar;
pub mod shimenawas_reminiscence;
pub mod tenacity_of_the_millelith;
pub mod the_exile;
pub mod thundering_fury;
pub mod thundersoother;
pub mod viridescent_venerer;
pub mod wanderers_troupe;

pub struct NoEffect {}

impl<T: Attribute> ArtifactEffect<T> for NoEffect {}

pub fn get_effect<T: Attribute>(name: ArtifactSetName, config: &ArtifactEffectConfig, character: &Character<T>) -> Box<dyn ArtifactEffect<T>> {
    match name {
        ArtifactSetName::Adventurer => Box::new(AdventurerEffect::new()),
        ArtifactSetName::ArchaicPetra => Box::new(ArchaicPetraEffect::new(config)),
        ArtifactSetName::Berserker => Box::new(BerserkerEffect::new(config)),
        ArtifactSetName::BlizzardStrayer => Box::new(BlizzardStrayerEffect::new(config)),
        ArtifactSetName::BloodstainedChivalry => Box::new(BloodstainedChivalryEffect::new(config)),
        ArtifactSetName::BraveHeart => Box::new(BraveHeartEffect::new(config)),
        ArtifactSetName::CrimsonWitchOfFlames => Box::new(CrimsonWitchOfFlamesEffect::new(config)),
        ArtifactSetName::DefendersWill => Box::new(DefendersWillEffect::new()),
        ArtifactSetName::EmblemOfSeveredFate => Box::new(EmblemOfSeveredFataEffect::new()),
        ArtifactSetName::Gambler => Box::new(GamblerEffect::new()),
        ArtifactSetName::GladiatorsFinale => Box::new(GladiatorsFinaleEffect::new(character)),
        ArtifactSetName::HeartOfDepth => Box::new(HeartOfDepthEffect::new(config)),
        ArtifactSetName::HuskOfOpulentDreams => Box::new(HuskOfOpulentDreamsEffect::new(config)),
        ArtifactSetName::Instructor => Box::new(InstructorEffect::new(config)),
        ArtifactSetName::Lavawalker => Box::new(LavawalkerEffect::new(config)),
        ArtifactSetName::LuckyDog => Box::new(LuckyDogEffect::new()),
        ArtifactSetName::MaidenBeloved => Box::new(MaidenBelovedEffect::new()),
        ArtifactSetName::MartialArtist => Box::new(MartialArtistEffect::new(config)),
        ArtifactSetName::NoblesseOblige => Box::new(NoblesseObligeEffect::new(config)),
        ArtifactSetName::OceanHuedClam => Box::new(OceanHuedClamEffect::new()),
        ArtifactSetName::PaleFlame => Box::new(PaleFlameEffect::new(config)),
        ArtifactSetName::ResolutionOfSojourner => Box::new(ResolutionOfSojournerEffect::new()),
        ArtifactSetName::RetracingBolide => Box::new(RetracingBolideEffect::new(config)),
        ArtifactSetName::Scholar => Box::new(ScholarEffect::new()),
        ArtifactSetName::ShimenawasReminiscence => Box::new(ShimenawasReminiscenceEffect::new(config)),
        ArtifactSetName::TenacityOfTheMillelith => Box::new(TenacityOfTheMillelithEffect::new(config)),
        ArtifactSetName::TheExile => Box::new(TheExileEffect::new()),
        ArtifactSetName::ThunderingFury => Box::new(ThunderingFuryEffect::new()),
        ArtifactSetName::Thundersoother => Box::new(ThundersootherEffect::new(config)),
        ArtifactSetName::ViridescentVenerer => Box::new(ViridescentVenererEffect::new()),
        ArtifactSetName::WanderersTroupe => Box::new(WanderersTroupeEffect::new(character.common_data.static_data.weapon_type)),
        _ => Box::new(NoEffect {}),
    }
}
