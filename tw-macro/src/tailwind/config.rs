use super::tailwind_config::{ColorValue, TailwindConfig};
use crate::tailwind::class_type::TAILWIND_CSS;
use serde_json;
use std::env;
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

fn read_tailwind_config() -> Result<TailwindConfig, std::io::Error> {
    let current_dir = std::env::current_dir()?;

    // Construct the path to tailwind.config.json relative to the current directory
    let config_path = current_dir.join("tailwind.config.json");

    if !config_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "tailwind.config.json was not found in the top-level directory - \n{config_path:?}. Ensure it exists."
            ),
        ));
    }

    let content = fs::read_to_string(config_path)?;
    let config: TailwindConfig = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn get_classes() -> Result<Vec<String>, std::io::Error> {
    let config = read_tailwind_config()?;
    let bg = Bg;
    let border_color = BorderColor;
    let mut classes = Vec::new();

    add_classes_for_field(&bg, &config, &mut classes);
    add_classes_for_field(&border_color, &config, &mut classes);

    // for class in classes.iter() {
    //     println!("{}", class);
    // }
    Ok(classes)
}
