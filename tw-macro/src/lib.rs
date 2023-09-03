use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;
use syn::{parse_macro_input, LitStr};
mod tailwind;
use tailwind::{
    class_type::{self, TAILWIND_CSS},
    modifiers,
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

    valid_class_names
}

#[proc_macro]
pub fn tw(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    // let mut valid_class_names = [
    // valid_class_names.extend_from_slice(&class_type::TAILWIND_CSS.columns);

    for word in input.value().split_whitespace() {
        let modifiers_and_class = word.split(':');
        let last_word = modifiers_and_class.clone().last().unwrap();

        let modifiers_from_word = modifiers_and_class
            .clone()
            .take(modifiers_and_class.count() - 1)
            .collect::<Vec<&str>>();
        let is_valid_modifier = modifiers_from_word
            .iter()
            .all(|modifier| modifiers::MODIFIERS.contains(&modifier));

        let valid_class_names = get_class_names();

        let is_valid_class = valid_class_names.contains(&last_word);

        // TODO:
        // Check arbitrary class names and also one with shash(/). Those can be exempted but the
        // prefixes should also be valid class names.
        // Use official tailwind rust run function to further check integrity of the class name.
        // Complete the classes list
        // prefixing with minus sign should be allowed i.e -.
        if valid_class_names.contains(&last_word) && is_valid_modifier {
        } else {
            return syn::Error::new_spanned(input, format!("Invalid string: {}", word))
                .to_compile_error()
                .into();
        }
    }

    TokenStream::from(quote! {#input})
}
