/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use super::TailwindField;
use crate::tailwind::class_type::TAILWIND_CSS;
use crate::tailwind::tailwind_config::{ColorValue, Key, TailwindConfig};
use serde_json;
use std::env;
use std::path::Path;
use std::{collections::HashMap, fs};

pub(crate) fn extract_keys_from_colors(
    specific_colors: &Option<HashMap<Key, ColorValue>>,
    inherited_colors: &Option<HashMap<Key, ColorValue>>,
) -> Vec<String> {
    let mut keys = Vec::new();
    let mut extract_color_keys = |colors: &Option<HashMap<Key, ColorValue>>| {
        if let Some(colors_map) = colors {
            for (key, value) in colors_map.iter() {
                match value {
                    // e.g for, bg-red => red
                    ColorValue::Simple(_) => keys.push(key.to_string()),
                    ColorValue::Shades(shades) => {
                        for shade_key in shades.keys() {
                            if shade_key == "DEFAULT" {
                                keys.push(key.to_string());
                            } else {
                                // e.g for bg-green-500 => green-500
                                keys.push(format!("{key}-{shade_key}"));
                            }
                        }
                    }
                }
            }
        };
    };
    extract_color_keys(specific_colors);
    extract_color_keys(inherited_colors);
    keys
}

// Define Color Fields implementations
#[macro_use]
macro_rules! define_tailwind_color_field {
    ({name: $name:ident, prefix: $prefix:expr, field_name: $default_field:ident, variants: $variants:expr}) => {
        pub(crate) struct $name;

        impl TailwindField for $name {
            fn get_prefix(&self) -> &'static str {
                $prefix
            }

            fn get_variants(&self) -> Vec<&'static str> {
                $variants.to_vec()
            }

            fn get_default(&self) -> Vec<&str> {
                TAILWIND_CSS.$default_field.to_vec()
            }

            fn get_override(&self, config: &TailwindConfig) -> Vec<String> {
                $crate::config::macros::extract_keys_from_colors(
                    &config.theme.overrides.$default_field,
                    &config.theme.overrides.colors,
                )
            }

            fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
                $crate::config::macros::extract_keys_from_colors(
                    &config.theme.extend.$default_field,
                    &config.theme.extend.colors,
                )
            }

            fn handle_special_cases(&self, _config: &TailwindConfig) -> Vec<String> {
                vec![]
            }
        }
    };
}

pub(crate) use define_tailwind_color_field;

#[macro_use]
macro_rules! define_tailwind_field {
    ({name : $name:ident, prefix: $prefix:expr, inherited: $inherited:ident,  field_name: $field_name:ident, variants: $variants:expr}) => {
        pub(crate) struct $name;

        impl TailwindField for $name {
            fn get_prefix(&self) -> &'static str {
                $prefix
            }

            fn get_variants(&self) -> Vec<&'static str> {
                $variants.to_vec()
            }

            fn get_default(&self) -> Vec<&str> {
                TAILWIND_CSS.$field_name.to_vec()
            }

            fn get_override(&self, config: &TailwindConfig) -> Vec<String> {
                $crate::config::macros::extract_keys(
                    &config.theme.overrides.$field_name,
                    &config.theme.overrides.$inherited,
                )
            }

            fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
                $crate::config::macros::extract_keys(
                    &config.theme.extend.$field_name,
                    &config.theme.extend.$inherited,
                )
            }

            fn handle_special_cases(&self, _config: &TailwindConfig) -> Vec<String> {
                vec![]
            }
        }
    };
}
pub(crate) use define_tailwind_field;

pub(crate) fn extract_keys(
    specific_config: &Option<HashMap<Key, String>>,
    inherited_config: &Option<HashMap<Key, String>>,
) -> Vec<String> {
    let mut keys = Vec::new();

    if let Some(confing) = specific_config {
        for key in confing.keys() {
            keys.push(key.to_string());
        }
    }

    if let Some(config) = inherited_config {
        for key in config.keys() {
            keys.push(key.to_string());
        }
    }

    keys
}
