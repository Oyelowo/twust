/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use syn::{parse_macro_input, LitStr};
mod config;
mod tailwind;
use tailwind::{
    class_type::TAILWIND_CSS,
    lengthy::LENGTHY,
    modifiers::{self, get_modifiers},
    tailwind_config::{CustomisableClasses, TailwindConfig},
    valid_baseclass_names::VALID_BASECLASS_NAMES,
};

use config::{get_classes, noconfig::UNCONFIGURABLE, read_tailwind_config};
use regex;
use std::{env, fs};
use tailwind::signable::SIGNABLES;
use tailwindcss_core::parser::{Extractor, ExtractorOptions};

fn run(input: &str, loose: bool) -> Vec<&str> {
    Extractor::unique_ord(
        input.as_bytes(),
        ExtractorOptions {
            preserve_spaces_in_arbitrary: loose,
        },
    )
    .into_iter()
    .map(|s| unsafe { std::str::from_utf8_unchecked(s) })
    .collect()
}

use proc_macro::TokenStream;
use quote::quote;

fn is_valid_length(value: &str) -> bool {
    let re = regex::Regex::new(r"^(-?\d+(\.?\d+)?(px|em|rem|%|cm|mm|in|pt|pc|vh|vw|vmin|vmax)|0)$")
        .expect("Invalid regex");
    re.is_match(value)
}

fn is_valid_calc(value: &str) -> bool {
    let re = regex::Regex::new(r"^calc\([^)]+\)$").expect("Invalid regex");
    re.is_match(value)
}

//
// Spacing:
#[proc_macro]
pub fn tw(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    // let config = read_tailwind_config()?;
    let ref config = match read_tailwind_config() {
        Ok(config) => config,
        Err(e) => {
            return syn::Error::new_spanned(input, format!("Error reading Tailwind config: {}", e))
                .to_compile_error()
                .into();
        }
    };
    let modifiers = get_modifiers(config);
    let valid_class_names = get_classes(config);
    let is_unconfigurable = |classes: &CustomisableClasses, action_type_str: &str| {
        serde_json::to_value(classes)
            .expect("Unable to convert to value")
            .as_object()
            .expect("Unable to convert to object")
            .iter()
            .any(|(key, value)| {
                if UNCONFIGURABLE.contains(&key.as_str()) && !value.is_null() {
                    panic!(
                        "You cannot {action_type_str} the key: {key:?} in tailwind.config.json",
                        key = key
                    );
                }
                false
            })
    };
    is_unconfigurable(&config.theme.overrides, "override");
    is_unconfigurable(&config.theme.extend, "extend");

    for word in input.value().split_whitespace() {
        let modifiers_and_class = word.split(':');
        // TODO:  check the first and the last character are not open and close brackets
        // respectively i.e arbitrary property e.g [mask_type:aplha];
        // hover:[mask-type:alpha];
        let mut word_for_arb_prop = word.split(":[");

        // modifiers e.g hover: in
        // hover:[mask-type:alpha]
        let is_valid_arb_prop = word_for_arb_prop
            .next()
            // e.g for hover:[mask-type:alpha], this will be hover,
            // for [mask-type:alpha], this will be [mask-type:alpha]
            .is_some_and(|modifiers_or_full_arb_prop| {
                let is_arbitrary_property = modifiers_or_full_arb_prop.starts_with('[') && modifiers_or_full_arb_prop.ends_with(']');

                let is_valid = if is_arbitrary_property {
                    modifiers_or_full_arb_prop.matches('[').count() == 1 &&
                    modifiers_or_full_arb_prop.matches(']').count() == 1 &&
                    modifiers_or_full_arb_prop
                        .trim_start_matches('[')
                        .trim_end_matches(']')
                        .split(':')
                        .count() == 2
                } else {
                    // e.g mask-type:alpha] in hover:[mask-type:alpha]
                    let full_arb_prop = word_for_arb_prop.next().unwrap_or_default();
                // e.g for single, hover in hover:[mask-type:alpha]
                    // for multiple, hover:first:last, in hover:first:last:[mask-type:alpha]
                modifiers_or_full_arb_prop
                    .split(':')
                    .all(|modifier| modifiers.contains(&modifier.to_string())) &&
                    full_arb_prop.matches(']').count() == 1 &&
                    full_arb_prop
                        .trim_end_matches(']')
                        .split(':')
                        .count() == 2

                };
                is_valid
            })
            ||
        // value e.g [mask-type:alpha] in hover:[mask-type:alpha]
        // potential addition checks(probably not a good idea. Imagine a new css property, we would
        // have to open a PR for every new or esoteric css property.)
         word_for_arb_prop.next().is_some_and(|value| {
            value.ends_with(']')
                && value.split(':').count() == 2
            // We had already split by ":[", so there should be no "[" anymore
                && value.matches('[').count() == 0
                && value.matches(']').count() == 1
        });

        // let is_arbitrary_property = word.starts_with('[') && word.ends_with(']');
        let last_word_signed = modifiers_and_class.clone().last().unwrap_or_default();
        let last_word_unsigned = last_word_signed
            .strip_prefix('-')
            .unwrap_or(last_word_signed);

        let modifiers_from_word = modifiers_and_class
            .clone()
            .take(modifiers_and_class.count() - 1)
            .collect::<Vec<&str>>();
        let is_valid_modifier = modifiers_from_word
            .iter()
            .all(|modifier| modifiers.contains(&modifier.to_string()));

        let is_valid_class =
            { !is_valid_arb_prop && valid_class_names.contains(&last_word_unsigned.to_string()) };

        let (base_classname, arbitrary_value_with_bracket) =
            last_word_unsigned.split_once("-[").unwrap_or_default();

        let is_valid_negative_baseclass = {
            // tw!("-m-4 p-4 p-4");
            (valid_class_names.contains(&last_word_unsigned.to_string())
                && last_word_signed.starts_with("-")
                && SIGNABLES
                    .iter()
                    .any(|s| (last_word_unsigned.starts_with(s))))
                || (is_valid_arb_prop
                    && last_word_signed.starts_with('-')
                    && SIGNABLES.iter().any(|s| last_word_unsigned.starts_with(s)))
        };

        let prefix_is_valid_tailwind_keyword = VALID_BASECLASS_NAMES.contains(&base_classname);
        let is_arbitrary_value =
            prefix_is_valid_tailwind_keyword && arbitrary_value_with_bracket.ends_with(']');

        let arbitrary_value = arbitrary_value_with_bracket.trim_end_matches(']');
        let is_lengthy_class = LENGTHY.contains(&base_classname);
        let is_valid_length = is_arbitrary_value
            && is_lengthy_class
            && (is_valid_length(arbitrary_value) || is_valid_calc(arbitrary_value));

        // lg:[&:nth-child(3)]:hover:underline
        // [&_p]:mt-4
        // flex [@supports(display:grid)]:grid
        // [@media(any-hover:hover){&:hover}]:opacity-100
        let has_arb_variant = {
            // lg:[&:nth-child(3)]:hover:underline => :nth-child(3)
            // [&_p]:mt-4 => _p
            let mut ampersand_variant_selector =
                word.split("[@").last().unwrap_or_default().split("]:");
            let mut and_variant_selector = word.split("[&").last().unwrap_or_default().split("]:");
            let is_valid_arbitrary_variant_selector = ampersand_variant_selector.clone().count()
                >= 2
                && !ampersand_variant_selector
                    .next()
                    .unwrap_or_default()
                    .is_empty();
            let is_valid_arbitrary_variant_queries = and_variant_selector.clone().count() >= 2
                && !and_variant_selector
                    .clone()
                    .last()
                    .unwrap_or_default()
                    .split("]:")
                    .next()
                    .unwrap_or_default()
                    .is_empty();
            let is_query = word.starts_with("[@");

            (is_valid_arbitrary_variant_selector || is_valid_arbitrary_variant_queries)
            // &&
            // ((!is_query  && !word.split("[&").next().unwrap_or_default().is_empty() && word.split(":[&").count() >= 2)  || is_query)
        };

        let is_valid_opacity = {
            let (class_name, opacity_raw) = last_word_unsigned.split_once("/").unwrap_or_default();
            let opacity_arb = opacity_raw
                .trim_start_matches('[')
                .trim_end_matches(']')
                .parse::<f32>();
            let is_valid_number = opacity_arb.is_ok_and(|opacity_num| {
                let is_valid_number = opacity_num >= 0.0 && opacity_num <= 100.0;
                is_valid_number
            });
            valid_class_names.contains(&class_name.to_string()) && is_valid_number
        };

        // Use official tailwind rust run function to further check integrity of the class name.
        // Complete the classes list
        // prefixing with minus sign should be allowed i.e -.

        if (is_valid_class && is_valid_modifier)
            || is_valid_negative_baseclass
            || (!is_lengthy_class && is_arbitrary_value)
            || is_valid_length
            || is_valid_arb_prop
            || has_arb_variant
            || is_valid_opacity
        // || !run(word, false).is_empty()
        {
            if run(word, false).is_empty() {
                return syn::Error::new_spanned(input, format!("Invalid string: {}", word))
                    .to_compile_error()
                    .into();
            }
        } else {
            return syn::Error::new_spanned(input, format!("Invalid string: {}", word))
                .to_compile_error()
                .into();
        }
    }

    TokenStream::from(quote! {#input})
}
