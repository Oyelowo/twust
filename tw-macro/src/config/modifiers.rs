use crate::tailwind::{modifiers::ARIA_DEFAULT, tailwind_config::TailwindConfig};

use super::TailwindField;

macro_rules! define_tailwind_modifier {
    ({name: $struct_name:ident, field_name: $field_name:ident, prefix: $prefix:expr, default_values: $default_values:expr }) => {
        pub struct $struct_name;

        impl TailwindField for $struct_name {
            fn get_prefix(&self) -> &'static str {
                $prefix
            }

            fn get_variants(&self) -> Vec<&'static str> {
                vec![]
            }

            fn get_default(&self, _config: &TailwindConfig) -> Vec<&'static str> {
                $default_values.to_vec()
            }

            fn get_override(&self, config: &TailwindConfig) -> Vec<String> {
                config
                    .theme
                    .overrides
                    .$field_name
                    .clone()
                    .unwrap_or_default()
                    .into_keys()
                    .collect()
            }

            fn get_extend(&self, config: &TailwindConfig) -> Vec<String> {
                config
                    .theme
                    .extend
                    .$field_name
                    .clone()
                    .unwrap_or_default()
                    .into_keys()
                    .collect()
            }

            fn handle_special_cases(&self, _config: &TailwindConfig) -> Vec<String> {
                vec![]
            }
        }
    };
}

define_tailwind_modifier!({
    name: Aria,
    field_name: aria,
    prefix: "aria",
    default_values: ARIA_DEFAULT
});

define_tailwind_modifier!({
    name: Supports,
    field_name: supports,
    prefix: "supports",
    default_values: []
});

define_tailwind_modifier!({
    name: Data,
    field_name: data,
    prefix: "data",
    default_values: []
});
