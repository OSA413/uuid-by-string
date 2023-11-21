use uuid_by_string::generate_uuid_with_namespace;


#[test]
fn namespace_accept_valid_uuid() {
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace_v3("hello world", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace_v3("hello world", "D3486AE9-136e-5856-bc42-212385ea7970"), Ok("c8aeb76a-1204-3f07-995e-5c5fa3494b7f".to_owned()));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace_v5("hello world", "d3486ae9-136e-5856-bc42-212385ea7970"), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace_v5("hello world", "D3486AE9-136e-5856-bc42-212385ea7970"), Ok("1825ed38-348f-5b46-99de-fd84b83aba5e".to_owned()));
}

#[test]
fn namespace_reject_invalid_uuid() {
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace_v3("hello world", ""), Err("Invalid UUID"));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace_v3("hello world", "Lorem ipsum"), Err("Invalid UUID"));
    assert_eq!(generate_uuid_with_namespace::generate_uuid_with_namespace_v5("hello world", "123"), Err("Invalid UUID"));
}