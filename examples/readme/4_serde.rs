use base64id::Base64Id;
use serde_json::Result;

#[derive(Base64Id)]
#[base64id(Serialize, Deserialize)]
struct MyId(i32);

fn main() -> Result<()> {
    let id = MyId(897100256);

    println!("{}", serde_json::to_string(&id)?); // "NXip4A"

    Ok(())
}
