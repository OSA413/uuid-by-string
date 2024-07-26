use uuid_by_string::uuid;

mod common {
    pub mod japanese_text;
    pub mod long_text;
}

use crate::common::{japanese_text::JAPANESE_TEXT, long_text::LONG_TEXT};

#[test]
fn hello_world() {
    assert_eq!(uuid::generate_v3("hello world", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(uuid::generate_v3("hello world", Some("D3486AE9-136e-5856-bc42-212385ea7970")), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(uuid::generate_v5("hello world", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
    assert_eq!(uuid::generate_v5("hello world", Some("D3486AE9-136e-5856-bc42-212385ea7970")), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
}

#[test]
fn hello_world_default() {
    assert_eq!(uuid::generate_v3("hello world", None), Ok("c72c207b-0847-386d-bdbc-2e5def81cf81".to_owned()));
    assert_eq!(uuid::generate_v5("hello world", None), Ok("191333f6-c83e-5b3b-bdb0-bd483ad1bcb7".to_owned()));
}

#[test]
fn namespace_reject_invalid_uuid() {
    assert_eq!(uuid::generate_v3("hello world", Some("")), Err("Invalid UUID"));
    assert_eq!(uuid::generate_v3("hello world", Some("Lorem ipsum")), Err("Invalid UUID"));
    assert_eq!(uuid::generate_v5("hello world", Some("123")), Err("Invalid UUID"));
}

#[test]
fn hello_world_again() {
    assert_eq!(uuid::generate_v3("Hello world!", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("5928d9ca-9acd-3608-a6dc-24ebd0c49283".to_owned()));
    assert_eq!(uuid::generate_v5("Hello world!", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("6c85afc9-ff2a-56d9-b451-75a825751bed".to_owned()));
}

#[test]
fn hello_world_again_default() {
    assert_eq!(uuid::generate_v3("Hello world!", None), Ok("f7c44786-7e81-386b-a7a5-95ef58bcb389".to_owned()));
    assert_eq!(uuid::generate_v5("Hello world!", None), Ok("ec74fa6c-5be6-5388-8cb3-d91001210130".to_owned()));
}

#[test]
fn should_generate_uuid_v3_from_string_with_namespace() {
    assert_eq!(uuid::generate_v3("", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("c9d82834-1d69-3199-81e0-ccf73cedd3b9".to_owned()));
    assert_eq!(uuid::generate_v3("Hello world!", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("5928d9ca-9acd-3608-a6dc-24ebd0c49283".to_owned()));
    assert_eq!(uuid::generate_v3("Lorem ipsum", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("254f8a36-7528-3566-87ee-55a1cfef1be2".to_owned()));
    assert_eq!(uuid::generate_v3(JAPANESE_TEXT, Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("53fb63e9-0855-398e-b5e1-322bb5296c50".to_owned()));
    assert_eq!(uuid::generate_v3(LONG_TEXT, Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("d1c2812d-10d2-3cf4-9fa9-236cec2dd3cf".to_owned()));
}

#[test]
fn should_generate_uuid_v5_from_string_with_namespace() {
    assert_eq!(uuid::generate_v5("", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("6d72c664-05f7-5ae7-8653-ee9ebed25b00".to_owned()));
    assert_eq!(uuid::generate_v5("Hello world!", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("6c85afc9-ff2a-56d9-b451-75a825751bed".to_owned()));
    assert_eq!(uuid::generate_v5("Lorem ipsum", Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("a359b647-73dd-52b9-ac27-6119382182db".to_owned()));
    assert_eq!(uuid::generate_v5(JAPANESE_TEXT, Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("a7928b1a-03e8-5a2e-adf9-dee22ea1d1d5".to_owned()));
    assert_eq!(uuid::generate_v5(LONG_TEXT, Some("d3486ae9-136e-5856-bc42-212385ea7970")), Ok("5daf5b6d-b5e8-5e22-96c4-71e0fd8c4540".to_owned()));
}

#[test]
fn should_generate_uuid_v3_from_string_with_default_namespace() {
    assert_eq!(uuid::generate_v3("", None), Ok("4ae71336-e44b-39bf-b9d2-752e234818a5".to_owned()));
    assert_eq!(uuid::generate_v3("Hello world!", None), Ok("f7c44786-7e81-386b-a7a5-95ef58bcb389".to_owned()));
    assert_eq!(uuid::generate_v3("Lorem ipsum", None), Ok("7e7c264e-35a5-3e17-97c9-6467e3b4359c".to_owned()));
    assert_eq!(uuid::generate_v3(JAPANESE_TEXT, None), Ok("8c085e86-f500-3556-b08e-3fe16c7d7724".to_owned()));
    assert_eq!(uuid::generate_v3(LONG_TEXT, None), Ok("0d12c6ed-bc6d-356f-8932-ee6ab1262870".to_owned()));
}

#[test]
fn should_generate_uuid_v5_from_string_with_default_namespace() {
    assert_eq!(uuid::generate_v5("", None), Ok("e129f27c-5103-5c5c-844b-cdf0a15e160d".to_owned()));
    assert_eq!(uuid::generate_v5("Hello world!", None), Ok("ec74fa6c-5be6-5388-8cb3-d91001210130".to_owned()));
    assert_eq!(uuid::generate_v5("Lorem ipsum", None), Ok("e09defe7-e8dd-5995-a741-57669517728a".to_owned()));
    assert_eq!(uuid::generate_v5(JAPANESE_TEXT, None), Ok("583709b5-7c35-5f81-8a64-136786bda678".to_owned()));
    assert_eq!(uuid::generate_v5(LONG_TEXT, None), Ok("57cccf14-e652-5428-8013-4380d74a86e6".to_owned()));
}