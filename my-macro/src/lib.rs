use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;

#[proc_macro_derive(CustomDebug)]

pub fn derive_custom_debug(input: TokenStream) -> TokenStream {
    println!("{:?}", input);

    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let data = &ast.data;

    let fields = match data {
        syn::Data::Struct(s) => {
            match &s.fields {
                syn::Fields::Named(fields) => fields.named.iter().map(|f| f.ident.as_ref()).collect::<Vec<_>>(),
                _ => panic!("Expected named fields in struct"),
            }
        }
        syn::Data::Enum(e) => {
            e.variants.iter().flat_map(|v| {
                match &v.fields {
                    syn::Fields::Named(fields) => fields.named.iter().map(|f| f.ident.as_ref()).collect::<Vec<_>>(),
                    _ => panic!("Expected named fields in enum variant"),
                }
            }).collect::<Vec<_>>()
        }
        _ => panic!("Expected struct or enum"),
    };

    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f,concat!("🐞 ",stringify!(#name), " {{ ", #(stringify!(#fields), ": {:?}, ",)* "}}"),#(self.#fields),*)
            }
        }
    };

    TokenStream::from(expanded)
}
