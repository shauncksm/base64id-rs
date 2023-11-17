use base64id_derive::Base64Id;
use core::str::FromStr;

#[derive(Base64Id)]
struct MyId64(i64);

#[test]
fn id64_str_from_struct() {
    let id = MyId64(0);
    assert_eq!("AAAAAAAAAAA", format!("{id}"));
}

#[test]
fn id64_struct_from_str() {
    let _id =
        MyId64::from_str("AAAAAAAAAAA").expect("failed to convert str to struct via FromStr trait");
}

#[derive(Base64Id)]
struct MyId32(i32);

#[test]
fn id32_str_from_struct() {
    let id = MyId32(0);
    assert_eq!("AAAAAA", format!("{id}"));
}

#[test]
fn id32_struct_from_str() {
    let _id =
        MyId32::from_str("AAAAAA").expect("failed to convert str to struct via FromStr trait");
}

#[derive(Base64Id)]
struct MyId16(i16);

#[test]
fn id16_str_from_struct() {
    let id = MyId16(0);
    assert_eq!("AAA", format!("{id}"));
}

#[test]
fn id16_struct_from_str() {
    let _id = MyId16::from_str("AAA").expect("failed to convert str to struct via FromStr trait");
}
