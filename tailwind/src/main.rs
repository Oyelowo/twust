/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use tw_macro::tw;

fn main() {
    tw!("bg-taxvhiti bg-tahiti-500 bg-tahiti bg-midnight bg-red-50");
    tw!("bg-taxvhiti bg-tahiti-500 bg-tahiti bg-midnight bg-purple bg-red-50 bg-tahiti-800 border-s-tahiti-800");
    tw!("md:text-red-50 text-slate-50 text-purple text-tahiti-500");
    tw!("py-sm md:py-md tablet:py-sm lg:py-lg xl:py-xl");
    tw!("group");
    tw!("text-sm font-medium text-slate-300 group-hover:text-white");
    tw!("text-sm font-medium text-slate-500 group-hover:text-slate-300");
    tw!("hover:-translate-y-0.5 transition motion-reduce:hover:translate-y-0 motion-reduce:transition-none");
    tw!("motion-safe:hover:-translate-x-0.5 motion-safe:transition");

    tw!("group/edit block invisible md:hover:bg-slate-200 group-hover/item:visible");
    tw!("group-[:nth-of-type(3)_&]:block group-hover/edit:text-gray-700 group-[:nth-of-type(3)_&]:block");

    tw!("tracking-widest text-xs title-font font-medium text-gray-400 mb-1");

    tw!("bg-gray-600 aria-checked:bg-sky-700 aria-asc:bg-midnight data-checked:underline");
    tw!("scroll-m-14 flex supports-grid:grid supports-[display:grid]:grid");
    tw!("scroll-m-sm group-aria-[sort=ascending]:rotate-0");
    tw!("scroll-mx-sm");
    tw!("scroll-mx-md");
    tw!("scroll-my-md");
    tw!("px-sm pt-sm pb-sm pr-sm pl-sm");
    tw!("px-md pt-md pb-md pr-md pl-md");
    tw!("scroll-m-14 scroll-mx-14");
    tw!("m-4 p-4 p-4");
    tw!("-m-[4px] p-4 p-4");
    tw!("-m-4 p-4 p-4");
    tw!("lg:[&:nth-child(3)]:hover:underline");
    // tw!("[0]");
    // tw!("[color:red]/dark");
    // tw!("![feature(slice_as_chunks)]");
    // tw!("!-[feature(slice_as_chunks)]");
    tw!("[@supports(display:grid)]:grid");
    tw!("[@media(any-hover:hover){&:hover}]:opacity-100");
    tw!("underline-offset-[3px]");

    tw!("[&_p]:mt-4");
    // tw!("-[&_p]:mt-4");
    tw!("lg:[&:nth-child(3)]:hover:underline");
    tw!("outline-blue-500/50");
    tw!("text-blue-600/[.07]");

    // tw!("[something]");
    tw!("px-[-45px]");
    tw!("px-[-45cm]");
    tw!("px-[-45rem]");
    tw!("px-[-45em]");
    tw!("px-[-45%]");
    tw!("px-[-45in]");
    tw!("px-[-45vh]");
    tw!("px-[-45vw]");
    tw!("px-[-45vmin]");
    tw!("px-[-45vmax]");
    tw!("px-[-45mm]");
    tw!("px-[-45pc]");
    tw!("px-[0]");
    tw!("px-[45px]");
    tw!("px-[45cm]");
    tw!("px-[45rem]");
    tw!("px-[45em]");
    tw!("px-[45%]");
    tw!("px-[45in]");
    tw!("px-[45vh]");
    tw!("px-[45vw]");
    tw!("px-[45vmin]");
    tw!("px-[45vmax]");
    tw!("px-[45mm]");
    tw!("px-[45pc]");
    tw!("py-[0]");
    tw!("-px-[45pc]");
    tw!("hover:[mask-type:alpha]");
    tw!(
        "m-4 last:first:invalid:last:first:p-4 last:m-4 pb-[calc(100%-34px)] pb-[23px] [mask-type:luminance]
    [mask-type:luminance] hover:[mask-type:alpha] lg:[--scroll-offset:44px] oyelowo oyedayo break-after-avoid"
    );
    tw!("p-4 md:w-1/3");

    tw!("opacity-50 md:opacity-100 hover:opacity-100");
    tw!("tracking-widest text-xs font-medium text-gray-400 mb-1");
    // border color is overriden here in tailwind.config.json
    tw!("h-full border-2 border-mana-200 border-opacity-60 rounded-lg overflow-hidden");
}
