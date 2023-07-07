use proc_macro::TokenStream;

#[proc_macro_derive(Base64Id)]
pub fn derive(input: TokenStream) -> TokenStream {
    input
}
