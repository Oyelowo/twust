use crate::tailwind::class_type::TAILWIND_CSS;

use super::tailwind_config::{ColorValue, TailwindConfig};
use serde_json;
use std::path::Path;
use std::{collections::HashMap, fs};

trait TailwindField {
    fn get_prefix(&self) -> &'static str;
    fn get_variants(&self) -> Vec<&'static str>;
    fn get_default(&self) -> Vec<&str>;
    fn get_override(&self, config: &TailwindConfig) -> Vec<String>;
    fn get_extend(&self, config: &TailwindConfig) -> Vec<String>;
    fn handle_special_cases(&self, config: &TailwindConfig) -> Vec<String>;
}

fn extract_keys_from_colors(colors: &Option<HashMap<String, ColorValue>>) -> Vec<String> {
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

struct Bg;

impl TailwindField for Bg {
    fn get_prefix(&self) -> &'static str {
        "bg"
    }

    fn get_variants(&self) -> Vec<&'static str> {
        vec![] // bg does not have variants like border does
    }

    fn get_default(&self) -> Vec<&str> {
        // Return the default classes for background
        // vec!["bg-red".to_string(), "bg-blue".to_string()]
        TAILWIND_CSS.background_color.to_vec()
    }

    fn get_override(&self, config: &TailwindConfig) -> Vec<String> {
        let specific_colors = &config.theme.overrides.background_color;
        if specific_colors.is_some() {
            return extract_keys_from_colors(specific_colors);
        }
        extract_keys_from_colors(&config.theme.overrides.colors)
    }

    fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
        let specific_colors = &config.theme.extend.background_color;
        if specific_colors.is_some() {
            return extract_keys_from_colors(specific_colors);
        }
        extract_keys_from_colors(&config.theme.extend.colors)
    }

    fn handle_special_cases(&self, _config: &TailwindConfig) -> Vec<String> {
        // You can implement any special cases for backgrounds here
        vec![]
    }
}

struct BorderColor;

impl TailwindField for BorderColor {
    fn get_prefix(&self) -> &'static str {
        "border"
    }

    fn get_variants(&self) -> Vec<&'static str> {
        vec!["t", "r", "b", "l", "s", "e"]
    }

    fn get_default(&self) -> Vec<&str> {
        // Return the default classes for borders
        // vec!["border-red".to_string(), "border-blue".to_string()]
        TAILWIND_CSS.border_color.to_vec()
    }

    fn get_override(&self, config: &TailwindConfig) -> Vec<String> {
        let specific_colors = &config.theme.overrides.border_color;
        if specific_colors.is_some() {
            return extract_keys_from_colors(specific_colors);
        }
        extract_keys_from_colors(&config.theme.overrides.colors)
    }

    fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
        let specific_colors = &config.theme.extend.border_color;
        if specific_colors.is_some() {
            return extract_keys_from_colors(specific_colors);
        }
        extract_keys_from_colors(&config.theme.extend.colors)
    }

    fn handle_special_cases(&self, _config: &TailwindConfig) -> Vec<String> {
        // You can implement any special cases for borders here
        vec![]
    }
}

fn generate_classes_for_keys(field: &dyn TailwindField, keys: &Vec<String>) -> Vec<String> {
    let mut classes = Vec::new();
    let variants = field.get_variants();
    let prefix = field.get_prefix();

    for key in keys.iter() {
        // e.g bg-red
        classes.push(format!("{prefix}-{key}"));
        for variant in variants.iter() {
            // e.g border-t-red, border-r-red-500,
            classes.push(format!("{prefix}-{variant}-{key}"));
        }
    }

    classes
}

fn add_classes_for_field(
    field: &dyn TailwindField,
    config: &TailwindConfig,
    classes: &mut Vec<String>,
) {
    let overrides = field.get_override(&config);
    if !overrides.is_empty() {
        classes.extend(generate_classes_for_keys(field, &overrides));
    } else {
        let default = field.get_default();
        classes.extend(default.iter().map(|x| x.to_string()));
    }
    let extend = field.get_extend(&config);
    classes.extend(generate_classes_for_keys(field, &extend));
    classes.extend(field.handle_special_cases(&config));
}

// fn read_tailwind_config(path: &str) -> Result<TailwindConfig, std::io::Error> {
//     let content = fs::read_to_string(path)?;
//     let config: TailwindConfig = serde_json::from_str(&content)?;
//     Ok(config)
// }

fn read_tailwind_config(filename: &str) -> Result<TailwindConfig, std::io::Error> {
    // Construct the path to the file relative to the directory containing Cargo.toml
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(filename);

    let content = fs::read_to_string(path)?;
    let config: TailwindConfig = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn get_classes() -> Vec<String> {
    // let content = fs::read_to_string("tailwind.config.json").expect("Unable to read file");
    // let config: TailwindConfig = read_tailwind_config("tailwind.config.json").unwrap_or_default();
    let config: TailwindConfig =
        read_tailwind_config("tailwind.config.json").expect("Unable to read file");
    let bg = Bg;
    let border_color = BorderColor;
    let mut classes = Vec::new();

    add_classes_for_field(&bg, &config, &mut classes);
    add_classes_for_field(&border_color, &config, &mut classes);

    // for class in classes.iter() {
    //     println!("{}", class);
    // }
    classes
}
