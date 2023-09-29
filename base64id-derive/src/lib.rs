use proc_macro::TokenStream;
use syn::DeriveInput;

/// Create your own base64id tuple struct
#[proc_macro_derive(Base64Id)]
pub fn tuple_struct_into_base64id(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("failed to parse token stream");

    let ident = ast.ident;

    quote::quote! {
        impl ::core::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "AAAAAAAAAAA")
            }
        }
    }
    .into()
}
