use std::string::ToString;
use strum_macros::Display;
use num_derive::FromPrimitive;
use serde::{Serialize, Deserialize};
use mona_derive::{WeaponData, EnumLen};
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::common::item_config_type::ItemConfig;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[derive(FromPrimitive, Display)]
#[derive(WeaponData, EnumLen)]
pub enum WeaponName {
    MistsplitterReforged,
    AquilaFavonia,
    SummitShaper,
    SkywardBlade,
    FreedomSworn,
    PrimordialJadeCutter,
    TheFlute,
    TheBlackSword,
    TheAlleyFlash,
    SwordOfDescension,
    SacrificialSword,
    RoyalLongsword,
    PrototypeRancour,
    AmenomaKageuchi,
    LionsRoar,
    IronSting,
    FesteringDesire,
    FavoniusSword,
    CinnabarSpindle,
    BlackcliffLongsword,
    // HarbingerOfDawn,     // todo below *3 weapon
    // FilletBlade,
    // SkyriderSword,
    // DarkIronSword,
    // CoolSteel,
    // TravelersHandySword,
    // SilverSword,
    // DullBlade,

    WolfsGravestone,
    SkywardPride,
    TheUnforged,
    SongOfBrokenPines,
    RedhornStonethresher,
    Akuoumaru,
    RoyalGreatsword,
    Whiteblind,
    TheBell,
    SnowTombedStarsilver,
    FavoniusGreatsword,
    KatsuragikiriNagamasa,
    SacrificialGreatsword,
    SerpentSpine,
    BlackcliffSlasher,
    Rainslasher,
    PrototypeArchaic,
    LuxuriousSeaLord,
    LithicBlade,
    // SkyriderGreatsword,     // todo below *3
    // DebateClub,
    // BloodstainedGreatsword,
    // WhiteIronGreatsword,
    // FerrousShadow,
    // OldMercsPal,
    // WasterGreatsword,

    EngulfingLightning,
    SkywardSpine,
    PrimordialJadeWingedSpear,
    CalamityQueller,
    StaffOfHoma,
    VortexVanquisher,
    PrototypeStarglitter,
    LithicSpear,
    KitainCrossSpear,
    TheCatch,
    FavoniusLance,
    DragonspineSpear,
    DragonsBane,
    Deathmatch,
    CrescentPike,
    BlackcliffPole,
    WavebreakersFin,
    RoyalSpear,
    // Halberd,        // todo 3*
    // BlackTassel,
    // WhiteTassel,
    // IronPoint,
    // BeginnersProtector,

    LostPrayerToTheSacredWinds,
    SkywardAtlas,
    EverlastingMoonglow,
    MemoryOfDust,
    WindAndSong,
    TheWidsith,
    SolarPearl,
    SacrificialFragments,
    RoyalGrimoire,
    PrototypeAmber,
    MappaMare,
    HakushinRing,
    Frostbearer,
    FavoniusCodex,
    EyeOfPerception,
    DodocoTales,
    BlackcliffAgate,
    // MagicGuide,         // todo 3*
    // OtherworldlyStory,
    // EmeraldOrb,
    // ThrillingTalesOfDragonSlayers,
    // TwinNephrite,
    // PocketGrimoire,
    // ApprenticesNotes,

    PolarStar,
    ThunderingPulse,
    ElegyOfTheEnd,
    SkywardHarp,
    AmosBow,
    AlleyHunter,
    TheViridescentHunt,
    TheStringless,
    SacrificialBow,
    Rust,
    RoyalBow,
    Predator,
    PrototypeCrescent,
    MouunsMoon,
    MitternachtsWaltz,
    Hamayumi,
    FavoniusWarbow,
    CompoundBow,
    BlackcliffWarbow,
    WindblumeOde,
    // RavenBow,       // todo 3*
    // RecurveBow,
    // Messenger,
    // SharpshootersOath,
    // Slingshot,
    // SeasonedHuntersBow,
    // HuntersBow,
}