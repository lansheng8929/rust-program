
extern crate proc_macro;

mod component;

use proc_macro::TokenStream;


/// Implement the `BufferedEvent` trait.
#[proc_macro_derive(BufferedEvent)]
pub fn derive_buffered_event(input: TokenStream) -> TokenStream {
    component::derive_buffered_event(input)
}
