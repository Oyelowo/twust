use tw_macro::tw;

fn main() {
    tw!("m-4 p-4");
    tw!("m-4 invalid:last:first:p-4 last:m-4");
}
