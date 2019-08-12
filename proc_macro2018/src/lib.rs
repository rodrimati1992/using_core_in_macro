extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn using_from_core(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        use core::num::NonZeroU8;
    })
}
