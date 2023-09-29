use base64id::Base64Id;

#[derive(Base64Id)]
struct MyId64(i64);

#[test]
fn id64_str_from_struct() {
    let id = MyId64(0);
    assert_eq!("AAAAAAAAAAA", format!("{id}"));
}
