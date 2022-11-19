macro_rules! type_test_suite {
    ($lib_type:ident, $lib_type_name:ident, $u_type:ident, $u_value:literal, $u_zero:literal, $i_type:ident, $i_value:literal, $i_zero:literal, $str_value:expr) => {
        #[cfg(test)]
        mod $lib_type_name {
            use crate::$lib_type;
            use ::core::str::FromStr;

            #[test]
            fn create_from_u64() {
                let number: $u_type = $u_value;
                let id = $lib_type::from(number);
                assert_eq!(number, $u_type::from(id));
            }

            #[test]
            fn create_from_i64() {
                let number: $i_type = $i_value;
                let id = $lib_type::from(number);
                assert_eq!(number, $i_type::from(id));
            }

            #[test]
            fn create_from_u64_ref() {
                let number: $u_type = $u_value;
                let id = $lib_type::from(&number);
                assert_eq!(number, $u_type::from(id));
            }

            #[test]
            fn create_from_i64_ref() {
                let number: $i_type = $i_value;
                let id = $lib_type::from(&number);
                assert_eq!(number, $i_type::from(id));
            }

            #[test]
            fn create_i64() {
                let id = $lib_type::new();
                let number = $i_type::from(id);
                assert_eq!(id, $lib_type::from(number));
            }

            #[test]
            fn create_u64() {
                let id = $lib_type::new();
                let number = $u_type::from(id);
                assert_eq!(id, $lib_type::from(number));
            }

            #[test]
            fn create_i64_from_ref() {
                let id = $lib_type::new();
                let number = $i_type::from(&id);
                assert_eq!(id, $lib_type::from(number));
            }

            #[test]
            fn create_u64_from_ref() {
                let id = $lib_type::new();
                let number = $u_type::from(&id);
                assert_eq!(id, $lib_type::from(number));
            }

            #[test]
            fn create_from_str() {
                let id = $lib_type::from_str($str_value[0]).unwrap();
                assert_eq!($lib_type::from($u_zero), id);
            }

            #[test]
            fn display_impl() {
                extern crate std;
                use std::format;

                let id_str_list = $str_value;
                for id_str in id_str_list {
                    let id = $lib_type::from_str(id_str).unwrap();
                    assert_eq!(id_str, format!("{id}"));
                }
            }

            #[test]
            fn id64_min_const() {
                assert_eq!(
                    $u_type::MIN.to_be_bytes(),
                    $u_type::from($lib_type::MIN).to_be_bytes()
                );
            }

            #[test]
            fn id64_max_const() {
                assert_eq!(
                    $u_type::MAX.to_be_bytes(),
                    $u_type::from($lib_type::MAX).to_be_bytes()
                );
            }

            #[test]
            fn id64_min_max_ord() {
                assert!($lib_type::MIN < $lib_type::MAX);
            }

            #[test]
            fn id64_new() {
                assert_eq!($lib_type($i_zero), $lib_type::new());
            }
        }
    };
}

type_test_suite!(
    Id64,
    id64,
    u64,
    25519u64,
    0u64,
    i64,
    -25519i64,
    0i64,
    [
        "AAAAAAAAAAA",
        "Yc9P3-xdNvs",
        "ekEG7AofcJg",
        "xVGMdimcrMU",
        "U3qb2eQPdYs",
        "Z-WOv92w6CM",
        "RRvfLRwc6LA",
        "SITpCH_VLpI",
        "2ZhAjsFPPlU",
        "1OLRIV5oHtM",
    ]
);
