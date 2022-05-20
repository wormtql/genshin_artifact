use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use clap::{Parser};

use mona_generate::gen_meta::gen_artifact_meta::gen_artifact_meta_as_js_file;
use mona_generate::gen_meta::gen_buff_meta::gen_buff_meta_as_js_file;
use mona_generate::gen_meta::gen_character_meta::gen_character_meta_as_js_file;
use mona_generate::gen_meta::gen_pf_meta::gen_pf_meta_as_js_file;
use mona_generate::gen_meta::gen_tf_meta::gen_tf_meta_as_js_file;
use mona_generate::gen_meta::gen_weapon_meta::gen_weapon_meta_as_js_file;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    dir: String
}

fn main() {
    let args: Args = Args::parse();

    let weapon_file = Path::new(&args.dir).join("_gen_weapon.js");
    let artifact_file = Path::new(&args.dir).join("_gen_artifact.js");
    let buff_file = Path::new(&args.dir).join("_gen_buff.js");
    let character_file = Path::new(&args.dir).join("_gen_character.js");
    let pf_file = Path::new(&args.dir).join("_gen_pf.js");
    let tf_file = Path::new(&args.dir).join("_gen_tf.js");

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
}
