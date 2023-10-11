/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while1},
    character::complete::{digit1, multispace0, multispace1, space0, space1},
    combinator::{all_consuming, not, opt, recognize},
    multi::separated_list0,
    number,
    sequence::{delimited, preceded, tuple},
    IResult,
};
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
            .strip_prefix("-")
            .unwrap_or(class_name)
            .starts_with(s)
    });

    if is_signable && is_valid_classname(class_name.strip_prefix("-").unwrap_or(class_name)) {
        Ok((input, ()))
    } else if !is_signable && is_valid_classname(class_name) {
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
    LENGTHY.contains(&class_name.strip_prefix("-").unwrap_or(class_name))
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

// text-[22px]
fn lengthy_arbitrary_classname(input: &str) -> IResult<&str, ()> {
    let (input, class_name) = take_until("-[")(input)?;
    let ((input, _)) = if is_lengthy_classname(class_name) {
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

// #bada55
fn parse_hex_color(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("#")(input)?;
    let (input, color) = take_while1(|c: char| c.is_ascii_hexdigit())(input)?;
    let ((input, _)) = if color.chars().count() == 3 || color.chars().count() == 6 {
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
    let (input, _) = alt((parse_hex_color, parse_rgb_color, parse_rgba_color))(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

// e.g: [mask-type:alpha]
fn kv_pair_classname(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("[")(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = take_while1(is_ident_char)(input)?;
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
    let (input, _) = take_while1(|char| is_ident_char(char))(input)?;
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

// supports-[backdrop-filter]
fn supports_arbitrary(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("supports-[")(input)?;
    let (input, _) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, ()))
}

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
    let (input, class_names) = tuple((
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

    quote::quote! {
        #input
    }
    .into()
}
