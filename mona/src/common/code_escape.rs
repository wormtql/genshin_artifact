use std::fmt::Write;
use askama_escape::Escaper;

pub struct CodeEscaper;

impl Escaper for CodeEscaper {
    fn write_escaped<W>(&self, mut fmt: W, string: &str) -> std::fmt::Result where W: Write {
        let mut x = String::new();
        for c in string.chars() {
            match c {
                '\"' => x.write_str("\\\""),
                '\'' => x.write_str("\\\'"),
                '\\' => x.write_str("\\\\"),
                _ => x.write_char(c)
            }.unwrap();
        }

        fmt.write_str(&x).unwrap();

        std::fmt::Result::Ok(())
    }
}
