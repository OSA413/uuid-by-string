use uuid_by_string::generate_uuid;
use uuid_by_string::generate_uuid_with_namespaces;


#[test]
fn namespace_accept_valid_uuid() {
    assert!(cfg!(feature = "namespaces"));
    assert_eq!(generate_uuid::generate_uuid("hello world", 3), "5eb63bbb-e01e-3ed0-93cb-22bb8f5acdc3");
}