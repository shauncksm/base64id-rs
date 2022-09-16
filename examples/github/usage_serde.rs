use base64id::Id64;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), serde_json::Error> {
    #[derive(Serialize, Deserialize)]
    struct Record {
        id: Id64,
    }

    let record = Record {
        id: Id64::from(0u64),
    };

    println!("{}", serde_json::to_string(&record)?); // {"id":"AAAAAAAAAAA"}

    Ok(())
}
