Create a PR for these changes:

Skip to content
Oyelowo
/
tailwind-rust

Type / to search

Code
Issues
1
Pull requests
Actions
Projects
Wiki
Security
Insights
Settings
Open a pull request
Create a new pull request by comparing changes across two branches. If you need to, you can also .
 
...
 
  Able to merge. These branches can be automatically merged.
@Oyelowo
5 use a more robust parsing for tailwind classes
 

Leave a comment
No file chosen
Attach files by dragging & dropping, selecting or pasting them.
Remember, contributions to this repository should follow our GitHub Community Guidelines.
Reviewers
No reviews—at least 1 approving review is required.
Assignees
No one—
Labels
None yet
Projects
None yet
Milestone
No milestone
Development
Use Closing keywords in the description to automatically close issues

Helpful resources
GitHub Community Guidelines
 1 contributor
 Commits 44
 Files changed 8
Showing  with 836 additions and 283 deletions.
  1 change: 1 addition & 0 deletions1  
Cargo.toml
@@ -26,6 +26,7 @@ tw-macro = { path = "tw-macro" }
proc-macro2 = "1.0.66"
quote = "1.0.33"
syn = "2.0.29"
nom = "7.1.3"
static_assertions = "1.1.0"
serde = { version = "1.0.188", features = ["derive"] }
serde_json = "1.0.105"
  15 changes: 10 additions & 5 deletions15  
tailwind/src/lib.rs
@@ -82,9 +82,9 @@ fn _unsupported_media_query() {}
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("px-[45]");
/// tw!("px-45]");
/// ```
fn _missing_unit_after_arbitrary_value() {}
fn _malformed_arbitrary_value() {}

/// Invalid group usage.
///
@@ -166,10 +166,11 @@ fn _happy_paths() {
        let _classnames = tw!("text-blue-600/[.07]");

        // tw!("[something]");
        let _classnames = tw!("px-[-45px]");
        let _classnames = tw!("px-[45.43px]");
        let _classnames = tw!("px-[-45cm]");
        let _classnames = tw!("px-[-45rem]");
        let _classnames = tw!("px-[-45em]");
        let _classnames = tw!("px-[45em]");
        let _classnames = tw!("px-[-45%]");
        let _classnames = tw!("px-[-45in]");
        let _classnames = tw!("px-[-45vh]");
@@ -178,20 +179,24 @@ fn _happy_paths() {
        let _classnames = tw!("px-[-45vmax]");
        let _classnames = tw!("px-[-45mm]");
        let _classnames = tw!("px-[-45pc]");
        let _classnames = tw!("px-[0px]");
        let _classnames = tw!("px-[0]");
        let _classnames = tw!("px-[45px]");
        let _classnames = tw!("px-[45cm]");
        let _classnames = tw!("px-[45rem]");
        let _classnames = tw!("px-[45em]");
        tw!("bg-taxvhiti");

        // let _classnames = tw!("px-[45em]");
        let _classnames = tw!("px-[45%]");
        let _classnames = tw!("px-[45in]");
        let _classnames = tw!("px-[45vh]");
        let _classnames = tw!("px-[45vw]");
        let _classnames = tw!("px-[45vmin]");
        let _classnames = tw!("px-[45vmax]");
        let _classnames = tw!("px-[45mm]");
        let _classnames = tw!("px-[45.5mm]");
        let _classnames = tw!("px-[45pc]");
        let _classnames = tw!("py-[0]");
        let _classnames = tw!("px-[45pc]");
        let _classnames = tw!("-px-[45pc]");
        let _classnames = tw!("hover:[mask-type:alpha]");
        let _classnames = tw!(
  116 changes: 116 additions & 0 deletions116  
tailwind/src/main.rs
  1 change: 1 addition & 0 deletions1  
tw-macro/Cargo.toml
 958 changes: 680 additions & 278 deletions958  
tw-macro/src/lib.rs
@@ -4,116 +4,31 @@
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{digit1, multispace0, multispace1},
    combinator::{all_consuming, not, opt, recognize},
    multi::separated_list0,
    number,
    sequence::{preceded, tuple},
    IResult,
};
use syn::{parse_macro_input, LitStr};
mod config;
mod plugins;
mod tailwind;
use tailwind::{
    lengthy::LENGTHY, modifiers::get_modifiers, tailwind_config::CustomisableClasses,
    valid_baseclass_names::VALID_BASECLASS_NAMES,
    colorful::COLORFUL_BASECLASSES, lengthy::LENGTHY, modifiers::get_modifiers,
    tailwind_config::CustomisableClasses, valid_baseclass_names::VALID_BASECLASS_NAMES,
};

use config::{get_classes, noconfig::UNCONFIGURABLE, read_tailwind_config};
use proc_macro::TokenStream;
use regex::{self, Regex};
use tailwind::signable::SIGNABLES;
// use tailwindcss_core::parser::{Extractor, ExtractorOptions};

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
@@ -146,199 +61,686 @@ fn setup(input: &LitStr) -> Result<(Vec<String>, Vec<String>), TokenStream> {
    Ok((modifiers, valid_class_names))
}

fn get_last_word_types(word: &str) -> (&str, &str) {
    let modifiers_and_class = word.split(':');
fn get_classes_straight() -> Vec<String> {
    get_classes(&read_tailwind_config().unwrap())
}

fn is_valid_classname(class_name: &str) -> bool {
    get_classes_straight().contains(&class_name.to_string())
}

fn is_valid_modifier(modifier: &str) -> bool {
    get_modifiers(&read_tailwind_config().unwrap()).contains(&modifier.to_string())
}

fn parse_predefined_tw_classname(input: &str) -> IResult<&str, ()> {
    let (input, class_name) = recognize(|i| {
        // Considering a Tailwind class consists of alphanumeric, dashes, and slash
        nom::bytes::complete::is_a(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-./",
        )(i)
    })(input)?;

    let is_signable = SIGNABLES.iter().any(|s| {
        class_name
            .strip_prefix('-')
            .unwrap_or(class_name)
            .starts_with(s)
    });

    if is_signable && is_valid_classname(class_name.strip_prefix('-').unwrap_or(class_name))
        || !is_signable && is_valid_classname(class_name)
    {
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
    LENGTHY.contains(&class_name.strip_prefix('-').unwrap_or(class_name))
}

// Custom number parser that handles optional decimals and signs, and scientific notation
fn float_strict(input: &str) -> IResult<&str, f64> {
    let (input, number) = recognize(tuple((
        opt(alt((tag("-"), tag("+")))),
        digit1,
        opt(preceded(tag("."), digit1)),
        opt(tuple((
            alt((tag("e"), tag("E"))),
            opt(alt((tag("-"), tag("+")))),
            digit1,
        ))),
    )))(input)?;

    let float_val: f64 = number.parse().unwrap();
    Ok((input, float_val))
}

fn parse_length_unit(input: &str) -> IResult<&str, String> {
    let (input, number) = float_strict(input)?;
    let (input, unit) = {
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
            // TODO: Should i allow unitless values? Would need something like this in caller
            // location if so:
            // let (input, _) = alt((parse_length_unit, parse_number))(input)?;
            tag(""),
        ))
    }(input)?;
    Ok((input, format!("{}{}", number, unit)))
}

    // let is_arbitrary_property = word.starts_with('[') && word.ends_with(']');
    let last_word_signed = modifiers_and_class.clone().last().unwrap_or_default();
    let last_word_unsigned = last_word_signed
        .strip_prefix('-')
        .unwrap_or(last_word_signed);
// text-[22px]
fn lengthy_arbitrary_classname(input: &str) -> IResult<&str, ()> {
    let (input, class_name) = take_until("-[")(input)?;
    let (input, _) = if is_lengthy_classname(class_name) {
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
    // is number
    let (input, _) = parse_length_unit(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

    (last_word_signed, last_word_unsigned)
// #bada55
fn parse_hex_color(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("#")(input)?;
    let (input, color) = take_while1(|c: char| c.is_ascii_hexdigit())(input)?;
    let (input, _) = if color.chars().count() == 3 || color.chars().count() == 6 {
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }?;
    let color = format!("#{}", color);
    Ok((input, color))
}

fn is_valid_modifier(word: &str, modifiers: &[String]) -> bool {
    let modifiers_and_class = word.split(':');
    let modifiers_from_word = modifiers_and_class
        .clone()
        .take(modifiers_and_class.count() - 1)
        .collect::<Vec<&str>>();
    modifiers_from_word
fn parse_u8(input: &str) -> IResult<&str, u8> {
    let (input, num) = number::complete::double(input)?;
    let input = match num as u32 {
        0..=255 => input,
        _ => {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };
    Ok((input, num as u8))
}

// rgb(255, 255, 255) rgb(255_255_255)
fn parse_rgb_color(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("rgb(")(input)?;
    let (input, r) = parse_u8(input)?;
    let (input, _) = alt((tag(","), tag("_")))(input)?;
    let (input, g) = parse_u8(input)?;
    let (input, _) = alt((tag(","), tag("_")))(input)?;
    let (input, b) = parse_u8(input)?;
    let (input, _) = tag(")")(input)?;
    let color = format!("rgb({}, {}, {})", r, g, b);
    Ok((input, color))
}

// rgba(255, 255, 255, 0.5) rgba(255_255_255_0.5)
fn parse_rgba_color(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("rgba(")(input)?;
    let (input, r) = parse_u8(input)?;
    let (input, _) = alt((tag(","), tag("_")))(input)?;
    let (input, g) = parse_u8(input)?;
    let (input, _) = alt((tag(","), tag("_")))(input)?;
    let (input, b) = parse_u8(input)?;
    let (input, _) = alt((tag(","), tag("_")))(input)?;
    let (input, a) = number::complete::double(input)?;
    let (input, _) = tag(")")(input)?;
    let color = format!("rgba({}, {}, {}, {})", r, g, b, a);
    Ok((input, color))
}

fn is_colorful_baseclass(class_name: &str) -> bool {
    COLORFUL_BASECLASSES.contains(&class_name)
}

// text-[#bada55]
fn colorful_arbitrary_baseclass(input: &str) -> IResult<&str, ()> {
    let (input, class_name) = take_until("-[")(input)?;
    let (input, _) = if is_colorful_baseclass(class_name) {
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
    let (input, _) = alt((parse_hex_color, parse_rgb_color, parse_rgba_color))(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// e.g: [mask-type:alpha]
fn kv_pair_classname(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("[")(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_until("]")(input)?;
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

// content-[>] content-[<]
fn arbitrary_with_arrow(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_while1(is_ident_char)(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = alt((tag(">"), tag("<")))(input)?;
    let (input, _) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// bg-black/25
fn predefined_colorful_opacity(input: &str) -> IResult<&str, ()> {
    let input = if COLORFUL_BASECLASSES
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
        .any(|cb| input.trim().starts_with(cb))
    {
        input
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
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

    Ok((input, ()))
}

// bg-black/[27] bg-black/[27%]
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
    let (input, _) = opt(tag("%"))(input)?;
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
    let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = tag("url('")(input)?;
    let (input, _) = take_until("')")(input)?;
    let (input, _) = tag("')")(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// grid-cols-[fit-content(theme(spacing.32))]
fn arbitrary_css_value(input: &str) -> IResult<&str, ()> {
    // is prefixed by valid base class
    // take until -[
    let (input, base_class) = take_until("-[")(input)?;
    let input = if VALID_BASECLASS_NAMES
        .iter()
        .any(|cb| base_class.trim().eq(*cb))
    {
        input
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            base_class,
            nom::error::ErrorKind::Tag,
        )));
    };
    let (input, _) = tag("-[")(input)?;
    let (input, _) = not(alt((
        tag("--"),
        tag("var(--"),
        // <ident>:var(--
    )))(input)?;
    let (input, _) = take_while1(|char| is_ident_char(char) && char != '(')(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = take_until(")]")(input)?;

    // allow anything inthe brackets
    let (input, _) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// bg-[--my-color]
fn arbitrary_css_var(input: &str) -> IResult<&str, ()> {
    // is prefixed by valid base class
    let input = if VALID_BASECLASS_NAMES
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
    let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = tag("--")(input)?;
    let (input, _) = take_while1(|char| is_ident_char(char) && char != ']')(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}
// text-[var(--my-var)]
fn arbitrary_css_var2(input: &str) -> IResult<&str, ()> {
    // is prefixed by valid base class
    let input = if VALID_BASECLASS_NAMES
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
    let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = tag("var(--")(input)?;
    let (input, _) = take_while1(|char| is_ident_char(char) && char != ')')(input)?;
    let (input, _) = tag(")]")(input)?;
    Ok((input, ()))
}

// text-[length:var(--my-var)]
fn arbitrary_css_var3(input: &str) -> IResult<&str, ()> {
    // is prefixed by valid base class
    let input = if VALID_BASECLASS_NAMES
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
    let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = take_while1(|char| is_ident_char(char) && char != ':')(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = tag("var(--")(input)?;
    let (input, _) = take_while1(|char| is_ident_char(char) && char != ')')(input)?;
    let (input, _) = tag(")]")(input)?;
    Ok((input, ()))
}

// group/edit
fn arbitrary_group_classname(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((tag("group"),))(input)?;
    let (input, _) = tag("/")(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    Ok((input, ()))
}

fn parse_single_tw_classname(input: &str) -> IResult<&str, ()> {
    alt((
        // bg-[url('/what_a_rush.png')]
        bg_arbitrary_url,
        // bg-black/25
        predefined_colorful_opacity,
        // group/edit
        arbitrary_group_classname,
        // bg-black/[27]
        arbitrary_opacity,
        // btn
        parse_predefined_tw_classname,
        // [mask-type:luminance] [mask-type:alpha]
        kv_pair_classname,
        // text-[22px]
        lengthy_arbitrary_classname,
        // text-[#bada55]
        colorful_arbitrary_baseclass,
        // before:content-['Festivus']
        arbitrary_content,
        // content-[>] content-[<]
        arbitrary_with_arrow,
        // bg-[--my-color]
        arbitrary_css_var,
        // text-[var(--my-var)]
        arbitrary_css_var2,
        // text-[length:var(--my-var)]
        arbitrary_css_var3,
        // grid-cols-[fit-content(theme(spacing.32))]
        arbitrary_css_value,
    ))(input)
}

// hover:underline
fn predefined_modifier(input: &str) -> IResult<&str, ()> {
    let (input, modifier) = recognize(|i| {
        // Assuming a Tailwind class consists of alphanumeric, dashes, and colons
        nom::bytes::complete::is_a(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-",
        )(i)
    })(input)?;

    if is_valid_modifier(modifier) {
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

// predefined special modifiers e.g peer-checked:p-4 group-hover:visible
fn predefined_special_modifier(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((
        // peer-checked:p-4
        tuple((tag("peer-"), predefined_modifier)),
        // group-hover:visible
        tuple((tag("group-"), predefined_modifier)),
    ))(input)?;
    Ok((input, ()))
}

// [&:nth-child(3)]:underline
// [&_p]:mt-4
fn arbitrary_front_selector_modifier(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("[&")(input)?;
    let (input, _) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// group-[:nth-of-type(3)_&]:block
fn arbitrary_back_selector_modifier(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?;
    let (input, _) = tag("-[")(input)?;
    let (input, _) = take_until("&]")(input)?;
    let (input, _) = tag("&]")(input)?;
    Ok((input, ()))
}

// [@supports(display:grid)]:grid
fn arbitrary_at_supports_rule_modifier(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("[@supports(")(input)?;
    let (input, _) = take_until(")")(input)?;
    let (input, _) = tag(")]")(input)?;
    Ok((input, ()))
}

// [@media(any-hover:hover){&:hover}]:opacity-100
fn arbitrary_at_media_rule_modifier(input: &str) -> IResult<&str, ()> {
    // starts with [@media and ends with ]
    let (input, _) = tag("[@media(")(input)?;
    let (input, _) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// group/edit invisible hover:bg-slate-200 group-hover/item:visible
fn group_peer_modifier(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((
        tuple((tag("group-"), predefined_modifier)),
        // https://tailwindcss.com/docs/hover-focus-and-other-states#differentiating-peers
        // peer-checked/published:text-sky-500
        tuple((tag("peer-"), predefined_modifier)),
    ))(input)?;
    let (input, _) = tag("/")(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    Ok((input, ()))
}

// hidden group-[.is-published]:block
// group-[:nth-of-type(3)_&]:block
// peer-[.is-dirty]:peer-required:block hidden
// hidden peer-[:nth-of-type(3)_&]:block
fn group_modifier_selector(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((tag("group"), tag("peer")))(input)?;
    let (input, _) = tag("-[")(input)?;
    let (input, _) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

    is_valid_last_word
// supports-[backdrop-filter]
fn supports_arbitrary(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("supports-[")(input)?;
    let (input, _) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

fn is_valid_group_classname(class_name: &str) -> bool {
    !class_name.contains(':')
        && !class_name.contains('[')
        && !class_name.contains(']')
        && class_name.starts_with("group/")
// aria-[sort=ascending]:bg-[url('/img/down-arrow.svg')]
// aria-[sort=descending]:bg-[url('/img/up-arrow.svg')]
fn aria_arbitrary(input: &str) -> IResult<&str, ()> {
    let (input, _) = opt(tag("group-"))(input)?;
    let (input, _) = tag("aria-[")(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

fn is_valid_string(s: &str) -> bool {
    // Matches strings that contain only alphanumeric characters, underscores, and hyphens.
    let re = Regex::new(r"^[a-zA-Z0-9_-]*$").expect("Invalid regex");
    re.is_match(s) && !s.is_empty()
// data-[size=large]:p-8
fn data_arbitrary(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("data-[")(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// min-[320px]:text-center max-[600px]:bg-sky-300
fn min_max_arbitrary_modifier(input: &str) -> IResult<&str, ()> {
    let (input, _) = alt((tag("min-"), tag("max-")))(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = parse_length_unit(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

fn modifier(input: &str) -> IResult<&str, ()> {
    alt((
        group_modifier_selector,
        group_peer_modifier,
        predefined_special_modifier,
        arbitrary_front_selector_modifier,
        arbitrary_back_selector_modifier,
        arbitrary_at_supports_rule_modifier,
        arbitrary_at_media_rule_modifier,
        predefined_modifier,
        supports_arbitrary,
        aria_arbitrary,
        data_arbitrary,
        min_max_arbitrary_modifier,
    ))(input)
}

fn modifiers_chained(input: &str) -> IResult<&str, ()> {
    let (input, _modifiers) = separated_list0(tag(":"), modifier)(input)?;
    Ok((input, ()))
}

fn parse_tw_full_classname(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _class_names) = tuple((
        opt(tuple((modifiers_chained, tag(":")))),
        parse_single_tw_classname,
    ))(input)?;

    Ok((input, vec![]))
}

// Edge cases
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
fn parse_class_names(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = multispace0(input)?;
    let (input, _class_names) = separated_list0(multispace1, parse_tw_full_classname)(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, vec![]))
}

fn parse_top(input: &str) -> IResult<&str, Vec<&str>> {
    all_consuming(parse_class_names)(input)
}

#[proc_macro]
pub fn tw(raw_input: TokenStream) -> TokenStream {
    let r_input = raw_input.clone();
    let input = parse_macro_input!(r_input as LitStr);
    let (_modifiers, _valid_class_names) = match setup(&input) {
        Ok(value) => value,
        Err(value) => {
            return syn::Error::new_spanned(input, value)
                .to_compile_error()
                .into()
        }
    };
    let full_classnames = input.value();

    let (input, _class_names) = match parse_top(&full_classnames) {
        Ok(value) => value,
        Err(value) => {
            return syn::Error::new_spanned(input, value)
                .to_compile_error()
                .into()
        }
    };

    quote::quote! {
        #input
    }
    .into()
}
 24 changes: 24 additions & 0 deletions24  
tw-macro/src/tailwind/colorful.rs
@@ -0,0 +1,24 @@
pub const COLORFUL_BASECLASSES: [&str; 22] = [
    "text",
    "bg",
    "border",
    "border-x",
    "border-y",
    "border-s",
    "border-e",
    "border-t",
    "border-r",
    "border-b",
    "border-l",
    "divide",
    "outline",
    "ring",
    "ring-offset",
    "shadow",
    "caret",
    "accent",
    "fill",
    "stroke",
    "placeholder",
    "decoration",
];
  1 change: 1 addition & 0 deletions1  
tw-macro/src/tailwind/mod.rs
@@ -4,6 +4,7 @@
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
pub mod colorful;
pub mod default_classnames;
pub mod lengthy;
pub mod modifiers;
  3 changes: 3 additions & 0 deletions3  
tw-macro/src/tailwind/signable.rs
@@ -45,4 +45,7 @@ pub const SIGNABLES: [&str; 40] = [
    "grid-auto-columns",
    "z",
    "order",
    // "scroll-mx",
    // "scroll-my",
    // "scroll-m",
];
Footer
© 2023 GitHub, Inc.
Footer navigation
Terms
Privacy
Security
Status
Docs
Contact GitHub
Pricing
API
Training
Blog
About
@@ -4,116 +4,31 @@ * Copyright (c) 2023 Oyelowo Oyedayo * Licensed under the MIT license */ use nom::{ branch::alt, bytes::complete::{tag, take_until, take_while1}, character::complete::{digit1, multispace0, multispace1}, combinator::{all_consuming, not, opt, recognize}, multi::separated_list0, number, sequence::{preceded, tuple}, IResult, }; use syn::{parse_macro_input, LitStr}; mod config; mod plugins; mod tailwind; use tailwind::{ lengthy::LENGTHY, modifiers::get_modifiers, tailwind_config::CustomisableClasses, valid_baseclass_names::VALID_BASECLASS_NAMES, colorful::COLORFUL_BASECLASSES, lengthy::LENGTHY, modifiers::get_modifiers, tailwind_config::CustomisableClasses, valid_baseclass_names::VALID_BASECLASS_NAMES, }; use config::{get_classes, noconfig::UNCONFIGURABLE, read_tailwind_config}; use proc_macro::TokenStream; use regex::{self, Regex}; use tailwind::signable::SIGNABLES; // use tailwindcss_core::parser::{Extractor, ExtractorOptions}; #[proc_macro] pub fn tw(raw_input: TokenStream) -> TokenStream { let r_input = raw_input.clone(); let input = parse_macro_input!(r_input as LitStr); let (modifiers, valid_class_names) = match setup(&input) { Ok(value) => value, Err(value) => { return syn::Error::new_spanned(input, value) .to_compile_error() .into() } }; for word in input.value().split_whitespace() { let (last_word_signed, last_word_unsigned) = get_last_word_types(word); // modifiers e.g hover: in // hover:[mask-type:alpha] let is_valid_arb_prop = is_valid_arb_prop(word, &modifiers); let is_valid_class = is_valid_class(is_valid_arb_prop, &valid_class_names, last_word_unsigned); let (base_classname, arbitrary_value_with_bracket) = last_word_unsigned.split_once("-[").unwrap_or_default(); let is_valid_negative_baseclass = is_valid_negative_baseclass( &valid_class_names, last_word_unsigned, last_word_signed, is_valid_arb_prop, ); let prefix_is_valid_tailwind_keyword = VALID_BASECLASS_NAMES.contains(&base_classname); let is_arbitrary_value = prefix_is_valid_tailwind_keyword && arbitrary_value_with_bracket.ends_with(']'); let arbitrary_value = arbitrary_value_with_bracket.trim_end_matches(']'); let is_lengthy_class = LENGTHY.contains(&base_classname); let is_valid_length = is_arbitrary_value && is_lengthy_class && (is_valid_length(arbitrary_value) || is_valid_calc(arbitrary_value)); let has_arb_variant = has_arb_variant(word); let is_valid_opacity = is_valid_opacity(last_word_unsigned, &valid_class_names); if (is_valid_class && is_valid_modifier(word, &modifiers)) || is_valid_negative_baseclass || (!is_lengthy_class && is_arbitrary_value) || is_valid_length || is_valid_arb_prop || has_arb_variant || is_valid_opacity || is_valid_group_classname(last_word_unsigned) || is_validate_modifier_or_group(word, &modifiers, &valid_class_names) { // if check_word(word, false).is_empty() { // return syn::Error::new_spanned(input, format!("Invalid string: {}", word)) // .to_compile_error() // .into(); // } } else { return syn::Error::new_spanned(input, format!("Invalid string: {word}")) .to_compile_error() .into(); } } raw_input } // fn check_word(input: &str, loose: bool) -> Vec<&str> { // Extractor::unique_ord( // input.as_bytes(), // ExtractorOptions { // preserve_spaces_in_arbitrary: loose, // }, // ) // .into_iter() // .map(|s| unsafe { std::str::from_utf8_unchecked(s) }) // .collect() // } fn is_valid_length(value: &str) -> bool { let re = regex::Regex::new(r"^(-?\d+(\.?\d+)?(px|em|rem|%|cm|mm|in|pt|pc|vh|vw|vmin|vmax)|0)$") .expect("Invalid regex"); re.is_match(value) } fn is_valid_calc(value: &str) -> bool { let re = regex::Regex::new(r"^calc\([^)]+\)$").expect("Invalid regex"); re.is_match(value) } fn setup(input: &LitStr) -> Result<(Vec<String>, Vec<String>), TokenStream> { let config = &(match read_tailwind_config() { Ok(config) => config, @@ -146,199 +61,686 @@ fn setup(input: &LitStr) -> Result<(Vec<String>, Vec<String>), TokenStream> { Ok((modifiers, valid_class_names)) } fn get_last_word_types(word: &str) -> (&str, &str) { let modifiers_and_class = word.split(':'); fn get_classes_straight() -> Vec<String> { get_classes(&read_tailwind_config().unwrap()) } fn is_valid_classname(class_name: &str) -> bool { get_classes_straight().contains(&class_name.to_string()) } fn is_valid_modifier(modifier: &str) -> bool { get_modifiers(&read_tailwind_config().unwrap()).contains(&modifier.to_string()) } fn parse_predefined_tw_classname(input: &str) -> IResult<&str, ()> { let (input, class_name) = recognize(|i| { // Considering a Tailwind class consists of alphanumeric, dashes, and slash nom::bytes::complete::is_a( "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-./", )(i) })(input)?; let is_signable = SIGNABLES.iter().any(|s| { class_name .strip_prefix('-') .unwrap_or(class_name) .starts_with(s) }); if is_signable && is_valid_classname(class_name.strip_prefix('-').unwrap_or(class_name)) || !is_signable && is_valid_classname(class_name) { Ok((input, ())) } else { Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))) } } fn is_ident_char(c: char) -> bool { c.is_alphanumeric() || c == '_' || c == '-' } fn is_lengthy_classname(class_name: &str) -> bool { LENGTHY.contains(&class_name.strip_prefix('-').unwrap_or(class_name)) } // Custom number parser that handles optional decimals and signs, and scientific notation fn float_strict(input: &str) -> IResult<&str, f64> { let (input, number) = recognize(tuple(( opt(alt((tag("-"), tag("+")))), digit1, opt(preceded(tag("."), digit1)), opt(tuple(( alt((tag("e"), tag("E"))), opt(alt((tag("-"), tag("+")))), digit1, ))), )))(input)?; let float_val: f64 = number.parse().unwrap(); Ok((input, float_val)) } fn parse_length_unit(input: &str) -> IResult<&str, String> { let (input, number) = float_strict(input)?; let (input, unit) = { // px|em|rem|%|cm|mm|in|pt|pc|vh|vw|vmin|vmax alt(( tag("px"), tag("em"), tag("rem"), tag("%"), tag("cm"), tag("mm"), tag("in"), tag("pt"), tag("pc"), tag("vh"), tag("vw"), tag("vmin"), tag("vmax"), // TODO: Should i allow unitless values? Would need something like this in caller // location if so: // let (input, _) = alt((parse_length_unit, parse_number))(input)?; tag(""), )) }(input)?; Ok((input, format!("{}{}", number, unit))) } // let is_arbitrary_property = word.starts_with('[') && word.ends_with(']'); let last_word_signed = modifiers_and_class.clone().last().unwrap_or_default(); let last_word_unsigned = last_word_signed .strip_prefix('-') .unwrap_or(last_word_signed); // text-[22px] fn lengthy_arbitrary_classname(input: &str) -> IResult<&str, ()> { let (input, class_name) = take_until("-[")(input)?; let (input, _) = if is_lengthy_classname(class_name) { Ok((input, ())) } else { Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))) }?; // arbitrary value let (input, _) = tag("-")(input)?; let (input, _) = tag("[")(input)?; // is number let (input, _) = parse_length_unit(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } (last_word_signed, last_word_unsigned) // #bada55 fn parse_hex_color(input: &str) -> IResult<&str, String> { let (input, _) = tag("#")(input)?; let (input, color) = take_while1(|c: char| c.is_ascii_hexdigit())(input)?; let (input, _) = if color.chars().count() == 3 || color.chars().count() == 6 { Ok((input, ())) } else { Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))) }?; let color = format!("#{}", color); Ok((input, color)) } fn is_valid_modifier(word: &str, modifiers: &[String]) -> bool { let modifiers_and_class = word.split(':'); let modifiers_from_word = modifiers_and_class .clone() .take(modifiers_and_class.count() - 1) .collect::<Vec<&str>>(); modifiers_from_word fn parse_u8(input: &str) -> IResult<&str, u8> { let (input, num) = number::complete::double(input)?; let input = match num as u32 { 0..=255 => input, _ => { return Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))) } }; Ok((input, num as u8)) } // rgb(255, 255, 255) rgb(255_255_255) fn parse_rgb_color(input: &str) -> IResult<&str, String> { let (input, _) = tag("rgb(")(input)?; let (input, r) = parse_u8(input)?; let (input, _) = alt((tag(","), tag("_")))(input)?; let (input, g) = parse_u8(input)?; let (input, _) = alt((tag(","), tag("_")))(input)?; let (input, b) = parse_u8(input)?; let (input, _) = tag(")")(input)?; let color = format!("rgb({}, {}, {})", r, g, b); Ok((input, color)) } // rgba(255, 255, 255, 0.5) rgba(255_255_255_0.5) fn parse_rgba_color(input: &str) -> IResult<&str, String> { let (input, _) = tag("rgba(")(input)?; let (input, r) = parse_u8(input)?; let (input, _) = alt((tag(","), tag("_")))(input)?; let (input, g) = parse_u8(input)?; let (input, _) = alt((tag(","), tag("_")))(input)?; let (input, b) = parse_u8(input)?; let (input, _) = alt((tag(","), tag("_")))(input)?; let (input, a) = number::complete::double(input)?; let (input, _) = tag(")")(input)?; let color = format!("rgba({}, {}, {}, {})", r, g, b, a); Ok((input, color)) } fn is_colorful_baseclass(class_name: &str) -> bool { COLORFUL_BASECLASSES.contains(&class_name) } // text-[#bada55] fn colorful_arbitrary_baseclass(input: &str) -> IResult<&str, ()> { let (input, class_name) = take_until("-[")(input)?; let (input, _) = if is_colorful_baseclass(class_name) { Ok((input, ())) } else { Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))) }?; // arbitrary value let (input, _) = tag("-")(input)?; let (input, _) = tag("[")(input)?; let (input, _) = alt((parse_hex_color, parse_rgb_color, parse_rgba_color))(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // e.g: [mask-type:alpha] fn kv_pair_classname(input: &str) -> IResult<&str, ()> { let (input, _) = tag("[")(input)?; let (input, _) = take_while1(is_ident_char)(input)?; let (input, _) = tag(":")(input)?; let (input, _) = take_until("]")(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // before:content-['Festivus'] fn arbitrary_content(input: &str) -> IResult<&str, ()> { let (input, _) = tag("content-['")(input)?; let (input, _) = take_until("']")(input)?; let (input, _) = tag("']")(input)?; Ok((input, ())) } // content-[>] content-[<] fn arbitrary_with_arrow(input: &str) -> IResult<&str, ()> { let (input, _) = take_while1(is_ident_char)(input)?; let (input, _) = tag("[")(input)?; let (input, _) = alt((tag(">"), tag("<")))(input)?; let (input, _) = take_until("]")(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // bg-black/25 fn predefined_colorful_opacity(input: &str) -> IResult<&str, ()> { let input = if COLORFUL_BASECLASSES .iter() .all(|modifier| modifiers.contains(&modifier.to_string())) } fn is_valid_opacity(last_word_unsigned: &str, valid_class_names: &[String]) -> bool { let is_valid_opacity = { let (class_name, opacity_raw) = last_word_unsigned.split_once('/').unwrap_or_default(); let opacity_arb = opacity_raw .trim_start_matches('[') .trim_end_matches(']') .parse::<f32>(); let is_valid_number = opacity_arb.is_ok_and(|opacity_num| (0.0..=100.0).contains(&opacity_num)); valid_class_names.contains(&class_name.to_string()) && is_valid_number .any(|cb| input.trim().starts_with(cb)) { input } else { return Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))); }; is_valid_opacity } fn has_arb_variant(word: &str) -> bool { // lg:[&:nth-child(3)]:hover:underline // [&_p]:mt-4 // flex [@supports(display:grid)]:grid // [@media(any-hover:hover){&:hover}]:opacity-100 let has_arb_variant = { // lg:[&:nth-child(3)]:hover:underline => :nth-child(3) // [&_p]:mt-4 => _p let mut ampersand_variant_selector = word.split("[@").last().unwrap_or_default().split("]:"); let and_variant_selector = word.split("[&").last().unwrap_or_default().split("]:"); let is_valid_arbitrary_variant_selector = ampersand_variant_selector.clone().count() >= 2 && !ampersand_variant_selector .next() .unwrap_or_default() .is_empty(); let is_valid_arbitrary_variant_queries = and_variant_selector.clone().count() >= 2 && !and_variant_selector .clone() .last() .unwrap_or_default() .split("]:") .next() .unwrap_or_default() .is_empty(); let is_query = word.starts_with("[@"); is_valid_arbitrary_variant_selector || is_valid_arbitrary_variant_queries || is_query // && // ((!is_query && !word.split("[&").next().unwrap_or_default().is_empty() && word.split(":[&").count() >= 2) || is_query) let (input, _) = take_while1(|char| is_ident_char(char) && char != '/')(input)?; // let (input, _) = take_until("/")(input)?; let (input, _) = tag("/")(input)?; let (input, num) = number::complete::double(input)?; let input = match num as u8 { 0..=100 => input, _ => { return Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))) } }; has_arb_variant } fn is_valid_negative_baseclass( valid_class_names: &[String], last_word_unsigned: &str, last_word_signed: &str, is_valid_arb_prop: bool, ) -> bool { let is_valid_negative_baseclass = { // tw!("-m-4 p-4 p-4"); (valid_class_names.contains(&last_word_unsigned.to_string()) && last_word_signed.starts_with('-') && SIGNABLES .iter() .any(|s| (last_word_unsigned.starts_with(s)))) || (is_valid_arb_prop && last_word_signed.starts_with('-') && SIGNABLES.iter().any(|s| last_word_unsigned.starts_with(s))) Ok((input, ())) } // bg-black/[27] bg-black/[27%] fn arbitrary_opacity(input: &str) -> IResult<&str, ()> { let input = if COLORFUL_BASECLASSES .iter() .any(|cb| input.trim().starts_with(cb)) { input } else { return Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))); }; is_valid_negative_baseclass } fn is_valid_class( is_valid_arb_prop: bool, valid_class_names: &[String], last_word_unsigned: &str, ) -> bool { !is_valid_arb_prop && valid_class_names.contains(&last_word_unsigned.to_string()) } fn is_valid_arb_prop(word: &str, modifiers: &[String]) -> bool { // TODO: check the first and the last character are not open and close brackets // respectively i.e arbitrary property e.g [mask_type:aplha]; // hover:[mask-type:alpha]; let mut word_for_arb_prop = word.split(":["); word_for_arb_prop .next() // e.g for hover:[mask-type:alpha], this will be hover, // for [mask-type:alpha], this will be [mask-type:alpha] .is_some_and(|modifiers_or_full_arb_prop| { let is_arbitrary_property = modifiers_or_full_arb_prop.starts_with('[') && modifiers_or_full_arb_prop.ends_with(']'); let is_valid = if is_arbitrary_property { modifiers_or_full_arb_prop.matches('[').count() == 1 && modifiers_or_full_arb_prop.matches(']').count() == 1 && modifiers_or_full_arb_prop .trim_start_matches('[') .trim_end_matches(']') .split(':') .count() == 2 } else { // e.g mask-type:alpha] in hover:[mask-type:alpha] let full_arb_prop = word_for_arb_prop.next().unwrap_or_default(); // e.g for single, hover in hover:[mask-type:alpha] // for multiple, hover:first:last, in hover:first:last:[mask-type:alpha] modifiers_or_full_arb_prop .split(':') .all(|modifier| modifiers.contains(&modifier.to_string())) && full_arb_prop.matches(']').count() == 1 && full_arb_prop .trim_end_matches(']') .split(':') .count() == 2 }; is_valid }) || // value e.g [mask-type:alpha] in hover:[mask-type:alpha] // potential addition checks(probably not a good idea. Imagine a new css property, we would // have to open a PR for every new or esoteric css property.) word_for_arb_prop.next().is_some_and(|value| { value.ends_with(']') && value.split(':').count() == 2 // We had already split by ":[", so there should be no "[" anymore && value.matches('[').count() == 0 && value.matches(']').count() == 1 }) } fn is_valid_group_pattern(modifier: &str, valid_modifiers: &[String]) -> bool { let parts: Vec<&str> = modifier.split('/').collect(); let group_modifier = parts[0]; parts.len() == 2 && valid_modifiers.contains(&group_modifier.to_string()) && group_modifier.starts_with("group") } // tw!("group/edit invisible hover:bg-slate-200 group-hover/item:visible"); // tw!("group-[:nth-of-type(3)_&]:block group-hover/edit:text-gray-700 group-[:nth-of-type(3)_&]:block"); fn is_validate_modifier_or_group( word: &str, valid_modifiers: &[String], valid_class_names: &[String], ) -> bool { let valid_arb_group = word.split(':').collect::<Vec<&str>>(); let modifiers = &valid_arb_group[..valid_arb_group.len() - 1]; let last_word = valid_arb_group.last().unwrap_or(&""); let is_valid_last_word = is_valid_string(last_word) && valid_class_names.contains(&last_word.to_string()); for modifier in modifiers { if modifier.starts_with("group") { if !is_valid_group_pattern(modifier, valid_modifiers) && is_valid_last_word { return false; } } else if !valid_modifiers.contains(&modifier.to_string()) && is_valid_last_word { return false; let (input, _) = take_while1(|char| is_ident_char(char) && char != '/')(input)?; let (input, _) = tag("/")(input)?; let (input, _) = tag("[")(input)?; // 0-100 integer let (input, num) = number::complete::double(input)?; let input = match num as u8 { 0..=100 => input, _ => { return Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))) } }; let (input, _) = opt(tag("%"))(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // bg-[url('/img/down-arrow.svg')] fn bg_arbitrary_url(input: &str) -> IResult<&str, ()> { // prefixed by baseclass let input = if COLORFUL_BASECLASSES .iter() .any(|cb| input.trim().starts_with(cb)) { input } else { return Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))); }; let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?; let (input, _) = tag("[")(input)?; let (input, _) = tag("url('")(input)?; let (input, _) = take_until("')")(input)?; let (input, _) = tag("')")(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // grid-cols-[fit-content(theme(spacing.32))] fn arbitrary_css_value(input: &str) -> IResult<&str, ()> { // is prefixed by valid base class // take until -[ let (input, base_class) = take_until("-[")(input)?; let input = if VALID_BASECLASS_NAMES .iter() .any(|cb| base_class.trim().eq(*cb)) { input } else { return Err(nom::Err::Error(nom::error::Error::new( base_class, nom::error::ErrorKind::Tag, ))); }; let (input, _) = tag("-[")(input)?; let (input, _) = not(alt(( tag("--"), tag("var(--"), // <ident>:var(-- )))(input)?; let (input, _) = take_while1(|char| is_ident_char(char) && char != '(')(input)?; let (input, _) = tag("(")(input)?; let (input, _) = take_until(")]")(input)?; // allow anything inthe brackets let (input, _) = take_until("]")(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // bg-[--my-color] fn arbitrary_css_var(input: &str) -> IResult<&str, ()> { // is prefixed by valid base class let input = if VALID_BASECLASS_NAMES .iter() .any(|cb| input.trim().starts_with(cb)) { input } else { return Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))); }; let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?; let (input, _) = tag("[")(input)?; let (input, _) = tag("--")(input)?; let (input, _) = take_while1(|char| is_ident_char(char) && char != ']')(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // text-[var(--my-var)] fn arbitrary_css_var2(input: &str) -> IResult<&str, ()> { // is prefixed by valid base class let input = if VALID_BASECLASS_NAMES .iter() .any(|cb| input.trim().starts_with(cb)) { input } else { return Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))); }; let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?; let (input, _) = tag("[")(input)?; let (input, _) = tag("var(--")(input)?; let (input, _) = take_while1(|char| is_ident_char(char) && char != ')')(input)?; let (input, _) = tag(")]")(input)?; Ok((input, ())) } // text-[length:var(--my-var)] fn arbitrary_css_var3(input: &str) -> IResult<&str, ()> { // is prefixed by valid base class let input = if VALID_BASECLASS_NAMES .iter() .any(|cb| input.trim().starts_with(cb)) { input } else { return Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))); }; let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?; let (input, _) = tag("[")(input)?; let (input, _) = take_while1(|char| is_ident_char(char) && char != ':')(input)?; let (input, _) = tag(":")(input)?; let (input, _) = tag("var(--")(input)?; let (input, _) = take_while1(|char| is_ident_char(char) && char != ')')(input)?; let (input, _) = tag(")]")(input)?; Ok((input, ())) } // group/edit fn arbitrary_group_classname(input: &str) -> IResult<&str, ()> { let (input, _) = alt((tag("group"),))(input)?; let (input, _) = tag("/")(input)?; let (input, _) = take_while1(is_ident_char)(input)?; Ok((input, ())) } fn parse_single_tw_classname(input: &str) -> IResult<&str, ()> { alt(( // bg-[url('/what_a_rush.png')] bg_arbitrary_url, // bg-black/25 predefined_colorful_opacity, // group/edit arbitrary_group_classname, // bg-black/[27] arbitrary_opacity, // btn parse_predefined_tw_classname, // [mask-type:luminance] [mask-type:alpha] kv_pair_classname, // text-[22px] lengthy_arbitrary_classname, // text-[#bada55] colorful_arbitrary_baseclass, // before:content-['Festivus'] arbitrary_content, // content-[>] content-[<] arbitrary_with_arrow, // bg-[--my-color] arbitrary_css_var, // text-[var(--my-var)] arbitrary_css_var2, // text-[length:var(--my-var)] arbitrary_css_var3, // grid-cols-[fit-content(theme(spacing.32))] arbitrary_css_value, ))(input) } // hover:underline fn predefined_modifier(input: &str) -> IResult<&str, ()> { let (input, modifier) = recognize(|i| { // Assuming a Tailwind class consists of alphanumeric, dashes, and colons nom::bytes::complete::is_a( "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-", )(i) })(input)?; if is_valid_modifier(modifier) { Ok((input, ())) } else { Err(nom::Err::Error(nom::error::Error::new( input, nom::error::ErrorKind::Tag, ))) } } // predefined special modifiers e.g peer-checked:p-4 group-hover:visible fn predefined_special_modifier(input: &str) -> IResult<&str, ()> { let (input, _) = alt(( // peer-checked:p-4 tuple((tag("peer-"), predefined_modifier)), // group-hover:visible tuple((tag("group-"), predefined_modifier)), ))(input)?; Ok((input, ())) } // [&:nth-child(3)]:underline // [&_p]:mt-4 fn arbitrary_front_selector_modifier(input: &str) -> IResult<&str, ()> { let (input, _) = tag("[&")(input)?; let (input, _) = take_until("]")(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // group-[:nth-of-type(3)_&]:block fn arbitrary_back_selector_modifier(input: &str) -> IResult<&str, ()> { let (input, _) = take_while1(|char| is_ident_char(char) && char != '[')(input)?; let (input, _) = tag("-[")(input)?; let (input, _) = take_until("&]")(input)?; let (input, _) = tag("&]")(input)?; Ok((input, ())) } // [@supports(display:grid)]:grid fn arbitrary_at_supports_rule_modifier(input: &str) -> IResult<&str, ()> { let (input, _) = tag("[@supports(")(input)?; let (input, _) = take_until(")")(input)?; let (input, _) = tag(")]")(input)?; Ok((input, ())) } // [@media(any-hover:hover){&:hover}]:opacity-100 fn arbitrary_at_media_rule_modifier(input: &str) -> IResult<&str, ()> { // starts with [@media and ends with ] let (input, _) = tag("[@media(")(input)?; let (input, _) = take_until("]")(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // group/edit invisible hover:bg-slate-200 group-hover/item:visible fn group_peer_modifier(input: &str) -> IResult<&str, ()> { let (input, _) = alt(( tuple((tag("group-"), predefined_modifier)), // https://tailwindcss.com/docs/hover-focus-and-other-states#differentiating-peers // peer-checked/published:text-sky-500 tuple((tag("peer-"), predefined_modifier)), ))(input)?; let (input, _) = tag("/")(input)?; let (input, _) = take_while1(is_ident_char)(input)?; Ok((input, ())) } // hidden group-[.is-published]:block // group-[:nth-of-type(3)_&]:block // peer-[.is-dirty]:peer-required:block hidden // hidden peer-[:nth-of-type(3)_&]:block fn group_modifier_selector(input: &str) -> IResult<&str, ()> { let (input, _) = alt((tag("group"), tag("peer")))(input)?; let (input, _) = tag("-[")(input)?; let (input, _) = take_until("]")(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } is_valid_last_word // supports-[backdrop-filter] fn supports_arbitrary(input: &str) -> IResult<&str, ()> { let (input, _) = tag("supports-[")(input)?; let (input, _) = take_until("]")(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } fn is_valid_group_classname(class_name: &str) -> bool { !class_name.contains(':') && !class_name.contains('[') && !class_name.contains(']') && class_name.starts_with("group/") // aria-[sort=ascending]:bg-[url('/img/down-arrow.svg')] // aria-[sort=descending]:bg-[url('/img/up-arrow.svg')] fn aria_arbitrary(input: &str) -> IResult<&str, ()> { let (input, _) = opt(tag("group-"))(input)?; let (input, _) = tag("aria-[")(input)?; let (input, _) = take_while1(is_ident_char)(input)?; let (input, _) = tag("=")(input)?; let (input, _) = take_while1(is_ident_char)(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } fn is_valid_string(s: &str) -> bool { // Matches strings that contain only alphanumeric characters, underscores, and hyphens. let re = Regex::new(r"^[a-zA-Z0-9_-]*$").expect("Invalid regex"); re.is_match(s) && !s.is_empty() // data-[size=large]:p-8 fn data_arbitrary(input: &str) -> IResult<&str, ()> { let (input, _) = tag("data-[")(input)?; let (input, _) = take_while1(is_ident_char)(input)?; let (input, _) = tag("=")(input)?; let (input, _) = take_while1(is_ident_char)(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } // min-[320px]:text-center max-[600px]:bg-sky-300 fn min_max_arbitrary_modifier(input: &str) -> IResult<&str, ()> { let (input, _) = alt((tag("min-"), tag("max-")))(input)?; let (input, _) = tag("[")(input)?; let (input, _) = parse_length_unit(input)?; let (input, _) = tag("]")(input)?; Ok((input, ())) } fn modifier(input: &str) -> IResult<&str, ()> { alt(( group_modifier_selector, group_peer_modifier, predefined_special_modifier, arbitrary_front_selector_modifier, arbitrary_back_selector_modifier, arbitrary_at_supports_rule_modifier, arbitrary_at_media_rule_modifier, predefined_modifier, supports_arbitrary, aria_arbitrary, data_arbitrary, min_max_arbitrary_modifier, ))(input) } fn modifiers_chained(input: &str) -> IResult<&str, ()> { let (input, _modifiers) = separated_list0(tag(":"), modifier)(input)?; Ok((input, ())) } fn parse_tw_full_classname(input: &str) -> IResult<&str, Vec<&str>> { let (input, _class_names) = tuple(( opt(tuple((modifiers_chained, tag(":")))), parse_single_tw_classname, ))(input)?; Ok((input, vec![])) } // Edge cases // [&:nth-child(3)]:underline // lg:[&:nth-child(3)]:hover:underline // [&_p]:mt-4 // flex [@supports(display:grid)]:grid // [@media(any-hover:hover){&:hover}]:opacity-100 // group/edit invisible hover:bg-slate-200 group-hover/item:visible // hidden group-[.is-published]:block // group-[:nth-of-type(3)_&]:block // peer-checked/published:text-sky-500 // peer-[.is-dirty]:peer-required:block hidden // hidden peer-[:nth-of-type(3)_&]:block // after:content-['*'] after:ml-0.5 after:text-red-500 block text-sm font-medium text-slate-700 // before:content-[''] before:block // bg-black/75 supports-[backdrop-filter]:bg-black/25 supports-[backdrop-filter]:backdrop-blur // aria-[sort=ascending]:bg-[url('/img/down-arrow.svg')] aria-[sort=descending]:bg-[url('/img/up-arrow.svg')] // group-aria-[sort=ascending]:rotate-0 group-aria-[sort=descending]:rotate-180 // data-[size=large]:p-8 // open:bg-white dark:open:bg-slate-900 open:ring-1 open:ring-black/5 dark:open:ring-white/10 open:shadow-lg p-6 rounded-lg // lg:[&:nth-child(3)]:hover:underline // min-[320px]:text-center max-[600px]:bg-sky-300 // top-[117px] lg:top-[344px] // bg-[#bada55] text-[22px] before:content-['Festivus'] // grid grid-cols-[fit-content(theme(spacing.32))] // bg-[--my-color] // [mask-type:luminance] hover:[mask-type:alpha] // [--scroll-offset:56px] lg:[--scroll-offset:44px] // lg:[&:nth-child(3)]:hover:underline // bg-[url('/what_a_rush.png')] // before:content-['hello\_world'] // text-[22px] // text-[#bada55] // text-[var(--my-var)] // text-[length:var(--my-var)] // text-[color:var(--my-var)] fn parse_class_names(input: &str) -> IResult<&str, Vec<&str>> { let (input, _) = multispace0(input)?; let (input, _class_names) = separated_list0(multispace1, parse_tw_full_classname)(input)?; let (input, _) = multispace0(input)?; Ok((input, vec![])) } fn parse_top(input: &str) -> IResult<&str, Vec<&str>> { all_consuming(parse_class_names)(input) } #[proc_macro] pub fn tw(raw_input: TokenStream) -> TokenStream { let r_input = raw_input.clone(); let input = parse_macro_input!(r_input as LitStr); let (_modifiers, _valid_class_names) = match setup(&input) { Ok(value) => value, Err(value) => { return syn::Error::new_spanned(input, value) .to_compile_error() .into() } }; let full_classnames = input.value(); let (input, _class_names) = match parse_top(&full_classnames) { Ok(value) => value, Err(value) => { return syn::Error::new_spanned(input, value) .to_compile_error() .into() } }; quote::quote! { #input } .into() }