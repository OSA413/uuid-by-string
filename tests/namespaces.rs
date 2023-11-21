use uuid_by_string::generate_uuid_with_namespace::{generate_uuid_with_namespace_v3, generate_uuid_with_namespace_v5};

mod common {
    pub mod japanese_text;
    pub mod long_text;
}

use crate::common::{japanese_text::JAPANESE_TEXT, long_text::LONG_TEXT};

#[test]
fn hello_world() {
    assert_eq!(generate_uuid_with_namespace_v3("hello world", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v3("hello world", "D3486AE9-136e-5856-bc42-212385ea7970"), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5("hello world", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5("hello world", "D3486AE9-136e-5856-bc42-212385ea7970"), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
}

#[test]
fn namespace_reject_invalid_uuid() {
    assert_eq!(generate_uuid_with_namespace_v3("hello world", ""), Err("Invalid UUID"));
    assert_eq!(generate_uuid_with_namespace_v3("hello world", "Lorem ipsum"), Err("Invalid UUID"));
    assert_eq!(generate_uuid_with_namespace_v5("hello world", "123"), Err("Invalid UUID"));
}

#[test]
fn hello_world_again() {
    assert_eq!(generate_uuid_with_namespace_v3("Hello world!", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("5928d9ca-9acd-3608-a6dc-24ebd0c49283".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5("Hello world!", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("6c85afc9-ff2a-56d9-b451-75a825751bed".to_owned()));
}

#[test]
fn should_generate_uuid_v3_from_string_with_namespace() {
    assert_eq!(generate_uuid_with_namespace_v3("", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("c9d82834-1d69-3199-81e0-ccf73cedd3b9".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v3("Hello world!", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("5928d9ca-9acd-3608-a6dc-24ebd0c49283".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v3("Lorem ipsum", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("254f8a36-7528-3566-87ee-55a1cfef1be2".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v3(JAPANESE_TEXT, "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("53fb63e9-0855-398e-b5e1-322bb5296c50".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v3(LONG_TEXT, "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("d1c2812d-10d2-3cf4-9fa9-236cec2dd3cf".to_owned()));
}

#[test]
fn should_generate_uuid_v5_from_string_with_namespace() {
    assert_eq!(generate_uuid_with_namespace_v5("", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("6d72c664-05f7-5ae7-8653-ee9ebed25b00".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5("Hello world!", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("6c85afc9-ff2a-56d9-b451-75a825751bed".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5("Lorem ipsum", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("a359b647-73dd-52b9-ac27-6119382182db".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5(JAPANESE_TEXT, "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("a7928b1a-03e8-5a2e-adf9-dee22ea1d1d5".to_owned()));
    assert_eq!(generate_uuid_with_namespace_v5(LONG_TEXT, "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("5daf5b6d-b5e8-5e22-96c4-71e0fd8c4540".to_owned()));
}