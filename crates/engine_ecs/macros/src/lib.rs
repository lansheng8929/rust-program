
extern crate proc_macro;

mod component;

use engine_macro_utils::prelude::EngineManifest;
use proc_macro::TokenStream;


/// Implement the `BufferedEvent` trait.
#[proc_macro_derive(BufferedEvent)]
pub fn derive_buffered_event(input: TokenStream) -> TokenStream {
    component::derive_buffered_event(input)
}

pub(crate) fn engine_ecs_path() -> syn::Path {
    EngineManifest::shared().get_path("engine_ecs")
}
