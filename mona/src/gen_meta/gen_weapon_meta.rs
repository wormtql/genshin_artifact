use askama::{Template};
use crate::common::item_config_type::ItemConfig;
use crate::weapon::weapons::get_static_data;
use crate::weapon::weapon_name::WeaponName;
use crate::weapon::weapon_static_data::WeaponStaticData;

struct WeaponMetaDataForJS {
    name: String,
    chs: String,
    star: usize,
    t: String,
    effect: String,
    configs: Vec<String>,
}

#[derive(Template)]
#[template(path = "weapon_meta_template.js")]
struct WeaponMetaAllForJS {
    weapons: Vec<WeaponMetaDataForJS>,
}

pub fn gen_weapon_meta_as_js_file() -> String {
    let mut data: Vec<WeaponMetaDataForJS> = Vec::new();

    for i in 0_usize..WeaponName::LEN {
        let weapon_name: WeaponName = num::FromPrimitive::from_usize(i).unwrap();

        let meta_data: WeaponStaticData = weapon_name.get_static_data();
        let config_data: Option<&[ItemConfig]> = weapon_name.get_config_data();
        let mut configs: Vec<String> = Vec::new();

        if let Some(x) = config_data {
            for config in x.iter() {
                configs.push(config.to_json());
            }
        }

        let my_data = WeaponMetaDataForJS {
            name: weapon_name.to_string(),
            chs: String::from(meta_data.chs),
            star: meta_data.star,
            t: meta_data.weapon_type.to_string(),
            effect: if let Some(s) = meta_data.effect {
                String::from(s)
            } else {
                String::new()
            },
            configs,
        };
        data.push(my_data);
    }

    let t = WeaponMetaAllForJS {
        weapons: data
    };

    t.render().unwrap()
}
