mod repeat;

use std::iter::{chain, once};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input};

#[proc_macro]
pub fn prototype(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn underscore_params(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::LitInt);
    let count = u32::from_str_radix(input.base10_digits(), 10).expect("Failed to parse int literal.");
    chain(
        once(TokenStream::from(quote!(_))),
        (0..count).map(|_| TokenStream::from(quote!(,_)))
    ).collect::<TokenStream>()
}