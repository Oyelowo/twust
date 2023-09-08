// use std::collections::HashMap;

// use super::TailwindField;
// use crate::tailwind::class_type::TAILWIND_CSS;
// use crate::tailwind::tailwind_config::{ColorValue, TailwindConfig, Key};

// macro_rules! define_spacing_field {
//     ($name:ident, $prefix:expr, $css_field:ident, $variants:expr) => {
//         pub struct $name;

//         impl TailwindField for $name {
//             fn get_prefix(&self) -> &'static str {
//                 $prefix
//             }

//             fn get_variants(&self) -> Vec<&'static str> {
//                 $variants.to_vec()
//             }

//             fn get_default(&self) -> Vec<&str> {
//                 TAILWIND_CSS.$css_field.to_vec()
//             }

//             fn get_override(&self, config: &TailwindConfig) -> Vec<String> {
//                 extract_keys_from_spacing(
//                     &config.theme.overrides.$css_field,
//                     &config.theme.overrides.spacing,
//                 )
//             }

//             fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
//                 extract_keys_from_spacing(
//                     &config.theme.extend.$css_field,
//                     &config.theme.extend.spacing,
//                 )
//             }

//             fn handle_special_cases(&self, _config: &TailwindConfig) -> Vec<String> {
//                 vec![]
//             }
//         }
//     };
// }

// // e.g { 76: 300px, 80: 320px, 84: 340px, 88: 360px, 92: 380px, 96: 400px }
// fn extract_keys_from_spacing(
//     specific_spacing: &Option<HashMap<Key, String>>,
//     general_spacing: &Option<HashMap<Key, String>>,
// ) -> Vec<String> {
//     let mut keys: Vec<Key> = Vec::new();

//     if let Some(spacing) = general_spacing {
//         keys.extend(spacing.keys());
//     }

//     if let Some(spacing) = specific_spacing {
//         keys.extend(spacing.keys().cloned());
//     }

//     keys
// }

// // By default the spacing scale is inherited by the
// // padding, margin, width, height, maxHeight, gap, inset,
// // space, translate, scrollMargin, and scrollPadding core

// define_spacing_field!(Padding, "p", padding, ["t", "r", "b", "l", "x", "y"]);
// define_spacing_field!(Margin, "m", margin, ["t", "r", "b", "l", "x", "y"]);
// define_spacing_field!(Width, "w", width, []);
// define_spacing_field!(Height, "h", height, []);
// define_spacing_field!(MaxHeight, "max-h", max_height, []);
// define_spacing_field!(Gap, "gap", margin, ["x", "y"]);
// define_spacing_field!(Inset, "inset", margin, ["x", "y"]);
// define_spacing_field!(Space, "space", margin, ["x", "y"]);
// define_spacing_field!(Translate, "translate", margin, ["x", "y"]);
// define_spacing_field!(TextIndent, "indent", text_indent, []);
// define_spacing_field!(BorderSpacing, "border-spacing", border_spacing, ["x", "y"]);
// // no separator between scroll-m prefix and the value
// define_spacing_field!(
//     ScrollMargin,
//     "scroll-m",
//     scroll_margin,
//     ["x", "y", "s", "e", "t", "r", "b", "l"]
// );
// // no separator between scroll-p prefix and the value
// define_spacing_field!(
//     ScrollPadding,
//     "scroll-p",
//     scroll_margin,
//     ["x", "y", "s", "e", "t", "r", "b", "l"]
// );
