macro_rules! generate_derive_test_suite {
    ($test_suite:ident, $struct_type:ident, $int_type:ident, $int_value:literal, $int_type_alt:ident, $int_value_alt:literal, $struct_str:expr) => {
        #[cfg(test)]
        mod $test_suite {
            use base64id::{Base64Id, Error};
            use core::str::FromStr;

            #[derive(Base64Id, Debug)]
            #[base64id(Serialize, Deserialize)]
            struct $struct_type($int_type);

            #[test]
            fn str_from_struct() {
                let id = $struct_type(0);
                assert_eq!($struct_str, format!("{id}"));
            }

            #[test]
            fn struct_from_str() {
                let _id = $struct_type::from_str($struct_str)
                    .expect("failed to convert str to struct via FromStr trait");
            }

            #[test]
            fn int_from_struct() {
                let int = $int_type::from($struct_type($int_value));
                assert_eq!(int, $int_value);
            }

            #[test]
            fn struct_from_int() {
                let id = $struct_type::from($int_value);
                assert!(matches!(id, $struct_type($int_value)));
            }

            #[test]
            fn int_from_struct_u() {
                let int_u = $int_type_alt::from($struct_type($int_value));
                assert_eq!(int_u, $int_value_alt);
            }

            #[test]
            fn struct_from_int_u() {
                let id = $struct_type::from($int_value_alt);
                assert!(matches!(id, $struct_type($int_value)));
            }

            #[test]
            fn min_const() {
                assert_eq!(
                    $int_type::from($struct_type::MIN).to_be_bytes(),
                    $int_type_alt::from($struct_type::MIN).to_be_bytes()
                );
            }

            #[test]
            fn max_const() {
                assert_eq!(
                    $int_type::from($struct_type::MAX).to_be_bytes(),
                    $int_type_alt::from($struct_type::MAX).to_be_bytes()
                );
            }

            #[test]
            fn partial_eq() {
                assert_eq!($struct_type($int_value), $struct_type($int_value));
            }

            #[test]
            fn min_max_ord() {
                assert!($struct_type::MIN < $struct_type::MAX);
            }

            #[test]
            fn error_bad_char() {
                let err = $struct_type::from_str("A").expect_err("failed to get an error");
                assert_eq!(Error::InvalidLength, err);
            }

            #[test]
            fn serde_serialize() {
                let id = $struct_type($int_value);
                let serialized_id = serde_json::to_string(&id).expect("failed to serialize value");

                assert_eq!(serialized_id, format!("\"{}\"", $struct_str));
            }

            #[test]
            fn serde_deserialize() {
                let string = $struct_str;
                let id = serde_json::from_str(format!("\"{string}\"").as_str())
                    .expect("failed to deserialize value");

                assert_eq!($struct_type($int_value), id);
            }

            /// Ensure deserialize impl works without use of external FromStr import
            mod can_deserialize_without_use_from_str {
                use base64id::Base64Id;

                #[derive(Base64Id)]
                #[base64id(Deserialize)]
                struct $struct_type($int_type);
            }
        }
    };
}

generate_derive_test_suite!(derive_64_i, MyIdi64, i64, 0i64, u64, 0u64, "AAAAAAAAAAA");
generate_derive_test_suite!(derive_64_u, MyIdu64, u64, 0u64, i64, 0i64, "AAAAAAAAAAA");

generate_derive_test_suite!(derive_32_i, MyIdi32, i32, 0i32, u32, 0u32, "AAAAAA");
generate_derive_test_suite!(derive_32_u, MyIdu32, u32, 0u32, i32, 0i32, "AAAAAA");

generate_derive_test_suite!(derive_16, MyId16, i16, 0i16, u16, 0u16, "AAA");
