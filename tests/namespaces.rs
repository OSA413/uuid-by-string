use uuid_by_string::generate_uuid_with_namespace;


#[test]
fn namespace_accept_valid_uuid() {
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace("hello world", "", 3), Err("Invalid UUID"));
}

#[test]
fn namespace_reject_invalid_uuid() {
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace("hello world", "", 3), Err("Invalid UUID"));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace("hello world", "Lorem ipsum", 3), Err("Invalid UUID"));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace("hello world", "123", 5), Err("Invalid UUID"));
}