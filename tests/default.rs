use uuid_by_string::generate_uuid::{generate_uuid_v3, generate_uuid_v5};

mod common {
    pub mod japanese_text;
    pub mod long_text;
}

use crate::common::{japanese_text::JAPANESE_TEXT, long_text::LONG_TEXT};

#[test]
fn hello_world() {
    assert_eq!(generate_uuid_v3("hello world"), "5eb63bbb-e01e-3ed0-93cb-22bb8f5acdc3");
    assert_eq!(generate_uuid_v5("hello world"), "2aae6c35-c94f-5fb4-95db-e95f408b9ce9");
}

#[test]
fn hello_world_again() {
    assert_eq!(generate_uuid_v3("Hello world!"), "86fb269d-190d-3c85-b6e0-468ceca42a20");
    assert_eq!(generate_uuid_v5("Hello world!"), "d3486ae9-136e-5856-bc42-212385ea7970");
}

#[test]
fn should_generate_uuid_v3_from_string() {
    assert_eq!(generate_uuid_v3(""), "d41d8cd9-8f00-3204-a980-0998ecf8427e");
    assert_eq!(generate_uuid_v3("Hello world!"), "86fb269d-190d-3c85-b6e0-468ceca42a20");
    assert_eq!(generate_uuid_v3("Lorem ipsum"), "0956d2fb-d5d5-3298-84a4-d21ed2f76e0c");
    assert_eq!(generate_uuid_v3(JAPANESE_TEXT), "20b085c2-2e91-324f-89c2-648fc03ee626");
    assert_eq!(generate_uuid_v3(LONG_TEXT), "34e58612-20fa-3978-bbe1-656b34ab2f2f");
}

#[test]
fn should_generate_uuid_v5_from_string() {
    assert_eq!(generate_uuid_v5(""), "da39a3ee-5e6b-5b0d-b255-bfef95601890");
    assert_eq!(generate_uuid_v5("Hello world!"), "d3486ae9-136e-5856-bc42-212385ea7970");
    assert_eq!(generate_uuid_v5("Lorem ipsum"), "94912be8-b3fb-57d4-961e-a50e5948c629");
    assert_eq!(generate_uuid_v5(JAPANESE_TEXT), "ae49974d-2750-5eb2-b004-24bf83a04950");
    assert_eq!(generate_uuid_v5(LONG_TEXT), "c81386c7-744f-5af9-8899-cfdd14664aa7");
}