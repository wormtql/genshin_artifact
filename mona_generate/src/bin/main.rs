use mona::artifacts::ArtifactSetName;
use mona::weapon::WeaponName;
use mona_generate::gen_locale::gen_artifact::gen_artifact_locale;
use mona_generate::utils::{get_artifact_icon_names, get_internal_weapon_name};
use mona_generate::gen_locale::gen_character::gen_character_locale;
use mona_generate::gen_locale::gen_weapon::{gen_weapon_locale, gen_weapon_effect};
use mona_generate::utils::common::{get_color_content, get_color_from_color_tag, merge_affix_string};

fn main() {
    // let locale = gen_weapon_locale("en");
    // println!("{}", locale);

    println!("{}", gen_weapon_effect("en"));
}
