macro_rules! generate_derive_test_suite {
    ($test_suite:ident, $struct_type:ident, $int_type:ident, $int_value:literal, $struct_str:expr) => {
        #[cfg(test)]
        mod $test_suite {
            use base64id::{Base64Id, Error};
            use core::str::FromStr;

            #[derive(Base64Id, Debug)]
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
            fn error_bad_char() {
                let err = $struct_type::from_str("A").expect_err("failed to get an error");
                assert_eq!(Error::InvalidLength, err);
            }
        }
    };
}

generate_derive_test_suite!(derive_64, MyId64, i64, 0i64, "AAAAAAAAAAA");
generate_derive_test_suite!(derive_32, MyId32, i32, 0i32, "AAAAAA");
generate_derive_test_suite!(derive_16, MyId16, i16, 0i16, "AAA");
