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
    lengthy::LENGTHY, modifiers::get_modifiers, tailwind_config::CustomisableClasses,
    valid_baseclass_names::VALID_BASECLASS_NAMES,
};

use config::{get_classes, noconfig::UNCONFIGURABLE, read_tailwind_config};
use proc_macro::TokenStream;
use quote::quote;
use regex::{self, Regex};
use tailwind::signable::SIGNABLES;
use tailwindcss_core::parser::{Extractor, ExtractorOptions};

#[proc_macro]
pub fn tw(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let (modifiers, valid_class_names) = match setup(&input) {
        Ok(value) => value,
        Err(value) => return value,
    };

    for word in input.value().split_whitespace() {
        let (is_valid_arb_prop, last_word_signed, last_word_unsigned, is_valid_modifier) =
            get_modifiers_and_words(word, &modifiers);

        let is_valid_class =
            is_valid_class(is_valid_arb_prop, &valid_class_names, last_word_unsigned);

        let (base_classname, arbitrary_value_with_bracket) =
            last_word_unsigned.split_once("-[").unwrap_or_default();

        let is_valid_negative_baseclass = is_valid_negative_baseclass(
            &valid_class_names,
            last_word_unsigned,
            last_word_signed,
            is_valid_arb_prop,
        );

        let prefix_is_valid_tailwind_keyword = VALID_BASECLASS_NAMES.contains(&base_classname);
        let is_arbitrary_value =
            prefix_is_valid_tailwind_keyword && arbitrary_value_with_bracket.ends_with(']');

        let arbitrary_value = arbitrary_value_with_bracket.trim_end_matches(']');
        let is_lengthy_class = LENGTHY.contains(&base_classname);
        let is_valid_length = is_arbitrary_value
            && is_lengthy_class
            && (is_valid_length(arbitrary_value) || is_valid_calc(arbitrary_value));

        let has_arb_variant = has_arb_variant(word);

        let is_valid_opacity = is_valid_opacity(last_word_unsigned, &valid_class_names);

        if (is_valid_class && is_valid_modifier)
            || is_valid_negative_baseclass
            || (!is_lengthy_class && is_arbitrary_value)
            || is_valid_length
            || is_valid_arb_prop
            || has_arb_variant
            || is_valid_opacity
            || is_valid_group_classname(last_word_unsigned)
            || is_validate_modifier_or_group(word, &modifiers, &valid_class_names)
        {
            if check_word(word, false).is_empty() {
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

fn check_word(input: &str, loose: bool) -> Vec<&str> {
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

fn is_valid_length(value: &str) -> bool {
    let re = regex::Regex::new(r"^(-?\d+(\.?\d+)?(px|em|rem|%|cm|mm|in|pt|pc|vh|vw|vmin|vmax)|0)$")
        .expect("Invalid regex");
    re.is_match(value)
}

fn is_valid_calc(value: &str) -> bool {
    let re = regex::Regex::new(r"^calc\([^)]+\)$").expect("Invalid regex");
    re.is_match(value)
}

fn setup(input: &LitStr) -> Result<(Vec<String>, Vec<String>), TokenStream> {
    let ref config = match read_tailwind_config() {
        Ok(config) => config,
        Err(e) => {
            return Err(syn::Error::new_spanned(
                input,
                format!("Error reading Tailwind config: {}", e),
            )
            .to_compile_error()
            .into());
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
    Ok((modifiers, valid_class_names))
}

fn get_modifiers_and_words<'a>(
    word: &'a str,
    modifiers: &'a Vec<String>,
) -> (bool, &'a str, &'a str, bool) {
    let modifiers_and_class = word.split(':');
    // TODO:  check the first and the last character are not open and close brackets
    // respectively i.e arbitrary property e.g [mask_type:aplha];
    // hover:[mask-type:alpha];
    let word_for_arb_prop = word.split(":[");

    // modifiers e.g hover: in
    // hover:[mask-type:alpha]
    let is_valid_arb_prop = is_valid_arb_prop(word_for_arb_prop, modifiers);

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
    (
        is_valid_arb_prop,
        last_word_signed,
        last_word_unsigned,
        is_valid_modifier,
    )
}

fn is_valid_opacity(last_word_unsigned: &str, valid_class_names: &Vec<String>) -> bool {
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
    is_valid_opacity
}

fn has_arb_variant(word: &str) -> bool {
    // lg:[&:nth-child(3)]:hover:underline
    // [&_p]:mt-4
    // flex [@supports(display:grid)]:grid
    // [@media(any-hover:hover){&:hover}]:opacity-100
    let has_arb_variant = {
        // lg:[&:nth-child(3)]:hover:underline => :nth-child(3)
        // [&_p]:mt-4 => _p
        let mut ampersand_variant_selector =
            word.split("[@").last().unwrap_or_default().split("]:");
        let and_variant_selector = word.split("[&").last().unwrap_or_default().split("]:");
        let is_valid_arbitrary_variant_selector = ampersand_variant_selector.clone().count() >= 2
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

        is_valid_arbitrary_variant_selector || is_valid_arbitrary_variant_queries || is_query
        // &&
        // ((!is_query  && !word.split("[&").next().unwrap_or_default().is_empty() && word.split(":[&").count() >= 2)  || is_query)
    };
    has_arb_variant
}

fn is_valid_negative_baseclass(
    valid_class_names: &Vec<String>,
    last_word_unsigned: &str,
    last_word_signed: &str,
    is_valid_arb_prop: bool,
) -> bool {
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
    is_valid_negative_baseclass
}

fn is_valid_class(
    is_valid_arb_prop: bool,
    valid_class_names: &Vec<String>,
    last_word_unsigned: &str,
) -> bool {
    let is_valid_class =
        { !is_valid_arb_prop && valid_class_names.contains(&last_word_unsigned.to_string()) };
    is_valid_class
}

fn is_valid_arb_prop(
    mut word_for_arb_prop: std::str::Split<'_, &str>,
    modifiers: &Vec<String>,
) -> bool {
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
    is_valid_arb_prop
}

fn is_valid_group_pattern(modifier: &str, valid_modifiers: &Vec<String>) -> bool {
    let parts: Vec<&str> = modifier.split('/').collect();
    let group_modifier = parts[0];
    if parts.len() == 2
        && valid_modifiers.contains(&group_modifier.to_string())
        && group_modifier.starts_with("group")
    {
        return true;
    } else {
        false
    }
}

// tw!("group/edit invisible hover:bg-slate-200 group-hover/item:visible");
// tw!("group-[:nth-of-type(3)_&]:block group-hover/edit:text-gray-700 group-[:nth-of-type(3)_&]:block");
fn is_validate_modifier_or_group(
    word: &str,
    valid_modifiers: &Vec<String>,
    valid_class_names: &Vec<String>,
) -> bool {
    let valid_arb_group = word.split(":").collect::<Vec<&str>>();
    let modifiers = &valid_arb_group[..valid_arb_group.len() - 1];
    let last_word = valid_arb_group.last().unwrap_or(&"");
    let is_valid_last_word =
        is_valid_string(last_word) && valid_class_names.contains(&last_word.to_string());

    for modifier in modifiers {
        if modifier.starts_with("group") {
            return is_valid_group_pattern(modifier, valid_modifiers) && is_valid_last_word;
        } else {
            return valid_modifiers.contains(&modifier.to_string()) && is_valid_last_word;
        }
    }

    is_valid_last_word
}

fn is_valid_group_classname(class_name: &str) -> bool {
    return !class_name.contains(':')
        && !class_name.contains('[')
        && !class_name.contains(']')
        && class_name.starts_with("group/");
}

fn is_valid_string(s: &str) -> bool {
    // Matches strings that contain only alphanumeric characters, underscores, and hyphens.
    let re = Regex::new(r"^[a-zA-Z0-9_-]*$").unwrap();
    re.is_match(s) && !s.is_empty()
}
