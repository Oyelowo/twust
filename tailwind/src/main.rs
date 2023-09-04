use tw_macro::tw;

fn main() {
    tw!("m-4 p-4");
    tw!("px-[-45em]");
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
