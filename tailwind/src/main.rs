#![warn(clippy::no_effect)]
/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */
use tw_macro::tw;

fn main() {
    let _ = tw!("btn btn");

    let test = tw!(
        r#"[mask-type:alpha] [ mask-type:   alpha   ] before:content-['rerer  erer re rr r  \re  reFestivus']
    after:content-['I am a content'] after:content-['I am a content'] after:content-['I am a content']
        active:hover:text-[#bada55] active:hover:text-[  #ba5   ] text-[#bada55] hover:aria-checked:text-[22px]
        text-[22.34e434cm]
        before:content-['hello\_world']
    grid grid-cols-[fit-content(theme(spacing.32))]
    bg-[--my-color]
        text-[var(--my-var)]
        text-[length:var(--my-var)]
        text-[color:var(--my-var)]
        [--scroll-offset:56px] lg:[--scroll-offset:44px]
        btn bg-[url('/img/down-arrow.svg')] ring-white/10   bg-black/25  bg-black/[100] bg-black/[0.75]    active:hover:collapse-arrow

    -mt-4

        lg:[&:nth-child(3)]:hover:underline
        group-[:nth-of-type(3)_&]:block
     [&_p]:mt-4

     flex [@supports(display:grid)]:grid
     flex active:hover:[@supports(display:grid)]:grid

     [@media(any-hover:hover){&:hover}]:opacity-100

         hidden group-[.is-published]:block
     group-[:nth-of-type(3)_&]:block
     peer-[.is-dirty]:peer-required:block hidden
     hidden peer-[:nth-of-type(3)_&]:block

         group/edit invisible hover:bg-slate-200 group-hover/item:visible

     peer-checked/published:text-sky-500
    
after:content-['*'] after:ml-0.5 after:text-red-500 block text-sm font-medium text-slate-700

    before:content-[''] before:block
content-[>]
content-[<]
    
    bg-black/75 supports-[backdrop-filter]:bg-black/25 supports-[backdrop-filter]:backdrop-blur

aria-[sort=ascending]:bg-[url('/img/down-arrow.svg')] aria-[sort=descending]:bg-[url('/img/up-arrow.svg')]


 group-aria-[sort=ascending]:rotate-0 group-aria-[sort=descending]:rotate-180
    
 data-[size=large]:p-8

    open:bg-white dark:open:bg-slate-900 open:ring-1 open:ring-black/5 dark:open:ring-white/10 open:shadow-lg p-6 rounded-lg
    
    lg:[&:nth-child(3)]:hover:underline

     min-[320rem]:text-center max-[600px]:bg-sky-300

 top-[117px] lg:top-[344px]

     bg-[#bada55] text-[22px] before:content-['Festivus']


     grid grid-cols-[fit-content(theme(spacing.32))]

     bg-[--my-color]

 [mask-type:luminance] hover:[mask-type:alpha]

 [--scroll-offset:56px] lg:[--scroll-offset:44px]

 lg:[&:nth-child(3)]:hover:underline
 bg-[url('/what_a_rush.png')]
 before:content-['hello\_world']
 text-[22px]
 text-[#bada55]
 text-[var(--my-var)]
 text-[length:var(--my-var)]
 text-[color:var(--my-var)]


     p-6 max-w-sm mx-auto bg-white rounded-xl shadow-lg flex items-center space-x-4

        "#
    );
    // 'content-[>]',
    //       //        ^
    //       'content-[<]',
    //       //        ^
    //
    //       // With functions and math expressions
    //       'px-[calc(100%-1rem)]',
    //       'px-[theme(spacing.1)]',
    //       'px-[theme(spacing[1.5])]',
    //
    //       // With spaces (replaced by `_`)
    //       'bg-[rgb(255_0_0)]',
    //
    //       // Examples with combinations
    //       'w-[calc(100%_-_theme("spacing[1.5]))"]',
    //       'fill-[oklab(59.69%_0.1007_0.1191_/_0.5)]/[33.7%]',
    //       'fill-[color:oklab(59.69%_0.1007_0.1191_/_0.5)]/[33.7%]',
    //       'shadow-[inset_0_-3em_3em_rgba(0,_0,_0,_0.1),_0_0_0_2px_rgb(255,_255,_255),_0.3em_0.3em_1em_rgba(0,_0,_0,_0.3)]'
    //

    // let test =
    //     tw!("peer[.is-dirty]:peer-required:block hidden  hidden peer-[:nth-of-type(3)_&]:block");
    println!("TEXT - {}", test);
    let _ = tw!("btn collapse-arrow");
    tw!("bg-gray-600 bg-sky-700 bg-midnight underline");
    tw!("bg-gray-600 aria-checked:bg-sky-700 aria-asc:bg-midnight data-checked:underline");
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
