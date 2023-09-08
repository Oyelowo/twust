use super::TailwindField;
use crate::tailwind::class_type::TAILWIND_CSS;
use crate::tailwind::tailwind_config::{ColorValue, TailwindConfig};
use serde_json;
use std::env;
use std::path::Path;
use std::{collections::HashMap, fs};

pub(crate) fn extract_keys_from_colors(
    colors: &Option<HashMap<String, ColorValue>>,
) -> Vec<String> {
    let mut keys = Vec::new();
    if let Some(colors_map) = colors {
        for (key, value) in colors_map.iter() {
            match value {
                ColorValue::Simple(_) => keys.push(key.clone()),
                ColorValue::Shades(shades) => {
                    for shade_key in shades.keys() {
                        if shade_key == "DEFAULT" {
                            keys.push(key.clone());
                            continue;
                        }
                        keys.push(format!("{key}-{shade_key}"));
                    }
                }
            }
        }
    }
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
                let specific_colors = &config.theme.overrides.$default_field;
                if specific_colors.is_some() {
                    return $crate::config::macros::extract_keys_from_colors(specific_colors);
                }
                $crate::config::macros::extract_keys_from_colors(&config.theme.overrides.colors)
            }

            fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
                let specific_colors = &config.theme.extend.$default_field;
                if specific_colors.is_some() {
                    return $crate::config::macros::extract_keys_from_colors(specific_colors);
                }
                $crate::config::macros::extract_keys_from_colors(&config.theme.extend.colors)
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
                vec![]
            }

            fn get_default(&self) -> Vec<&str> {
                TAILWIND_CSS.$field_name.to_vec()
            }

            fn get_override(&self, config: &TailwindConfig) -> Vec<String> {
                $crate::config::macros::extract_keys(
                    $prefix,
                    &config.theme.overrides.$field_name,
                    &config.theme.overrides.$inherited,
                )
            }

            fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
                $crate::config::macros::extract_keys(
                    $prefix,
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
    prefix: &str,
    specific_config: &Option<HashMap<String, String>>,
    inherited_config: &Option<HashMap<String, String>>,
) -> Vec<String> {
    let mut keys = Vec::new();

    if let Some(confing) = specific_config {
        for key in confing.keys() {
            if key == "DEFAULT" {
                keys.push(prefix.to_string());
            } else {
                keys.push(format!("{}-{}", prefix, key));
            }
        }
    }

    if let Some(config) = inherited_config {
        for key in config.keys() {
            if key == "DEFAULT" {
                keys.push(prefix.to_string());
            } else {
                keys.push(format!("{}-{}", prefix, key));
            }
        }
    }

    keys
}
