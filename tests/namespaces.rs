use uuid_by_string::generate_uuid_with_namespace;


#[test]
fn namespace_accept_valid_uuid() {
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace("hello world", "d3486ae9-136e-5856-bc42-212385ea7970", 3), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace("hello world", "D3486AE9-136e-5856-bc42-212385ea7970", 3), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
}

#[test]
fn namespace_reject_invalid_uuid() {
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace("hello world", "", 3), Err("Invalid UUID"));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace("hello world", "Lorem ipsum", 3), Err("Invalid UUID"));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace("hello world", "123", 5), Err("Invalid UUID"));
}