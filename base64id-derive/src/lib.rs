//! This crate contains the derive macro for [base64id-rs](https://github.com/shauncksm/base64id-rs).
//! You shouldn't use this crate directly. See [here](https://docs.rs/base64id/latest/base64id/) instead.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenTree};
use quote::quote;
use syn::{Attribute, DeriveInput, Meta};

const ERROR_INVALID_INNER_TYPE: &str =
    "invalid type within tuple struct, expected i64, u64, i32, u32, i16 or u16";

/// Create your own base64id tuple struct
#[proc_macro_derive(Base64Id, attributes(base64id))]
pub fn tuple_struct_into_base64id(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("failed to parse token stream");

    let ident = ast.ident;
    let struct_inner_type = get_validated_struct_data(ast.data);

    let char_len = match struct_inner_type.to_string().as_str() {
        "i64" | "u64" => 11,
        "i32" | "u32" => 6,
        "i16" | "u16" => 3,
        _ => panic!("{ERROR_INVALID_INNER_TYPE}"),
    };

    let (
        encode_fn,
        decode_fn,
        char_array_type,
        struct_inner_type_u,
        struct_inner_type_alt,
        int_min,
        int_max,
    ) = match struct_inner_type.to_string().as_str() {
        "i64" => (
            quote! {::base64id_core::base64::encode_i64},
            quote! {::base64id_core::base64::decode_i64},
            quote! {[char; #char_len]},
            quote! {u64},
            quote! {u64},
            quote! {0},
            quote! {-1},
        ),
        "u64" => (
            quote! {::base64id_core::base64::encode_u64},
            quote! {::base64id_core::base64::decode_u64},
            quote! {[char; #char_len]},
            quote! {u64},
            quote! {i64},
            quote! {0},
            quote! {#struct_inner_type::MAX},
        ),
        "i32" => (
            quote! {::base64id_core::base64::encode_i32},
            quote! {::base64id_core::base64::decode_i32},
            quote! {[char; #char_len]},
            quote! {u32},
            quote! {u32},
            quote! {0},
            quote! {-1},
        ),
        "u32" => (
            quote! {::base64id_core::base64::encode_u32},
            quote! {::base64id_core::base64::decode_u32},
            quote! {[char; #char_len]},
            quote! {u32},
            quote! {i32},
            quote! {0},
            quote! {#struct_inner_type::MAX},
        ),
        "i16" => (
            quote! {::base64id_core::base64::encode_i16},
            quote! {::base64id_core::base64::decode_i16},
            quote! {[char; #char_len]},
            quote! {u16},
            quote! {u16},
            quote! {0},
            quote! {-1},
        ),
        "u16" => (
            quote! {::base64id_core::base64::encode_u16},
            quote! {::base64id_core::base64::decode_u16},
            quote! {[char; #char_len]},
            quote! {u16},
            quote! {i16},
            quote! {0},
            quote! {#struct_inner_type::MAX},
        ),
        _ => panic!("{ERROR_INVALID_INNER_TYPE}"),
    };

    let mut implementation = quote! {
        impl #ident {
            const MIN: #ident = #ident(#int_min);
            const MAX: #ident = #ident(#int_max);
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

        impl ::core::convert::From<#ident> for #struct_inner_type_alt {
            fn from(id: #ident) -> Self {
                #struct_inner_type_alt::from_be_bytes(id.0.to_be_bytes())
            }
        }

        impl ::core::convert::From<#struct_inner_type_alt> for #ident {
            fn from(id: #struct_inner_type_alt) -> Self {
                Self(#struct_inner_type::from_be_bytes(id.to_be_bytes()))
            }
        }

        impl ::core::convert::TryFrom<#char_array_type> for #ident {
            type Error = ::base64id_core::Error;

            fn try_from(input: #char_array_type) -> ::core::result::Result<Self, Self::Error> {
                Ok(Self(#decode_fn(input)?))
            }
        }

        impl ::core::str::FromStr for #ident {
            type Err = ::base64id_core::Error;

            fn from_str(id: &str) -> ::core::result::Result<Self, Self::Err> {
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
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
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

    evaluate_attributes(ident, ast.attrs, char_len, &mut implementation);

    implementation.into()
}

/// Determines if the base64id attribute is present
/// and if it contains expected keywords
fn evaluate_attributes(
    ident: proc_macro2::Ident,
    attrs: Vec<Attribute>,
    char_len: usize,
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

            if token_ident == "Deserialize" {
                apply_deserialize_trait(&ident, char_len, implementation);
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

/// Enable the following syntax:
/// ```ignore
/// #[derive(base64id::Base64Id)]
/// #[base64id(Deserialize)]
/// struct MyType(i64);
/// ```
fn apply_deserialize_trait(
    ident: &proc_macro2::Ident,
    char_len: usize,
    implementation: &mut proc_macro2::TokenStream,
) {
    let visitor = Ident::new(
        format!("{ident}__Base64Id_Serde_Visitor").as_str(),
        Span::call_site(),
    );

    let last_char_range = match char_len {
        11 | 3 => "AEIMQUYcgkosw048",
        6 => "AQgw",
        _ => panic!("unexpected character length {char_len}. cannot get last_char_range"),
    };

    implementation.extend(quote!(
        impl<'de> ::serde::de::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                deserializer.deserialize_str(#visitor)
            }
        }

        #[allow(non_camel_case_types)]
        struct #visitor;

        impl<'de> ::serde::de::Visitor<'de> for #visitor {
            type Value = #ident;

            fn expecting(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.write_str("a base64url encoded string")
            }

            fn visit_str<E>(self, v: &str) -> ::core::result::Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                use ::core::str::FromStr;

                const EXP1: &str = concat!("exactly ", #char_len, " base64url characters");
                const EXP2: &str = concat!(
                    "the last character must be one of the following: ",
                    #last_char_range
                );
                const ERR: &str = concat!("unknown error! expected exactly ", #char_len, "base64url characters");

                #ident::from_str(v).map_err(|e| match e {
                    ::base64id_core::Error::InvalidLength => E::invalid_length(v.len(), &EXP1),
                    ::base64id_core::Error::InvalidCharacter => E::invalid_value(
                        ::serde::de::Unexpected::Other("1 or more non-base64url characters"),
                        &EXP1,
                    ),
                    ::base64id_core::Error::OutOfBoundsCharacter => E::invalid_value(
                        ::serde::de::Unexpected::Other("the last character was out of bounds"),
                        &EXP2,
                    ),
                    _ => E::custom(ERR)
                })
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
        _ => panic!("{ERROR_INVALID_INNER_TYPE}"),
    };

    let item_type = match item_path.get_ident() {
        Some(t) => t,
        None => panic!("{ERROR_INVALID_INNER_TYPE}"),
    };

    match item_type.to_string().as_str() {
        "i64" | "i32" | "i16" | "u64" | "u32" | "u16" => item_type.clone(),
        _ => panic!("{ERROR_INVALID_INNER_TYPE}"),
    }
}
