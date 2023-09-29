use base64id::Base64Id;

#[derive(Base64Id)]
struct MyId64(i64);

#[test]
fn id64_str_from_struct() {
    let id = MyId64(0);
    assert_eq!("AAAAAAAAAAA", format!("{id}"));
}

#[derive(Base64Id)]
struct MyId32(i32);

#[test]
fn id32_str_from_struct() {
    let id = MyId32(0);
    assert_eq!("AAAAAA", format!("{id}"));
}

#[derive(Base64Id)]
struct MyId16(i16);

#[test]
fn id16_str_from_struct() {
    let id = MyId16(0);
    assert_eq!("AAA", format!("{id}"));
}
