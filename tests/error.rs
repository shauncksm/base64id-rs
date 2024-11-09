macro_rules! generate_error_test_suite {
    ($int_type:ident, $lib_type_name:ident, $bad_char:expr) => {
        #[cfg(test)]
        mod $lib_type_name {
            use base64id::{
                Base64Id,
                Error::{InvalidCharacter, InvalidLength},
            };
            use core::str::FromStr;

            #[derive(Base64Id, Debug)]
            struct TestId($int_type);

            #[test]
            fn bad_length() {
                let id = TestId::from_str("A").unwrap_err();
                assert_eq!(id, InvalidLength);
            }

            #[test]
            fn invalid_character() {
                let id = TestId::from_str($bad_char).unwrap_err();
                debug_assert_eq!(id, InvalidCharacter);
            }

            #[test]
            fn core_error_trait_impl() {
                fn test<E: core::error::Error>(_: E) {}

                let error = InvalidLength;

                test(error);
            }

            #[test]
            fn std_error_trait_impl() {
                fn test<E: std::error::Error>(_: E) {}

                let error = InvalidLength;

                test(error);
            }
        }
    };
}

generate_error_test_suite!(i64, id64, "AAAAAAAAAA=");

generate_error_test_suite!(i32, id32, "AAAAA=");

generate_error_test_suite!(i16, id16, "AA=");
