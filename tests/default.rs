use uuid_by_string::generate_uuid;

#[test]
fn test1() {
    assert_eq!(generate_uuid::generate_uuid_v3("hello world"), "5eb63bbb-e01e-3ed0-93cb-22bb8f5acdc3");
}

#[test]
fn test2() {
    assert_eq!(generate_uuid::generate_uuid_v5("hello world"), "2aae6c35-c94f-5fb4-95db-e95f408b9ce9");
}

#[test]
fn test3() {
    assert_eq!(generate_uuid::generate_uuid_v5("Hello world!"), "d3486ae9-136e-5856-bc42-212385ea7970");
}