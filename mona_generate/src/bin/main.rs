use mona::artifacts::ArtifactSetName;
use mona::weapon::WeaponName;
use mona_generate::gen_locale::gen_artifact::gen_artifact_locale;
use mona_generate::utils::{get_artifact_icon_names, get_internal_weapon_name};
use mona_generate::gen_locale::gen_character::gen_character_locale;

fn main() {
    let locale = gen_artifact_locale("en");
    println!("{}", locale);
}
