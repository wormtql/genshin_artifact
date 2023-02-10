use askama::Template;
use mona::potential_function::potential_function::PotentialFunctionMetaData;
use mona::potential_function::potential_function_name::PotentialFunctionName;
use crate::utils::config_to_json;

struct PFMeta {
    name: String,
    chs: String,
    description: String,
    image: String,
    config: Vec<String>,
}

#[derive(Template)]
#[template(path = "pf_meta_template.js")]
struct PFMetaTemplate {
    pfs: Vec<PFMeta>
}

pub fn gen_pf_meta_as_js_file() -> String {
    let mut data: Vec<PFMeta> = Vec::new();
    for i in 0_usize..PotentialFunctionName::LEN {
        let e: PotentialFunctionName = num::FromPrimitive::from_usize(i).unwrap();

        let meta: PotentialFunctionMetaData = e.get_meta();
        let config = if let Some(x) = e.get_config() {
            x.iter().map(|c| config_to_json(&c)).collect()
        } else {
            Vec::new()
        };

        data.push(PFMeta {
            name: meta.name.to_string(),
            chs: String::from(meta.chs),
            description: String::from(meta.description),
            image: String::from(meta.image),
            config
        });
    }

    let t = PFMetaTemplate {
        pfs: data,
    };

    t.render().unwrap()
}
