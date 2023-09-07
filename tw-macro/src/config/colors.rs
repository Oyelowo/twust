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
                    return $crate::config::colors::extract_keys_from_colors(specific_colors);
                }
                $crate::config::colors::extract_keys_from_colors(&config.theme.overrides.colors)
            }

            fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
                let specific_colors = &config.theme.extend.$default_field;
                if specific_colors.is_some() {
                    return $crate::config::colors::extract_keys_from_colors(specific_colors);
                }
                $crate::config::colors::extract_keys_from_colors(&config.theme.extend.colors)
            }

            fn handle_special_cases(&self, _config: &TailwindConfig) -> Vec<String> {
                vec![]
            }
        }
    };
}

pub(crate) use define_tailwind_color_field;

// // Now, use the macro to define the structs and their implementations:
// define_tailwind_color_field!(AccentColor, "accent", accent_color, []);
// define_tailwind_color_field!(BgColor, "bg", background_color, []);
// // border color has hyphen between the prefix and the variant
// // e.g border<prefix>-t<variant>-red-500<key>
// define_tailwind_color_field!(
//     BorderColor,
//     "border",
//     border_color,
//     ["-t", "-r", "-b", "-l", "-s", "-e"]
// );
// define_tailwind_color_field!(TextColor, "text", text_color, []);
// define_tailwind_color_field!(TextDecorationColor, "decoration", text_decoration_color, []);
// define_tailwind_color_field!(PlaceholderColor, "placeholder", placeholder_color, []);
// define_tailwind_color_field!(RingColor, "ring", ring_color, []);
// define_tailwind_color_field!(RingOffsetColor, "ring-offset", ring_offset_color, []);
// define_tailwind_color_field!(BoxShadowColor, "shadow", box_shadow_color, []);
// define_tailwind_color_field!(DivideColor, "divide", divide_color, []);
// define_tailwind_color_field!(OutlineColor, "outline", outline_color, []);
// define_tailwind_color_field!(FillColor, "fill", fill, []);
// define_tailwind_color_field!(StrokeColor, "stroke", stroke, []);
// define_tailwind_color_field!(BackgroundColor, "bg", background_color, []);
//
// // define_tailwind_field!(GradientColorStops, "gradient", gradient_color_stops);
// // If you want to customize the gradient color stops positions, you can add a
// // gradientColorStopPositions section to your theme.gradientColorStopPositions
// // or theme.extend.gradientColorStopPositions section
// define_tailwind_color_field!(GradientColorStopsFrom, "from", gradient_color_stops, []);
// define_tailwind_color_field!(GradientColorStopsVia, "via", gradient_color_stops, []);
// define_tailwind_color_field!(GradientColorStopsTo, "to", gradient_color_stops, []);
