use tw_macro::tw;

fn main() {
    tw!("m-4 p-4");
    tw!("hover:[mask-typealpha]");
    tw!(
        "m-4 last:first:invalid:last:first:p-4 last:m-4 pb-[43cm] [mask-type:luminance] 
    [mask-type:luminance] hover:[mask-type:alpha] lg:[--scroll-offset:44px]"
    );
}
