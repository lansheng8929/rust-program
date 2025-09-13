use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, DeriveInput, Path};
use quote::{quote};

pub fn derive_buffered_event(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let engine_ecs_path: Path = crate::engine_ecs_path();

    ast.generics
        .make_where_clause()
        .predicates
        .push(parse_quote! { Self: Send + Sync + 'static });

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics #engine_ecs_path::event::BufferedEvent for #struct_name #type_generics #where_clause {}
    })
}
