use askama::Template;
use mona::artifacts::artifact_trait::ArtifactMetaData;
use mona::artifacts::ArtifactSetName;
use mona::common::item_config_type::ItemConfig;
use crate::utils::get_artifact_icon_names;

struct ArtifactMeta {
    chs: String,
    name: String,
    name_mona: String,
    min_star: usize,
    max_star: usize,
    effect1: String,
    effect2: String,
    effect3: String,
    effect4: String,
    effect5: String,
    config4: Vec<String>,

    flower: String,
    feather: String,
    sand: String,
    goblet: String,
    head: String,
    
    flower_icon: String,
    feather_icon: String,
    sand_icon: String,
    goblet_icon: String,
    head_icon: String,
}

#[derive(Template)]
#[template(path = "artifact_meta_template.js")]
struct ArtifactMetaTemplate {
    artifacts: Vec<ArtifactMeta>
}

pub fn gen_artifact_meta_as_js_file() -> String {
    let mut data = Vec::new();

    for i in 1_usize..ArtifactSetName::LEN {
        let e: ArtifactSetName = num::FromPrimitive::from_usize(i).unwrap();
        let meta: ArtifactMetaData = e.get_meta();
        let config4: Option<&'static [ItemConfig]> = e.get_config4();
        
        let icons = get_artifact_icon_names(e);

        data.push(ArtifactMeta {
            chs: String::from(meta.chs),
            name: meta.name.to_string(),
            name_mona: String::from(meta.name_mona),
            min_star: meta.star.0,
            max_star: meta.star.1,
            effect1: String::from(meta.effect1.unwrap_or("")),
            effect2: String::from(meta.effect2.unwrap_or("")),
            effect3: String::from(meta.effect3.unwrap_or("")),
            effect4: String::from(meta.effect4.unwrap_or("")),
            effect5: String::from(meta.effect5.unwrap_or("")),
            config4: config4.unwrap_or(&[]).iter().map(|x| x.to_json()).collect(),
            flower: String::from(meta.flower.unwrap_or("")),
            feather: String::from(meta.feather.unwrap_or("")),
            sand: String::from(meta.sand.unwrap_or("")),
            goblet: String::from(meta.goblet.unwrap_or("")),
            head: String::from(meta.head.unwrap_or("")),
            flower_icon: icons[0].clone().unwrap_or(String::new()),
            feather_icon: icons[1].clone().unwrap_or(String::new()),
            sand_icon: icons[2].clone().unwrap_or(String::new()),
            goblet_icon: icons[3].clone().unwrap_or(String::new()),
            head_icon: icons[4].clone().unwrap_or(String::new())
        })
    }

    let t = ArtifactMetaTemplate {
        artifacts: data
    };

    t.render().unwrap()
}
