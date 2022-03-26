use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use mona::gen_meta::gen_artifact_meta::gen_artifact_meta_as_js_file;
use mona::gen_meta::gen_buff_meta::gen_buff_meta_as_js_file;
use mona::gen_meta::gen_character_meta::gen_character_meta_as_js_file;
use mona::gen_meta::gen_pf_meta::gen_pf_meta_as_js_file;
use mona::gen_meta::gen_tf_meta::gen_tf_meta_as_js_file;
use mona::gen_meta::gen_weapon_meta::gen_weapon_meta_as_js_file;


fn main() {
    let mut file_weapon = OpenOptions::new().write(true).create(true).open("../src/assets/_gen_weapon.js").expect("cannot open/create weapon file");
    file_weapon.write_all(gen_weapon_meta_as_js_file().as_bytes());

    let mut file_artifact = OpenOptions::new().write(true).create(true).open("../src/assets/_gen_artifact.js").expect("cannot open/create artifact file");
    file_artifact.write_all(gen_artifact_meta_as_js_file().as_bytes());

    let mut file_buff = OpenOptions::new().write(true).create(true).open("../src/assets/_gen_buff.js").expect("cannot open/create buff file");
    file_buff.write_all(gen_buff_meta_as_js_file().as_bytes());

    let mut file_character = OpenOptions::new().write(true).create(true).open("../src/assets/_gen_character.js").expect("cannot open/create character file");
    file_character.write_all(gen_character_meta_as_js_file().as_bytes());

    let mut file_pf = OpenOptions::new().write(true).create(true).open("../src/assets/_gen_pf.js").expect("cannot open/create pf file");
    file_pf.write_all(gen_pf_meta_as_js_file().as_bytes());

    let mut file_tf = OpenOptions::new().write(true).create(true).open("../src/assets/_gen_tf.js").expect("cannot open/create tf file");
    file_tf.write_all(gen_tf_meta_as_js_file().as_bytes());
}
