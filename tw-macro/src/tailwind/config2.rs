use super::tailwind_config::TailwindConfig;
use serde_json::Value;
use std::collections::{hash_map, HashMap};
// use super::tailwind_config::TailwindConfig;
// use serde_json::Value;
// use std::collections::{hash_map, HashMap};

// trait TailwindField {
//     fn get_default(&self) -> Vec<&'static str>;
//     fn get_override(&self, config: &TailwindConfig) -> Option<Vec<&'static str>>;
//     fn get_extend(&self, config: &TailwindConfig) -> Option<Vec<&'static str>>;
//     fn handle_special_cases(&mut self, config: &TailwindConfig);
// }



trait TailwindField {
    fn get_default(&self) -> Vec<&'static str>;
    fn get_override(&self, config: &TailwindConfig) -> Option<Vec<&'static str>>;
    fn get_extend(&self, config: &TailwindConfig) -> Option<Vec<&'static str>>;
    fn handle_special_cases(&mut self, config: &TailwindConfig);
}

struct Bg;
struct BorderColor;

impl TailwindField for Bg {
    fn get_default(&self) -> Vec<&'static str> {
        vec!["bg-red", "bg-green"] // just an example
    }
    fn get_override(&self, config: &TailwindConfig) -> Option<Vec<&'static str>> {
        match &config.theme.overrides.background_color {
            Some(bg_colors) => Some(generate_classes_for_color_based_types("bg", bg_colors)),
            None => None,
        }
    }

    fn get_extend(&self, config: &TailwindConfig) -> Option<Vec<&'static str>> {
        todo!()
    }

    fn handle_special_cases(&mut self, config: &TailwindConfig) {
        todo!()
    }
    // ... implement other methods ...
}

impl TailwindField for BorderColor {
    fn get_default(&self) -> Vec<&'static str> {
        vec!["border-red".to_string(), "border-green".to_string()] // just an example
    }
    fn get_override(&self, config: &TailwindConfig) -> Option<Vec<&'static str>> {
        match &config.theme.overrides.border_color {
            Some(border_colors) => Some(generate_classes_for_color_based_types("border", border_colors)),
            None => None,
        }
    }

    fn get_extend(&self, config: &TailwindConfig) -> Option<Vec<&'static str>> {
        todo!()
    }

    fn handle_special_cases(&mut self, config: &TailwindConfig) {
        todo!()
    }

}


// Defining the variants for each class type
static CLASS_VARIANTS: hash_map::HashMap<&'static str, Vec<&'static str>> =
    hash_map::HashMap::from([
        ("bg", vec![]),
        ("text", vec![]),
        ("border", vec!["t", "r", "b", "l", "s", "e"]),
        ("divide", vec!["x", "y"]),
        ("placeholder", vec![]),
        ("from", vec![]),
        ("via", vec![]),
        ("to", vec![]),
        ("ring", vec![]),
        ("ring-offset", vec![]),
    ]);

fn generate_classes_for_color_based_types(colors: &HashMap<&'static str, Value>) -> Vec<&'static str> {
    let mut classes = Vec::new();

    for (class_type, variants) in CLASS_VARIANTS.iter() {
        for (color_name, color_value) in colors.iter() {
            match color_value {
                Value::String(color_code) => {
                    classes.push(format!("{}-{}", class_type, color_name));
                    for variant in variants.iter() {
                        classes.push(format!("{}-{}-{}", class_type, variant, color_name));
                    }
                }
                Value::Object(shades) => {
                    for (shade_name, shade_value) in shades.iter() {
                        if shade_name == "DEFAULT" {
                            classes.push(format!("{}-{}", class_type, color_name));
                            for variant in variants.iter() {
                                classes.push(format!("{}-{}-{}", class_type, variant, color_name));
                            }
                        } else {
                            classes.push(format!("{}-{}-{}", class_type, color_name, shade_name));
                            for variant in variants.iter() {
                                classes.push(format!(
                                    "{}-{}-{}-{}",
                                    class_type, variant, color_name, shade_name
                                ));
                            }
                        }
                    }
                }
                _ => {} // Handle other cases or error out
            }
        }
    }

    classes
}



fn main() {
    // Deserialize the config JSON into TailwindConfig
    let config: TailwindConfig = serde_json::from_str("YOUR_JSON_STRING").unwrap();

    let bg = Bg;
    let border_color = BorderColor;
    // ... other instances ...

    let mut classes = Vec::new();

    add_classes_for_field(&bg, &config, &mut classes);
    add_classes_for_field(&border_color, &config, &mut classes);
    // ... similarly for other instances ...

    for class in classes.iter() {
        println!("{}", class);
    }
}

fn add_classes_for_field(
    field: &dyn TailwindField,
    config: &TailwindConfig,
    classes: &mut Vec<&'static str>,
) {
    if let Some(override_classes) = field.get_override(&config) {
        classes.extend(override_classes);
    } else {
        classes.extend(field.get_default());
    }
    classes.extend(field.get_extend(&config).unwrap_or_default());
    classes.extend(field.handle_special_cases(&config));
}
