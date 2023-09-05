use tw_macro::tw;

fn main() {
    tw!("m-4 p-4 p-4");
    tw!("lg:[&:nth-child(3)]:hover:underline");
    // tw!("[0]");
    // tw!("[color:red]/dark");
    // tw!("![feature(slice_as_chunks)]");
    // tw!("!-[feature(slice_as_chunks)]");
    tw!("[@supports(display:grid)]:grid");
    tw!("[@media(any-hover:hover){&:hover}]:opacity-100");
    tw!("underline-offset-[3px]");

    tw!("[&_p]:mt-4");
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
    tw!("px-[0]");
    tw!("hover:[mask-type:alpha]");
    tw!(
        "m-4 last:first:invalid:last:first:p-4 last:m-4 pb-[calc(100%-34px)] pb-[23px] [mask-type:luminance] 
    [mask-type:luminance] hover:[mask-type:alpha] lg:[--scroll-offset:44px]"
    );
    tw!("p-4 md:w-1/3");
 
    tw!("opacity-50 md:opacity-100 hover:opacity-100");
    tw!("tracking-widest text-xs font-medium text-gray-400 mb-1");
    // tw!("h-full border-2 border-gray-200 border-opacity-60 rounded-lg overflow-hidden");
}
