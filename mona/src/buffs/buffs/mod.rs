use crate::attribute::Attribute;
use crate::buffs::{Buff, BuffConfig};

pub use common::atk_percentage::BuffATKPercentage;
pub use common::def_percentage::BuffDEFPercentage;
pub use common::hp_percentage::BuffHPPercentage;
pub use common::atk_fixed::BuffATKFixed;
pub use common::def_fixed::BuffDEFFixed;
pub use common::hp_fixed::BuffHPFixed;
pub use common::critical::BuffCritical;
pub use common::critical_damage::BuffCriticalDamage;
pub use common::custom_bonus::BuffCustomBonus;
pub use common::elemental_mastery::BuffElementalMastery;
pub use common::recharge::BuffRecharge;
pub use resonance::{BuffResonanceGeo2, BuffResonancePyro2, BuffResonanceCryo2};
pub use character::albedo::BuffAlbedoTalent2;
pub use character::aloy::BuffAloyTalent1;
pub use character::arataki_itto::BuffAratakiIttoC4;
pub use character::beidou::BuffBeidouC6;
pub use character::bennett::{BuffBennettQ, BuffBennettC6};
pub use character::chongyun::BuffChongyunTalent2;
pub use character::diona::BuffDionaC6G50;
pub use character::ganyu::{BuffGanyuTalent2, BuffGanyuC1};
pub use character::gorou::{BuffGorouE1, BuffGorouE3, BuffGorouTalent1, BuffGorouC6};
pub use character::hu_tao::BuffHuTaoTalent1;
pub use character::jean::BuffJeanC4;
pub use character::kaedehara_kazuha::{BuffKaedeharaKazuhaC2, BuffKaedeharaKazuhaTalent2};
pub use character::kamisato_ayaka::BuffKamisatoAyakaC4;
pub use character::klee::{BuffKleeC2, BuffKleeC6};
pub use character::kujou_sara::{BuffKujouSaraEOrQ};
pub use character::lisa::BuffLisaTalent2;
pub use character::mona::{BuffMonaC1, BuffMonaQ};
pub use character::ningguang::BuffNingguangTalent2;
pub use character::raiden_shogun::{BuffRaidenShogunE, BuffRaidenShogunC4};
pub use character::razor::BuffRazorC4;
pub use character::rosaria::{BuffRosariaC6, BuffRosariaTalent2};
pub use character::shenhe::{BuffShenheE, BuffShenheQ, BuffShenheTalent1, BuffShenheTalent2};
pub use character::sucrose::{BuffSucroseC6, BuffSucroseTalent1, BuffSucroseTalent2};
pub use character::thoma::{BuffThomaC6, BuffThomaTalent1};
pub use character::venti::{BuffVentiC6, BuffVentiC2};
pub use character::xiangling::{BuffXianglingC6, BuffXianglingTalent2, BuffXianglingC1};
pub use character::xingqiu::BuffXingqiuC2;
pub use character::xinyan::{BuffXinyanC4, BuffXinyanTalent2};
pub use character::yoimiya::BuffYoimiyaTalent2;
pub use character::yunjin::{BuffYunjinC2, BuffYunjinQ};
pub use character::zhongli::BuffZhongliShield;
pub use weapon::sword::freedom_sworn::BuffFreedomSworn;
pub use weapon::claymore::song_of_broken_pines::BuffSongOfBrokenPines;
pub use weapon::claymore::wolfs_gravestone::BuffWolfsGravestone;
pub use weapon::catalyst::thrilling_tales_of_dragon_slayers::BuffThrillingTalesOfDragonSlayers;
pub use weapon::bow::elegy_of_the_end::BuffElegyOfTheEnd;
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
