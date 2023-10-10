use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{multispace0, multispace1, space0, space1},
    combinator::{all_consuming, opt, recognize},
    multi::separated_list0,
    number,
    sequence::{delimited, tuple},
    IResult,
};
/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use syn::{parse_macro_input, LitStr};
mod config;
mod plugins;
mod tailwind;
use tailwind::{
    colorful::COLORFUL_BASECLASSES, default_classnames::TAILWIND_CSS, lengthy::LENGTHY,
    modifiers::get_modifiers, tailwind_config::CustomisableClasses,
    valid_baseclass_names::VALID_BASECLASS_NAMES,
};

use config::{get_classes, noconfig::UNCONFIGURABLE, read_tailwind_config};
use proc_macro::TokenStream;
use regex::{self, Regex};
use tailwind::signable::SIGNABLES;
// use tailwindcss_core::parser::{Extractor, ExtractorOptions};
//
// p-6 max-w-sm mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-4

// OLD IMPLEMENTATION
#[proc_macro]
pub fn tw(raw_input: TokenStream) -> TokenStream {
    let r_input = raw_input.clone();
    let input = parse_macro_input!(r_input as LitStr);
    let (modifiers, valid_class_names) = match setup(&input) {
        Ok(value) => value,
        Err(value) => {
            return syn::Error::new_spanned(input, value)
                .to_compile_error()
                .into()
        }
    };

    for word in input.value().split_whitespace() {
        let (last_word_signed, last_word_unsigned) = get_last_word_types(word);

        // modifiers e.g hover: in
        // hover:[mask-type:alpha]
        let is_valid_arb_prop = is_valid_arb_prop(word, &modifiers);

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

        if (is_valid_class && is_valid_modifier(word, &modifiers))
            || is_valid_negative_baseclass
            || (!is_lengthy_class && is_arbitrary_value)
            || is_valid_length
            || is_valid_arb_prop
            || has_arb_variant
            || is_valid_opacity
            || is_valid_group_classname(last_word_unsigned)
            || is_validate_modifier_or_group(word, &modifiers, &valid_class_names)
        {
            // if check_word(word, false).is_empty() {
            //     return syn::Error::new_spanned(input, format!("Invalid string: {}", word))
            //         .to_compile_error()
            //         .into();
            // }
        } else {
            return syn::Error::new_spanned(input, format!("Invalid string: {word}"))
                .to_compile_error()
                .into();
        }
    }

    raw_input
}

// fn check_word(input: &str, loose: bool) -> Vec<&str> {
//     Extractor::unique_ord(
//         input.as_bytes(),
//         ExtractorOptions {
//             preserve_spaces_in_arbitrary: loose,
//         },
//     )
//     .into_iter()
//     .map(|s| unsafe { std::str::from_utf8_unchecked(s) })
//     .collect()
// }

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
    let config = &(match read_tailwind_config() {
        Ok(config) => config,
        Err(e) => {
            return Err(syn::Error::new_spanned(
                input,
                format!("Error reading Tailwind config: {}", e),
            )
            .to_compile_error()
            .into());
        }
    });
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
                    panic!("You cannot {action_type_str} the key: {key} in tailwind.config.json",);
                }
                false
            })
    };
    is_unconfigurable(&config.theme.overrides, "override");
    is_unconfigurable(&config.theme.extend, "extend");
    Ok((modifiers, valid_class_names))
}

fn get_last_word_types(word: &str) -> (&str, &str) {
    let modifiers_and_class = word.split(':');

    // let is_arbitrary_property = word.starts_with('[') && word.ends_with(']');
    let last_word_signed = modifiers_and_class.clone().last().unwrap_or_default();
    let last_word_unsigned = last_word_signed
        .strip_prefix('-')
        .unwrap_or(last_word_signed);

    (last_word_signed, last_word_unsigned)
}

fn is_valid_modifier(word: &str, modifiers: &[String]) -> bool {
    let modifiers_and_class = word.split(':');
    let modifiers_from_word = modifiers_and_class
        .clone()
        .take(modifiers_and_class.count() - 1)
        .collect::<Vec<&str>>();
    modifiers_from_word
        .iter()
        .all(|modifier| modifiers.contains(&modifier.to_string()))
}

fn is_valid_opacity(last_word_unsigned: &str, valid_class_names: &[String]) -> bool {
    let is_valid_opacity = {
        let (class_name, opacity_raw) = last_word_unsigned.split_once('/').unwrap_or_default();
        let opacity_arb = opacity_raw
            .trim_start_matches('[')
            .trim_end_matches(']')
            .parse::<f32>();
        let is_valid_number =
            opacity_arb.is_ok_and(|opacity_num| (0.0..=100.0).contains(&opacity_num));
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
    valid_class_names: &[String],
    last_word_unsigned: &str,
    last_word_signed: &str,
    is_valid_arb_prop: bool,
) -> bool {
    let is_valid_negative_baseclass = {
        // tw!("-m-4 p-4 p-4");
        (valid_class_names.contains(&last_word_unsigned.to_string())
            && last_word_signed.starts_with('-')
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
    valid_class_names: &[String],
    last_word_unsigned: &str,
) -> bool {
    !is_valid_arb_prop && valid_class_names.contains(&last_word_unsigned.to_string())
}

fn is_valid_arb_prop(word: &str, modifiers: &[String]) -> bool {
    // TODO:  check the first and the last character are not open and close brackets
    // respectively i.e arbitrary property e.g [mask_type:aplha];
    // hover:[mask-type:alpha];
    let mut word_for_arb_prop = word.split(":[");

    word_for_arb_prop
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
    })
}

fn is_valid_group_pattern(modifier: &str, valid_modifiers: &[String]) -> bool {
    let parts: Vec<&str> = modifier.split('/').collect();
    let group_modifier = parts[0];
    parts.len() == 2
        && valid_modifiers.contains(&group_modifier.to_string())
        && group_modifier.starts_with("group")
}

// tw!("group/edit invisible hover:bg-slate-200 group-hover/item:visible");
// tw!("group-[:nth-of-type(3)_&]:block group-hover/edit:text-gray-700 group-[:nth-of-type(3)_&]:block");
fn is_validate_modifier_or_group(
    word: &str,
    valid_modifiers: &[String],
    valid_class_names: &[String],
) -> bool {
    let valid_arb_group = word.split(':').collect::<Vec<&str>>();
    let modifiers = &valid_arb_group[..valid_arb_group.len() - 1];
    let last_word = valid_arb_group.last().unwrap_or(&"");
    let is_valid_last_word =
        is_valid_string(last_word) && valid_class_names.contains(&last_word.to_string());

    for modifier in modifiers {
        if modifier.starts_with("group") {
            if !is_valid_group_pattern(modifier, valid_modifiers) && is_valid_last_word {
                return false;
            }
        } else if !valid_modifiers.contains(&modifier.to_string()) && is_valid_last_word {
            return false;
        }
    }

    is_valid_last_word
}

fn is_valid_group_classname(class_name: &str) -> bool {
    !class_name.contains(':')
        && !class_name.contains('[')
        && !class_name.contains(']')
        && class_name.starts_with("group/")
}

fn is_valid_string(s: &str) -> bool {
    // Matches strings that contain only alphanumeric characters, underscores, and hyphens.
    let re = Regex::new(r"^[a-zA-Z0-9_-]*$").expect("Invalid regex");
    re.is_match(s) && !s.is_empty()
}

// OLD IMPLEMENTATION
fn get_classes_straight() -> Vec<String> {
    get_classes(&read_tailwind_config().unwrap())
    // get_classes
}
fn is_valid_classname2(class_name: &str) -> bool {
    get_classes_straight().contains(&class_name.to_string())
}

fn is_valid_modifier2(modifier: &str) -> bool {
    get_modifiers(&read_tailwind_config().unwrap()).contains(&modifier.to_string())
}

// [&:nth-child(3)]:underline
// lg:[&:nth-child(3)]:hover:underline
// [&_p]:mt-4
// flex [@supports(display:grid)]:grid
// [@media(any-hover:hover){&:hover}]:opacity-100
// group/edit invisible hover:bg-slate-200 group-hover/item:visible
// hidden group-[.is-published]:block
// group-[:nth-of-type(3)_&]:block
// peer-checked/published:text-sky-500
// peer-[.is-dirty]:peer-required:block hidden
// hidden peer-[:nth-of-type(3)_&]:block
// after:content-['*'] after:ml-0.5 after:text-red-500 block text-sm font-medium text-slate-700
// before:content-[''] before:block
// bg-black/75 supports-[backdrop-filter]:bg-black/25 supports-[backdrop-filter]:backdrop-blur
// aria-[sort=ascending]:bg-[url('/img/down-arrow.svg')] aria-[sort=descending]:bg-[url('/img/up-arrow.svg')]
// group-aria-[sort=ascending]:rotate-0 group-aria-[sort=descending]:rotate-180
// data-[size=large]:p-8
// open:bg-white dark:open:bg-slate-900 open:ring-1 open:ring-black/5 dark:open:ring-white/10 open:shadow-lg p-6 rounded-lg
// lg:[&:nth-child(3)]:hover:underline
// min-[320px]:text-center max-[600px]:bg-sky-300
// top-[117px] lg:top-[344px]
// bg-[#bada55] text-[22px] before:content-['Festivus']
// grid grid-cols-[fit-content(theme(spacing.32))]
// bg-[--my-color]
// [mask-type:luminance] hover:[mask-type:alpha]
// [--scroll-offset:56px] lg:[--scroll-offset:44px]
// lg:[&:nth-child(3)]:hover:underline
// bg-[url('/what_a_rush.png')]
// before:content-['hello\_world']
// text-[22px]
// text-[#bada55]
// text-[var(--my-var)]
// text-[length:var(--my-var)]
// text-[color:var(--my-var)]
fn parse_predefined_tw_classname(input: &str) -> IResult<&str, ()> {
    let (input, class_name) = recognize(|i| {
        // Assuming a Tailwind class consists of alphanumeric, dashes, and colons
        nom::bytes::complete::is_a(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-:",
        )(i)
    })(input)?;

    if is_valid_classname2(class_name) {
        // Ok((input, class_name))
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-'
}

fn is_lengthy_classname(class_name: &str) -> bool {
    LENGTHY.contains(&class_name)
}

// text-[22px]
fn lengthy_arbitrary_classname(input: &str) -> IResult<&str, ()> {
    let (input, class_name) = take_until("-[")(input)?;
    let ((input, _)) = if is_lengthy_classname(class_name) {
        // if is_lengthy_classname(class_name) {
        //     // Do something special for lengthy class names
        // }
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }?;

    // arbitrary value
    let (input, _) = tag("-")(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = multispace0(input)?;
    // is number
    let (input, _) = number::complete::double(input)?;
    let (input, _) = {
        // px|em|rem|%|cm|mm|in|pt|pc|vh|vw|vmin|vmax
        alt((
            tag("px"),
            tag("em"),
            tag("rem"),
            tag("%"),
            tag("cm"),
            tag("mm"),
            tag("in"),
            tag("pt"),
            tag("pc"),
            tag("vh"),
            tag("vw"),
            tag("vmin"),
            tag("vmax"),
        ))
    }(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

fn is_hex_color(color: &str) -> bool {
    let re = regex::Regex::new(r"^#[0-9a-fA-F]{3,6}$").expect("Invalid regex");
    re.is_match(color)
}

fn is_colorful_baseclass(class_name: &str) -> bool {
    COLORFUL_BASECLASSES.contains(&class_name)
}

// text-[#bada55]
fn colorful_arbitrary_baseclass(input: &str) -> IResult<&str, ()> {
    let (input, class_name) = take_until("-[")(input)?;
    let ((input, _)) = if is_colorful_baseclass(class_name) {
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }?;

    // arbitrary value
    let (input, _) = tag("-")(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = multispace0(input)?;
    // is hex color
    // let (input, _) = tag("#")(input)?;
    // let (input, color) = take_while1(|c: char| c.is_ascii_hexdigit())(input)?;
    // should be length 3 or 6
    let (input, color) = take_until("]")(input)?;
    let ((input, _)) = if is_hex_color(color.trim()) {
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }?;

    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}
// e.g: [mask-type:alpha]
fn kv_pair_classname(input: &str) -> IResult<&str, ()> {
    // let Ok((input, _)) = delimited(
    //     tag("["),
    //     tuple((
    //         take_while1(is_ident_char),
    //         tag(":"),
    //         take_while1(is_ident_char),
    //     )),
    //     tag("]"),
    // )(input);
    // Ok((input, ()))
    let (input, _) = tag("[")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// before:content-['Festivus']
fn arbitrary_content(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("content-['")(input)?;
    let (input, _) = take_until("']")(input)?;
    let (input, _) = tag("']")(input)?;
    Ok((input, ()))
}

// bg-black/25
fn predefined_colorful_opacity(input: &str) -> IResult<&str, ()> {
    let input = if COLORFUL_BASECLASSES
        .iter()
        .any(|cb| input.trim().starts_with(cb))
    {
        input
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };
    let (input, _) = take_while1(|char| is_ident_char(char) && char != '/')(input)?;
    // let (input, _) = take_until("/")(input)?;
    let (input, _) = tag("/")(input)?;

    let (input, num) = number::complete::double(input)?;
    let input = match num as u8 {
        0..=100 => input,
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    Ok((input, ()))
}

// bg-black/[27]
fn arbitrary_opacity(input: &str) -> IResult<&str, ()> {
    let input = if COLORFUL_BASECLASSES
        .iter()
        .any(|cb| input.trim().starts_with(cb))
    {
        input
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };
    let (input, _) = take_while1(|char| is_ident_char(char) && char != '/')(input)?;
    let (input, _) = tag("/")(input)?;
    let (input, _) = tag("[")(input)?;
    // 0-100 integer
    let (input, num) = number::complete::double(input)?;
    let input = match num as u8 {
        0..=100 => input,
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// bg-[url('/img/down-arrow.svg')]
fn bg_arbitrary_url(input: &str) -> IResult<&str, ()> {
    // prefixed by baseclass
    let input = if COLORFUL_BASECLASSES
        .iter()
        .any(|cb| input.trim().starts_with(cb))
    {
        input
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };
    let (input, _) = take_while1(|char| is_ident_char(char) && char != '/')(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("url('")(input)?;
    let (input, _) = take_until("')")(input)?;
    let (input, _) = tag("')")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

fn parse_single_tw_classname(input: &str) -> IResult<&str, ()> {
    alt((
        bg_arbitrary_url,
        predefined_colorful_opacity,
        arbitrary_opacity,
        parse_predefined_tw_classname,
        kv_pair_classname,
        lengthy_arbitrary_classname,
        colorful_arbitrary_baseclass,
        arbitrary_content,
    ))(input)
}

// rules: colon(:) preceeded by either valid identifier or closed bracket
// // postceeded by either valid identifier or open bracket
// // e.g
fn modifier_separator(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(":")(input)?;
    Ok((input, ""))
}

fn modifier(input: &str) -> IResult<&str, &str> {
    let (input, modifier) = recognize(|i| {
        // Assuming a Tailwind class consists of alphanumeric, dashes, and colons
        nom::bytes::complete::is_a(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-",
        )(i)
    })(input)?;

    if is_valid_modifier2(modifier) {
        Ok((input, modifier))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn modifiers_chained(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, modifiers) = separated_list0(tag(":"), modifier)(input)?;
    Ok((input, modifiers))
}

fn parse_tw_full_classname(input: &str) -> IResult<&str, Vec<&str>> {
    // Parses one or more Tailwind class names separated by spaces, allowing optional spaces before and after each class name
    // let (input, class_names) = delimited(
    //     multispace0,
    //     separated_list0(multispace1, parse_single_tw_classname),
    //     multispace0,
    // )(input)?;

    let (input, class_names) = tuple((
        opt(tuple((modifiers_chained, tag(":")))),
        parse_single_tw_classname,
    ))(input)?;

    // Ok((input, class_names))
    Ok((input, vec![]))
}

fn parse_class_names(input: &str) -> IResult<&str, Vec<&str>> {
    // let (input, _) = space0(input)?;
    // let (input, class_names) = separated_list0(space1, parse_tw_full_classname)(input)?;
    // let (input, class_names) = separated_list0(space1, tag("btn"))(input)?;
    let (input, class_names) = separated_list0(multispace1, parse_tw_full_classname)(input)?;
    // let (input, _) = space0(input)?;

    Ok((input, vec![]))
}

fn parse_top(input: &str) -> IResult<&str, Vec<&str>> {
    parse_class_names(input)
    // all_consuming(parse_class_names)(input)
}

// p-6 max-w-sm mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-4
#[proc_macro]
pub fn tww(raw_input: TokenStream) -> TokenStream {
    let r_input = raw_input.clone();
    let input = parse_macro_input!(r_input as LitStr);
    let (modifiers, valid_class_names) = match setup(&input) {
        Ok(value) => value,
        Err(value) => {
            return syn::Error::new_spanned(input, value)
                .to_compile_error()
                .into()
        }
    };
    let full_classnames = input.value();

    let (input, class_names) = match parse_top(&full_classnames) {
        Ok(value) => value,
        Err(value) => {
            return syn::Error::new_spanned(input, value)
                .to_compile_error()
                .into()
        }
    };

    // for word in input.value().split_whitespace() {

    // raw_input
    quote::quote! {
        #input
    }
    .into()
}
