// use std::error::Error;
use syn::parse::{Parse, ParseStream};
use syn::Token;
use syn::{parse_macro_input, LitStr};
mod config;
mod tailwind;
use tailwind::lengthy::LENGTHY;
use tailwind::tailwind_config::TailwindConfig;
use tailwind::{
    class_type::{self, TAILWIND_CSS},
    modifiers,
    valid_baseclass_names::VALID_BASECLASS_NAMES,
};

// // use tailwind::;
use config::get_classes;
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

// // struct CheckInput(Vec<LitStr>);
// // // fn xx() -> Token![^] {
// // //     todo!()
// // // }
// // impl Parse for CheckInput {
// //     fn parse(input: ParseStream) -> Result<Self> {
// //         // let x = xx();
// //         let mut strings = Vec::new();
// //         while !input.is_empty() {
// //             let s = input.parse::<LitStr>()?;
// //             strings.push(s);
// //             let _ = input.parse::<Token![,]>();
// //         }
// //         Ok(CheckInput(strings))
// //     }
// // }

// // #[proc_macro]
// // pub fn tw(input: TokenStream) -> TokenStream {
// //     let CheckInput(strings) = parse_macro_input!(input as CheckInput);

// //     let valid = ["lowo", "dayo"];

// //     for s in strings {
// //         if !valid.contains(&s.value().as_str()) {
// //             return syn::Error::new_spanned(s, "Invalid string")
// //                 .to_compile_error()
// //                 .into();
// //         }
// //     }

// //     TokenStream::from(quote! {})
// // }

fn concatenate_arrays(arrays: &[&[&'static str]]) -> Vec<&'static str> {
    arrays.iter().cloned().flatten().cloned().collect()
}

fn is_valid_length(value: &str) -> bool {
    let re = regex::Regex::new(r"^(-?\d+(\.?\d+)?(px|em|rem|%|cm|mm|in|pt|pc|vh|vw|vmin|vmax)|0)$")
        .expect("Invalid regex");
    re.is_match(value)
}
//     let value = "-10px";
//     println!("{}", is_valid_length(value));

fn is_valid_calc(value: &str) -> bool {
    let re = regex::Regex::new(r"^calc\([^)]+\)$").expect("Invalid regex");
    re.is_match(value)
}
// //     let value = "calc(100% - 80px)";
// //     println!("{}", is_valid_calc(value));

fn read_tailwind_config(path: &str) -> Result<TailwindConfig, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let config: TailwindConfig = serde_json::from_str(&content)?;
    Ok(config)
}

// use static_assertions::
macro_rules! concat_arrays {
    ($($field:ident),*) => {
        {

        #[allow(non_camel_case_types)]
        trait Unique {}
        impl Unique for () {}
        $(
            #[allow(non_camel_case_types)]
            struct $field;
            impl Unique for $field {}
        )*

        // Ensure that all the classes are included. Increase or reduce when necessary. e.g
        // 119 is the number of categories in tailwindcss v3.0.0
        // If the number of categories increases, increase the number of elements in the array
        // below. If it reduces, reduce the number of elements in the array below.
            let arr: [&[&'static str]; 168] = [$(&TAILWIND_CSS.$field[..]),*];
            assert_eq!(arr.len(), 168);
            let arr = vec![arr];
            arr.concat()
        }
    };
}

fn get_class_names() -> Vec<&'static str> {
    let config = read_tailwind_config("tailwind.config.json").unwrap_or_default();
    let theme = config.theme;
    let mut xx = TAILWIND_CSS;

    //     // let break_after = if theme.overrides.break_after.is_empty() {
    //     //     // Concatenate the default break_after classes with the custom ones from extend field.
    //     //     // TODO: Check if the custom ones are valid
    //     //     // vec![TAILWIND_CSS.break_after.to_vec(), theme.extend.break_after.into_keys().collect()].concat()
    //     //     xx.break_after.extend(theme.extend.break_after.into_keys().collect::<Vec<&str>>());
    //     // } else {
    //     //     // break_after_overwrite
    //     //    theme.overrides.break_after.into_keys().collect::<Vec<&str>>();
    //     // };

    let mut valid_class_names = concat_arrays![
        aspect_ratio,
        container,
        columns,
        break_after,
        break_before,
        break_inside,
        box_decoration_break,
        box_sizing,
        display,
        float,
        clear,
        isolation,
        object_fit,
        object_position,
        overflow,
        overscroll_behavior,
        position,
        inset,
        top,
        bottom,
        right,
        left,
        visibility,
        z_index,
        flex_basis,
        flex_direction,
        flex,
        flex_grow,
        flex_shrink,
        flex_wrap,
        order,
        gap,
        justify_content,
        justify_items,
        justify_self,
        align_content,
        align_items,
        align_self,
        place_content,
        place_items,
        place_self,
        grid_template_columns,
        grid_auto_columns,
        grid_column,
        grid_column_start,
        grid_column_end,
        grid_template_rows,
        grid_auto_rows,
        grid_row,
        grid_row_start,
        grid_row_end,
        grid_auto_flow,
        padding,
        margin,
        space,
        width,
        min_width,
        max_width,
        height,
        min_height,
        max_height,
        font_family,
        font_size,
        font_smoothing,
        font_style,
        font_weight,
        font_variant_numeric,
        letter_spacing,
        line_clamp,
        line_height,
        line_style_image,
        line_style_position,
        line_style_type,
        text_align,
        text_color,
        text_decoration,
        text_decoration_color,
        text_decoration_style,
        text_decoration_thickness,
        text_underline_offset,
        text_transform,
        text_overflow,
        text_indent,
        vertical_align,
        whitespace,
        word_break,
        hyphens,
        content,
        background_attachment,
        background_clip,
        background_color,
        background_origin,
        background_position,
        background_repeat,
        background_size,
        background_image,
        gradient_color_stops,
        border_radius,
        border_width,
        border_color,
        border_syle,
        border_collapse,
        border_spacing,
        table_layout,
        caption_side,
        divide_width,
        divide_color,
        divide_style,
        outline_width,
        outline_color,
        outline_style,
        outline_offset,
        ring_width,
        ring_color,
        ring_offset_width,
        ring_offset_color,
        box_shadow,
        box_shadow_color,
        opacity,
        mix_blend_mode,
        background_blend_mode,
        blur,
        brightness,
        contrast,
        drop_shadow,
        gray_scale,
        hue_rotate,
        invert,
        saturate,
        sepia,
        backdrop_blur,
        backdrop_brightness,
        backdrop_contrast,
        backdrop_grayscale,
        backdrop_hue_rotate,
        backdrop_invert,
        backdrop_opacity,
        backdrop_saturate,
        backdrop_sepia,
        caret_color,
        scroll_margin,
        transform_origin,
        accent_color,
        scale,
        rotate,
        translate,
        skew,
        transition_property,
        transition_timing_function,
        transition_duration,
        transition_delay,
        animation,
        appearance,
        cursor,
        pointer_events,
        resize,
        user_select,
        fill,
        stroke,
        stroke_width,
        scroll_behavior,
        scroll_padding,
        scroll_snap_align,
        scroll_snap_stop,
        scroll_snap_type,
        touch_action,
        will_change,
        scree_readers
    ]
    .concat();

    //     // valid_class_names.extend(break_after);
    valid_class_names
}

// // const ARBITRARY_BASE_CLASS_NAMES: [&'static str; 160] = [];

//
// Spacing:
// border_radius
// flex_basis
// gap
// border_spacing
// height
// inset
// margin
// width
// padding
// max_height
// space
// scroll_padding
// text_indent
// translate
#[proc_macro]
pub fn tw(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    // let valid_class_names = get_class_names();
    let valid_class_names = match get_classes() {
        Ok(config) => config,
        Err(e) => {
            return syn::Error::new_spanned(input, format!("Error reading Tailwind config: {}", e))
                .to_compile_error()
                .into();
        }
    };

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
                    .all(|modifier| modifiers::MODIFIERS.contains(&modifier)) &&
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
            .all(|modifier| modifiers::MODIFIERS.contains(&modifier));

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

#[proc_macro]
pub fn print_current_dir(_input: TokenStream) -> TokenStream {
    let dir = env::current_dir().expect("cant get cur dir");
    let path = dir.to_str().expect("cant covert to str");

    let output = quote! {
        compile_error!(#path);
    };
    output.into()
}
