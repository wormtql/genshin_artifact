use mona::weapon::weapon_static_data::WeaponStaticData;
use mona::weapon::WeaponName;
use crate::escape::code_escape::escape_code;
use crate::utils::text_map::get_text_map;
use crate::utils::weapon::get_weapon_excel_config_data_item;

pub fn gen_weapon_locale(lang: &str) -> String {
    let mut result = String::new();

    let text_map = get_text_map(lang);

    for i in 0..WeaponName::LEN {
        let name: WeaponName = num::FromPrimitive::from_usize(i).unwrap();
        let weapon_excel_config_data_item = get_weapon_excel_config_data_item(name);
        // let meta: WeaponStaticData = name.get_meta();
        //
        // let name = meta.
        let locale_name = text_map.get(weapon_excel_config_data_item.nameTextMapHash.to_string().as_str()).unwrap();

        result.push_str(&format!("{}: \"{}\",\n", name.to_string(), locale_name));
    }

    result
}

pub fn gen_weapon_effect(lang: &str) -> String {
    let mut result = String::new();

    let text_map = get_text_map(lang);

    for i in 0..WeaponName::LEN {
        let name: WeaponName = num::FromPrimitive::from_usize(i).unwrap();
        let weapon_excel_config_data_item = get_weapon_excel_config_data_item(name);

        let effect_string = weapon_excel_config_data_item.get_effect_string(lang);
        let effect = escape_code(&effect_string);
        let effect = effect.replace("\\\\n", "<br>");

        result.push_str(&format!("{}: \"{}\",\n", name.to_string(), effect));
    }

//     let name = WeaponName::MistsplitterReforged;
//     let config = get_weapon_excel_config_data_item(name);
//     let effect_string = config.get_effect_string(lang);
// return effect_string;
    result
}