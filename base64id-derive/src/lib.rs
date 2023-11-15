use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Create your own base64id tuple struct
#[proc_macro_derive(Base64Id)]
pub fn tuple_struct_into_base64id(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("failed to parse token stream");

    let ident = ast.ident;
    let struct_inner_type = get_validated_struct_data(ast.data);

    let (encode_fn) = match struct_inner_type.to_string().as_str() {
        "i64" => (quote! {::base64id_core::base64::encode_i64}),
        "i32" => (quote! {::base64id_core::base64::encode_i32}),
        "i16" => (quote! {::base64id_core::base64::encode_i16}),
        _ => panic!("invalid type within tuple struct, expected i64, i32 or i16"),
    };

    quote! {
        impl ::core::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use ::core::fmt::Write;

                for c in #encode_fn(self.0) {
                    f.write_char(c)?;
                }

                Ok(())
            }
        }
    }
    .into()
}

fn get_validated_struct_data(data: syn::Data) -> syn::Ident {
    let data = match data {
        syn::Data::Struct(s) => s,
        _ => panic!("unsupported data type. expected a tuple struct"),
    };

    let fields = match data.fields {
        syn::Fields::Unnamed(f) => f.unnamed,
        _ => panic!("unsupported data type. expected a tuple struct"),
    };

    let item = match fields.len() {
        1 => fields.first().unwrap(),
        _ => panic!("expected a tuple struct with exactly 1 field"),
    };

    let item_path = match item.ty.clone() {
        syn::Type::Path(p) => p.path,
        _ => panic!("invalid type within tuple struct, expected i64, i32 or i16"),
    };

    let item_type = match item_path.get_ident() {
        Some(t) => t,
        None => panic!("invalid type within tuple struct, expected i64, i32 or i16"),
    };

    match item_type.to_string().as_str() {
        "i64" | "i32" | "i16" => return item_type.clone(),
        _ => panic!("invalid type within tuple struct, expected i64, i32 or i16"),
    };
}
