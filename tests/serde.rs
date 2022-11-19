macro_rules! generate_serde_test_suite {
    ($lib_type:ident, $lib_type_name:ident, $i_type:ident, $test_int:literal, $test_str:literal) => {
        #[cfg(feature = "serde")]
        mod $lib_type_name {
            use base64id::$lib_type;
            use serde::{Deserialize, Serialize};

            const INT: $i_type = $test_int;
            const JSON: &str = $test_str;

            #[derive(Serialize, Deserialize)]
            struct Record {
                pub id: $lib_type,
            }

            #[test]
            fn serde_serialize() {
                let record = Record {
                    id: $lib_type::from(INT),
                };
                let encoded = serde_json::to_string(&record).unwrap();

                assert_eq!(JSON, encoded);
            }

            #[test]
            fn serde_deserialize() {
                let decoded: Record = serde_json::from_str(JSON).unwrap();

                assert_eq!(INT, $i_type::from(decoded.id));
            }
        }
    };
}

generate_serde_test_suite!(
    Id64,
    id64,
    i64,
    2063772195469131459,
    r#"{"id":"HKP94KBD_sM"}"#
);
