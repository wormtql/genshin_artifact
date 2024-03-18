#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(clippy::approx_constant)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_must_use)]
#![allow(noop_method_call)]

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use clap::{Parser};

use mona_generate::gen_meta::gen_artifact_meta::gen_artifact_meta_as_js_file;
use mona_generate::gen_meta::gen_buff_meta::gen_buff_meta_as_js_file;
use mona_generate::gen_meta::gen_character_meta::gen_character_meta_as_js_file;
use mona_generate::gen_meta::gen_locale::generate_locale_vec;
use mona_generate::gen_meta::gen_pf_meta::gen_pf_meta_as_js_file;
use mona_generate::gen_meta::gen_tf_meta::gen_tf_meta_as_js_file;
use mona_generate::gen_meta::gen_weapon_meta::gen_weapon_meta_as_js_file;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    dir: String,
    #[clap(long)]
    i18n_dir: String,
}

fn main() {
    let args: Args = Args::parse();

    let weapon_file = Path::new(&args.dir).join("_gen_weapon.js");
    let artifact_file = Path::new(&args.dir).join("_gen_artifact.js");
    let buff_file = Path::new(&args.dir).join("_gen_buff.js");
    let character_file = Path::new(&args.dir).join("_gen_character.js");
    let pf_file = Path::new(&args.dir).join("_gen_pf.js");
    let tf_file = Path::new(&args.dir).join("_gen_tf.js");

    let get_file = |path: &PathBuf| {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect(&format!("cannot open/create file `{:?}`", path))
    };

    let mut file_weapon = OpenOptions::new().write(true).create(true).truncate(true).open(&weapon_file).expect("cannot open/create weapon file");
    file_weapon.write_all(gen_weapon_meta_as_js_file().as_bytes());

    let mut file_artifact = OpenOptions::new().write(true).create(true).truncate(true).open(&artifact_file).expect("cannot open/create artifact file");
    file_artifact.write_all(gen_artifact_meta_as_js_file().as_bytes());

    let mut file_buff = OpenOptions::new().write(true).create(true).truncate(true).open(&buff_file).expect("cannot open/create buff file");
    file_buff.write_all(gen_buff_meta_as_js_file().as_bytes());

    let mut file_character = OpenOptions::new().write(true).create(true).truncate(true).open(&character_file).expect("cannot open/create character file");
    file_character.write_all(gen_character_meta_as_js_file().as_bytes());

    let mut file_pf = OpenOptions::new().write(true).create(true).truncate(true).open(&pf_file).expect("cannot open/create pf file");
    file_pf.write_all(gen_pf_meta_as_js_file().as_bytes());

    let mut file_tf = OpenOptions::new().write(true).create(true).truncate(true).open(&tf_file).expect("cannot open/create tf file");
    file_tf.write_all(gen_tf_meta_as_js_file().as_bytes());


    for loc in ["zh-cn", "en"] {
        let mut file = get_file(&Path::new(&args.i18n_dir).join(format!("{}.json", loc)));
        let json = serde_json::to_string_pretty(&generate_locale_vec(loc)).unwrap();
        file.write_all(json.as_bytes());
    }
}
