use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};
mod sighash;
use sighash::sig_hash;

#[proc_macro_attribute]
pub fn get_fn_discriminator(_metadata: TokenStream, body: TokenStream) -> TokenStream {
    let input = parse_macro_input!(body as ItemFn);

    let fn_name: &str = &input.sig.ident.to_string();
    let name_str = format!("{}_SIGHASH", fn_name.to_ascii_uppercase());
    let new_name = Ident::new(&name_str[..], Span::call_site());
    let sig_hash = sig_hash(fn_name).into_iter();
    let discrim = quote! {pub const #new_name : [u8;8] = [#(#sig_hash),*];
    #input}
    .into();
    discrim
}
