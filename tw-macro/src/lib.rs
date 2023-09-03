use std::error::Error;
use std::fs;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;
use syn::{parse_macro_input, LitStr};
mod tailwind;
use tailwind::tailwind_config::TailwindConfig;
use tailwind::{
    class_type::{self, TAILWIND_CSS},
    modifiers,
    valid_baseclass_names::{ VALID_BASECLASS_NAMES},
};

// use tailwind::;

// struct CheckInput(Vec<LitStr>);
// // fn xx() -> Token![^] {
// //     todo!()
// // }
// impl Parse for CheckInput {
//     fn parse(input: ParseStream) -> Result<Self> {
//         // let x = xx();
//         let mut strings = Vec::new();
//         while !input.is_empty() {
//             let s = input.parse::<LitStr>()?;
//             strings.push(s);
//             let _ = input.parse::<Token![,]>();
//         }
//         Ok(CheckInput(strings))
//     }
// }

// #[proc_macro]
// pub fn tw(input: TokenStream) -> TokenStream {
//     let CheckInput(strings) = parse_macro_input!(input as CheckInput);

//     let valid = ["lowo", "dayo"];

//     for s in strings {
//         if !valid.contains(&s.value().as_str()) {
//             return syn::Error::new_spanned(s, "Invalid string")
//                 .to_compile_error()
//                 .into();
//         }
//     }

//     TokenStream::from(quote! {})
// }


pub fn read_tailwind_config(path: &str) -> Result<TailwindConfig, Error> {
    let content = fs::read_to_string(path)?;
    let config: TailwindConfig = serde_json::from_str(&content)?;
    Ok(config)
}



fn concatenate_arrays(arrays: &[&[&'static str]]) -> Vec<&'static str> {
    arrays.iter().cloned().flatten().cloned().collect()
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
    let has_break_after = true;
    let break_after_custom_overwrite = vec!["break-after-avoid", "break-after-auto", "break-after-all"];
    let break_after_custom_extend = vec!["break-after-avoid", "break-after-auto", "break-after-all"];
    
    let break_after = if has_break_after { 
        break_after_custom_overwrite
    } else {
        // Concatenate the default break_after classes with the custom ones from extend field.
        // TODO: Check if the custom ones are valid
        vec![TAILWIND_CSS.break_after.to_vec(), break_after_custom_extend].concat() 
    };
    
    let valid_class_names = concat_arrays![
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

    valid_class_names
}

// const ARBITRARY_BASE_CLASS_NAMES: [&'static str; 160] = [];

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

    for word in input.value().split_whitespace() {
        let modifiers_and_class = word.split(':');
        // TODO:  check the first and the last character are not open and close brackets
        // respectively i.e arbitrary property e.g [mask_type:aplha];
        let mut word_for_arb_prop = word.split(":[");

        // modifiers e.g hover: in
        // hover:[mask-type:alpha]
        let is_valid_arb_prop = word_for_arb_prop
            .next()
            .is_some_and(|modifiers_or_full_arb_prop| {
                let is_arbitrary_property = modifiers_or_full_arb_prop.starts_with('[')
                    && modifiers_or_full_arb_prop.ends_with(']');
                
                let is_valid = if is_arbitrary_property {
                    let is_valid_arb_property = modifiers_or_full_arb_prop
                        .starts_with('[') && modifiers_or_full_arb_prop.ends_with(']') && modifiers_or_full_arb_prop.matches('[').count() == 1 && modifiers_or_full_arb_prop.matches(']').count() == 1 && 
                     modifiers_or_full_arb_prop
                        .trim_start_matches('[')
                        .trim_end_matches(']').split(':').count() == 2;
                    is_valid_arb_property
                } else {
                modifiers_or_full_arb_prop
                    .split(':')
                    .all(|modifier| modifiers::MODIFIERS.contains(&modifier))
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
        let last_word = modifiers_and_class.clone().last().unwrap();

        let modifiers_from_word = modifiers_and_class
            .clone()
            .take(modifiers_and_class.count() - 1)
            .collect::<Vec<&str>>();
        let is_valid_modifier = modifiers_from_word
            .iter()
            .all(|modifier| modifiers::MODIFIERS.contains(&modifier));

        let valid_class_names = get_class_names();

        let is_arb_prop = |string: &str| {
            string.starts_with('[') && string.ends_with(']') && string.split(':').count() == 2
        };
        let is_valid_class = !is_valid_arb_prop && valid_class_names.contains(&last_word);

        // let is_valid_class = word.split(':').last()
        //     // .next()
        //     .is_some_and(|class_name| {
        //         let is_arbitrary_property = class_name.starts_with('[')
        //             && class_name.ends_with(']');
        //
        //         let is_valid = if is_arbitrary_property {
        //             let is_valid_arb_property = class_name
        //                 .starts_with('[') && class_name.ends_with(']') && class_name.matches('[').count() == 1 && class_name.matches(']').count() == 1 && 
        //              class_name
        //                 .trim_start_matches('[')
        //                 .trim_end_matches(']').split(':').count() == 2;
        //             is_valid_arb_property
        //         } else {
        //         class_name
        //             .split(':')
        //             .all(|modifier| modifiers::MODIFIERS.contains(&modifier))
        //         };
        //         is_valid
        //     })
        //     ||
        // // value e.g [mask-type:alpha] in hover:[mask-type:alpha]
        // // potential addition checks(probably not a good idea. Imagine a new css property, we would
        // // have to open a PR for every new or esoteric css property.)
        //  word_for_arb_prop.next().is_some_and(|value| {
        //     value.ends_with(']')
        //         && value.split(':').count() == 2
        //     // We had already split by ":[", so there should be no "[" anymore
        //         && value.matches('[').count() == 0
        //         && value.matches(']').count() == 1
        // });
        let (base_classname, arbitrary_value_with_bracket) =
            last_word.split_once("-").unwrap_or_default();
        // TODO: Validate the base class name.
        // TODO: Check if valid tailwind keyword. e.g
                                                    // pb etc
                                                    // TODO: Validate at least spacing dimensions. e.g px, em, rem, cm, mm, in, pt, for
                                                    // classes that support spacing e.g padding, margin, width, height, min-width etc
                                                    //
        let prefix_is_valid_tailwind_keyword = VALID_BASECLASS_NAMES.contains(&base_classname);
        let is_arbitrary_value = prefix_is_valid_tailwind_keyword
            && arbitrary_value_with_bracket.starts_with('[')
            && arbitrary_value_with_bracket.ends_with(']');

        // TODO:
        // Check arbitrary class names and also one with shash(/). Those can be exempted but the
        // prefixes should also be valid class names.
        // Support arbitrary variant selector:     e.g: <li
        // class="lg:[&:nth-child(3)]:hover:underline">{item}</li>,
        // arbitrary values, aribitrary properties.
        //
        // Use official tailwind rust run function to further check integrity of the class name.
        // Complete the classes list
        // prefixing with minus sign should be allowed i.e -.

        // Validate artbitrary css values, especially for spacing. i.e px, em, rem, cm, mm, in,
        // pt,
        if (is_valid_class && is_valid_modifier)
            || is_arbitrary_value
            || is_valid_arb_prop
        {
        } else {
            return syn::Error::new_spanned(input, format!("Invalid string: {}", word))
                .to_compile_error()
                .into();
        }
    }

    TokenStream::from(quote! {#input})
}
