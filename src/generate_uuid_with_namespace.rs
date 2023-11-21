#[path="./common.rs"]
mod common;
#[path="./for_namespace.rs"]
mod for_namespace;

pub fn generate_uuid_with_namespace(target: &str, namespace: &str) -> Result<String, &'static str> {
    generate_uuid_with_namespace_v5(target, namespace)
}

pub fn generate_uuid_with_namespace_v3(target: &str, namespace: &str) -> Result<String, &'static str> {
    let target_char_buffer = target.as_bytes();
    let namespace_char_buffer = for_namespace::parse_uuid(namespace);

    match namespace_char_buffer {
        Err(x) => return Err(x),
        _ => (),
    }

    let namespace_char_buffer = namespace_char_buffer.unwrap();
    let buffer = [&namespace_char_buffer, target_char_buffer].concat();
    let result = common::md5_hash(buffer);
    return Ok(common::hash_to_uuid(result, 3));
}

pub fn generate_uuid_with_namespace_v5(target: &str, namespace: &str) -> Result<String, &'static str> {
    let target_char_buffer = target.as_bytes();
    let namespace_char_buffer = for_namespace::parse_uuid(namespace);

    match namespace_char_buffer {
        Err(x) => return Err(x),
        _ => (),
    }

    let namespace_char_buffer = namespace_char_buffer.unwrap();
    let buffer = [&namespace_char_buffer, target_char_buffer].concat();
    let result = common::sha1_hash(buffer);
    return Ok(common::hash_to_uuid(result, 5));
}
