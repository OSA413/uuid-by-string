use uuid_by_string::generate_uuid;

fn main() {
    assert_eq!(generate_uuid::generate_uuid("hello world", 3), "5eb63bbb-e01e-3ed0-93cb-22bb8f5acdc3");
    assert_eq!(generate_uuid::generate_uuid("hello world", 5), "2aae6c35-c94f-5fb4-95db-e95f408b9ce9");
}