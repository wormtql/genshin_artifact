use mona::buffs::buff_meta::{BuffImage, BuffMetaData};
use mona::buffs::buff_name::BuffName;
use askama::Template;

struct BuffMeta {
    name: String,
    chs: String,
    image: String,
    genre: String,
    config: Vec<String>,
    description: String,
}

#[derive(Template)]
#[template(path = "buff_meta_template.js")]
struct BuffMetaTemplate {
    buffs: Vec<BuffMeta>
}

fn convert_image(i: &BuffImage) -> String {
    match *i {
        BuffImage::Avatar(x) => format!("characters/{}_avatar", x.to_string()),
        BuffImage::Custom(x) => String::from(x),
        BuffImage::Misc(x) => format!("misc/{}", x),
    }
}

fn main() {
    let mut data: Vec<BuffMeta> = Vec::new();

    for i in 0_usize..BuffName::LEN {
        let e: BuffName = num::FromPrimitive::from_usize(i).unwrap();

        let meta: BuffMetaData = e.get_meta();
        let config = if let Some(x) = e.get_config() {
            x.iter().map(|c| c.to_json()).collect()
        } else {
            Vec::new()
        };

        let description = if let Some(x) = meta.description {
            String::from(x)
        } else {
            String::new()
        };

        data.push(BuffMeta {
            name: meta.name.to_string(),
            chs: String::from(meta.chs),
            image: convert_image(&meta.image),
            genre: meta.genre.to_string(),
            config,
            description,
        })
    }

    let t = BuffMetaTemplate {
        buffs: data
    };

    println!("{}", t.render().unwrap());
}