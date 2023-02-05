use mona_generate::gen_meta::gen_locale::collect_locale;

fn main() {
    let locales = collect_locale();
    println!("{:?}", locales);
}
