use mona::artifacts::ArtifactSetName;
use mona::weapon::WeaponName;
use mona_generate::gen_locale::gen_artifact::gen_artifact_locale;
use mona_generate::utils::{get_artifact_icon_names, get_internal_weapon_name};
use mona_generate::gen_locale::gen_character::{gen_character_locale, gen_character_skill_names_locale};
use mona_generate::gen_locale::gen_weapon::{gen_weapon_locale, gen_weapon_effect};
use mona_generate::gen_meta::gen_character_meta::gen_character_meta_as_js_file;
use mona_generate::utils::character::get_character_dmg_names_chs;
use mona_generate::utils::common::{get_color_content, get_color_from_color_tag, merge_affix_string};
use mona_generate::utils::text_map::translate;

fn main() {
    // let locale = gen_weapon_locale("en");
    // println!("{}", locale);

    // println!("{:?}", get_character_dmg_names_chs());
    // println!("{}", serde_json::to_string(&get_character_dmg_names_chs()).unwrap());
    // println!("{}", gen_character_meta_as_js_file());

    let skill_names = get_character_dmg_names_chs();
    for item in skill_names.iter() {
        println!("{}: {}", item, translate(&item, "chs", "en"));
    }
    // println!("{}", translate("最大切割伤害", "chs", "en"));
}
