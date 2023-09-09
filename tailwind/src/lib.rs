/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use tw_macro::tw;

/// Invalid character in class name.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("bg-taxvhiti$");
/// ```
fn _invalid_character_in_class_name() {}

/// Unsupported unit.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("px-[45xyz]");
/// ```
fn _unsupported_unit() {}

/// Mixing classes without spaces.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("bg-taxvhiti.bg-tahiti-500");
/// ```
fn _mixing_classes_without_spaces() {}

/// Unsupported combination of classes.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("bg-taxvhiti hover");
/// ```
fn _unsupported_combination_of_classes() {}

/// Missing `-` in between hover and class name.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("hover[bg-taxvhiti]");
/// ```
fn _missing_dash_in_class_name() {}

/// Missing `[]` for arbitrary values.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("px-45pc");
/// ```
fn _missing_brackets_for_arbitrary_values() {}

/// Invalid usage of negative sign.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("-[&_p]mt-4");
/// ```
fn _invalid_usage_of_negative_sign() {}

/// Improper use of brackets. No units present for length dimension.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("px-[45");
/// ```
fn _improper_use_of_brackets() {}

/// Unsupported media query.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("![feature(slice_as_chunks)]");
/// ```
fn _unsupported_media_query() {}

/// Missing unit after arbitrary value.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("px-[45]");
/// ```
fn _missing_unit_after_arbitrary_value() {}

/// Invalid group usage.
///
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("group-unknownmodifier:underline");
/// ```
fn _invalid_group_usage() {}

/// Wont compile overriden border color.
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("border-red-500");
/// ```
fn _wont_compile_overriden_border_color() {}

/// Invalid group usage with no slash.
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("groupedit block invisible md:hover:bg-slate-200 md:group-hover/item:visible");
/// ```
fn _invalid_group_usage_2() {}

/// Invalid group usage 3 with invalid class
/// ```compile_fail
/// use tw_macro::tw;
/// tw!("md:hover:bg-slate-200 md:group-hover/item:isible");
/// ```
fn _invalid_group_usage_3() {}

fn _happy_paths() {
    fn main() {
        let _classnames = tw!("bg-taxvhiti bg-tahiti-500 bg-tahiti bg-midnight bg-red-50");
        let _classnames = tw!("bg-taxvhiti bg-tahiti-500 bg-tahiti bg-midnight bg-purple bg-red-50 bg-tahiti-800 border-s-tahiti-800");
        let _classnames = tw!("md:text-red-50 text-slate-50 text-purple text-tahiti-500");
        let _classnames = tw!("py-sm md:py-md tablet:py-sm lg:py-lg xl:py-xl");
        let _classnames = tw!("group");
        let _classnames = tw!("text-sm font-medium text-slate-300 group-hover:text-white");
        let _classnames = tw!("text-sm font-medium text-slate-500 group-hover:text-slate-300");
        let _classnames = tw!("hover:-translate-y-0.5 transition motion-reduce:hover:translate-y-0 motion-reduce:transition-none");
        let _classnames = tw!("motion-safe:hover:-translate-x-0.5 motion-safe:transition");

        let _classnames =
            tw!("group/edit block invisible md:hover:bg-slate-200 group-hover/item:visible");
        let _classnames = tw!("group-[:nth-of-type(3)_&]:block group-hover/edit:text-gray-700 group-[:nth-of-type(3)_&]:block");

        let _classnames = tw!("tracking-widest text-xs title-font font-medium text-gray-400 mb-1");

        let _classnames =
            tw!("bg-gray-600 aria-checked:bg-sky-700 aria-asc:bg-midnight data-checked:underline");
        let _classnames = tw!("scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
        let _classnames = tw!("scroll-m-sm group-aria-[sort=ascending]:rotate-0");
        let _classnames = tw!("scroll-mx-sm");
        let _classnames = tw!("scroll-mx-md");
        let _classnames = tw!("scroll-my-md");
        let _classnames = tw!("px-sm pt-sm pb-sm pr-sm pl-sm");
        let _classnames = tw!("px-md pt-md pb-md pr-md pl-md");
        let _classnames = tw!("scroll-m-14 scroll-mx-14");
        let _classnames = tw!("m-4 p-4 p-4");
        let _classnames = tw!("-m-[4px] p-4 p-4");
        let _classnames = tw!("-m-4 p-4 p-4");
        let _classnames = tw!("lg:[&:nth-child(3)]:hover:underline");
        let _classnames = // tw!("[0]");
         // tw!("[color:red]/dark");
         // tw!("![feature(slice_as_chunks)]");
         // tw!("!-[feature(slice_as_chunks)]");
         tw!("[@supports(display:grid)]:grid");
        let _classnames = tw!("[@media(any-hover:hover){&:hover}]:opacity-100");
        let _classnames = tw!("underline-offset-[3px]");

        let _classnames = tw!("[&_p]:mt-4");
        // tw!("-[&_p]:mt-4");
        let _classnames = tw!("lg:[&:nth-child(3)]:hover:underline");
        let _classnames = tw!("outline-blue-500/50");
        let _classnames = tw!("text-blue-600/[.07]");

        // tw!("[something]");
        let _classnames = tw!("px-[-45px]");
        let _classnames = tw!("px-[-45cm]");
        let _classnames = tw!("px-[-45rem]");
        let _classnames = tw!("px-[-45em]");
        let _classnames = tw!("px-[-45%]");
        let _classnames = tw!("px-[-45in]");
        let _classnames = tw!("px-[-45vh]");
        let _classnames = tw!("px-[-45vw]");
        let _classnames = tw!("px-[-45vmin]");
        let _classnames = tw!("px-[-45vmax]");
        let _classnames = tw!("px-[-45mm]");
        let _classnames = tw!("px-[-45pc]");
        let _classnames = tw!("px-[0]");
        let _classnames = tw!("px-[45px]");
        let _classnames = tw!("px-[45cm]");
        let _classnames = tw!("px-[45rem]");
        let _classnames = tw!("px-[45em]");
        let _classnames = tw!("px-[45%]");
        let _classnames = tw!("px-[45in]");
        let _classnames = tw!("px-[45vh]");
        let _classnames = tw!("px-[45vw]");
        let _classnames = tw!("px-[45vmin]");
        let _classnames = tw!("px-[45vmax]");
        let _classnames = tw!("px-[45mm]");
        let _classnames = tw!("px-[45pc]");
        let _classnames = tw!("py-[0]");
        let _classnames = tw!("-px-[45pc]");
        let _classnames = tw!("hover:[mask-type:alpha]");
        let _classnames = tw!(
        "m-4 last:first:invalid:last:first:p-4 last:m-4 pb-[calc(100%-34px)] pb-[23px] [mask-type:luminance]
    [mask-type:luminance] hover:[mask-type:alpha] lg:[--scroll-offset:44px] oyelowo oyedayo break-after-avoid"
    );
        let _classnames = tw!("p-4 md:w-1/3");

        let _classnames = tw!("opacity-50 md:opacity-100 hover:opacity-100");
        let _classnames = tw!("tracking-widest text-xs font-medium text-gray-400 mb-1");
        // border color is overriden here in tailwind.config.json
        let _classnames =
            tw!("h-full border-2 border-mana-53 border-opacity-60 rounded-lg overflow-hidden");
    }
}
