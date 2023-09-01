extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;
use syn::{parse_macro_input, LitStr};

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
// pub fn check(input: TokenStream) -> TokenStream {
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
pub fn check(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let valid = ["lowo", "dayo"];

    for word in input.value().split_whitespace() {
        if !valid.contains(&word) {
            return syn::Error::new_spanned(input, format!("Invalid string: {}", word))
                .to_compile_error()
                .into();
        }
    }

    TokenStream::from(quote! {})
}
