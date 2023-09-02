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

#[proc_macro]
pub fn tw(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let mut valid_class_names = [
        class_type::TAILWIND_CSS.margin,
        class_type::TAILWIND_CSS.padding,
        TAILWIND_CSS.columns,
    ]
    .concat();
    valid_class_names.extend_from_slice(&class_type::TAILWIND_CSS.columns);

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
