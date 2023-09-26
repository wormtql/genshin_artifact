use serde_json::json;
use crate::common::{Element, SkillType};
use crate::common::i18n::{I18nLocale, locale};

pub enum ItemConfigType {
    Float {
        min: f64,
        max: f64,
        default: f64,
    },
    Int {
        min: i32,
        max: i32,
        default: i32
    },
    IntInput {
        min: i32,
        max: i32,
        default: i32
    },
    Bool {
        default: bool
    },
    Option {
        options: &'static str, // comma separated
        default: usize
    },
    // NullOrValueInput {
    //     min: f64,
    //     max: f64,
    //     default: f64,
    // },
    FloatPercentageInput {
        default: f64,
    },
    FloatInput {
        default: f64,
    },
    Element4 {      // cryo, pyro, electro, hydro
        default: Element
    },
    Element8 {
        default: Element
    },
    Skill4 {
        default: SkillType
    }
}

pub struct ItemConfig {
    pub title: I18nLocale,
    pub name: &'static str,
    pub config: ItemConfigType,
}

impl ItemConfigType {
    pub fn to_json(&self, title: &str, name: &str) -> String {
        let j = match *self {
            ItemConfigType::Skill4 { default } => {
                json!({
                    "type": "skill4",
                    "title": title,
                    "name": name,
                    "default": default
                })
            },
            ItemConfigType::IntInput { min, max, default } => {
                json!({
                    "type": "intInput",
                    "title": title,
                    "name": name,
                    "min": min,
                    "max": max,
                    "default": default
                })
            },
            ItemConfigType::Element4 { default } => {
                json!({
                    "type": "element4",
                    "title": title,
                    "name": name,
                    "default": default
                })
            },
            ItemConfigType::Element8 { default } => {
                json!({
                    "type": "element8",
                    "title": title,
                    "name": name,
                    "default": default
                })
            },
            ItemConfigType::FloatPercentageInput { default } => {
                json!({
                    "type": "floatPercentageInput",
                    "title": title,
                    "name": name,
                    "default": default
                })
            },
            ItemConfigType::FloatInput { default } => {
                json!({
                    "type": "floatInput",
                    "title": title,
                    "name": name,
                    "default": default
                })
            }
            ItemConfigType::Float { min, max, default } => {
                json!({
                    "type": "float",
                    "title": title,
                    "name": name,
                    "min": min,
                    "max": max,
                    "default": default
                })
            },
            ItemConfigType::Int { min, max, default } => {
                json!({
                    "type": "int",
                    "title": title,
                    "name": name,
                    "min": min,
                    "max": max,
                    "default": default
                })
            },
            ItemConfigType::Bool { default } => {
                json!({
                    "type": "bool",
                    "title": title,
                    "name": name,
                    "default": default
                })
            },
            ItemConfigType::Option { options, default } => {
                let temp: Vec<&str> = options.split(",").collect();
                json!({
                    "type": "option",
                    "title": title,
                    "name": name,
                    "default": default,
                    "options": temp
                })
            }
        };

        j.to_string()
    }
}

impl ItemConfig {
    pub const DEFAULT_RATE_TITLE: I18nLocale = locale!(zh_cn: "被动应用比例", en: "Avg Effect Ratio");
    pub const DEFAULT_STACK_TITLE: I18nLocale = locale!(zh_cn: "被动等效层数", en: "Avg Effect Stack");
    pub const DEFAULT_RECHARGE_TITLE: I18nLocale = locale!(zh_cn: "充能需求", en: "Recharge demand");
    pub const DEFAULT_BUFF_TITLE: I18nLocale = locale!(zh_cn: "数值", en: "Number");

    pub const RATE01_TYPE: ItemConfigType = ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 };
    pub const RATE01: ItemConfig = ItemConfig { name: "rate", title: Self::DEFAULT_RATE_TITLE, config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 } };
    pub const STACK02: ItemConfig = ItemConfig { name: "stack", title: Self::DEFAULT_STACK_TITLE, config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 } };
    pub const STACK03: ItemConfig = ItemConfig { name: "stack", title: Self::DEFAULT_STACK_TITLE, config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 3.0 } };
    pub const STACK04: ItemConfig = ItemConfig { name: "stack", title: Self::DEFAULT_STACK_TITLE, config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 } };
    pub const STACK05: ItemConfig = ItemConfig { name: "stack", title: Self::DEFAULT_STACK_TITLE, config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 0.0 } };
    pub const BUFFV1P: ItemConfig = ItemConfig { name: "p", title: Self::DEFAULT_BUFF_TITLE, config: ItemConfigType::FloatPercentageInput { default: 0.0 } };
    pub const BUFFV1: ItemConfig = ItemConfig { name: "value", title: Self::DEFAULT_BUFF_TITLE, config: ItemConfigType::FloatInput { default: 0.0 } };
    pub const REFINE: ItemConfig = ItemConfig { name: "refine", title: locale!(zh_cn: "精炼", en: "Refine"), config: ItemConfigType::IntInput { min: 1, max: 5, default: 1 } };
}
