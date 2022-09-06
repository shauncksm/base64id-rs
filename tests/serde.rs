#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize};

use base64id::Id64;

const INT: i64 = 2063772195469131459;
const JSON: &str = r#"{"id":"HKP94KBD_sM"}"#;

#[derive(Serialize, Deserialize)]
struct Record {
    pub id: Id64,
}

#[test]
#[cfg(feature = "serde")]
fn serde_id64_serialize() {
    let record = Record {
        id: Id64::from(INT),
    };
    let encoded = serde_json::to_string(&record).unwrap();

    assert_eq!(JSON, encoded);
}

#[test]
#[cfg(feature = "serde")]
fn serde_id64_deserialize() {
    let decoded: Record = serde_json::from_str(JSON).unwrap();

    assert_eq!(INT, i64::from(decoded.id));
}
