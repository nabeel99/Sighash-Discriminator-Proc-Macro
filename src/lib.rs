use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};
mod sighash;
use sighash::sig_hash;

#[proc_macro_attribute]
pub fn get_fn_discriminator(_metadata: TokenStream, body: TokenStream) -> TokenStream {
    //syn exposes a parse trait to use the type in parse_macro_input
    //parse_macro_input takes 2 arguments, a identifier assuming its a token stream
    //and a type to call syn parse with,
    //here we pass token stream as first argument
    //and item fn as the type we want the parse macro to parse out.
    let input = parse_macro_input!(body as ItemFn);
    // Repetition is done using #(...)* or #(...),* again similar to macro_rules!.
    // This iterates through the elements of any variable interpolated within the repetition
    //  and inserts a copy of the repetition body for each one.
    //  The variables in an interpolation may be a Vec, slice, BTreeSet, or any Iterator.
    //no need to convert sig_hash_arr into an iterator
    let fn_name: &str = &input.sig.ident.to_string();
    let name_str = format!("{}_SIGHASH", fn_name.to_ascii_uppercase());
    let new_name = Ident::new(&name_str[..], Span::call_site());
    let sig_hash_arr = sig_hash(fn_name);
    let discrim = quote! {pub const #new_name : [u8;8] = [#(#sig_hash_arr),*];
    #input}
    .into();
    discrim
}
