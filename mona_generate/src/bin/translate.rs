use mona_generate::utils::text_map::{translate, translate2};

fn main() {
    loop {
        let mut s = String::new();

        std::io::stdin().read_line(&mut s).unwrap();

        match translate2(&s.trim(), "chs", "en") {
            Some((a, b)) => println!("{} -> {}", a, b),
            None => (),
        }
    }
}