macro_rules! skill_type {
    ($name:tt $($fields:tt)*) => {
        pub struct $name {
            $(pub $fields: [f64; 15],)*
        }
    }
}

macro_rules! damage_enum {
    ($name:tt $($items:tt)*) => {
        use strum_macros::{EnumCount as EnumCountMacro, EnumString};
        use num_derive::FromPrimitive;
        use strum::EnumCount;
        #[derive(Copy, Clone, Eq, PartialEq, EnumString)]
        #[derive(FromPrimitive, EnumCountMacro)]
        pub enum $name {
            $($items,)*
        }

        impl Into<usize> for $name {
            fn into(self) -> usize {
                self as usize
            }
        }
    }
}

macro_rules! skill_map {
    ($damage_enum:tt $($name:tt $text:expr)*) => {
        Some(&[
            $(CharacterSkillMapItem { index: $damage_enum::$name as usize, text: $text },)*
            // $(CharacterSkillMapItem { index: concat_idents!($damage_name, ::, $name) as usize, chs: $chs },)*
        ])
    }
}

macro_rules! damage_ratio {
    ($damage_const:ident $match_item:ident $($damage_enum:ident $damage_name:ident $skill:ident)*) => {
        match $match_item {
            $($damage_enum => $damage_const.$damage_name[$skill],)*
            _ => 0.0
        }
    }
}

pub(crate) use skill_type;
pub(crate) use damage_enum;
pub(crate) use skill_map;
pub(crate) use damage_ratio;