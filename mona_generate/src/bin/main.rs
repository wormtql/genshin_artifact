use mona::artifacts::ArtifactSetName;
use mona::weapon::WeaponName;
use mona_generate::utils::{get_artifact_icon_names, get_internal_weapon_name};

fn main() {
    let name = get_artifact_icon_names(ArtifactSetName::EchoesOfAnOffering);
    println!("{:?}", name);
}
