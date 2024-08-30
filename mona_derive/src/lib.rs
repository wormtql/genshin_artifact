#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(clippy::approx_constant)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_must_use)]
#![allow(noop_method_call)]

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;
use crate::utils::get_enum_variants;

mod utils;

#[proc_macro_derive(PotentialFunctionData)]
pub fn derive_potential_function_data(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut vars = get_enum_variants(&ast);

    let mut rows_create = String::new();
    let mut rows_meta = String::new();
    let mut rows_config = String::new();

    for v in vars.iter() {
        rows_create.push_str(&format!("PotentialFunctionName::{n} => crate::potential_function::potential_functions::PotentialFunction{n}::create(config),\n", n=v));
        rows_meta.push_str(&format!("PotentialFunctionName::{n} => crate::potential_function::potential_functions::PotentialFunction{n}::META,\n", n=v));
        rows_config.push_str(&format!("PotentialFunctionName::{n} => crate::potential_function::potential_functions::PotentialFunction{n}::CONFIG,\n", n=v));
    }

    let output = format!(
        r#"
        use crate::potential_function::potential_function::{{PotentialFunction, PotentialFunctionMeta, PotentialFunctionMetaData}};
        use crate::potential_function::potential_function_config::PotentialFunctionConfig;
        use crate::common::item_config_type::ItemConfig;
        impl PotentialFunctionName {{
            pub fn create(&self, config: &PotentialFunctionConfig) -> Box<dyn PotentialFunction> {{
                match *self {{
                    {rows_create}
                }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_meta(&self) -> PotentialFunctionMetaData {{
                match *self {{
                    {rows_meta}
                }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_config(&self) -> Option<&'static [ItemConfig]> {{
                match *self {{
                    {rows_config}
                }}
            }}
        }}
        "#,
        rows_create=rows_create,
        rows_meta=rows_meta,
        rows_config=rows_config
    );

    output.parse().unwrap()
}

#[proc_macro_derive(TeamTargetFunctionData)]
pub fn derive_team_target_function_data(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut vars = get_enum_variants(&ast);

    let mut rows_members = String::new();
    let mut rows_get_buff = String::new();
    let mut rows_create = String::new();
    let mut rows_get_meta = String::new();
    for v in vars.iter() {
        rows_members.push_str(&format!("TeamName::{n} => crate::team_target::team_targets::TeamTarget{n}::MEMBERS,\n", n=v));
        rows_get_buff.push_str(&format!("TeamName::{n} => crate::team_target::team_targets::TeamTarget{n}::get_default_buffs(team),\n", n=v));
        rows_create.push_str(&format!("TeamName::{n} => crate::team_target::team_targets::TeamTarget{n}::create(config, team),\n", n=v));
        rows_get_meta.push_str(&format!("TeamName::{n} => crate::team_target::team_targets::TeamTarget{n}::META,\n", n=v));
    }

    let output = format!(
        r#"
        use std::collections::HashMap;
        use crate::character::CharacterName;
        use crate::attribute::Attribute;
        use crate::team::team::Team;
        use crate::buffs::Buff;
        use crate::team_target::team_target_config::TeamTargetFunctionConfig;
        use crate::target_functions::TargetFunction;
        use crate::team_target::team_target_function::{{TeamTargetFunction, TeamTargetFunctionMetaData, TeamTargetFunctionMetaTrait}};
        impl TeamName {{
            pub fn get_members(&self) -> &'static [CharacterName] {{
                match *self {{
                    {rows_members}
                }}
            }}

            pub fn get_default_buffs<A: Attribute>(&self, team: &Team<A>) -> HashMap<usize, Vec<Box<dyn Buff<A>>>> {{
                match *self {{
                    {rows_get_buff}
                }}
            }}

            pub fn create<A: Attribute>(&self, config: &TeamTargetFunctionConfig, team: &Team<A>) -> Box<dyn TeamTargetFunction> {{
                match *self {{
                    {rows_create}
                }}
            }}

            pub fn get_meta(&self) -> TeamTargetFunctionMetaData {{
                match *self {{
                    {rows_meta_data}
                }}
            }}
        }}
        "#,
        rows_members=rows_members,
        rows_get_buff=rows_get_buff,
        rows_create=rows_create,
        rows_meta_data=rows_get_meta,
    );

    output.parse().unwrap()
}

#[proc_macro_derive(ArtifactData)]
pub fn derive_artifact_data(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut vars = get_enum_variants(&ast);

    let mut rows_effect = String::new();
    let mut rows_meta = String::new();
    let mut rows_config4 = String::new();
    let mut rows_config2 = String::new();
    for v in vars.iter() {
        rows_effect.push_str(&format!("ArtifactSetName::{n} => crate::artifacts::effects::{n}::create_effect(config, common),\n", n=v));
        rows_meta.push_str(&format!("ArtifactSetName::{n} => crate::artifacts::effects::{n}::META_DATA,\n", n=v));
        rows_config4.push_str(&format!("ArtifactSetName::{n} => crate::artifacts::effects::{n}::CONFIG4,\n", n=v));
        rows_config2.push_str(&format!("ArtifactSetName::{n} => crate::artifacts::effects::{n}::CONFIG2,\n", n=v))
    }

    let output = format!(
        r#"
        use crate::artifacts::artifact_trait::ArtifactTrait;
        use crate::character::character_common_data::CharacterCommonData;
        use crate::artifacts::artifact_trait::{{ArtifactMetaData}};
        use crate::common::item_config_type::ItemConfig;
        impl ArtifactSetName {{
            pub fn create_effect<A: Attribute>(&self, config: &ArtifactEffectConfig, common: &CharacterCommonData) ->Box<dyn ArtifactEffect<A>> {{
                match *self {{
                    {rows_effect}
                }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_meta(&self) -> ArtifactMetaData {{
                match *self {{
                    {rows_meta}
                }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_config4(&self) -> Option<&'static [ItemConfig]> {{
                match *self {{
                    {rows_config4}
                }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_config2(&self) -> Option<&'static [ItemConfig]> {{
                match *self {{
                    {rows_config2}
                }}
            }}
        }}
        "#,
        rows_effect=rows_effect,
        rows_meta=rows_meta,
        rows_config4=rows_config4,
        rows_config2=rows_config2
    );

    output.parse().unwrap()
}

#[proc_macro_derive(WeaponData)]
pub fn derive_weapon_meta_data(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let mut vars = get_enum_variants(&ast);

    let mut rows = String::new();
    let mut rows_get_config_data = String::new();
    let mut rows_effect = String::new();
    for v in vars.iter() {
        rows.push_str(&format!("WeaponName::{} => crate::weapon::weapons::{}::META_DATA,\n", v, v));
        rows_get_config_data.push_str(&format!("WeaponName::{} => crate::weapon::weapons::{}::CONFIG_DATA,\n", v, v));
        rows_effect.push_str(&format!("WeaponName::{} => crate::weapon::weapons::{}::get_effect(common_data, config),\n", v, v));
    }

    let output = format!(
        r#"
        impl WeaponName {{
            pub fn get_static_data(&self) -> WeaponStaticData {{
                match *self {{
                    {}
                }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_config_data(&self) -> Option<&'static [ItemConfig]> {{
                match *self {{
                    {}
                }}
            }}

            pub fn get_effect<T: crate::attribute::Attribute>(
                &self,
                config: &crate::weapon::weapon_config::WeaponConfig,
                common_data: &crate::character::character_common_data::CharacterCommonData
            ) -> Option<Box<dyn crate::weapon::weapon_effect::WeaponEffect<T>>> {{
                match *self {{
                    {}
                }}
            }}
        }}
        "#,
        rows,
        rows_get_config_data,
        rows_effect
    );

    output.parse().unwrap()
}

#[proc_macro_derive(TargetFunctionData)]
pub fn derive_tf_data(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut vars = get_enum_variants(&ast);

    let mut rows_create = String::new();
    let mut rows_get_meta = String::new();
    let mut rows_config = String::new();

    for v in vars.iter() {
        rows_create.push_str(&format!("TargetFunctionName::{n} => crate::target_functions::target_functions::{n}TargetFunction::create(character, weapon, config),\n", n=v));
        rows_get_meta.push_str(&format!("TargetFunctionName::{n} => crate::target_functions::target_functions::{n}TargetFunction::META_DATA,\n", n=v));
        rows_config.push_str(&format!("TargetFunctionName::{n} => crate::target_functions::target_functions::{n}TargetFunction::CONFIG,\n", n=v));
    }

    let output = format!(
        r#"
            use crate::target_functions::target_function::*;
            use crate::target_functions::target_function_meta::TargetFunctionMeta;
            use crate::common::item_config_type::ItemConfig;
            impl TargetFunctionName {{
                pub fn create(&self, character: &crate::character::character_common_data::CharacterCommonData, weapon: &crate::weapon::weapon_common_data::WeaponCommonData, config: &crate::target_functions::target_function_config::TargetFunctionConfig) -> Box<dyn crate::target_functions::target_function::TargetFunction> {{
                    match *self {{
                        {rows_create}
                    }}
                }}

                #[cfg(not(target_family = "wasm"))]
                pub fn get_meta_data(&self) -> TargetFunctionMeta {{
                    match *self {{
                        {rows_get_meta}
                    }}
                }}

                #[cfg(not(target_family = "wasm"))]
                pub fn get_config(&self) -> Option<&'static [ItemConfig]> {{
                    match *self {{
                        {rows_config}
                    }}
                }}
            }}
        "#,
        rows_create=rows_create,
        rows_get_meta=rows_get_meta,
        rows_config=rows_config,
    );

    output.parse().unwrap()
}

#[proc_macro_derive(CharacterData)]
pub fn derive_character_data(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let mut vars = get_enum_variants(&ast);

    let mut rows_meta_data = String::new();
    let mut rows_effect = String::new();
    let mut rows_damage = String::new();
    let mut rows_tf = String::new();
    let mut rows_skill_map = String::new();
    let mut rows_config_data = String::new();
    let mut rows_config_skill = String::new();
    let mut rows_skill_len = String::new();
    let mut rows_skill_from_str = String::new();

    for v in vars.iter() {
        rows_meta_data.push_str(&format!("CharacterName::{n} => crate::character::characters::{n}::STATIC_DATA,\n", n=v));
        rows_effect.push_str(&format!("CharacterName::{n} => crate::character::characters::{n}::new_effect(common_data, config),\n", n=v));
        rows_damage.push_str(&format!("CharacterName::{n} => crate::character::characters::{n}::damage_internal::<D>(context, skill_index, skill_config, fumo),\n", n=v));
        rows_tf.push_str(&format!("CharacterName::{n} => crate::character::characters::{n}::get_target_function_by_role(role_index, team, character, weapon),\n", n=v));
        rows_skill_map.push_str(&format!("CharacterName::{n} => crate::character::characters::{n}::SKILL_MAP,\n", n=v));
        rows_config_data.push_str(&format!("CharacterName::{n} => crate::character::characters::{n}::CONFIG_DATA,\n", n=v));
        rows_config_skill.push_str(&format!("CharacterName::{n} => crate::character::characters::{n}::CONFIG_SKILL,\n", n=v));
        rows_skill_len.push_str(&format!("CharacterName::{n} => <crate::character::characters::{n} as CharacterTrait>::DamageEnumType::COUNT,\n", n=v));
        rows_skill_from_str.push_str(
            &format!("CharacterName::{n} => <crate::character::characters::{n} as CharacterTrait>::DamageEnumType::from_str(s).ok().map(|x| x as usize),\n", n=v)
        );
    }

    let output = format!(
        r#"
        impl CharacterName {{
            pub fn get_static_data(&self) -> CharacterStaticData {{
                match *self {{ {rows_meta_data} }}
            }}

            pub fn get_effect<A: Attribute>(&self, common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {{
                match *self {{ {rows_effect} }}
            }}

            pub fn damage<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, skill_index: usize, skill_config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {{
                match context.character_common_data.name {{ {rows_damage} }}
            }}

            pub fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, character: &CharacterCommonData, weapon: &WeaponCommonData) -> Box<dyn TargetFunction> {{
                match character.name {{ {rows_tf} }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_skill_map(&self) -> CharacterSkillMap {{
                match *self {{ {rows_skill_map} }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_config_data(&self) -> Option<&'static [ItemConfig]> {{
                match *self {{ {rows_config_data} }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_config_skill(&self) -> Option<&'static [ItemConfig]> {{
                match *self {{ {rows_config_skill} }}
            }}

            pub fn get_skill_len(&self) -> usize {{
                match *self {{ {rows_skill_len} }}
            }}

            pub fn get_skill_from_str(&self, s: &str) -> Option<usize> {{
                match *self {{
                    {rows_skill_from_str}
                }}
            }}
        }}
        "#,
        rows_meta_data=rows_meta_data,
        rows_effect=rows_effect,
        rows_damage=rows_damage,
        rows_tf=rows_tf,
        rows_skill_map=rows_skill_map,
        rows_config_data=rows_config_data,
        rows_config_skill=rows_config_skill,
        rows_skill_len=rows_skill_len,
        rows_skill_from_str=rows_skill_from_str,
    );

    output.parse().unwrap()
}

#[proc_macro_derive(BuffData)]
pub fn derive_buff_data(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut vars = get_enum_variants(&ast);

    // let name = ast.ident.to_string();

    let mut row_create = String::new();
    let mut row_meta = String::new();
    let mut row_config = String::new();
    for v in vars.iter() {
        row_create.push_str(&format!("BuffName::{n} => crate::buffs::buffs::Buff{n}::create(b),\n", n=v));
        row_meta.push_str(&format!("BuffName::{n} => crate::buffs::buffs::Buff{n}::META_DATA,\n", n=v));
        row_config.push_str(&format!("BuffName::{n} => crate::buffs::buffs::Buff{n}::CONFIG,\n", n=v));
    }

    let output = format!(
        r#"
        use crate::attribute::Attribute;
        use crate::buffs::buff::Buff;
        use crate::buffs::buff_config::BuffConfig;
        use crate::buffs::buff_meta::BuffMetaData;
        use crate::common::item_config_type::ItemConfig;
        impl BuffName {{
            pub fn create<A: Attribute>(&self, b: &BuffConfig) -> Box<dyn Buff<A>> {{
                match *self {{
                    {row_create}
                }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_meta(&self) -> BuffMetaData {{
                match *self {{
                    {row_meta}
                }}
            }}

            #[cfg(not(target_family = "wasm"))]
            pub fn get_config(&self) -> Option<&'static [ItemConfig]> {{
                match *self {{
                    {row_config}
                }}
            }}
        }}
        "#,
        row_create=row_create,
        row_meta=row_meta,
        row_config=row_config,
    );

    output.parse().unwrap()
}

#[proc_macro_derive(EnumLen)]
pub fn derive_enum_len(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut vars = get_enum_variants(&ast);
    let name = ast.ident.to_string();

    let output = format!(
        r#"
        impl {} {{
            pub const LEN: usize = {};
        }}
        "#,
        name,
        vars.len()
    );

    output.parse().unwrap()
}
