use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{Attribute, DeriveInput, Meta};

/// Create your own base64id tuple struct
#[proc_macro_derive(Base64Id, attributes(base64id))]
pub fn tuple_struct_into_base64id(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("failed to parse token stream");

    let ident = ast.ident;
    let struct_inner_type = get_validated_struct_data(ast.data);

    let (encode_fn, decode_fn, char_array_type, struct_inner_type_u) =
        match struct_inner_type.to_string().as_str() {
            "i64" => (
                quote! {::base64id_core::base64::encode_i64},
                quote! {::base64id_core::base64::decode_i64},
                quote! {[char; 11]},
                quote! {u64},
            ),
            "i32" => (
                quote! {::base64id_core::base64::encode_i32},
                quote! {::base64id_core::base64::decode_i32},
                quote! {[char; 6]},
                quote! {u32},
            ),
            "i16" => (
                quote! {::base64id_core::base64::encode_i16},
                quote! {::base64id_core::base64::decode_i16},
                quote! {[char; 3]},
                quote! {u16},
            ),
            _ => panic!("invalid type within tuple struct, expected i64, i32 or i16"),
        };

    let mut implementation = quote! {
        impl #ident {
            const MIN: #ident = #ident(0);
            const MAX: #ident = #ident(-1);
        }

        impl ::core::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                use ::core::fmt::Write;

                for c in #encode_fn(self.0) {
                    f.write_char(c)?;
                }

                Ok(())
            }
        }

        impl ::core::convert::From<#ident> for #struct_inner_type {
            fn from(id: #ident) -> Self {
                id.0
            }
        }

        impl ::core::convert::From<#struct_inner_type> for #ident {
            fn from(id: #struct_inner_type) -> Self {
                Self(id)
            }
        }

        impl ::core::convert::From<#ident> for #struct_inner_type_u {
            fn from(id: #ident) -> Self {
                #struct_inner_type_u::from_be_bytes(id.0.to_be_bytes())
            }
        }

        impl ::core::convert::From<#struct_inner_type_u> for #ident {
            fn from(id: #struct_inner_type_u) -> Self {
                Self(#struct_inner_type::from_be_bytes(id.to_be_bytes()))
            }
        }

        impl ::core::convert::TryFrom<#char_array_type> for #ident {
            type Error = ::base64id_core::Error;

            fn try_from(input: #char_array_type) -> Result<Self, Self::Error> {
                Ok(Self(#decode_fn(input)?))
            }
        }

        impl ::core::str::FromStr for #ident {
            type Err = ::base64id_core::Error;

            fn from_str(id: &str) -> Result<Self, Self::Err> {
                let mut array: #char_array_type = ::core::default::Default::default();
                let mut id_iter = id.chars();

                for c in array.iter_mut() {
                    *c = match id_iter.next() {
                        Some(d) => d,
                        None => return Err(::base64id_core::Error::InvalidLength),
                    };
                }

                if id_iter.next().is_some() {
                    return Err(::base64id_core::Error::InvalidLength);
                }

                #ident::try_from(array)
            }
        }

        impl ::core::cmp::PartialEq for #ident {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl ::core::cmp::Eq for #ident {}

        impl ::core::cmp::PartialOrd for #ident {
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl ::core::cmp::Ord for #ident {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                let this = #struct_inner_type_u::from_be_bytes(self.0.to_be_bytes());
                let other = #struct_inner_type_u::from_be_bytes(other.0.to_be_bytes());

                this.cmp(&other)
            }
        }
    };

    evaluate_attributes(ident, ast.attrs, &mut implementation);

    implementation.into()
}

/// Determines if the base64id attribute is present
/// and if it contains expected keywords
fn evaluate_attributes(
    ident: proc_macro2::Ident,
    attrs: Vec<Attribute>,
    implementation: &mut proc_macro2::TokenStream,
) {
    for attr in attrs {
        let attr_ident = match attr.path().get_ident() {
            Some(i) => i,
            None => continue,
        };

        if attr_ident != "base64id" {
            continue;
        }

        let meta_list = match attr.meta {
            Meta::List(l) => l,
            _ => continue,
        };

        for token in meta_list.tokens {
            let token_ident = match token {
                TokenTree::Ident(i) => i,
                _ => continue,
            };

            if token_ident == "Serialize" {
                apply_serialize_trait(&ident, implementation);
            }
        }

        return;
    }
}

/// Enable the following syntax:
/// ```ignore
/// #[derive(base64id::Base64Id)]
/// #[base64id(Serialize)]
/// struct MyType(i64);
/// ```
fn apply_serialize_trait(
    ident: &proc_macro2::Ident,
    implementation: &mut proc_macro2::TokenStream,
) {
    implementation.extend(quote!(
        impl ::serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer
            {
                serializer.collect_str(self)
            }
        }
    ));
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
        "i64" | "i32" | "i16" => item_type.clone(),
        _ => panic!("invalid type within tuple struct, expected i64, i32 or i16"),
    }
}
